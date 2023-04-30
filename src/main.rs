use std::process::Command;
use std::thread;

fn main() {
    let mut ss = Command::new("./ss/sslocal.exe")
        .arg("-b")
        .arg("127.0.0.1:3128")
        .arg("--protocol")
        .arg("http")
        .arg("-s")
        .arg("1.1.1.1:8388")
        .arg("-m")
        .arg("aes-128-gcm")
        .arg("-k")
        .arg("114512")
        .spawn()
        .expect("无法启动代理");

    let ss_handle = thread::spawn(move || {  //创建一个新的线程，并将 ss 从主线程移动到新创建的线程中等待子进程结束。
        ss.wait().expect("进程错误")
    });

    Command::new("./browser/chrome.exe")
        .arg("--proxy-server=http://127.0.0.1:3128")
        .arg("https://www.office.com")
        .output()
        .expect("浏览器启动错误");

    ss_handle.join().expect("线程错误");    //在主程序退出之前使用 join 方法等待新创建的线程结束。
                             
    //ss.kill().expect("警告：无法关闭 Shadow socks ！")
}
