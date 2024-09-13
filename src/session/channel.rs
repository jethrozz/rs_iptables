use std::net::TcpStream;
use std::path::Path;
use ssh2::{Channel,Session};
use crate::config::utils::read_config_file;

pub fn get_channel() -> Channel {
    //let host:String , port,user,pwd;
    //使用一个元组变量 my_config 在模式匹配中去赋值
    let mut addr = String::new();
    let mut user = String::new();
    let mut pwd = String::new();
    match read_config_file(Path::new("session_config.toml")) {
        Ok(config) => {
            addr = format!("{}:{}", config.host.host, config.host.port);
            user = config.host.user;
            pwd = config.host.password;
        }
        Err(e) => {
            eprintln!("Error reading config: {}", e);
            std::process::exit(1);
        },
    };

    let tcp = TcpStream::connect(addr).unwrap();
    let mut sess: Session = Session::new().unwrap();
    sess.set_tcp_stream(tcp);

    sess.handshake().unwrap();
    sess.userauth_password(&user, &pwd).unwrap();

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