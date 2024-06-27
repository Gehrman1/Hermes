use rfd::FileDialog;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufWriter}, net::{TcpListener, TcpStream}};
use tokio::io::split;

use std::{fs, io::{self, Write}, path::{Path, PathBuf}};
use std::fs::File as FilesStd;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use local_ip_address::local_ip;


#[tauri::command(async)]
pub async fn connect_command(ip_connect:String) -> Result<(), String> {
    println!("Here");
    connect(ip_connect).await;
  Ok(())
}




async fn connect(ip_connect:String)   {
    let mut final_ip:String;
    println!("Here");
    // need to make it more specific
    let value_to_compare = "0.0.0.0";
    let my_local_ip = local_ip().unwrap();

    // Extract the first 5 characters
    let substring = &ip_connect[0..7];
    if substring == value_to_compare {
      
        let port_substring = &ip_connect[7..12];
       final_ip = format!("{}{}", my_local_ip, port_substring);

    } else {
        final_ip = ip_connect
    }


    
    println!("Here");
    
   
    let mut socket = TcpStream::connect(final_ip.to_string()).await.unwrap();
    let (mut rd, mut wr) = tokio::io::split(socket);


   
    let mut buffer = Vec::new();
    
 
    let n = rd.read_to_end(&mut buffer).await;
    // println!("{:?} {:?} ", buffer, n);
   let mut file = File::create("C:/Hermes/tst.gz").await.unwrap();
   println!("dog");
   file.write_all(&buffer).await;


    

    
//     Ok(())
// }

}
