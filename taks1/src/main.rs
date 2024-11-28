use eframe::egui;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

struct MyApp {
    person: Option<Person>,
    json: String,
    name: String,
    age: String,
    email: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            person: None,
            json: String::new(),
            name: String::new(),
            age: String::new(),
            email: String::new(),
        }
    }
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Person Information");

            if let Some(person) = &self.person {
                ui.label(format!("Name: {}", person.name));
                ui.label(format!("Age: {}", person.age));
                ui.label(format!("Email: {}", person.email));
            }

            ui.separator();
            ui.horizontal(|ui|{
                ui.label("name:");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui|{
                ui.label("age:");
                ui.text_edit_singleline(&mut self.age);
            });
            ui.horizontal(|ui|{
                ui.label("email:");
                ui.text_edit_singleline(&mut self.email);
            });

            if ui.button("Edit").clicked() {
                if let Ok(age) = self.age.parse::<u8>() {
                    self.person = Some(Person {
                        name: self.name.clone(),
                        age,
                        email: self.email.clone(),
                    });
                }
            }

            if ui.button("Serialize").clicked() {
                if let Some(person) = &self.person {
                    self.json = serde_json::to_string(&person).unwrap();
                }
            }

            if ui.button("Deserialize").clicked() {
                if let Ok(person) = serde_json::from_str::<Person>(&self.json) {
                    self.person = Some(person);
                }
            }

            if ui.button("Reset").clicked() {
                self.person = None;
                self.json.clear();
                self.name.clear();
                self.age.clear();
                self.email.clear();
            }

            ui.separator();

            ui.label("JSON:");
            ui.text_edit_multiline(&mut self.json);
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Person Info App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    ).expect("Failed to run eframe");
}