use std::env;
use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;
use eframe::{self, egui::{self, Context, CentralPanel, ScrollArea}};
use eframe::NativeOptions; // 确保导入 NativeOptions

// 定义错误类型
#[derive(Debug, Error)]
pub enum MyError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    #[error("Invalid file format")]
    InvalidFormat,
}

// 定义记录结构体
#[derive(Debug)]
struct Record {
    record_type: i32,
    id: i32,
    name: String,
}

impl Record {
    fn from_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        if bytes.len() < 28 {
            return Err(MyError::InvalidFormat);
        }

        let record_type = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let id = i32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let name = String::from_utf8_lossy(&bytes[8..28]).trim_end().to_string();

        if name.is_empty() {
            return Err(MyError::InvalidFormat);
        }

        Ok(Record {
            record_type,
            id,
            name,
        })
    }
}

// 打开文件并处理错误
fn open_file(file_path: &str) -> Result<File, MyError> {
    File::open(file_path).map_err(|_| MyError::FileNotFound(file_path.to_string()))
}

// 处理单个文件并返回记录数据
fn process_file(file_path: &str) -> Result<Vec<Record>, MyError> {
    let mut file = open_file(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let record_size = 28;
    let num_records = buffer.len() / record_size;

    let mut records = Vec::new();
    for i in 0..num_records {
        let start = i * record_size;
        let end = start + record_size;
        let record_bytes = &buffer[start..end];

        // 解析记录
        let record = Record::from_bytes(record_bytes)?;
        records.push(record);
    }

    Ok(records)
}

// 从命令行解析文件路径
fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    // 返回文件路径参数，如果没有传入文件路径则使用默认值
    if args.len() > 1 {
        args[1..].to_vec() // 返回命令行参数中的所有路径
    } else {
        vec!["data.bin".to_string()] // 默认文件
    }
}

// 创建一个 egui 窗口并显示解析结果
struct MyApp {
    file_paths: Vec<String>,
    file_contents: Option<Vec<Record>>,
    error_message: Option<String>,
}

impl MyApp {
    fn new(file_paths: Vec<String>) -> Self {
        MyApp {
            file_paths,
            file_contents: None,
            error_message: None,
        }
    }

    fn parse_files(&mut self) {
        let mut all_records = Vec::new();

        for file_path in &self.file_paths {
            match process_file(file_path) {
                Ok(records) => all_records.extend(records),
                Err(e) => {
                    self.error_message = Some(format!("Error processing file {}: {}", file_path, e));
                    return;
                }
            }
        }

        self.file_contents = Some(all_records);
    }
}

// 实现 eframe 应用
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Binary File Parser");

            // 显示错误信息（如果有的话）
            if let Some(ref message) = self.error_message {
                ui.colored_label(egui::Color32::RED, message);
                return;
            }

            // 显示文件内容
            if let Some(ref records) = self.file_contents {
                ScrollArea::vertical().show(ui, |ui| {
                    for (i, record) in records.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Record #{}", i + 1));
                            ui.label(format!("Type: {}", record.record_type));
                            ui.label(format!("ID: {}", record.id));
                            ui.label(format!("Name: {}", record.name));
                        });
                    }
                });
            } else {
                // 让用户点击按钮加载文件
                if ui.button("Load Files").clicked() {
                    self.parse_files();
                }
            }
        });
    }
}

// 主函数
fn main() -> Result<(), MyError> {
    let file_paths = parse_args();

    // 创建并运行 eframe 应用
    eframe::run_native(
        "Binary File Parser",
        NativeOptions {
            // 移除不再支持的字段
            ..Default::default() // 仅使用默认选项
        },
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp::new(file_paths)))),
    );

    Ok(())
}
