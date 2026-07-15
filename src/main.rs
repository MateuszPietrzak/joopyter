use bytes::Bytes;
use serde_json::json;
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;
use zeromq::{DealerSocket, Socket, SocketRecv, SocketSend, SubSocket, ZmqMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_uri = "ipc:///tmp/joopyter-kernel";

    let mut iopub_socket = SubSocket::new();
    iopub_socket.connect(&format!("{}-2", base_uri)).await?;
    iopub_socket.subscribe("").await?;
    println!("IOPub socket connected and subscription initiated.");

    tokio::spawn(async move {
        loop {
            match iopub_socket.recv().await {
                Ok(zmq_message) => {
                    let frames = zmq_message.into_vec();

                    if frames.len() < 7 {
                        continue;
                    }

                    let header_json: serde_json::Value = match serde_json::from_slice(&frames[3]) {
                        Ok(json) => json,
                        Err(_) => continue,
                    };
                    let msg_type = header_json["msg_type"].as_str().unwrap_or("");

                    let content_json: serde_json::Value = match serde_json::from_slice(&frames[6]) {
                        Ok(json) => json,
                        Err(_) => serde_json::Value::Null,
                    };

                    match msg_type {
                        "status" => {
                            let state = content_json["execution_state"].as_str().unwrap_or("");
                            println!("[Status] Kernel is now: {}", state);
                        }
                        "stream" => {
                            let text = content_json["text"].as_str().unwrap_or("");
                            print!("[Stream] {}", text);
                        }
                        "execute_result" => {
                            if let Some(data) = content_json["data"]["text/plain"].as_str() {
                                println!("[Result] {}", data);
                            }
                        }
                        _ => {
                            println!("[IOPub Info] Received message type: {}", msg_type);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("IOPub loop error: {:?}", e);
                    break;
                }
            }
        }
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let mut shell_socket = DealerSocket::new();
    shell_socket.connect(&format!("{}-1", base_uri)).await?;

    let msg_id = Uuid::new_v4().to_string();
    let session_id = Uuid::new_v4().to_string();

    let header = json!({
        "msg_id": msg_id,
        "username": "rust_client",
        "session": session_id,
        "msg_type": "execute_request",
        "version": "5.3"
    })
    .to_string();

    let content = json!({
        "code": "print('Hello, world!'); 1 + 1",
        "silent": false,
        "store_history": true,
        "user_expressions": {},
        "allow_stdin": false,
        "stop_on_error": true
    })
    .to_string();

    let mut msg = ZmqMessage::from("client");
    msg.push_back(Bytes::from("<IDS|MSG>"));
    msg.push_back(Bytes::from(""));
    msg.push_back(Bytes::from(header));
    msg.push_back(Bytes::from("{}"));
    msg.push_back(Bytes::from("{}"));
    msg.push_back(Bytes::from(content));

    shell_socket.send(msg).await?;

    let _reply = shell_socket.recv().await?;

    tokio::time::sleep(Duration::from_millis(200)).await;

    Ok(())
}
