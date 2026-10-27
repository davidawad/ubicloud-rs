//! Demonstrate a create call against a local mock server using a recorded VM response.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

use ubicloud_rs::generated::{Client, types};

const RESPONSE: &str = include_str!("../fixtures/create-vm-response.json");

fn read_headers(stream: &mut TcpStream) {
    let mut bytes = Vec::new();
    let mut byte = [0_u8; 1];
    while !bytes.ends_with(b"\r\n\r\n") {
        stream.read_exact(&mut byte).expect("request headers");
        bytes.push(byte[0]);
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
    let address = listener.local_addr().expect("mock address");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("create request");
        read_headers(&mut stream);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{RESPONSE}",
            RESPONSE.len()
        );
        stream
            .write_all(response.as_bytes())
            .expect("write response");
    });

    let client = Client::new(&format!("http://{address}"));
    let project_id =
        types::CreateVmProjectId::try_from("pj123456789012345678901234").expect("valid project ID");
    let reference = types::Reference::try_from("compat-check").expect("VM reference");
    let request = types::CreateVmBody {
        boot_image: Some("ubuntu-noble".into()),
        enable_ip4: Some(true),
        gpu: None,
        init_script: None,
        private_subnet_id: None,
        public_key: "ssh-ed25519 AAAA mock".into(),
        size: Some("standard-2".into()),
        storage_size: None,
        unix_user: Some("ubuntu".into()),
    };
    let vm = client
        .create_vm(&project_id, "us-east-a2", &reference, &request)
        .await
        .expect("decode mock VM response")
        .into_inner();
    server.join().expect("mock server");
    let output = format!(
        "Created VM {} ({}) in {}: {}",
        vm.name, *vm.id, vm.location, vm.state
    );
    if let Some(path) = std::env::args()
        .nth(1)
        .and_then(|arg| (arg == "--svg").then(|| std::env::args().nth(2)).flatten())
    {
        write_svg(Path::new(&path), &output).expect("write showcase SVG");
    } else {
        println!("{output}");
    }
}

fn write_svg(path: &Path, output: &str) -> std::io::Result<()> {
    let escaped = output
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="270" viewBox="0 0 1040 270" role="img" aria-labelledby="title description">
  <title id="title">ubicloud-rs mock VM creation</title>
  <desc id="description">A local mock request returns a decoded virtual machine.</desc>
  <rect width="1040" height="270" rx="18" fill="#101923"/>
  <circle cx="38" cy="38" r="7" fill="#f7768e"/><circle cx="62" cy="38" r="7" fill="#e0af68"/><circle cx="86" cy="38" r="7" fill="#9ece6a"/>
  <text x="36" y="91" fill="#7aa2f7" font-family="monospace" font-size="19">$ cargo run --example mock_create_vm</text>
  <text x="36" y="153" fill="#c0caf5" font-family="monospace" font-size="21">{escaped}</text>
  <text x="36" y="218" fill="#9ece6a" font-family="monospace" font-size="16">mock server · recorded fixture · no cloud calls</text>
</svg>
"##
    );
    std::fs::write(path, svg)
}
