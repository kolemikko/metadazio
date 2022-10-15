use egui::epaint::{Color32, Stroke};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MetadazioApp {
    #[serde(skip)]
    filename: String,
    #[serde(skip)]
    parsed_metadata: String,
}

impl Default for MetadazioApp {
    fn default() -> Self {
        Self {
            filename: String::new(),
            parsed_metadata: String::new(),
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
                if ui.button("Choose a file").clicked() {}
                ui.add_space(15.0);
                if ui
                    .add_sized([120.0, 60.0], egui::Button::new("Parse metadata"))
                    .clicked()
                {}
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
            });
    }
}

impl eframe::App for MetadazioApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_sidepanel(ctx);

        self.render_centralpanel(ctx);
    }
}
