    
use serde_derive::{Serialize, Deserialize};
use std::time::{Instant, Duration};
use std::sync::Arc;
use atlas_http::{HttpClient, HttpRequest, HttpResponse, HttpClientConfig};
use tokio::net::TcpStream;
use std::thread::sleep;
use std::io::{BufReader, Cursor, Read, BufRead};
use std::net::ToSocketAddrs;
use tokio::io::{stdout, AsyncWriteExt, WriteHalf};
use tokio_stream::StreamExt;
use rustls::pki_types::ServerName;
use std::io::Write;
use crate::utils::api_client;
use crate::server::api;
use crate::llm::chat::ChatMessage;
use crate::CLIENT_CONFIG;

#[derive(Serialize, Deserialize)]
pub struct OllamaWord {
    pub response: String,
    pub done: bool
}


#[derive(Debug, Serialize, Deserialize)]
struct AjaxResponse {
    status: String,
    message: String
}

/// Handle AJAX request
pub async fn handle(path: &str, req: &HttpRequest, stream: &mut WriteHalf<TcpStream>) -> HttpResponse {

    // Handle
    let res = match path {
        "ajax/chat/reply" => chat_reply(&req, stream).await,
        "ajax/chat/asst_reply" => chat_asst_reply(&req),
        _ => api::response(200, "", String::new())
    };

    res
}

/// User chat reply
async fn chat_reply(req: &HttpRequest, stream: &mut WriteHalf<TcpStream>) -> HttpResponse {

    // Create http request
    let http_config = HttpClientConfig::default();
    let req = api_client::create_apollo_request("v1/chat/reply", "POST", "application/x-www-form-urlencoded", &req.body); 
    let (uri, port, message) = req.prepare(&http_config).unwrap();

    // Get IP address
    let mut address = format!("{}:{}", &uri.host_str().unwrap(), port).to_socket_addrs().unwrap();
    let addr = address.next().unwrap();
    let res_headers = vec!["Content-Type: application/json".to_string()];

    // Connect
    let mut sock = match std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(5)) {
        Ok(r) => r,
        Err(e) => return HttpResponse::new(&500, &res_headers, &e.to_string())
    };
    sock.set_nodelay(true).unwrap();

    // Connect over SSL
    if uri.scheme() == "https" {
        let dns_name = ServerName::try_from(uri.host_str().unwrap()).unwrap().to_owned();
        //let conn = rustls::ClientConnection::new(Arc::clone(&http_config.tls_config), dns_name).unwrap();
        //let mut tls_stream = rustls::StreamOwned::new(conn, sock);
        //tls_stream.flush().unwrap();
        //tls_stream.write_all(&message).unwrap();
        //reader = BufReader::with_capacity(2048, tls_stream);
    }

    // Get reader
    sock.write_all(&message).unwrap();
    let mut reader = BufReader::with_capacity(2048, sock);
    // Go through line by line
    let mut timer = Instant::now();
    let mut done_header = false;
    loop {

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => { },
            Err(_) => break
        };
        line = line.trim().to_string();

        // Check line
        if !done_header {
            if line.is_empty() { done_header = true; }
            continue;
        } else if line.is_empty() { 
            if timer.elapsed().as_secs() > 10 { break; }
            sleep(Duration::from_secs(1));
            continue;
        }

        // Check for final message
        if let Ok(final_msg) = serde_json::from_str::<ChatMessage>(&line) {
            stream.write(line.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
            break;
        }

        // Decode json
        let json: OllamaWord = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(_) => continue
        };

        // Output json
        stream.write(line.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
        timer = Instant::now();

        //if json.done == true { break; }
    }

    HttpResponse::new(&200, &res_headers, &String::new())
}

/// Chat assistant reply (manual testing purposes)
fn chat_asst_reply(req: &HttpRequest) -> HttpResponse {

    // Send API call
    let message = req.body.params().get("message").unwrap().clone();
    let res: AjaxResponse = match api_client::send_json::<AjaxResponse, String>("v1/chat/reply", "POST", message.to_string()) {
        Ok(r) => r,
        Err(e) => AjaxResponse { status: "error".to_string(), message: e.to_string() }
    };
        let json = serde_json::to_string(&res).unwrap();

    let headers = vec!["Content-Type: application/json".to_string()];
    HttpResponse::new(&200, &headers, &json)
}


