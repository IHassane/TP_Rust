use tokio::net::UdpSocket;

pub async fn run_client() {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let server_addr = "127.0.0.1:8053";
    println!("Requête envoyée à {}", server_addr);
    
    let domain = "test.local";
    let mut buf = vec![];
    dns::build_query(domain, &mut buf);
    
    socket.send_to(&buf, server_addr).await.unwrap();
    
    let mut recv_buf = [0u8; 512];
    let (len, _) = socket.recv_from(&mut recv_buf).await.unwrap();
    
    if let Some(ip) = dns::parse_response(&recv_buf[..len]) {
        println!("Réponse DNS: {} -> {}", domain, ip);
    } else {
        println!("Échec de résolution DNS");
    }
}

mod dns {
    pub fn build_query(domain: &str, buf: &mut Vec<u8>) {
        // Header
        buf.extend_from_slice(&[0x12, 0x34]); // ID
        buf.extend_from_slice(&[0x01, 0x00]); // flags (standard query)
        buf.extend_from_slice(&[0x00, 0x01]); // QDCOUNT
        buf.extend_from_slice(&[0x00, 0x00]); // ANCOUNT
        buf.extend_from_slice(&[0x00, 0x00]); // NSCOUNT
        buf.extend_from_slice(&[0x00, 0x00]); // ARCOUNT
        
        // Question section
        for part in domain.split('.') {
            buf.push(part.len() as u8);
            buf.extend_from_slice(part.as_bytes());
        }
        buf.push(0); // end of domain name
        buf.extend_from_slice(&[0x00, 0x01]); // Type A
        buf.extend_from_slice(&[0x00, 0x01]); // Class IN
    }

    pub fn parse_response(buf: &[u8]) -> Option<String> {
        if buf.len() < 12 {
            println!("Trop petit pour un header DNS !");
            return None;
        }
        
        let qdcount = u16::from_be_bytes([buf[4], buf[5]]);
        let ancount = u16::from_be_bytes([buf[6], buf[7]]);
        println!("QDCOUNT: {}, ANCOUNT: {}", qdcount, ancount);
        
        if ancount == 0 {
            println!("Aucune réponse dans le paquet");
            return None;
        }
        
        let mut idx = 12;
        
        // Sauter la section question
        for _ in 0..qdcount {
            // Lire le nom de domaine
            while idx < buf.len() && buf[idx] != 0 {
                let label_len = buf[idx] as usize;
                idx += 1 + label_len;
            }
            idx += 1; // null byte
            idx += 4; // type (2 bytes) + class (2 bytes)
        }
        
        // Maintenant nous sommes au début de la section answer
        if idx >= buf.len() {
            println!("Index hors limites après la section question");
            return None;
        }
        
        // Lire la première réponse
        // Ignorer le nom (peut être un pointeur ou un nom complet)
        if buf[idx] & 0xC0 == 0xC0 {
            // C'est un pointeur (compression DNS)
            idx += 2;
        } else {
            // C'est un nom complet, le sauter
            while idx < buf.len() && buf[idx] != 0 {
                let label_len = buf[idx] as usize;
                idx += 1 + label_len;
            }
            idx += 1; // null byte
        }
        
        if idx + 10 > buf.len() {
            println!("Réponse trop courte pour les champs fixes");
            return None;
        }
        
        let rtype = u16::from_be_bytes([buf[idx], buf[idx + 1]]);
        let _class = u16::from_be_bytes([buf[idx + 2], buf[idx + 3]]);
        let _ttl = u32::from_be_bytes([buf[idx + 4], buf[idx + 5], buf[idx + 6], buf[idx + 7]]);
        let rdlen = u16::from_be_bytes([buf[idx + 8], buf[idx + 9]]) as usize;
        
        idx += 10; // Passer les champs fixes
        
        println!("RTYPE: {}, RDLEN: {}", rtype, rdlen);
        
        if rtype == 1 && rdlen == 4 && idx + 4 <= buf.len() {
            let ip = &buf[idx..idx + 4];
            println!("IP brut: {:?}", ip);
            return Some(format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]));
        } else {
            println!("Type ou longueur invalide (rtype: {}, rdlen: {})", rtype, rdlen);
        }
        
        None
    }
}