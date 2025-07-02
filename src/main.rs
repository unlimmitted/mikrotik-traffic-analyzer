mod database;
mod migration;
pub mod models;
pub mod schema;

use crate::database::DbConnector;
use crate::models::Interface;
use crate::models::Traffic;
use mikrotik_api::{connect, Authenticated, MikrotikAPI};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let password = std::env::var("password").expect("Не задана переменная окружения password");
    let user = std::env::var("user").expect("Не задана переменная окружения user");
    let gateway = std::env::var("gateway").expect("Не задана переменная окружения gateway");

    let disconnected = connect((gateway.as_str(), 8728)).await.unwrap();
    let mut api = disconnected
        .authenticate(user.as_str(), password.as_str())
        .await
        .unwrap();
    loop {
        get_peers_traffic(&mut api).await;
        tokio::time::sleep(Duration::from_secs(5)).await
    }
}

async fn get_peers_traffic(api: &mut MikrotikAPI<Authenticated>) {
    let mut db = DbConnector::new();
    let args = [(".proplist", "name, tx, rx")];
    let interfaces = api
        .generic_array_call::<Interface>("/interface/wireguard/peers/print", Some(&args))
        .await
        .unwrap();
    for interface in interfaces {
        let peer = Traffic {
            name: interface.name,
            tx: interface.tx,
            rx: interface.rx,
        };
        db.insert_or_update_traffic(peer);
    }
}
