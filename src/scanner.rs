use crate::banner_grab::banner_grab;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::ops::RangeInclusive;
use std::thread::spawn;
use std::time::Duration;

#[allow(unused)]
pub fn scan_ports(
    target: Ipv4Addr,
    ports: RangeInclusive<u16>,
    thread_count: usize,
    t_out: u16,
    grab: bool,
) -> Vec<(u16, Option<String>)> {
    let total_ports: u16 = ports.end() - ports.start() + 1;

    // ceil division
    let ports_per_thread = (total_ports as usize + thread_count - 1) / thread_count;

    // The .start() and .end() returns &Idx, in this case &u16, so deferencing it is necessary
    let mut handles: Vec<std::thread::JoinHandle<Vec<(u16, Option<String>)>>> = vec![];
    let start_port = *ports.start();
    let end_port = *ports.end();

    // create the spliting of ports for threading
    for i in 0..thread_count {
        let chunk_start: u16 = start_port + (i * ports_per_thread) as u16;
        let chunk_end: u16 = std::cmp::min(chunk_start + ports_per_thread as u16 - 1, end_port);

        let handler  = spawn(move || {
            let mut open_ports: Vec<(u16, Option<String>)> = vec![];

            for port in chunk_start..=chunk_end {
                let sock_addr = SocketAddrV4::new(target, port);

                if TcpStream::connect_timeout(
                    &sock_addr.into(),
                    Duration::from_millis(t_out as u64),
                )
                .is_ok()
                {
                    let banner = if grab {
                        // Use a slightly longer timeout for banner read than for connect
                        banner_grab(target, port, t_out as u64 * 2)
                    } else {
                        None
                    };
                    open_ports.push((port, banner));
                }
            }
            open_ports
        });
        handles.push(handler);
    }

    // collect results from thread
    let mut result = vec![];

    for handle in handles {
        let open_ports = handle.join().unwrap();
        result.extend(open_ports);
    }

    result.sort_unstable_by_key(|(port, _)| *port);

    result // All open ports
}
