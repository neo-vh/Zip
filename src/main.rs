use eframe::egui;

struct Info { // struct containing information
    name: String,
    age: i32,
}

struct App {
    info: Info
}

// runs everytime the gui needs to update
impl eframe::App for App {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {

        egui::CentralPanel::default().show(ui, |ui| {
            self.info.ui(ui); // make a central panel that contains the contents described in info
        });

    }
}

impl Info { // what the gui needs to display
    fn ui(&mut self, ui: &mut egui::Ui) {
        // Saves us from writing `&mut self.name` etc
        let Self { name, age } = self;

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), // center contents inside block
    |ui| {

        ui.heading("Example"); // heading NOT window title

        ui.add(
            egui::TextEdit::singleline(name)
                .hint_text("Name"),
        );

        ui.add(
            egui::DragValue::new(age)
                .range(0..=120)
                .suffix(" years"),
        );
        if ui.button("Increment").clicked() { // atoms????
            *age += 1;
        }

        ui.label(format!("{name} is {age}"));

    });

    }
}

fn main() -> eframe::Result {

    // Configure the window.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]) // window size?
            .with_title("My First GUI"), // window title

        ..Default::default()
    };

    // Launch the application.
    eframe::run_native(
        "My First GUI",
        options,

        // Create an instance of App.
        Box::new(|cc| { // || shows lambda, cc is configuration options, adding _cc means not using cc inside the || function

            cc.egui_ctx.set_visuals(egui::Visuals::dark()); // set the window to dark mode

            Ok(Box::new(App {
                info: Info {
                    name: String::new(),
                    age: 18,
                },
            }))
        }),
    )
}