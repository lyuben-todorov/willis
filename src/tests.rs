use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, SocketAddr};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use piko::client::{ClientRes, ClientReq};
    use crate::input;
    use crate::tests::get_adrr;
    use std::time::Instant;

    const CLIENT_ID: u64 = 0000; //test id

    #[test]
    fn test_throughput_single_node() {
        let runs: usize = 1000;
        let socket_addr = get_adrr(8878);
        let timer = Instant::now();
        for i in 0..runs {
            let req = ClientReq::publ(CLIENT_ID, i.to_string().as_bytes().to_vec());
            let res = input(&socket_addr, req);
            match res {
                ClientRes::Error { message: _ } => { assert!(false) }
                _ => {}
            }
            if i % (runs / 10) == 0 { println!("{}%", 100 * i / runs) }
        }
        println!("Test took {:.2?}, avg {} ops/s", timer.elapsed(), runs as f64 / timer.elapsed().as_secs_f64())
    }

    #[test]
    fn test_throughput_two_nodes() {
        let runs: usize = 1000;
        let half = runs / 2;
        let socket_addr1 = get_adrr(8878);
        let socket_addr2 = get_adrr(8879);

        let timer = Instant::now();
        let thread1 = std::thread::spawn(move || {
            for i in 0..half {
                let req = ClientReq::publ(CLIENT_ID, i.to_string().as_bytes().to_vec());
                let res = input(&socket_addr1, req);
                match res {
                    ClientRes::Error { message: _ } => { assert!(false) }
                    _ => {}
                }
            }
        });
        let thread2 = std::thread::spawn(move || {
            for i in half..runs {
                let req = ClientReq::publ(CLIENT_ID, i.to_string().as_bytes().to_vec());
                let res = input(&socket_addr2, req);
                match res {
                    ClientRes::Error { message: _ } => { assert!(false) }
                    _ => {}
                }
            }
        });
        thread1.join().unwrap();
        thread2.join().unwrap();

        let wait_req = ClientReq::WaitUntilClear { client_id: CLIENT_ID };
        input(&socket_addr1, wait_req);
        let wait_req = ClientReq::WaitUntilClear { client_id: CLIENT_ID };
        input(&socket_addr2, wait_req);

        println!("Test took {:.2?}, avg {} ops/s", timer.elapsed(), (runs as f64) / timer.elapsed().as_secs_f64())
    }
}

fn get_adrr(port: u16) -> SocketAddr {
    let ip = IpAddr::from_str("0.0.0.0").unwrap();
    let address = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::from_str(ip.to_string().as_str()).unwrap(), port));
    address
}