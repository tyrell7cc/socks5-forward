use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};


const VERSION:u8=0x05;
const CONNECT: u8 = 0x01;
const BIND: u8 = 0x02;
const UDP_ASSOCIATE: u8 = 0x03;


fn main() {
    let listen = TcpListener::bind("0.0.0.0:8088").unwrap();
    for stream in listen.incoming() {
        std::thread::spawn(move || {
            handle(stream.unwrap());
        });
    }
}

fn handle(mut stream:TcpStream){
    //握手
    let mut bs = [0;50];

    let mut b1:[u8;1] = [0;1];
    stream.read_exact(&mut b1);
    // println!("握手版本是：{}",b1[0]);

    stream.read_exact(&mut b1);
    // println!("握手方法支持：{} 个",b1[0]);

    let mut bs=[0;2];
    stream.read_exact(&mut bs);
    // println!("支持的第1个方法是:{}",bs[0]);
    // println!("支持的第2个方法是:{}",bs[1]);

    stream.write(&[0x05,0x02]);
    // println!("响应写回，选择用户名密码验证");


    let mut bs=[0;1];
    stream.read_exact(&mut bs).unwrap();
    stream.read_exact(&mut bs);
    let len = bs[0];
    // println!("用户名长度是:{}",len);

    let mut bs=[0;5];
    stream.read_exact(&mut bs);

    let username = String::from_utf8_lossy(&bs);
    // println!("用户名是:{}",username);


    let mut bs=[0;1];
    stream.read_exact(&mut bs);
    // println!("用户密码长度是:{}",bs[0]);

    let mut bs=[0;6];
    stream.read_exact(&mut bs);
    let password = String::from_utf8_lossy(&bs);
    // println!("密码是:{}",password);

    if username!="xxXxx"||password!="abcfed" {
        stream.write(&[0x01,0x01]);
        println!("用户名密码验证错误");
        return;
    }

    stream.write(&[0x01,0x00]);
    // println!("\n一次结束\n");

    //与客户端认证完成，开始连接1080
    connect_1080(stream);
}



fn connect_1080(mut client:TcpStream){
    let mut server = TcpStream::connect("localhost:1080").unwrap();
    server.write(&[0x05,0x02,0x00,0x02]);

    let mut bs = [0;2];
    server.read_exact(&mut bs);
    // println!("{} {}",bs[0],bs[1]);

    let mut server_copy = server.try_clone().unwrap();
    let mut client_copy = client.try_clone().unwrap();

    //从client读，向server写
    std::thread::spawn(move|| {
        // println!("从client读，向server写");
        let mut bs =[0;10240];
        loop {
            match client.read(&mut bs) {
                Ok(n)=>{
                    if n==0 {
                        // println!("c端传输完成");
                        break;
                    }
                    server.write(&mut bs[0..n]);
                }
                Err(err)=>{
                    println!("{:?}",err);
                    break;
                }
            }
        }
    });

    //从server读，向client写
    // println!("从server读，向client写");
    let mut bs =[0;10240];
    loop {
        match server_copy.read(&mut bs) {
            Ok(n)=>{
                if n==0 {
                    // println!("c端传输完成");
                    break;
                }
                client_copy.write(&mut bs[0..n]);
            }
            Err(err)=>{
                println!("{:?}",err);
                break;
            }
        }
    }
}