use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::sync::{mpsc, RwLock};
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::net::TcpListener;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread::spawn;
use super::config::ServerConfig;

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response}
};


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub user: User,
    pub msg: String,
    pub timestamp: i64 
}

#[derive(Debug)]
pub struct ChatServer {
    pub users: Arc<RwLock<Vec<User>>>,
    pub messages: Arc<RwLock<Vec<Message>>>,
    pub config: ServerConfig,
    server: Option<Arc<TcpListener>>
}


fn parse_query_string(qstr: &str) -> HashMap<&str, &str> {
    let mut hash: HashMap<&str, &str> = HashMap::new();

    for pair in qstr.split("&") {
        match pair.find("=") {
            Some(idx) => {
                hash.insert(pair[..idx].into() , pair[idx+1..].into())
            },
            None => continue
        };
    }
    hash
}

enum ChannelMessage {
    NewUser(User),
    NewMessage(Message),
    ExitUser(User)
}

impl ChatServer {
    pub fn new(config: ServerConfig) -> ChatServer {
       ChatServer { users: Arc::new(RwLock::new(Vec::new())),
                    messages: Arc::new(RwLock::new(Vec::new())),
                    config,
                    server: None
       }
    }

    pub fn bind(mut self) -> Self {
        let url = format!("{}:{}", self.config.host, self.config.port);
        self.server = Some(Arc::new(TcpListener::bind(url).unwrap()));
        self
    }
    
    pub fn run(mut self) -> Self {
        
        if let Some(ref server) = self.server {
            let server = server.clone();

            let (tx, rx): (Sender<ChannelMessage>, Receiver<ChannelMessage>) = mpsc::channel();
            
            let thread_tx = tx.clone();

            let messages = self.messages.clone();
            let handler = spawn(move || {
                for stream in server.incoming() {
                   
                   let tttx = thread_tx.clone();
                    

                    let messages = messages.clone();
                    spawn(move || {
                        let mut user = User { username: "Unknown".into() };

                        let callback = |req: &Request, mut response: Response| { 
                                
                                let query_params = match req.uri().query() {
                                    Some(query) => parse_query_string(query), 
                                    None => HashMap::new() 
                                };

                                let username = String::from(*query_params.get("u").unwrap_or(&"Anonymous User"));
                                
                                user.username = username.clone();  
                                
                                tttx.send(ChannelMessage::NewUser(User { username })).unwrap();
                                
                                Ok(response)
                        };
                
                        let mut websocket = accept_hdr(stream.unwrap() , callback).unwrap();
                        
                        websocket.get_mut().set_read_timeout(Some(Duration::from_secs(1))).unwrap();

                        let mut prev_len: usize = 0;

                        loop {
                            match websocket.read_message() {
                                Ok(m) => {
                                    let msg = format!("{}", m);
                                    tttx.send(
                                        ChannelMessage::NewMessage(
                                            Message { user:user.clone(), msg , timestamp: 1_i64 }
                                            )).unwrap();
                                },
                                Err(_) => {
                                    let r = messages.read().unwrap();
                                    if r.len() != prev_len {
                                        for n_msg in r[prev_len..].iter() {
                                             
                                            websocket.write_message(
                                                serde_json::to_string(&n_msg)
                                                            .unwrap()
                                                            .into()
                                                ).unwrap();
                                        }
                                    }
                                    prev_len = r.len();
                                }
                            }
                            
                        }
                    });
                }
            });

            let server_user = User { username: "Server".into() };

            for msg in rx {
                match msg {
                    ChannelMessage::NewUser(user) => {
                        println!("New user: {}", user.username);
                        let mut guard = self.messages.write().unwrap();
                        guard.push(
                            Message { user: server_user.clone(),
                                        msg: format!("{} joined the server!", user.username),
                                        timestamp: 1_i64
                            });

                        println!("Len: {}", guard.len());
                    },
                    ChannelMessage::NewMessage(message) => {
                        let mut guard = self.messages.write().unwrap();
                        guard.push(message);
                    },
                    _ => println!("Other message")
                }
            }
            handler.join().unwrap();
        }else{
            panic!("No active TCPStream found! Bind before running");
        }

        self
    }

}
