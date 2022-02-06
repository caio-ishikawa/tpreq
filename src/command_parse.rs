use std::env;
use core::option::Option;

pub struct Instructions {
    pub method: Option<String>,
    pub url: Option<String>,
    pub header_name: Option<String>,
    pub header_value: Option<String>,
    pub body: Option<String>
}

pub fn read_flags() -> Instructions {
    let args: Vec<String> = env::args().collect();
    let mut instructions = Instructions{method: None, url: None, header_name: None, header_value: None, body: None};

    for (idx, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "/m" => instructions.method = Some(args[idx + 1].to_owned()),
            "/u" => instructions.url = Some(args[idx + 1].to_owned()),
            "/h" => {instructions.header_name = Some(args[idx + 1].to_owned()); instructions.header_value  = Some(args[idx + 2].to_owned())},
            "/b" => instructions.body = Some(args[idx + 1].to_owned()),
            _ => continue
        };
    }

    return instructions 
}

// -m for method
// -u for url
// -h for header
// -b for body
