use piko::client::{ClientReq, ClientRes};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, TcpStream, SocketAddr};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use piko::client::{ClientRes, ClientReq};
    use crate::input;
    use crate::tests::get_adrr;
    use std::time::Instant;

    const CLIENT_ID: u64 = 0000; //test id

    #[test]
    fn test_throughput_two_nodes() {
        let runs: usize = 1000;
        let socket_addr = get_adrr();
        let timer = Instant::now();
        for i in 0..runs {
            let req = ClientReq::publ(CLIENT_ID, i.to_string().as_bytes().to_vec());
            let res = input(&socket_addr, req);
            match res {
                ClientRes::Error { message } => { assert!(false) }
                _ => {}
            }
            if i % (runs/10) == 0 {println!("{}%", 100*i/runs)}
        }
        println!("Test took {:.2?}, avg {} ops/s", timer.elapsed(), runs as f64/timer.elapsed().as_secs_f64())
    }
}

fn get_adrr() -> SocketAddr {
    let port: u16 = 8878;
    let ip = IpAddr::from_str("0.0.0.0").unwrap();
    let address = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::from_str(ip.to_string().as_str()).unwrap(), port));
    address
}