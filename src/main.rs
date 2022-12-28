use std::process::{Child, Command};
use std::sync::Arc;
use std::time::Duration;

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

#[derive(Default)]
pub struct State {
    file: Option<String>,
    command: Option<String>,
    cmd: Option<Command>,
    child: Option<Child>,
    do_play: bool,
}
impl App {
    pub async fn watch(&self) -> anyhow::Result<()> {
        let state = self.state.clone();
        let mut client = Connector::new().connect().await?;
        let resolved = client
            .resolve_root(CanonicalPath::canonicalize("./sounds")?)
            .await?;
        let (mut subscription, _) = client
            .subscribe(
                &resolved,
                SubscribeRequest::default()
            )
            .await?;
        let jh = spawn(async move { loop {

                let next: SubscriptionData<NameOnly> = subscription.next().await?;

                match next {
                    SubscriptionData::FilesChanged(result) => {
                        state.write().await.do_play = true;
                    }
                    _ => {}
                }
            }
            #[allow(unreachable_code)]
            Ok(())
        });
        jh.await?
    }
    pub async fn run(&self) -> anyhow::Result<()>{
        let state = self.state.clone();
        let jh = spawn(async move { loop {
                let play = state.read().await.do_play;
                if play {
                    let mut command = Command::new("play");
                    command.args(["./sounds/total_commitment.wav"]);
                    if let Ok(mut child) = command.spawn() {
                        match child.wait().map_err(anyhow::Error::from) {
                            Ok(_) => {}
                            Err(e) => log::info!("wait error: {:?}", e),
                        }
                    }
                    else {
                        log::info!("error spawning process");
                    }
                    state.write().await.do_play = false;
                }
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
    set.spawn(async move { _app.run().await?; anyhow::Ok(()) });
    let _app = app.clone();
    set.spawn(async move { _app.watch().await?; anyhow::Ok(()) });
    while let Some(Ok(_)) = set.join_next().await {}
    Ok(())
}