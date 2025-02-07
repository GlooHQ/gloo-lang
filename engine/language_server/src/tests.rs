
use crossbeam_channel::{Sender, Receiver};
use tracing_subscriber::field::debug;
use std::num::NonZeroUsize;
use indexmap::IndexMap;

use crate::server::Server;
use crate::server::connection::ConnectionInitializer;
use lsp_server::Message;


struct TestServer {
    server: Server,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

struct TestCase {
    /// Files to load at initialization time.
    files: Vec<(String, String)>,
    /// A list of pairs of client message, and expected server response messages.
    interactions: Vec<(Message, Vec<Message>)>,
}

impl TestCase {
    pub fn run(self) -> anyhow::Result<()> {
        let test_server = new_test_server(NonZeroUsize::new(1).unwrap())?;
        for (file_name, file) in self.files {
            dbg!(file_name);
        }
        test_server.server.connection.handle_shutdown();
        // dbg!(&test_server.server.session.projects_by_workspace_folder);
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

    let (server_connection, client_connection) = lsp_server::Connection::memory();
    let connection = ConnectionInitializer::new(server_connection);
    let (id, init_params) = connection.initialize_start()?;

    let client_capabilities = init_params.capabilities.clone();
    let position_encoding = Server::find_best_position_encoding(&client_capabilities);
    let server_capabilities = Server::server_capabilities(position_encoding);

    let connection = connection.initialize_finish(
        id,
        &server_capabilities,
        crate::SERVER_NAME,
        crate::version(),
    )?;
    let server = Server::new_with_connection(worker_threads, connection, init_params)?;
    Ok(TestServer {
        server,
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