use ehttp_pocketbase::client;
use futures_lite::future;

fn main() {
    let client = client::Client::default();
    let result = future::block_on(async{client.health_check()});
    println!("{:?}", result);
}
