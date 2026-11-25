mod file_management;
use eframe::{egui};

struct Info {
    memo_text: String,
}

struct App {
    info: Info,
}

impl eframe::App for App {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.info.ui(ui);
        });
    }
}

impl Info {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { memo_text } = self;

        ui.vertical_centered(|ui| {

            ui.heading("Code Editor")

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
                            .desired_width(f32::INFINITY) // set the desired width to undefined (any width is fine)
                    );

                });

            });

    }
}

fn main() -> eframe::Result {
    let path = String::from(
        "C:\\Users\\USER\\Documents\\Zip\\src\\test.txt"
    );

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
                    memo_text: file_management::get_contents(&path)
                        .expect("Unable to read file"),
                },
            }))
        }),
    )
}
