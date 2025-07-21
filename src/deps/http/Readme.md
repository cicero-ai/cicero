
# Atlas HTTP - 

Clean, simplistic, easy to use HTTP client.  Features:

* Asynchronous and synchronous clients.
* Straight forward functions to download files, and send GET< POST, PUT< DELETE< HEAD, OPTIONS requests.
* Modelled after PHP's PSR-7 standard.
* HTTP and SOCKS5 proxy support
* Automated management of cookie jar

## Examples

```
use atlas_http::{HttpClient, HttpRequest, HttpBody, ProxyType};

// Simple GET request emulating web browser, utilizing cookies.txt as cookie jar
let mut http = HttpClient::builder()
    .browser()
    .cookie_jar("/path/to/cookies.txt")
    .build_sync();

let res = http.get("https://www.google.com/").unwrap();
println!("Status: {}\nBody:\n\n{}", res.status_code(), res.body());


// POST request
let body = HttpBody::from_string("name=John Smith&email=john@example.com&country=CA");
let res = http.post("https://domain.com/register", &body).unwrap();
println!("Status: {}\nBody:\n\n{}", res.status_code(), res.body());


// Normal PSR-7 style POST, with upload file
let mut body = HttpBody::empty();
body.set_param("name", "John SMith");
body.set_param("email", "john@example.com");
body.upload_file("document", "/path/to/document.pdf");

let headers = vec![
    "Site-User: myuser",
    "Site-API-Key: abc12345"
];

let req = HttpRequest::new("POST", "https://example.com/form", &headers, &body);
let res = http.send(&req).unwrap();
println!("Status: {}\nBody:\n\n{}", res.status_code(), res.body());


// Download file
let res = http.download("https://example.com/path/to/file.tar.gz", "/home/me/file.tar.gz").unwrap();
if res.status_code() == 200 {
    println!("GOt file!");
}


/// Send over SOCKS5 proxy at 192.168.0.24:1080
let mut http = HttpClient::builder()
    .browser()
    .cookie_jar("/path/to/cookies.txt")
    .proxy("192.168.0.24", 1080)
    .proxy_auth("myuser", "mypassword")
    .proxy_type(ProxyType::SOCKS5)
    .build_sync();

let res = http.get("https://some-domain.com/").unwrap();
println!("Status: {}\nBody:\n\n{}", res.status_code(), res.body());


/// Asynchronous example
let mut http = HttpClient::builder().browser().build_async();
let res = http.get("https://www.google.com/").await.unwrap();
println!("Status: {}\nBody:\n\n{}", res.status_code(), res.body());

// Async works exactly the same as syncronous.  The only 
// difference is you call ".build_async()" at the end of 
// the builder instead of ".build_sync()".  That's it.
```

## Contact

If you need any assistance or software development done, contact me via e-mail at matt@apexpl.io.



Coming shortly, almost done.


