use std::io::prelude::*;
use std::error::Error;
use std::net::TcpStream;
use ssh2::{Channel,Session};

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 22;
const USER_NAME: &'static str = "root";
const PASSWORD: &'static str = "123456";

pub fn get_channel() -> Channel {
    let tcp = TcpStream::connect((HOST, PORT)).unwrap();
    let mut sess: Session = Session::new().unwrap();
    sess.set_tcp_stream(tcp);

    sess.handshake().unwrap();
    sess.userauth_password(USER_NAME, PASSWORD).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel
    // channel.exec("iptables -L -n --line-numbers").unwrap();
    // let mut iptables_l_n_cmd = String::new();
    // channel.read_to_string(&mut iptables_l_n_cmd).unwrap();
    // println!("{}", iptables_l_n_cmd);
    // close_channel(&mut channel);
}

pub fn close_channel(channel: &mut Channel) {
    channel.close().unwrap();
    channel.wait_close().unwrap();
}