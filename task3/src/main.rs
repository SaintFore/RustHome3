use std::fs::File;
use std::io::{self, Read};
use std::path::{Path};
use thiserror::Error;
use eframe::{self, egui::{self, Context, CentralPanel, ScrollArea}};
use eframe::NativeOptions; // 确保导入 NativeOptions
use std::str::FromStr;

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
    id: u32,
    name: String,
}

// 实现 FromStr trait，用于从字符串中解析 Record
impl FromStr for Record {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 去除两边的括号
        let s = s.trim_matches(|c| c == '(' || c == ')');

        // 分割字段
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 3 {
            return Err("Invalid record format");
        }

        // 解析 record_type 和 id，返回错误时提示失败
        let record_type = parts[0].trim().parse::<u32>().map_err(|_| "Invalid record type")?;
        let id = parts[1].trim().parse::<u32>().map_err(|_| "Invalid ID")?;

        // 解析 name，去掉引号
        let name = parts[2].trim().trim_matches('"').to_string();

        Ok(Record {
            id,
            name,
        })
    }
}

// 解析二进制文件函数
fn parse_binary_file(file_path: &Path) -> Result<Vec<Record>, MyError> {
    let mut file = File::open(file_path).map_err(MyError::IoError)?; // 打开文件
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(MyError::IoError)?; // 读取文件内容到 buffer

    let mut records = Vec::new();
    let mut cursor = 0;

    while cursor < buffer.len() {
        // 跳过空格和其他非必要字符
        if buffer[cursor] == b' ' || buffer[cursor] == b'\n' || buffer[cursor] == b'\r' {
            cursor += 1;
            continue;
        }

        // 找到记录的开始位置
        let record_start = cursor;

        // 查找记录的结束位置（右括号）并提取记录
        while cursor < buffer.len() && buffer[cursor] != b')' {
            cursor += 1;
        }
        let record_end = cursor;

        // 将字节数据转换为字符串
        let record_str = String::from_utf8_lossy(&buffer[record_start..=record_end]);

        // 使用 FromStr 解析 Record
        match Record::from_str(&record_str) {
            Ok(record) => records.push(record),
            Err(_) => return Err(MyError::InvalidFormat), // 如果解析失败，返回格式错误
        }

        // 移动 cursor，跳过右括号
        cursor += 1;

        // 跳过下一条数据的左括号
        cursor += 1;
    }

    Ok(records)
}

// MyApp 结构体用于封装解析的数据
struct MyApp {
    file_contents: Option<Vec<Record>>,
    error_message: Option<String>,
}

impl MyApp {
    fn new() -> Self {
        MyApp {
            file_contents: None,
            error_message: None,
        }
    }

    fn parse_files(&mut self) {
        let file_path = "data.bin"; // 直接使用默认路径
        match parse_binary_file(Path::new(file_path)) {
            Ok(records) => self.file_contents = Some(records),
            Err(e) => {
                self.error_message = Some(format!("Error processing file {}: {}", file_path, e));
            }
        }
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
                        ui.group(|ui| {
                            ui.label(format!("Record #{}", i + 1));
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
fn main() -> Result<(), eframe::Error> {
    // 创建并运行 eframe 应用
    eframe::run_native(
        "Binary File Viewer",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
