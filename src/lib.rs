use futures_util::{Future, SinkExt, StreamExt};
use tokio::sync::mpsc::{self, Receiver};
use tokio_tungstenite::{connect_async, tungstenite::Message};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed twitch send text: {0}")]
    SendWssError(#[from] tokio_tungstenite::tungstenite::Error),
}

#[derive(Debug)]
pub struct TwitchIrcClient<'a> {
    commands: bool,
    membership: bool,
    tags: bool,
    nick: &'a str,
    channel: &'a str,
    access_token: &'a str,
    client: &'a str,
}

impl<'a> TwitchIrcClient<'a> {
    pub fn new(nick: &'a str, channel: &'a str, access_token: &'a str) -> TwitchIrcClient<'a> {
        TwitchIrcClient {
            commands: false,
            membership: false,
            tags: false,
            nick,
            channel,
            access_token,
            client: "wss://irc-ws.chat.twitch.tv:443",
        }
    }

    pub fn membership(mut self) -> Self {
        self.membership = true;
        self
    }

    pub fn commands(mut self) -> Self {
        self.commands = true;
        self
    }

    pub fn tags(mut self) -> Self {
        self.tags = true;
        self
    }

    pub async fn run(self) -> Result<(Receiver<String>, impl Future<Output = ()>)> {
        let (tx, rx) = mpsc::channel(1024);
        let mut capabilities = vec![];
        if self.commands {
            capabilities.push("twitch.tv/commands");
        }
        if self.membership {
            capabilities.push("twitch.tv/membership");
        }

        if self.tags {
            capabilities.push("twitch.tv/tags")
        }
        let (ws_stream, _) = connect_async(self.client)
            .await
            .expect("Failed to connect twitch irc");

        let cap = format!("CAP REQ :{}", capabilities.join(" "));
        let (mut write, mut read) = ws_stream.split();

        let auth_payload = format!("PASS oauth:{}", self.access_token);
        let nick_payload = format!("NICK {}", self.nick);
        let join_payload = format!("JOIN #{}", self.channel);

        write.send(Message::Text(cap)).await?;
        write.send(Message::Text(auth_payload)).await?;
        write.send(Message::Text(nick_payload)).await?;
        write.send(Message::Text(join_payload)).await?;

        let server = async move {
            loop {
                while let Ok(msg) = read.next().await.unwrap() {
                    match msg {
                        Message::Text(msg) => {
                            if let Err(e) = tx.send(msg).await {
                                eprint!("tx send error = {}", e);
                            };
                        }
                        Message::Ping(msg) => {
                            if let Err(e) = write.send(Message::Ping(msg)).await {
                                eprint!("ping Error : {}", e);
                            };
                        }
                        Message::Pong(msg) => {
                            println!("Pong {:?}", msg);
                        }
                        Message::Close(msg) => {
                            println!("Close {:?}", msg);
                        }
                        Message::Frame(msg) => {
                            println!("Frame {:?}", msg);
                        }
                        Message::Binary(msg) => {
                            println!("Binary {:?}", msg);
                        }
                    }
                }
            }
        };
        Ok((rx, server))
    }
}
