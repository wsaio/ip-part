#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::net::Ipv4Addr;

#[derive(Debug)]
struct IpMateData {
    ip: u32,
    sub_mark: u8,
}

#[derive(Debug)]
pub struct IpPart {
    ip: IpMateData,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d{1,3})$").unwrap();
}

fn u32_to_ipv4(ip: u32) -> Ipv4Addr {
    let mut u: [u8; 4] = [0; 4];

    for i in 0..4 {
        u[i] = ((ip >> (i * 8)) & 0xff) as u8;
    }

    Ipv4Addr::new(u[3], u[2], u[1], u[0])
}

impl IpPart {
    /// 创建IpPart
    /// # Example
    ///
    /// ```rust
    /// let ip = IpPart::new("192.168.1.0/24");
    /// ```
    pub fn new(ip: &str) -> Result<Self, String> {
        if RE.is_match(ip) {
            for cap in RE.captures_iter(ip) {
                let nums: Vec<u8> = [&cap[1], &cap[2], &cap[3], &cap[4], &cap[5]]
                    .iter()
                    .map(|n| n.parse().unwrap())
                    .collect();

                if nums[4] > 32 {
                    return Err(String::from("子网掩码不能大于32"));
                }

                let mut ip: u32 = (nums[0] as u32) << 24;
                ip += (nums[1] as u32) << 16;
                ip += (nums[2] as u32) << 8;
                ip += nums[3] as u32;

                ip = ip & !((1 << (32 - nums[4])) - 1);

                return Ok(IpPart {
                    ip: IpMateData {
                        ip: ip,
                        sub_mark: nums[4],
                    },
                });
            }
        }

        Err(String::from("ip格式错误"))
    }

    /// 返回当前ip段的ip个数
    pub fn size(&self) -> u32 {
        1 << (32 - self.ip.sub_mark)
    }

    /// 返回当前ip段的所有ip
    pub fn list(&self) -> Vec<Ipv4Addr> {
        let mut list: Vec<Ipv4Addr> = Vec::new();

        for i in 0..self.size() {
            list.push(u32_to_ipv4(self.ip.ip + i));
        }

        list
    }
}

#[test]
fn test() {
    let ip = IpPart::new("192.168.1.1/24").unwrap();

    println!("{:?} size {}", ip.list(), ip.size());
}
