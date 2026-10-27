use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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

#[tokio::test]
async fn create_vm_decodes_recorded_response() {
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
        .expect("decode the recorded create response")
        .into_inner();
    assert_eq!(vm.name, "compat-check");
    assert_eq!(&*vm.id, "vm123456789012345678901234");
    assert_eq!(vm.state, "running");
    server.join().expect("mock server");
}
