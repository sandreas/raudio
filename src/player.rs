use rodio::Source;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub enum PlayerCommand {
    PlayTest,
    Stop,
    PlayMedia(String, Duration),
}

#[derive(Debug)]
pub enum PlayerEvent {
    Position(String, Duration),
}

pub struct Player {}

impl Player {
    pub fn new() -> Player {
        Self {}
    }

    pub async fn run(
        &mut self,
        sink: Arc<rodio::Player>,
        mut cmd_rx: UnboundedReceiver<PlayerCommand>,
        evt_tx: UnboundedSender<PlayerEvent>,
    ) {
        loop {
            let mut last_player_pos = Duration::from_secs(0);
                tokio::select! {


                    Some(cmd) = cmd_rx.recv() => {
                        println!("============== cmd received ==============");
                        match cmd {
                            PlayerCommand::Stop => {
                                sink.stop();
                                sink.clear()
                            },
                            PlayerCommand::PlayTest => {
                                sink.clear();
                                let waves = vec![230f32, 270f32, 330f32, 270f32, 230f32];
                                for w in waves {
                                    let source = rodio::source::SineWave::new(w).amplify(0.1);
                                    sink.append(source);
                                    sink.play();
                                    tokio::time::sleep(Duration::from_millis(200)).await;
                                    sink.stop();
                                    sink.clear();
                                }
                            }
                            PlayerCommand::PlayMedia(s, position) => {
                                let path = Path::new(s.as_str());
                                let file_result = File::open(path);

                                println!("file result: {:?}", file_result);

                                if let Ok(file) = file_result {
                                    let decoder_result = rodio::Decoder::try_from(file);
                                    if let Ok(decoder) = decoder_result{
                                        sink.clear();
                                        sink.append(decoder);
                                        if position > Duration::from_secs(0) {
                                            let _ = sink.try_seek(position);
                                        }
                                        sink.play();
                                    }
                                }
                            }
                        }
                    }

                    _ = tokio::time::sleep(Duration::from_millis(1000)) => {
                        let pos = sink.get_pos();

                        if pos != last_player_pos {
                            self.update_position(&evt_tx, pos).await;
                            last_player_pos = pos;
                        }

                    }
                }

        }
    }

    async fn update_position(&self, _evt_tx: &UnboundedSender<PlayerEvent>, pos: Duration) {
        println!("position: {}", format_duration(pos));
    }
}


pub fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();
    let secs = millis / 1000;
    let h = secs / (60 * 60);
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
}