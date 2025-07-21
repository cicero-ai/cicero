
use atlas_http::{HttpRequest, HttpResponse, HttpBody};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncBufReadExt, WriteHalf};
use tokio::io;
use url::Url;
use crate::client::cfx::CfxClient;
use crate::server::echo::render::ASSETS;
use crate::server::echo::render::pages;
use crate::server::echo::ajax;
use crate::server::api;
use crate::error::Error;
use log::{info, debug, error};
use crate::CLIENT_CONFIG;

pub struct EchoServer { 
    cfx: Option<CfxClient>
}

impl EchoServer {

    pub fn new() -> Self {

        // Get cfx client
        let mut cfx_client: Option<CfxClient> = None;
        if CLIENT_CONFIG.current_user.is_some() {
            cfx_client = match CfxClient::new() {
                Ok(r) => Some(r),
                Err(e) => {
                    error!("Unable to authenticate via CFX, error: {}", e.to_string());
                    std::process::exit(1);
                }
            };
        }

        Self {
            cfx: cfx_client
        }
    }

    /// Start the echo server
    pub async fn start(&mut self, port: &u16) -> Result<(), Error> {

        // Start listening
        let address = format!("{}:{}", CLIENT_CONFIG.daemons.echo.0, port);
        let listener = match TcpListener::bind(&address) .await {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to start Echo server, error: {}", e.to_string());
                std::process::exit(1);
            }
        };
        info!("Started Echo server, listening for connections on {}...", address);

        // Get incoming connection
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                handle(stream).await;
            });
        }

        Ok(())
    }

}

/// Handle connection
async fn handle(mut stream: TcpStream) {

    // Get peer
    let peer = match stream.peer_addr() {
        Ok(r) => r,
        Err(e) => {
            let res = api::response(400, "Invalid request, no peer.", String::new());
            stream.write_all(&res.raw().as_bytes()).await.unwrap();
            return
        }
    };
    info!("Received connection from {}.", peer.ip());

    // Build http request
    let req = match HttpRequest::build_async(&mut stream).await {
        Ok(r) => r,
        Err(e) => {
            let res = api::response(400, "Invalid non-HTTP request", String::new());
            stream.write_all(&res.raw().as_bytes()).await.unwrap();
            return
        }
    };

    // Split TCP stream
    let (reader, mut writer) = tokio::io::split(stream);

    // Route request as needed
    let url = Url::parse(&req.url).unwrap();
    let res = route(&url.path().trim_start_matches("/").trim_end_matches("/"), &mut writer, &req).await;

    // Output response
    writer.write_all(&res.raw().as_bytes()).await.unwrap();
}

/// Route http request, get appropriate response
async fn route(path: &str, stream: &mut WriteHalf<TcpStream>, req: &HttpRequest) -> HttpResponse {

    // Check for static file
    if path.starts_with("static/") {
        return ASSETS.get_static(&path);
    } else if path.starts_with("ajax/") {
        return ajax::handle(&path, &req, stream).await;
    }
    debug!("Routing request to '{}'", path);

    // Get response
    let html = match path {
        "index" | "" => pages::index::render().render(),
        "chat" => pages::chat::render().render(),
        _ => "".to_string()
    };

    // Check for 404
    if html.is_empty() {
        return HttpResponse::new(&404, &vec![], &"404 File Not Found".to_string());
    }

    HttpResponse::new(&200, &vec![], &html)
}




