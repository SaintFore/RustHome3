use std::fs::File;
use std::io::{self, Read};
use std::path::{Path};
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
    id: u32,
    name: String,
}

impl Record {
    // 从字节数组中解析记录
    fn from_bytes(bytes: &[u8]) -> Result<Self, MyError> {
        if bytes.len() < 6 {
            return Err(MyError::InvalidFormat); // 确保数据格式正确
        }

        let id = u32::from_le_bytes(bytes[1..5].try_into().unwrap()); // 接下来4个字节是 ID
        let name_length = bytes[5] as usize; // 第6个字节是名字的长度

        if bytes.len() < 6 + name_length {
            return Err(MyError::InvalidFormat); // 数据不足
        }

        // 解析名字字段
        let name = String::from_utf8_lossy(&bytes[6..6 + name_length]).to_string();

        Ok(Record {
            id,
            name,
        })
    }
}

// 解析二进制文件
fn parse_binary_file(file_path: &Path) -> io::Result<Vec<Record>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut records = Vec::new();
    let mut cursor = 0;





    while cursor < buffer.len() {
        // 确保读取的记录长度合理
        if cursor + 6 > buffer.len() {
            break; // 至少要有6个字节（id 和 name_length）
        }

        let id = u32::from_le_bytes(buffer[cursor + 1..cursor + 5].try_into().unwrap()); // 接下来4个字节是 ID
        let name_length = buffer[cursor + 5] as usize; // 第6个字节是名字的长度

        if cursor + 6 + name_length > buffer.len() {
            break; // 防止越界，确保能读取整个名字
        }

        // 解析名字字段
        let name = String::from_utf8_lossy(&buffer[cursor + 6..cursor + 6 + name_length]).to_string();

        // 将解析到的记录添加到记录列表中
        records.push(Record {
            id,
            name,
        });

        // 更新游标位置，跳过当前记录
        cursor += 6 + name_length; // 6个字节用于ID和名字长度，剩余是名字
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
