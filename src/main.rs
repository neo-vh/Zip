mod file_management;
use std::{fs, path::PathBuf};
use eframe::{egui};
use egui::FontFamily::Name;
use tokio::net::windows::named_pipe;

struct Info {
    memo_text: String,
    heading: Option<String>,
}

struct App {
    info: Info,
    current_path: Option<PathBuf> // store the currently open file (none means no file is currently open)
}

impl eframe::App for App {

    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {

        if ui.input_mut(|i| i.consume_key(
            egui::Modifiers::CTRL,
            egui::Key::S,
        )) {
            self.save_file();
        }

        egui::Panel::top("menu_bar").show(ui, |ui| { // make the panel containing the menu bar
            egui::MenuBar::new().ui(ui, |ui| { // make the menubar

                ui.menu_button("File", |ui| { // create main button categories

                    if ui.button("Close").clicked() { 
                        self.info.memo_text.clear(); // clear memo text
                        self.current_path = None;
                        self.info.heading = None;
                        ui.close(); // close menu button drop down
                    }

                    if ui.button("Open").clicked() {

                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Text files", &["txt", "rs"]) // change later on
                            .pick_file() {

                                match std::fs::read_to_string(&path) { // check path (borrowed because ownership should go to currentPath)
                                    Ok(contents) => {
                                        self.info.memo_text = contents; // set the memo contetns to the chosen file
                                        self.info.heading = path
                                            .file_name()
                                            .map(|name| name.to_string_lossy().into_owned()); // set heading to the currently open filename (gets the filename and converts from an OsStr to a &String to an owned string)
                                        self.current_path = Some(path); // store the current path and pass ownership
                                    }

                                    Err(error) => {
                                        eprintln!("Failed to open file {:?}", error); // couldnt open file
                                    }
                                }

                            }
                        
                        ui.close();

                    }

                    if ui.button("Save").clicked() {

                        self.save_file();
                        ui.close();

                    }

                })

            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            self.info.ui(ui);
        });
    }
}

impl App {
    fn save_file(&self) {
        if let Some(path) = &self.current_path {
            match fs::write(path, &self.info.memo_text) {
                Ok(()) => {
                    println!("Saved {:?}", path);
                }
                Err(error) => {
                    eprintln!("Failed to save {:?}: {:?}", path, error);
                }
            }
        }
    }
}

impl Info {

    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { memo_text, heading } = self; // gives mutable refrences to memo text and heading

        ui.vertical_centered(|ui| {

            match heading {

                Some(name) => {
                    ui.heading(name.as_str())
                }
                None => {
                    ui.heading("Untitled")
                }

            }

        });

        egui::ScrollArea::both()
            .show(ui, |ui| {
                
                ui.horizontal_top(|ui| {

                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        let line_count = memo_text.lines().count().max(1);

                        for i in 1..=line_count {

                            ui.label(i.to_string());

                        }
                    });

                    ui.add(
                        egui::TextEdit::multiline(memo_text) // use ::show eventually for galley
                            .font(egui::TextStyle::Monospace)
                            //.desired_width(f32::INFINITY) // set the desired width to undefined (any width is fine)
                            .min_size(ui.available_size())
                    );

                });

            });

    }
}

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Zip"),

        ..Default::default()
    };

    eframe::run_native(
        "My First GUI",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Ok(Box::new(App {
                info: Info {
                    memo_text: String::new(),
                    heading: None
                },

                current_path: None
            }))
        }),
    )
}
