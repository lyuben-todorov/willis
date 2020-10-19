use std::net::{TcpStream, SocketAddr};
use piko::client::{ClientReq, ClientRes};
use std::io::{Read, Write};
use byteorder::{WriteBytesExt, ReadBytesExt};

mod tests;

fn main() {
}

pub fn write_req(stream: &mut TcpStream, client_req: ClientReq) {
    let req = serde_cbor::to_vec(&client_req).unwrap();

    let size = req.len();

    stream.write_u8(size as u8).unwrap();
    stream.write_all(req.as_slice()).unwrap();
}


pub fn read_res(stream: &mut TcpStream) -> ClientRes {
    let size = stream.read_u8().unwrap();

    let mut buf = vec![0u8; size as usize];
    stream.read_exact(&mut buf).unwrap();
    let res: ClientRes = serde_cbor::from_slice(buf.as_slice()).unwrap();
    res
}

fn input(address: &SocketAddr, req: ClientReq) -> ClientRes {
    let mut stream = TcpStream::connect(address).unwrap();

    write_req(&mut stream, req);

    read_res(&mut stream)
}