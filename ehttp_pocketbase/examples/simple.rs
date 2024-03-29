use ehttp_pocketbase::prelude::*;
use futures_lite::future;

fn main() {
    let client = Client::<User>::default();
    {
        let result = future::block_on(async { ehttp::fetch_async(client.health_check()).await });
        println!("{:?}", result);
    }
    {
        let result = future::block_on(async { ehttp::fetch_async(client.records("users")).await });
        println!("{:?}", result);
    }
}
