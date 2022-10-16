use bwavfile::WaveReader;
use egui::epaint::{Color32, Stroke};
use rfd::FileHandle;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MetadazioApp {
    #[serde(skip)]
    filename: String,
    #[serde(skip)]
    filepath: std::path::PathBuf,
    #[serde(skip)]
    parsed_metadata: String,

    #[serde(skip)]
    filehandle_channel: (
        std::sync::mpsc::Sender<FileHandle>,
        std::sync::mpsc::Receiver<FileHandle>,
    ),
}

impl Default for MetadazioApp {
    fn default() -> Self {
        Self {
            filename: String::new(),
            filepath: std::path::PathBuf::new(),
            parsed_metadata: String::new(),
            filehandle_channel: std::sync::mpsc::channel(),
        }
    }
}

impl MetadazioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn open_file_dialog(&mut self) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("Wav files", &["wav", "bwav"])
            .set_directory("/")
            .pick_file();

        let tx_f = self.filehandle_channel.0.clone();

        execute(async move {
            let file = task.await;

            if let Some(file) = file {
                tx_f.send(file);
            }
        });
    }

    fn read_file(&mut self) {}

    fn render_sidepanel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .frame(
                egui::Frame::none()
                    .stroke(Stroke::new(3.0, Color32::DARK_GREEN))
                    .inner_margin(egui::style::Margin::symmetric(20.0, 20.0)), // .fill(Color32::DARK_GRAY),
            )
            .show(ctx, |ui| {
                ui.set_width(200.0);
                ui.spacing_mut().item_spacing = egui::Vec2::new(15.0, 15.0);
                ui.heading("Input");
                ui.label("Filename:");
                ui.text_edit_singleline(&mut self.filename);
                if ui.button("Upload file").clicked() {
                    self.open_file_dialog();
                }
                ui.add_space(15.0);
            });
    }

    fn render_centralpanel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .stroke(Stroke::new(3.0, Color32::DARK_GREEN))
                    .inner_margin(egui::style::Margin::symmetric(20.0, 20.0)), // .fill(Color32::DARK_GRAY),
            )
            .show(ctx, |ui| {
                ui.heading("Output");
                ui.label(self.parsed_metadata.clone());
            });
    }
}

impl eframe::App for MetadazioApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        loop {
            match self.filehandle_channel.1.try_recv() {
                Ok(mes) => {
                    self.filename = mes.file_name();
                    // let file = mes.inner();
                    // self.filepath = file
                }
                Err(_) => {
                    break;
                }
            }
        }

        self.render_sidepanel(ctx);

        self.render_centralpanel(ctx);
    }
}

use std::future::Future;

fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
