
use crossbeam_channel::{Sender, Receiver};
use tracing_subscriber::field::debug;
use std::num::NonZeroUsize;
use indexmap::IndexMap;
use serde_json::json;
use std::thread;

use crate::server::Server;
use crate::server::connection::ConnectionInitializer;
use lsp_server::Message;


pub struct TestServer {
    // pub server: Server,
    pub thread_join_handle: thread::JoinHandle<()>,
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

impl TestServer {
  pub fn req_respond(&self, req: lsp_server::Message) -> Result<lsp_server::Message, anyhow::Error> {
    self.sender.send(req)?;
    let resp = self.receiver.recv()?;
    Ok(resp)
  }
}

struct TestCase {
    /// Files to load at initialization time.
    files: Vec<(String, String)>,
    /// A list of pairs of client message, and expected server response messages.
    interactions: Vec<(Message, Vec<Message>)>,
}

impl TestCase {
    pub fn run(self) -> anyhow::Result<()> {
        eprintln!("new_test_server");
        let test_server = new_test_server(NonZeroUsize::new(1).unwrap())?;
        eprintln!("about to loop");
        for (file_name, file_content) in self.files {
            let _resp = test_server.sender.send(lsp_server::Message::Notification(lsp_server::Notification{
              method: "textDocument/didOpen".to_string(),
              params: json!({
                "textDocument": {
                  "uri": format!("file:///{}", file_name),
                  "languageId": "baml",
                  "version": 1,
                  "text": file_content
                }
              }),
            }))?;

            // Consume the post-opening notification.
            eprintln!("Awaiting didOpen notif");
            let didOpenNotification = test_server.receiver.recv();
            eprintln!("Got didOpen notif");
            dbg!(&didOpenNotification);
        }
        
        for (req, expected_responses) in self.interactions {
          test_server.sender.send(req)?;
          for expected_response in expected_responses {
            let response = test_server.receiver.recv()?;
            match (&response, expected_response) {
              (lsp_server::Message::Response(r1), lsp_server::Message::Response(r2)) => {
                assert_eq!(r1.result, r2.result);
              },
              (_, lsp_server::Message::Response(r2)) => {
                panic!("Expected response {r2:?}, got {response:?}");
              },
              (lsp_server::Message::Notification(n1), lsp_server::Message::Notification(n2)) => {
                assert_eq!(n1.method, n2.method);
                assert_eq!(n1.params, n2.params);
              },
              (_, lsp_server::Message::Notification(n2)) => {
                panic!("Expected notification {n2:?}, got {response:?}");
              },
              _ => panic!("Should only expect responses and notifications."),
            }
          }
        }
        test_server.thread_join_handle.join().unwrap();
        Ok(())
    }

    pub fn mk_simple() -> Self {
        TestCase {
            files: vec![("test.baml".to_string(), SINGLE_FILE.to_string())],
            interactions: vec![],
        }
    }
}

pub fn new_test_server(worker_threads: NonZeroUsize) -> crate::Result<TestServer> {

  let initialize = lsp_server::Message::Request(lsp_server::Request{
    id: lsp_server::RequestId::from(1),
    method: "initialize".to_string(),
    params: json!({
      "capabilities": {},
      "rootPath": "./",
    }),
  });
  let (server_connection, client_connection) = lsp_server::Connection::memory();
  
  client_connection.sender.send(initialize).unwrap();
  let thread_join_handle = thread::spawn(move || {

    let connection = ConnectionInitializer::new(server_connection);
    let (id, init_params) = connection.initialize_start().unwrap();

    let client_capabilities = init_params.capabilities.clone();
    let position_encoding = Server::find_best_position_encoding(&client_capabilities);
    let server_capabilities = Server::server_capabilities(position_encoding);

    let connection = connection.initialize_finish(
        id,
        &server_capabilities,
        crate::SERVER_NAME,
        crate::version(),
    ).unwrap();

    let server = Server::new_with_connection(worker_threads, connection, init_params).unwrap();
    server.run().unwrap();
  });

  let _handshake = client_connection.receiver.recv()?;
  client_connection.sender.send(lsp_server::Message::Notification(lsp_server::Notification{
    method: "initialized".to_string(),
    params: json!({})
  }))?;

  Ok(TestServer {
      thread_join_handle,
      sender: client_connection.sender,
      receiver: client_connection.receiver
  })
}

static SINGLE_FILE: &str = r##"
client<llm> GPT4 {
  provider openai
  options {
    model gpt-4o
    api_key env.OPENAI_API_KEY
  }
}

generator lang_python {
  output_type python/pydantic
  output_dir "../python"
  version 0.74.0
}

class Foo {
  bar int
}

function Succ(inp: int) -> Foo {
  client GPT4
  prompt #"
    The successor of {{ inp }}.
    {{ ctx.output_format }}   
  "#
}

test TestSucc {
  functions [Succ]
  args { inp 1 }
}
"##;


#[test]
fn test_initialization() {
    TestCase::mk_simple().run().unwrap()
}