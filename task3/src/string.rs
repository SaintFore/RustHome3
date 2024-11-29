use std::str::FromStr;

#[derive(Debug)]
struct Record {
    record_type: u32,
    id: u32,
    name: String,
}

impl FromStr for Record {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(|c| c == '(' || c == ')'); // 去掉括号
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 3 {
            return Err("Invalid record format");
        }

        let record_type = parts.trim().parse::<u32>().map_err(|_| "Invalid record type")?;
        let id = parts.trim().parse::<u32>().map_err(|_| "Invalid ID")?;
        let name = parts.trim().trim_matches('"').to_string(); // 去掉引号

        Ok(Record {
            record_type,
            id,
            name,
        })
    }
}

fn main() {
    let data = vec![
        "(1, 1001, \"Alice\")",
        "(2, 1002, \"Bob\")",
        "(3, 1003, \"Charlie\")",
    ];

    let mut records = Vec::new();

    for entry in data {
        match Record::from_str(entry) {
            Ok(record) => records.push(record),
            Err(e) => eprintln!("Error parsing record: {}", e),
        }
    }

    for record in records {
        println!("{:?}", record);
    }
}
