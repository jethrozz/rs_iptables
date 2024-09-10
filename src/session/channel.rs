use std::io::prelude::*;
use std::error::Error;
use std::net::TcpStream;
use ssh2::{Channel,Session};
use crate::config::utils::read_config_file;

pub fn get_channel() -> Channel {
    //let host:String , port,user,pwd;
    match read_config_file("config.toml") {
        Ok(config) => {
            config.server.host
        }
        Err(e) => {
            eprintln!("Error reading config: {}", e);
            std::process::exit(1);
        },
    };

    let tcp = TcpStream::connect((config., PORT)).unwrap();
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