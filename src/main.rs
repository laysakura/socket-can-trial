use socketcan::CANSocket;

fn main() {
    let can = CANSocket::open("slcan0").unwrap();

    loop {
        let frame = can.read_frame().unwrap();
        println!("{:?}", frame);
    }
}
