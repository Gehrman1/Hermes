use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

#[tauri::command]
pub fn find_ip_address() -> String {
    println!("ip 2");
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
    let mut ip_address = String::from("");
    
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                if tcp_si.local_addr.to_string()  == "0.0.0.0" && tcp_si.local_port.to_string() == "8080" || tcp_si.local_port.to_string() == "8081" {
                    println!(
                        "TCP {}:{}",
                        tcp_si.local_addr,
                        tcp_si.local_port,
                       
                    );
                    ip_address = format!("{}:{}",tcp_si.local_addr, tcp_si.local_port);


                }
                
            }
            ProtocolSocketInfo::Udp(udp_si) => {}
        }
    }

    ip_address
}
