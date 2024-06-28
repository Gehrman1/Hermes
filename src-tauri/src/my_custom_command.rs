use rfd::FileDialog;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufWriter}, net::{TcpListener, TcpStream}};

use std::{fs, io::{self, Read, Write}, path::{Path, PathBuf}, thread};
use std::fs::File as FilesStd;


use flate2::{write::GzEncoder, Compression};
use std::io::Cursor;
use tar::{Archive, Builder, Header};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;


#[tauri::command(async)]
pub async fn my_custom_command(){
  // Call another async function and wait for it to finish
  println!("Here");
  file().await.unwrap();
// 
}

// async fn file() -> io::Result<()> {
    // let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;
    // println!("Server started");

//     loop {
//         println!("Server started and conneccted");
//         match listener.accept().await {
            
//             Ok((mut socket, addr)) => {
//                 println!("Server started and conneccted 2");
//                 let comp = dog();
//                 println!("{:?}", comp.len());
//                 let total_size = comp.len();
//     let mut sent_size = 0;

                
//                 let chunk_size = 4096;
//     // Read the file in chunks and send over the TCP stream
//     for chunk in comp.chunks(chunk_size) {
//         // Write each chunk to the TCP stream
//         socket.write_all(chunk).await?;
//         sent_size += chunk.len();

//         // Calculate and print the percentage of data sent
//         let percentage = (sent_size as f64 / total_size as f64) * 100.0;
//         println!("Progress: {:.2}%", percentage);
//     }
//     println!("compledted");
    
//     fs::remove_file("C:/Hermes/foo.tar").unwrap();
                
//                 // println!("comp {:?}", comp);
//             //     let mut f = File::open("foo.txt").await?;
//             //     let mut buffer = Vec::new();

//             //    // read the whole file
//             //     f.read_to_end(&mut buffer).await?;
//             //     println!("Recived {:?}", &buffer);
//                 // socket.write_all(&comp).await?;
//             },
//             Err(e) => println!("couldn't get client: {:?}", e),
//         }
//     }

    
// }



// fn dog() -> std::vec::Vec<u8> {
//     // Create a tar file to write to
//     println!("Here 53");

//     if !Path::new("C:/Hermes/").exists() {
//         fs::create_dir("C:/Hermes/");
//     }
    
//     let tar_file = FilesStd::create("C:/Hermes/foo.tar").unwrap();
//     let mut tar_builder = Builder::new(tar_file);

//     // Append a directory to the tar archive
//     let folder = FileDialog::new()
//     .add_filter("folder", &[""])
//     .set_directory("/")
//     .pick_folder();
// let v = folder.unwrap();
// println!("folder: {:?}", v.display());
//     tar_builder.append_dir_all(".", v.display().to_string()).unwrap();

//     // Create data to write into the new file in the tar archive
//     let data = b"Hello, world!";
//     let mut header = Header::new_gnu();
    
//     // Set the header size to the data length
//     header.set_size(data.len() as u64);
//     header.set_cksum();

//     // Append a new file with the data to the tar archive
//     tar_builder.append_data(&mut header, "hello.txt", &mut Cursor::new(data)).unwrap();

//     // Finish writing to the tar archive
//     tar_builder.finish().unwrap();

//     // Read the tar archive contents
//     let tar_contents = fs::read("C:/Hermes/foo.tar").unwrap();
//     println!("Tar archive contents: {} bytes", tar_contents.len());

//     // Extract the tar archive
//     let  tar_file = FilesStd::open("C:/Hermes/foo.tar").unwrap();
//     let mut archive = Archive::new(tar_file);
//     // archive.unpack("extracted").unwrap();
//     let x = fs::read("C:/Hermes/foo.tar");
//     x.unwrap()

    
// }


#[derive(Serialize, Deserialize)]
struct FileEntry {
    path: String,
    contents: Vec<u8>,
}

async fn send_folder(src: &Path, dst: &mut tokio::net::TcpStream) -> io::Result<()> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src).unwrap().to_str().unwrap().to_string();

        if path.is_file() {
            let mut file = fs::File::open(path)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            entries.push(FileEntry {
                path: relative_path,
                contents,
            });
        }
    }

    let serialized = bincode::serialize(&entries).unwrap();
    dst.write_all(&serialized).await?;

    Ok(())
}


async fn file() -> io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;
    println!("Server started");

     

    loop {
        // let (mut socket, _) = listener.accept().await?;
        //  match send_folder(src, &mut socket).await {
        //         Ok(_) => println!("Folder sent successfully!"),
        //         Err(e) => eprintln!("Error sending folder: {}", e),
        //     }

        match listener.accept().await {
            
                        Ok((mut socket, addr)) => {
                            let folder = FileDialog::new()
                            .add_filter("folder", &[""])
                            .set_directory("/")
                            .pick_folder();
                        let v = folder.unwrap();
                        
                        
                            let src = Path::new(&v);
                            println!("Server started and conneccted 2");
                            match send_folder(src, &mut socket).await {
                                Ok(_) => println!("Folder sent successfully!"),
                                Err(e) => eprintln!("Error sending folder: {}", e),
                            }
                        }
                        ,
             Err(e) => println!("couldn't get client: {:?}", e),

        }
        
    }
}
