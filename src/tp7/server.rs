use tokio::net::UdpSocket;
use std::collections::HashMap;

pub async fn run_server() {
    let socket = UdpSocket::bind("127.0.0.1:8053").await.unwrap();
    println!("Serveur DNS en écoute sur 127.0.0.1:8053");
    
    let mut db = HashMap::new();
    db.insert("test.local".to_string(), [192, 168, 0, 42]);
    
    let mut buf = [0u8; 512];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        println!("Requête reçue de {}", addr);
        
        let req = &buf[..len];
        if let Some((id, name)) = dns::parse_query(req) {
            println!("Requête pour {}", name);
            if let Some(ip) = db.get(&name) {
                let response = dns::build_response(id, req, *ip);
                socket.send_to(&response, addr).await.unwrap();
                println!("Réponse envoyée pour {}", name);
            } else {
                println!("Nom inconnu : {}", name);
            }
        }
    }
}

mod dns {
    pub fn parse_query(buf: &[u8]) -> Option<(u16, String)> {
        if buf.len() < 12 {
            return None;
        }
        
        let id = u16::from_be_bytes([buf[0], buf[1]]);
        let mut idx = 12;
        let mut name = String::new();
        
        while idx < buf.len() {
            let len = buf[idx] as usize;
            if len == 0 {
                idx += 1;
                break;
            }
            idx += 1;
            if idx + len > buf.len() {
                return None;
            }
            if !name.is_empty() {
                name.push('.');
            }
            name.push_str(&String::from_utf8_lossy(&buf[idx..idx + len]));
            idx += len;
        }
        
        Some((id, name))
    }

    pub fn build_response(id: u16, query: &[u8], ip: [u8; 4]) -> Vec<u8> {
        let mut resp = Vec::new();
        
        // Header
        resp.extend_from_slice(&id.to_be_bytes());         // ID
        resp.extend_from_slice(&[0x81, 0x80]);             // Flags: response, recursion available
        resp.extend_from_slice(&[0x00, 0x01]);             // QDCOUNT
        resp.extend_from_slice(&[0x00, 0x01]);             // ANCOUNT
        resp.extend_from_slice(&[0x00, 0x00]);             // NSCOUNT
        resp.extend_from_slice(&[0x00, 0x00]);             // ARCOUNT
        
        // Question section: copier depuis la requête
        let mut question_end = 12;
        while question_end < query.len() && query[question_end] != 0 {
            let label_len = query[question_end] as usize;
            question_end += 1 + label_len;
        }
        question_end += 5; // null byte + type (2) + class (2)
        
        resp.extend_from_slice(&query[12..question_end]);
        
        // Answer section
        resp.extend_from_slice(&[0xC0, 0x0C]);             // Name (pointer to domain)
        resp.extend_from_slice(&[0x00, 0x01]);             // Type A
        resp.extend_from_slice(&[0x00, 0x01]);             // Class IN
        resp.extend_from_slice(&[0x00, 0x00, 0x00, 0x3C]); // TTL (60s)
        resp.extend_from_slice(&[0x00, 0x04]);             // RDLENGTH
        resp.extend_from_slice(&ip);                       // RDATA (IPv4)
        
        resp
    }
}