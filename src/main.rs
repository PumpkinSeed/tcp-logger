use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut str = String::new();
    buf_reader.read_to_string(&mut str).expect("something");
    println!("{}", str);
    //
    // let mut next_end = false;
    // //println!("{}", std::str::from_utf8(buf_reader).unwrap());
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| {
    //         // TODO: Determine end
    //
    //         println!("l: {}", line);
    //         if line.as_str() == "}" {
    //             println!("lofasz");
    //             next_end = true;
    //             return true;
    //         }
    //         println!("geci");
    //
    //         !next_end
    //     })
    //     .collect();

    //println!("Request: {:#?}", http_request);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
