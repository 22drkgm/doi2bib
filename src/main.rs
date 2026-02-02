use eframe::egui;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() -> eframe::Result<()> {
    // Setup the GUI options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0]), // Set initial window size
        ..Default::default()
    };
    
    // Launch the app
    eframe::run_native(
        "DOI to BibTeX Converter",
        options,
        Box::new(|_cc| Box::new(DoiApp::default())),
    )
}

// -- The Application State --
struct DoiApp {
    csv_path: Option<String>,
    logs: String,
    is_processing: bool,
    processed_count: usize,
    total_count: usize,
    
    // Channel to receive updates from the background worker
    status_receiver: Receiver<WorkerMessage>,
    // Channel sender to pass to the worker (we keep a clone only when spawning)
    status_sender: Sender<WorkerMessage>,
}

// Messages sent from the background worker to the GUI
enum WorkerMessage {
    Log(String),
    Progress(usize), // Current count
    Total(usize),    // Total rows found
    Finished,
}

impl Default for DoiApp {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            csv_path: None,
            logs: String::new(),
            is_processing: false,
            processed_count: 0,
            total_count: 0,
            status_receiver: rx,
            status_sender: tx,
        }
    }
}

impl eframe::App for DoiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Check for messages from the background thread (non-blocking)
        while let Ok(msg) = self.status_receiver.try_recv() {
            match msg {
                WorkerMessage::Log(text) => {
                    self.logs.push_str(&text);
                    self.logs.push('\n');
                }
                WorkerMessage::Progress(count) => self.processed_count = count,
                WorkerMessage::Total(total) => self.total_count = total,
                WorkerMessage::Finished => {
                    self.is_processing = false;
                    self.logs.push_str("\n--- DONE! Check references.bib ---\n");
                }
            }
        }

        // 2. Draw the User Interface
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DOI to BibTeX Converter");
            ui.add_space(10.0);

            // -- File Selection Area --
            ui.horizontal(|ui| {
                if ui.button("📂 Select CSV File").clicked() && !self.is_processing {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CSV", &["csv"])
                        .pick_file() 
                    {
                        self.csv_path = Some(path.display().to_string());
                        self.logs.push_str(&format!("Selected: {}\n", path.display()));
                    }
                }
                
                if let Some(path) = &self.csv_path {
                    ui.label(path);
                } else {
                    ui.label("No file selected");
                }
            });

            ui.add_space(10.0);

            // -- Action Button --
            if ui.add_enabled(
                !self.is_processing && self.csv_path.is_some(), 
                egui::Button::new("🚀 Start Conversion").min_size(egui::vec2(120.0, 30.0))
            ).clicked() {
                // START THE PROCESS
                if let Some(path) = self.csv_path.clone() {
                    self.is_processing = true;
                    self.logs.clear();
                    self.processed_count = 0;
                    self.total_count = 0;
                    
                    let sender = self.status_sender.clone();
                    
                    // Spawn a thread so the GUI doesn't freeze
                    thread::spawn(move || {
                        run_conversion(path, sender);
                    });
                }
            }

            ui.add_space(10.0);

            // -- Progress Bar --
            if self.total_count > 0 {
                let progress = self.processed_count as f32 / self.total_count as f32;
                ui.add(egui::ProgressBar::new(progress)
                    .text(format!("{} / {}", self.processed_count, self.total_count)));
            }

            ui.separator();

            // -- Logs Area (Scrollable) --
            ui.label("Logs:");
            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.logs)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(15)
                        .lock_focus(true) // make it read-only-ish
                );
            });
        });
        
        // Force the GUI to refresh to show updates immediately
        if self.is_processing {
            ctx.request_repaint();
        }
    }
}

// -- The Background Logic (Runs on a separate thread) --
fn run_conversion(path: String, tx: Sender<WorkerMessage>) {
    // Create a runtime for async requests
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let _ = tx.send(WorkerMessage::Log(format!("Reading file: {}", path)));

        // Open File
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                let _ = tx.send(WorkerMessage::Log(format!("Error opening file: {}", e)));
                let _ = tx.send(WorkerMessage::Finished);
                return;
            }
        };

        // Read CSV to memory first to count total rows
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
        let records: Vec<Result<csv::StringRecord, csv::Error>> = rdr.records().collect();
        
        let total = records.len();
        let _ = tx.send(WorkerMessage::Total(total));

        let client = reqwest::Client::new();
        let mut bibtex_entries = Vec::new();
        let mut count = 0;

        for result in records {
            count += 1;
            let _ = tx.send(WorkerMessage::Progress(count));

            if let Ok(record) = result {
                if let Some(doi_raw) = record.get(0) {
                    let doi = doi_raw.trim();
                    if doi.is_empty() || doi.eq_ignore_ascii_case("DOI") {
                        continue;
                    }

                    // Fetch
                    let url = format!("https://doi.org/{}", doi);
                    let res = client.get(&url)
                        .header("Accept", "application/x-bibtex; charset=utf-8")
                        .send()
                        .await;

                    match res {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                if let Ok(text) = resp.text().await {
                                    bibtex_entries.push(text);
                                    let _ = tx.send(WorkerMessage::Log(format!("✔ OK: {}", doi)));
                                }
                            } else {
                                let _ = tx.send(WorkerMessage::Log(format!("✖ Failed: {} (Status {})", doi, resp.status())));
                            }
                        }
                        Err(_) => {
                            let _ = tx.send(WorkerMessage::Log(format!("✖ Error: {}", doi)));
                        }
                    }
                }
            }
        }

        // Save file
        if let Ok(mut output) = File::create("references.bib") {
            for entry in bibtex_entries {
                let _ = writeln!(output, "{}\n", entry.trim());
            }
            let _ = tx.send(WorkerMessage::Log("Saved to 'references.bib'".to_string()));
        }

        let _ = tx.send(WorkerMessage::Finished);
    });
}
