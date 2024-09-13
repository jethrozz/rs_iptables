use std::collections::HashMap;


pub struct IpTable{
    pub filter : Table,
    pub nat : Table,
    pub mangle : Table,
    pub raw : Table,
    pub security : Table,
}
//表 ，表里面有链
pub struct Table{
    pub input : Chain, //INPUT链，处理进入本机的数据包。
    pub docker : Chain, //docker链
    pub output : Chain, //OUTPUT链, 处理本机发出的数据包
    pub forward : Chain, //FORWARD链，处理经过本机但目的地不是本机的数据包。
    pub post_routing : Chain, //POSTROUTING链，处理离开本机、在路由之后的数据包。通常用于SNAT（源网络地址转换）和MASQUERADE。
    pub pre_routing : Chain, //PREROUTING链，处理到达本机、在路由决策之前的数据包。通常用于DNAT（目标网络地址转换）。
    pub other_chain_map : HashMap<&str, Chain>, //其他自定义链
}

//链
pub struct Chain{
    pub policies : Vec<Policy>, //策略集合
}

//策略
pub struct Policy{
    pub source: &str,  //来源地址
    pub destination: &str, // 目的地址
    pub action: &str,  //行为：DROP,ALLOW,REFUSE等
    pub protocol: &str, //协议：tcp,udp
    pub dport: &u32, // 目标端口
    pub sport: &u32, //源端口
    pub icmp_type : &str, //icmp类型
}
