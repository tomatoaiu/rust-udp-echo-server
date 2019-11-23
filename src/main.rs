use std::env;
use std::net::UdpSocket;
use std::fmt::Debug;

fn main() -> std::io::Result<()> {
    {
        let args: Vec<String> = env::args().collect();
        let port = &args[1]; // e.g. 34255
        let send_to = &args[2]; // e.g. 127.0.0.1:4242

        let mut socket = UdpSocket::bind("0.0.0.0:".to_string() + &port.to_string())?;
        println!("\nstart server\n");

        loop {
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)?;
            println!("From: {:?}, buf: {:?}", src, buf);

            socket.send_to(&buf, send_to)?;
        }
    }
    Ok(())
}
