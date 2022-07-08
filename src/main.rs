use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum MessageDataErros {
    NoAsciiMessage,
    InvalidFormat,
}

fn main() -> std::io::Result<()> {
    // The tcp server bind a IP and port for listen incoming connection.
    let tl = TcpListener::bind("0.0.0.0:12345")?;
    // We use loop at here for it can listen other connection after the previous one disconnected.
    loop {
        // start to listen the incoming connection and it's data
        for stream in tl.incoming() {
            // Check the stream of incoming, if it is Error, the server will exit with panic,
            // otherwise it will handle the result with handle_stream() function.
            match stream {
                // If the stream is ok, then handle it
                Ok(stream) => {
                    // It will println the new client IP and port when the client connected.
                    println!("Connected from client: {}", stream.peer_addr().unwrap());
                    // Then parse the data from the stream
                    match handle_stream(&stream) {
                        //If the client disconnect normally, so it will close the connection and wait for other
                        Ok(ret) => {
                            println!("Disconnected client: {}", stream.peer_addr().unwrap());
                            break;
                        }
                        //If the client send no ascii data, the server will print the error message and skip it
                        Err(MessageDataErros::NoAsciiMessage) => {
                            println!("Handle stream Error while message is not ascii.");
                            continue;
                        }
                        //If the data from client cannot be parsed,the server will show error and skip it.
                        Err(MessageDataErros::InvalidFormat) => {
                            println!("Handel stream Error with message invalid format.");
                            continue;
                        }
                    }
                }
                // The server will exit with panic when the network has issues.
                Err(e) => {
                    println!("Error for incoming stream: {e:?}");
                    panic!("Panic for {e:?}");
                }
            }
        }
    }
}

// Parse the data from the tcp stream, After print it to server console and then send back
fn handle_stream(mut stream: &TcpStream) -> Result<u8, MessageDataErros> {
    // The server continue to handle the stream with each data received until the client close
    // or send "exit" message.
    loop {
        //First step is to prepare the buffer to store the data from the tcp stream.
        let mut rbuf = [0; 1024];
        // Then it read data from tcp stream and store data to the buffer.
        let br = stream.read(&mut rbuf);
        // Check the result of read
        match br {
            Ok(bl) => {
                // If the size of read is zero, so it means the connection may closed.
                if bl == 0 {
                    println!("stream read end");
                    return Ok(0);
                }
            }
            // It will print error message while fail to read the stream.
            Err(e) =>
                println!("stream read error : {e:?}"),
        }
        // Now, it just support ascii message
        if !rbuf.is_ascii() {
            //it will show invalid character in console and skip it when the data is not ascii
            let reps = String::from("Invalid character");
            println!("From {} input {} , skip", stream.peer_addr().unwrap(), reps);
            continue;
        }
        // It parse the data to uft8 string
        let cmd = String::from_utf8(rbuf.to_vec());
        match cmd {
            Ok(cmds) => {
                //It just pick the data with end of line
                let cmdst = cmds.trim_end().lines().next();
                //It will print the data from client if the data has content and not only end line flag
                if cmdst.is_some() && cmdst.unwrap().len() > 0 {
                    println!("From {} say {}", stream.peer_addr().unwrap(), cmdst.unwrap());
                }
                //It will close the connection while the client send "exit"
                if cmdst == Some("exit") {
                    return Ok(0);
                }
                //Then it will write back the data from client if the data is valid.
                let wr = stream.write(&rbuf);
                //Check the result of write back action
                match wr {
                    Ok(wl) => {
                        //If the size of write is zero, it will close the connection.
                        if wl == 0 {
                            return Ok(0);
                        }
                    }
                    Err(e) =>
                        //If the write has wrong, then skip it.
                        println!("Failed to write back: {e:?}")
                }
            }
            Err(e) => {
                //If parse the data to string has error, it will print it and return error
                println!("parse input to string error : {e:?}");
                return Err(MessageDataErros::InvalidFormat);
            }
        }
    }
}
