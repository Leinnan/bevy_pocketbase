use ehttp_pocketbase::client;

fn main() {
    let client = client::Client::new("http://127.0.0.1:8090");
    let result = client.health_check();
    println!("{:?}", result);
}
