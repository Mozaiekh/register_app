use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::process;
use std::rc::Rc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RegisterApp {
    // Example stuff:
    first_name_input: String,
    last_name_input: String,
    company_input: String,
    contact_input: String,
    // entry_list: Option<Vec<EntryData>>,
}

#[derive(Clone)]
pub struct EntryData {
    first_name: String,
    last_name: String,
    company: String,
    contact: String,
    check_in: String,
    check_out: String,
}



impl Default for RegisterApp {
    fn default() -> Self {
        Self {
            first_name_input: "".to_owned(),
            last_name_input: "".to_owned(),
            company_input: "".to_owned(),
            contact_input: "".to_owned(),
            // entry_list: Vec::from_iter(iter),
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
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "din1451".to_owned(),
            egui::FontData::from_static(include_bytes!("../alte-din-1451-mittelschrift.regular.ttf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "din1451".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("din1451".to_owned());

        // Tell egui to use these fonts:
        cc.egui_ctx.set_fonts(fonts);

        

        Default::default()
    }

    fn new_entry(Self { first_name_input, last_name_input, company_input, contact_input }: Self) -> Result<(), Box<dyn Error>> {
        
        // create a variable wich stores the current date and time
        // let now = chrono::Local::now();
        let now = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

        let file = OpenOptions::new().write(true).create(true).append(true).open("data.csv").unwrap();
        let mut wtr = csv::Writer::from_writer(file);

        wtr.write_record(&[first_name_input, last_name_input, company_input, contact_input, now, "".to_string()])?;
    
        wtr.flush()?;
        Ok(())
    }

    // read the csv file and return a vector of EntryData
    fn read_entries() -> Result<Vec<EntryData>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path("data.csv")?;
        let mut entries = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let entry = EntryData {

                // HANDLE ERROR IF RECORD IS EMPTY !!!

                first_name: record[0].to_owned(),
                last_name: record[1].to_owned(),
                company: record[2].to_owned(),
                contact: record[3].to_owned(),
                check_in: record[4].to_owned(),
                check_out: record[5].to_owned(),
            };
            entries.push(entry);
        }
        tracing::debug!("Read {} entries.", entries.len());
        Ok(entries)
    }

    /* fn check_out(entry_list: &Vec<EntryData>) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new().write(true).open("data.csv").unwrap();
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(&["Vorname", "Nachname", "Firma", "Ansprechpartner", "Check-In", "Check-Out"])?;
        for entry in entry_list {
            wtr.write_record(&[&entry.first_name, &entry.last_name, &entry.company, &entry.contact, &entry.check_in, &entry.check_out])?;
        }

        wtr.flush()?;
        Ok(())
    } */


    
}

impl eframe::App for RegisterApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { first_name_input, last_name_input, company_input, contact_input, /* entry_list */} = self;

        const INPUT_X: f32 = 200.0;
        const INPUT_Y: f32 = 20.0;
        const CHECK_IN_BOX_WIDTH: f32 = 400.0;
        const CHECK_OUT_BOX_WIDTH: f32 = 400.0;

        let mut entry_list = Self::read_entries().unwrap();

        let central_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin { left: 20., right: 20., top: 20., bottom: 20. },
            outer_margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
            rounding: egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: egui::Color32::BLACK },
            fill: egui::Color32::from_rgb(14, 17, 26),
            stroke: egui::Stroke::new(0.0, egui::Color32::from_rgb(255, 128, 120)),
        };

        fn check_out(entry_list: &Vec<EntryData>) -> Result<(), Box<dyn Error>> {
            let file = OpenOptions::new().write(true).open("data.csv").unwrap();
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(&["Vorname", "Nachname", "Firma", "Ansprechpartner", "Check-In", "Check-Out"])?;
            for entry in entry_list {
                wtr.write_record(&[&entry.first_name, &entry.last_name, &entry.company, &entry.contact, &entry.check_in, &entry.check_out])?;
            }
    
            wtr.flush()?;
            Ok(())
        }

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /* egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        }); */

        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {


            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(255, 128, 120));
                    ui.style_mut().spacing.item_spacing = egui::vec2(5.0, 5.0);
                    ui.set_min_width(CHECK_IN_BOX_WIDTH);
                    ui.set_max_width(CHECK_IN_BOX_WIDTH);
                    ui.heading("Check-In");
                    ui.horizontal(|ui| {
                        ui.label("Vorname: ");
                        ui.add_sized([INPUT_X, INPUT_Y], egui::TextEdit::singleline(first_name_input));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Nachname: ");
                        ui.add_sized([INPUT_X, INPUT_Y], egui::TextEdit::singleline(last_name_input));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Firma: ");
                        ui.add_sized([INPUT_X, INPUT_Y], egui::TextEdit::singleline(company_input));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Ansprechpartner: ");
                        ui.add_sized([INPUT_X, INPUT_Y], egui::TextEdit::singleline(contact_input));
                    });
                    if ui.button("Check-In").clicked() {

                        if let Err(e) = RegisterApp::new_entry(Self { first_name_input: first_name_input.to_owned(), last_name_input: last_name_input.to_owned(), company_input: company_input.to_owned(), contact_input: contact_input.to_owned() }) {
                            eprintln!("error running example: {}", e);
                            process::exit(1);
                        }

                        first_name_input.clear();
                        last_name_input.clear();
                        company_input.clear();
                        contact_input.clear();
                    }
                });
                ui.vertical(|ui| {
                    ui.set_min_width(CHECK_OUT_BOX_WIDTH);
                    ui.set_max_width(CHECK_OUT_BOX_WIDTH);
                    ui.heading("Check-Out");
                    let cl_entry_list = entry_list.clone();
                    for (i, entry) in cl_entry_list.iter().enumerate() {
                        if ui.button(format!{ "{} {} {}", entry.first_name, entry.last_name, entry.company }).clicked() {
                            entry_list[i].check_out = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

                            if let Err(e) = /* RegisterApp:: */check_out(&entry_list) {
                                eprintln!("error running example: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                });
            });

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
