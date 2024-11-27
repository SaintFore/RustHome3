use eframe::{egui, App, CreationContext};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use rfd;  // 导入 rfd
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct WordCount {
    word: String,
    count: u32,
}

struct MyApp {
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
    word_counts: Vec<WordCount>,
    show_output: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file: None,
            output_file: None,
            word_counts: Vec::new(),
            show_output: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("单词计数应用程序");

            if ui.button("选择输入文件").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_file = Some(path);
                }
            }

            if let Some(ref path) = self.input_file {
                ui.label(format!("选择的文件: {}", path.to_string_lossy()));
                if ui.button("统计单词").clicked() {
                    match fs::read_to_string(path) {
                        Ok(contents) => {
                            self.word_counts = self.count_words(&contents);
                            self.show_output = true;
                        },
                        Err(e) => {
                            ui.label(format!("读取输入文件失败: {}", e));
                        }
                    }
                }
            }

            if self.show_output {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for wc in &self.word_counts {
                        ui.label(format!("{}: {}", wc.word, wc.count));
                    }
                });

                if ui.button("保存输出到文件").clicked() {
                    if let Some(path) = rfd::FileDialog::new().set_directory(".").save_file() {
                        self.output_file = Some(path);
                        if let Some(output_path) = &self.output_file {
                            match serde_json::to_string_pretty(&self.word_counts) {
                                Ok(json_data) => {
                                    if fs::write(output_path, json_data).is_ok() {
                                        ui.label("输出已成功保存!");
                                    } else {
                                        ui.label("保存输出失败，请检查文件权限或磁盘空间。");
                                    }
                                },
                                Err(e) => {
                                    ui.label(format!("序列化数据失败: {}", e));
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

impl MyApp {
    fn count_words(&self, contents: &str) -> Vec<WordCount> {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for word in contents.split_whitespace() {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        counts.into_iter()
            .map(|(word, count)| WordCount { word, count })
            .collect()
    }
}

fn main() {
    let app = MyApp::default();
    let options = eframe::NativeOptions::default();
    // 注意这里移除了 .unwrap()
    eframe::run_native(
        "单词计数应用程序",
        options,
        Box::new(|cc: &CreationContext| {
            // 设置自定义字体
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "my_font".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("my_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            Box::new(app) as Box<dyn eframe::App>
        }),
    );
}