use hyper::Client;

enum State {
    UP,
    DOWN,
    UNKNOWN
}

fn main() {
    println!("Starting request ...");
    create_request(String::from("http://google.com"));
}

async fn create_request(url: String) -> State {
    println!("Creating Client ...");
    let client = Client::new();
    let parsed_url = url.parse();
    let resp = client.get(parsed_url.unwrap()).await;
    println!("Response: {}", resp.unwrap().status());
    return State::UNKNOWN;
}
