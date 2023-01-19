use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use rppal::gpio::Gpio;
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tokio::time::sleep;
use watchman_client::prelude::*;
use watchman_client::SubscriptionData;

#[derive(Default)]
struct App {
    state: Arc<RwLock<State>>,
}

pub struct State {
    parameter: Vec<String>,
    do_play: bool,
}
impl Default for State {
    fn default() -> Self {
        State {
            parameter: vec![format!("{}/{}", sound_dir(), default_sound())],
            do_play: false,
        }
    }
}
impl App {
    pub async fn switch(&self) -> anyhow::Result<()> {
        let mut state = self.state.clone();
        let gpio = Gpio::new()?;
        let pin = gpio.get(4)?.into_input();
        let jh = spawn(async move {
            loop {
                let play = state.read().await.do_play;
                if !play && pin.is_high() {
                    state.write().await.do_play = true;
                    set_sound(&mut state).await;
                }

                sleep(Duration::from_millis(200)).await;
            }
            #[allow(unreachable_code)]
            Ok(())
        });
        jh.await?
    }
    pub async fn monitor(&self) -> anyhow::Result<()> {
        let state = self.state.clone();
        let gpio = Gpio::new()?;
        let mut pin = gpio.get(14)?.into_output();
        let jh = spawn(async move {
            loop {
                if state.read().await.do_play {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
                sleep(Duration::from_millis(200)).await;
            }
            #[allow(unreachable_code)]
            Ok(())
        });
        jh.await?
    }
    pub async fn watch(&self) -> anyhow::Result<()> {
        let mut state = self.state.clone();
        let client = Connector::new().connect().await?;
        let resolved = client
            .resolve_root(CanonicalPath::canonicalize(sound_dir())?)
            .await?;
        let (mut subscription, _) = client
            .subscribe(&resolved, SubscribeRequest::default())
            .await?;
        let jh = spawn(async move {
            loop {
                let next: SubscriptionData<NameOnly> = subscription.next().await?;

                if let SubscriptionData::FilesChanged(_result) = next {
                    state.write().await.do_play = true;
                    set_sound(&mut state).await;
                }
            }
            #[allow(unreachable_code)]
            Ok(())
        });
        jh.await?
    }
    pub async fn run(&self) -> anyhow::Result<()> {
        let state = self.state.clone();
        let jh = spawn(async move {
            loop {
                let play = state.read().await.do_play;
                log::info!("do_play is : {play}");
                if play {
                    let mut command = command(&state.read().await.parameter[..]);
                    log::info!("{:?}", command);
                    if let Ok(mut child) = command.spawn() {
                        match child.wait().map_err(anyhow::Error::from) {
                            Ok(_) => {}
                            Err(e) => log::info!("wait error: {:?}", e),
                        }
                    } else {
                        log::info!("error spawning process");
                    }
                }
                state.write().await.do_play = false;
                sleep(Duration::from_secs(1)).await;
            }
            #[allow(unreachable_code)]
            Ok(())
        });
        jh.await?
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let app = Arc::new(App::default());
    let mut set = JoinSet::new();
    let _app = app.clone();
    set.spawn(async move {
        _app.monitor().await?;
        anyhow::Ok(())
    });
    let _app = app.clone();
    set.spawn(async move {
        _app.switch().await?;
        anyhow::Ok(())
    });
    let _app = app.clone();
    set.spawn(async move {
        _app.run().await?;
        anyhow::Ok(())
    });
    let _app = app.clone();
    set.spawn(async move {
        _app.watch().await?;
        anyhow::Ok(())
    });
    while let Some(Ok(_)) = set.join_next().await {}
    Ok(())
}

pub fn command(subsitutes: &[String]) -> Command {
    let mut command = Command::new("echo");
    command.arg("nothing to do.");
    let mut sub = 0;
    if let Ok(value) = dotenv::var("cmd") {
        for (i, part) in value.split(' ').enumerate() {
            let mut part = part.trim();
            log::info!("{part}");
            if part.eq("") {
                continue;
            }
            let s;
            if part.eq("{}") {
                s = part.replace("{}", subsitutes.get(sub).unwrap_or(&"{}".to_string()));
                part = &s;
                sub += 1;
            }
            match i {
                0 => command = Command::new(part),
                _ => {
                    command.arg(part);
                }
            }
        }
    }
    command
}

pub fn sound_dir() -> String {
    dotenv::var("sound_dir").unwrap_or_else(|_| "./sounds".to_string())
}
pub fn default_sound() -> String {
    dotenv::var("default_sound").unwrap_or_else(|_| "".to_string())
}
pub async fn set_sound(state: &mut Arc<RwLock<State>>) {
    let mut v = vec![];
    for path in glob::glob(&format!("{}/*", sound_dir())).unwrap().flatten() {
        v.push(path.display().to_string());
    }
    let num = random_number::random!(0..v.len());
    log::info!("random {}", num);
    if let Some(entry) = v.get(num % v.len()).cloned() {
        state.write().await.parameter = vec![entry];
    }
}
