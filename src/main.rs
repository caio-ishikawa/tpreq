mod command_parse;
use reqwest::Response;
mod requests {
    pub mod get_req;
}
mod responses {
    pub mod output_response;
}

#[tokio::main]
async fn main() {
    let comm_values: command_parse::Instructions = command_parse::read_flags();

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
