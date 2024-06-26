use rfd::FileDialog;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufWriter}, net::{TcpListener, TcpStream}};
use tokio::io::split;

use std::{fs, io::{self, Write}, path::{Path, PathBuf}};
use std::fs::File as FilesStd;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use local_ip_address::local_ip;


#[tauri::command]
pub async fn connect_command() -> Result<(), String> {
    connect().await;
  Ok(())
}




async fn connect()   {
    println!("Here");
    let my_local_ip = local_ip().unwrap();
    println!("Here");
    
   
    let mut socket = TcpStream::connect(format!("{}:8080", my_local_ip)).await.unwrap();
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
