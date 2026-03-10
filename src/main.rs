use crate::player::{Player, PlayerCommand, PlayerEvent};
use clap::Parser;
use rodio::cpal::traits::HostTrait;
use rodio::cpal::{BufferSize, StreamConfig};
use rodio::{cpal, DeviceSinkBuilder, DeviceTrait};
use std::sync::Arc;
use std::{io, thread};
use std::path::Path;
use std::time::Duration;
use crossterm::event::{read, Event, KeyCode};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
mod player;

#[derive(Parser, Debug)]
#[command(version, about = "rodio example", long_about = None)]
struct Opt {
    /// The audio device to use
    #[arg(short, long)]
    device: Option<String>,

    #[arg(short, long, default_value = "")]
    file: Option<String>,
}


fn main() {
    let opt: Opt = Opt::parse();

    let (_player_cmd_tx, player_cmd_rx) = tokio::sync::mpsc::unbounded_channel::<PlayerCommand>();
    // let player_cmd_tx_shared = Arc::new(player_cmd_tx.clone());

    let (player_evt_tx, mut _player_evt_rx) = mpsc::unbounded_channel::<PlayerEvent>();

    let host = cpal::default_host();

    let device = if let Some(device) = opt.device.clone() {
        let id = &device.parse().expect("failed to parse device id");
        host.device_by_id(id)
    } else {
        host.default_output_device()
    }.expect("failed to find output device");

    let mut config: StreamConfig = device.default_output_config().unwrap().into();
    config.buffer_size = BufferSize::Fixed(2048);

    let stream = DeviceSinkBuilder::from_device(device)
        .expect("failed to create sink builder")
        .with_config(&config)
        .open_stream()
        .expect("failed to open stream");

    let rodio_player = rodio::Player::connect_new(stream.mixer());

    start_tokio_background_tasks(rodio_player, player_cmd_rx, player_evt_tx);

    let file = if let Some(f) = opt.file.clone() && Path::exists(Path::new(&f)) {
        f
    } else {
        "".to_string()
    };


    let mut quit = false;
    while !quit {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "t" => {
                let _ = _player_cmd_tx.send(PlayerCommand::PlayTest);
            },
            "s" => {
                let _ = _player_cmd_tx.send(PlayerCommand::Stop);
            },
            "f" => {
                let file = file.clone();
                if file.is_empty() {
                    println!("file not found or not playable");
                } else {
                    let _ = _player_cmd_tx.send(PlayerCommand::PlayMedia(file, Duration::from_millis(0)));
                }
            },
            "q" => quit = true,
            _ => println!("Other key"),
        }
    }



    /*
    loop {

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Char('t') => {
                    let _ = _player_cmd_tx.send(PlayerCommand::PlayTest());
                },
                KeyCode::Char('f') => {
                    let file = file.clone();
                    if file.is_empty() {
                        println!("file not found or not playable");
                    } else {
                        let _ = _player_cmd_tx.send(PlayerCommand::PlayMedia(file, Duration::from_millis(0)));
                    }
                },
                KeyCode::Char('q') => {
                    break;
                },
                _ => println!("Unknown key {}, try t", event.code),
            }
        }
    }

     */
}


pub fn start_tokio_background_tasks(rodio_player: rodio::Player, player_rx: UnboundedReceiver<PlayerCommand>, player_evt_tx: UnboundedSender<PlayerEvent>,) {
    println!("=== start_tokio_background_tasks");
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(background_tasks(
            rodio_player,
            player_rx,
            player_evt_tx,
        ));
    });


}

pub async fn background_tasks(
    rodio_player: rodio::Player,
    player_rx: UnboundedReceiver<PlayerCommand>,
    player_evt_tx: UnboundedSender<PlayerEvent>,
) {


    let rodio_player = Arc::new(rodio_player);
    let player_task = tokio::spawn(async {
        let _ = Player::new().run(rodio_player, player_rx, player_evt_tx).await;
    });


    let _ = tokio::join!(player_task);



}