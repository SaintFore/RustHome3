use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::path::Path;
use eframe::egui;

#[derive(Debug)]
struct Record {
    record_type: u8,
    id: u32,
    name: String,
}

fn parse_binary_file(file_path: &Path) -> io::Result<Vec<Record>> {
  let mut file = File::open(file_path)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  let mut records = Vec::new();
  let mut cursor = 0;


  while cursor < buffer.len() {
      if cursor + 6 > buffer.len() {
          break; // 防止越界
      }

      let record_type = buffer[cursor];
      let id = u32::from_le_bytes(buffer[cursor + 1..cursor + 5].try_into().unwrap());
      let name_length = buffer[cursor + 5] as usize;

      if cursor + 6 + name_length > buffer.len() {
          break; // 防止越界
      }

      let name = String::from_utf8_lossy(&buffer[cursor + 6..cursor + 6 + name_length]).to_string();
      records.push(Record {
          record_type,
          id,
          name,
      });

      cursor += 6 + name_length; // 更新游标位置
  }

  Ok(records)
}



struct MyApp {
  records: Vec<Record>,
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("Parsed Records");

          for record in &self.records {
              ui.group(|ui| {
                  ui.label(format!("Type: {}", record.record_type));
                  ui.label(format!("ID: {}", record.id));
                  ui.label(format!("Name: {}", record.name));
              });
          }
      });
  }
}


fn main() -> Result<(), eframe::Error> {
 // let file_path = "data.bin";
 let file_path = PathBuf::from("src/data.bin");


  let records = match parse_binary_file(file_path.as_path()) {
      Ok(records) => records,
      Err(e) => {
          eprintln!("Failed to read binary file: {}", e);
          return Ok(());
      }
  };

  // 打印解析结果到控制台
  for record in &records {
      println!("{:?}", record);
  }

  // 启动 egui
  eframe::run_native(
      "Binary File Viewer",
      eframe::NativeOptions::default(),
      Box::new(|_cc| Ok(Box::new(MyApp { records }))),
  )
}
