use crate::player::{Player, PlayerCommand, PlayerEvent};
use clap::Parser;
use std::{io, thread};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
mod player;

#[derive(Parser, Debug)]
#[command(version, about = "rodio example", long_about = None)]
struct Opt {
    /// The audio device to use
    #[arg(short, long)]
    device: Option<String>,
}


fn main() {
    let (_player_cmd_tx, player_cmd_rx) = tokio::sync::mpsc::unbounded_channel::<PlayerCommand>();
    // let player_cmd_tx_shared = Arc::new(player_cmd_tx.clone());

    let (player_evt_tx, mut _player_evt_rx) = mpsc::unbounded_channel::<PlayerEvent>();

    start_tokio_background_tasks(player_cmd_rx, player_evt_tx);

    let mut quit = false;
    while !quit {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "a" => println!("You pressed A"),
            "b" => println!("You pressed B"),
            "q" => quit = true,
            _ => println!("Other key"),
        }
    }



}


pub fn start_tokio_background_tasks(
    player_rx: UnboundedReceiver<PlayerCommand>,
    player_evt_tx: UnboundedSender<PlayerEvent>,
) {
    println!("=== start_tokio_background_tasks");
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(background_tasks(
            player_rx,
            player_evt_tx,
        ));
    });


}

pub async fn background_tasks(
    player_rx: UnboundedReceiver<PlayerCommand>,
    player_evt_tx: UnboundedSender<PlayerEvent>,
) {

    let player_task = tokio::spawn(async {
        let sink: Option<&rodio::Player> = None;
        let _ = Player::new().run(sink, player_rx, player_evt_tx).await;
    });






    let _ = tokio::join!(player_task);



}