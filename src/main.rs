use std::fmt::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let tl = TcpListener::bind("0.0.0.0:12345")?;

   /* match tl.accept() {
        Ok((_socket, addr)) => println!("This is new client from {addr:?}"),
        Err(e) => println!("Failed to get client : {e:?}"),
    }*/
    for stream in tl.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(&stream);
            }
            Err(e)=> {
                println!("Error for incoming stream: {e:?}");
                panic!("Panic for {e:?}");
            }
        }
    }
    //println!("Hello, world!");
    Ok(())
}

fn handle_stream(mut stream: &TcpStream) -> Result<(), Error> {
    let mut rbuf = [0;50];
    loop {
        let br = stream.read(&mut rbuf);
        match br {
            Ok(bl) => {
                if bl == 0 {
                    return Ok(())
                }
            },
            Err(e) => println!("error : {e:?}"),
        }
        stream.write(&rbuf);
        let cmd = String::from_utf8(rbuf.to_vec());
        match cmd {
            Ok(cmds) => {
                let cmdst = cmds.trim_end();
                if cmdst == "exit" {
                    return Ok(())
                }
            },
            Err(e) => println!("error : {e:?}"),
        }
        let mut rbuf = [0;50];
        //println!("rec message: {command:?}");
    }
    //Ok(())
}
