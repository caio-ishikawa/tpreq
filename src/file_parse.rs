use std::fs;

pub struct FileInstrusctions {
    pub method: Option<String>,
    pub url: Option<String>,
    pub header_name: Option<String>,
    pub header_value: Option<String>,
    pub body: Option<String>
}

pub fn read_file(file_name: String) -> String {
    let content = fs::read_to_string(file_name).unwrap();
    return content;
}

pub fn parse_file(file_content: String) -> FileInstrusctions {
    let clean_input = file_content.replace("\r", "");
    let commands: Vec<&str> = clean_input.split("\n").collect();
    let mut instructions = FileInstrusctions{method: None, url: None, header_name: None, header_value: None, body: None};

    for line in commands {
        let split_line: Vec<&str> = line.split(": ").collect();

        if line.contains("method:") {
            instructions.method = Some(split_line[1].to_owned());
        } else if line.contains("url:") {
            instructions.url = Some(split_line[1].to_owned())
        } else if line.contains("header") {
            let split_header: Vec<&str> = line.split(" ").collect();
            instructions.header_name = Some(split_header[1].to_owned());
            instructions.header_value = Some(split_header[2].to_owned());
        }
       //println!("{:?}", line);
    }
    return instructions;
} 