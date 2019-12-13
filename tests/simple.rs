// 1 输出 Hello world!
pub fn hello() -> &'static str {
    "Hello, World!"
 }

// 2 返回指定时间1亿秒后的时间
use chrono::{DateTime, Duration, Utc};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}

// 3 判定指定年是否闰年
pub fn is_leap_year(year: i32) -> bool {
    if year % 100 != 0 && year % 4 == 0 {
        true
    } else if year % 100 == 0 && year % 400 == 0 {
        true
    } else {
        false
    }
}
/*
pub fn is_leap_year(year: i32) -> bool {
   let has_factor = |n| year % n == 0;
   has_factor(4) && (!has_factor(100) || has_factor(400))
}
*/

/* 4 雨滴-Raindrops
    把一个数字转换成一个字符串,它的内容取决于，数字的因素.
    如果数字有 3 作为一个因素,输出”Pling”.
    如果数字有 5 作为一个因素,输出”Plang’”.
    如果数字有 7 作为一个因素,输出”Plong”.
    如果数字没有 3, 5,或 7 作为一个因素, 直接给数字
*/
pub fn raindrops(n: u32) -> String {
    if n % 3 == 0 {
        String::from("Pling")
    } else if n % 5 == 0 {
        String::from("Plang")
    } else if n % 7 == 0 {
        String::from("Plong")
    } else {
        String::from(n.to_string())
    }
}
/*
pub fn raindrops(n: u32) -> String {
   let is_pling = |n| n % 3 == 0;
   let is_plang = |n| n % 5 == 0;
   let is_plong = |n| n % 7 == 0;
   let mut drops = String::new();
   if is_pling(n) {
       drops.push_str("Pling");
   }
   if is_plang(n) {
       drops.push_str("Plang");
   }
   if is_plong(n) {
       drops.push_str("Plong");
   }
   if drops.is_empty() {
       let s = format!("{}", n);
       drops.push_str(&s);
   }
   drops
}
*/