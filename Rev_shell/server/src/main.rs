#![allow(unused)]
use core::panic;
use std::{
    io::{stdin, BufRead, BufReader, Write},
    net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream},
    process::{exit, Command},
};

fn main() {
    let mut bind_ip: String = String::new();
    stdin()
        .read_line(&mut bind_ip)
        .expect("Unable to Get Input");

    let bind_ip = bind_ip.trim();

    let mut bind_port: String = String::new();
    stdin()
        .read_line(&mut bind_port)
        .expect("Unable to get port");
    let bind_port = bind_port
        .trim()
        .parse::<u16>()
        .expect("Invalid Port Number");

    let ip = match bind_ip.parse::<Ipv4Addr>() {
        Ok(ip) => ip,
        Err(e) => panic!("{}", e),
    };

    let socket = SocketAddrV4::new(ip, bind_port);
    let listn = TcpListener::bind(socket);

    let listener = match listn {
        Ok(l) => l,
        Err(e) => {
            println!("{:?}", e);
            exit(0);
        }
    };

    let (mut client_socket, clinet_addr) = listener.accept().unwrap();
    println!("Client Connected From : {clinet_addr}");

    loop {
        print!("$ ");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Input Error In Entering Command");

        input.push('\0');

        client_socket.write(input.as_bytes());

        let mut buffer: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&client_socket);
        reader.read_until(b'\0', &mut buffer);
    }

    drop(listener);
}
