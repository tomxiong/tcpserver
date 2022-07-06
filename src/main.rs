use std::fmt::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let tl = TcpListener::bind("0.0.0.0:12345")?;

    loop {
        for stream in tl.incoming() {
            match stream {
                Ok(stream) => {
                    match handle_stream(&stream) {
                        Ok(ret) => {
                            if ret == 0 {
                                break;
                            } else {
                                continue;
                            }
                        },
                        Err(e) =>
                            println!("Handel stream Error: {e:?}")
                    }
                }
                Err(e)=> {
                    println!("Error for incoming stream: {e:?}");
                    panic!("Panic for {e:?}");
                }
            }
        }
    }
    //Ok(())
}

fn handle_stream(mut stream: &TcpStream) -> Result<u8, Error> {
    loop {
        let mut rbuf = [0;50];
        let br = stream.read(&mut rbuf);
        match br {
            Ok(bl) => {
                if bl == 0 {
                    return Ok(0)
                }
            },
            Err(e) =>
                println!("stream read error : {e:?}"),
        }
        if !rbuf.is_ascii() {
            let reps = String::from("Invalid character");
            println!("From {} say {}", stream.peer_addr().unwrap(), reps);
            stream.write(&rbuf);
            continue
        }

        let cmd = String::from_utf8(rbuf.to_vec());
        match cmd {
            Ok(cmds) => {
                let cmdst = cmds.trim_end().lines().next();
                println!("From {} say {}", stream.peer_addr().unwrap(), cmdst.unwrap());
                if cmdst == Some("exit") {
                    return Ok(0);
                }
                let wr = stream.write(&rbuf);
                match wr {
                    Ok(wl) => {
                        if wl == 0 {
                            return Ok(0)
                        }
                    }
                    Err(e) =>
                        println!("Failed to write back: {e:?}")
                }
            },
            Err(e) => println!("parse input to string error : {e:?}")
        }


    }
}
