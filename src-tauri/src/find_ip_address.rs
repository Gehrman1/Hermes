use std::net::IpAddr;

use dns_lookup::lookup_addr;
use netstat2::{get_sockets_info, iterate_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct IpInfo {
    pub hostName: String,
    pub ip_address:String
}
#[tauri::command]

pub fn find_ip_address() -> Vec<IpInfo> {
    println!("ip 2");
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
   
    let mut ip_address_list:Vec<IpInfo> = Vec::new();    
    for si in sockets_info {
        let mut ip_address = String::from("");
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                if tcp_si.local_addr.to_string()  == "0.0.0.0" && tcp_si.local_port.to_string() == "8080" || tcp_si.local_port.to_string() == "8081" {
                    let ip: IpAddr = tcp_si.local_addr.to_string().parse().unwrap();
                    let mut hostName = String::from("");

                    match lookup_addr(&ip) {
                        Ok(hostname) => hostName = hostname ,
                        Err(e) => eprintln!("Error: {}", e),
                    }
                    println!(
                        "TCP {}:{}",
                        tcp_si.local_addr,
                        tcp_si.local_port,
                       
                    );
                    ip_address = format!("{}:{}",tcp_si.local_addr, tcp_si.local_port);
                    println!(
                        "TCP {}",
                        ip_address
                    );
                    
                    ip_address_list.push(
                        IpInfo { hostName, ip_address }
                    )


                }
                
            }
            ProtocolSocketInfo::Udp(udp_si) => {}
        }
    }

    ip_address_list
}
