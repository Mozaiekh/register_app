/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RegisterApp {
    // Example stuff:
    first_name_input: String,
    last_name_input: String,
    company_input: String,
    contact_input: String,
    open_entries: Vec<EntryData>,
}

pub struct EntryData {
    first_name: String,
    last_name: String,
    company: String,
    contact: String,
}



impl Default for RegisterApp {
    fn default() -> Self {
        let iter = (0..10).map(|i| EntryData {
            first_name: format!("First Name {}", i),
            last_name: format!("Last Name {}", i),
            company: format!("Company {}", i),
            contact: format!("Contact {}", i),
        });
        Self {
            first_name_input: "".to_owned(),
            last_name_input: "".to_owned(),
            company_input: "".to_owned(),
            contact_input: "".to_owned(),
            open_entries: Vec::from_iter(iter),
        }
    }
}

impl RegisterApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for RegisterApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { first_name_input, last_name_input, company_input, contact_input, open_entries} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Check-In");

            ui.horizontal(|ui| {
                ui.label("Vorname: ");
                ui.text_edit_singleline(first_name_input);
            });
            ui.horizontal(|ui| {
                ui.label("Nachname: ");
                ui.text_edit_singleline(last_name_input);
            });
            ui.horizontal(|ui| {
                ui.label("Firma: ");
                ui.text_edit_singleline(company_input);
            });
            ui.horizontal(|ui| {
                ui.label("Ansprechpartner: ");
                ui.text_edit_singleline(contact_input);
            });
            if ui.button("Check-In").clicked() {

            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Check-Out");

            for i in &self.open_entries {
                ui.horizontal(|ui| {
                    ui.label(&i.first_name);
                    ui.label(&i.last_name);
                    ui.label(&i.company);
                    ui.label(&i.contact);
                });
            }

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
