mod command_parse;
use reqwest::Response;
mod file_parse;
mod requests {
    pub mod get_req;
}
mod responses {
    pub mod output_response;
}

#[tokio::main]
async fn main() {
    let comm_values: command_parse::Instructions = command_parse::read_flags();
    //let method = comm_values.method.expect("Error getting method from file");

    if comm_values.file {
        let file_content = file_parse::read_file(comm_values.file_name.expect("Error opening file"));
        let file_instructions = file_parse::parse_file(file_content);
      
        let res: Response = requests::get_req::get_with_header(file_instructions.url.unwrap(), file_instructions.header_name.unwrap(), file_instructions.header_value.unwrap()).await.unwrap();
        responses::output_response::print_all(res).await
    }

    let method = comm_values.method.unwrap();

    if method == "GET" {
        if comm_values.header_name == None {
            let res: Response = requests::get_req::get_no_header(comm_values.url.unwrap())
            .await
            .unwrap();
            responses::output_response::print_all(res).await;
        }
        else if comm_values.header_name != None {
            let res: Response = requests::get_req::get_with_header(comm_values.url.unwrap(), comm_values.header_name.unwrap(), comm_values.header_value.unwrap()).await.unwrap();
            responses::output_response::print_all(res).await;
        }
    }

    else if method == "POST" {
        println!("POST REQUEST")
    }

}
