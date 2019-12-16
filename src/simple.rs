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

// 5 翻转字符串
pub fn reverse(input: &str) -> String {
    let mut string = String::new();
    for i in input.chars().rev() {
        string.push(i);
    }
    string
}
/*
pub fn reverse(input: &str) -> String {
   let mut output = String::with_capacity(input.len());
   output.extend(input.chars().rev());
   output
}
*/

// 6 第 n 个素数
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2
    }
    let mut count = 0;
    let is_sushu = |x| {
        let mut result = true;
        for i in 2..x {
            if x % i == 0 {
                result = false;
                break
            }
        }
        result
    };
    let mut goal = 0;
    for i in 3.. {
        if is_sushu(i) {
            count += 1;
            if n == count {
                goal = i;
                break
            }
        }
    }
    goal
}
/*
fn is_prime(n: u32) -> bool {
   let mut i = 3;
   while (i * i) < (n + 1) {
       if n % i == 0 {
           return false;
       }
       i += 1;
   }
   return true;
}

pub fn nth(n: u32) -> u32 {
   if n == 0 {
       2
   } else {
       let mut count = 0;
       let mut candidate = 1;
       while count < n {
           candidate += 2;
           if is_prime(candidate) {
               count += 1;
           }
       }
       candidate
   }
}
*/

/* 7 鲍勃-bob
鲍伯是一个懒散的青少年.在谈话中,他的反应非常有限.

鲍伯回答:”Sure.”，如果你问他一个问题.

他回答:”Whoa, chill out!”，如果你对他大喊大叫.

他回答”Calm down, I know what I’m doing!”，如果你大声问他问题.

他说”Fine. Be that way!”，如果你喊他,而不说任何话.

他回答”Whatever”，给剩下的对话

鲍勃的对话伙伴，在书面交流方面是一个纯粹主义者,并且总是遵循关于 英语句子标点 的通用规则.
*/
pub fn reply(message: &str) -> &str {
    let is_question = |s: &str| s.trim_end().ends_with('?');
    let is_yelling = |s: &str| {
        let s = s.trim_matches(|c: char| !c.is_alphabetic());
        !s.is_empty() && s.to_uppercase() == s
    };
    let is_silence = |s: &str| {
        s.trim().is_empty()
    };

    if is_silence(message) {
        "Fine. Be that way!"
    } else if is_yelling(message) && is_question(message) {
        "Calm down, I know what I'm doing!"
    } else if is_yelling(message) {
        "Whoa, chill out!"
    } else if is_question(message) {
        "Sure."
    } else {
        "Whatever."
    }
}

// 8 啤酒之歌
pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
              Take one down and pass it around, 1 bottle of beer on the wall.\n"
            .to_string(),
        n if n > 2 && n <= 99 => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\n\
             Take one down and pass it around, {n_minus_1} bottles of beer on the wall.\n",
            n = n,
            n_minus_1 = n - 1
        ),
        _ => panic!(),
    }
}
 
pub fn sing(start: i32, end: i32) -> String {
    (end..start + 1)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}

// 9 谚语
pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0 {
        return String::new()
    }
    if list.len() == 1 {
        return String::from(format!("And all for the want of a {}.", list[0]))
    }
    let format = |(first, second)| {
        format!("For want of a {} the {} was lost.", first, second)
    };
    let mut expected: Vec<String> = Vec::with_capacity(list.len());
    for i in 0..list.len()-1 {
        expected.push(format((list[i], list[i+1])));
    }
    expected.push(format!("And all for the want of a {}.", list[0]));
    expected.join("\n")
}
/*
pub fn build_proverb(items: Vec<&str>) -> String {
   let mut stanzas = Vec::with_capacity(items.len());
   for index in 0..items.len() {
       if index == items.len() - 1 {
           stanzas.push(format!("And all for the want of a {}.", items[0]));
       } else {
           stanzas.push(format!(
               "For want of a {} the {} was lost.",
               items[index],
               items[index + 1]
           ));
       }
   }
   stanzas.join("\n")
}
*/

// 10 求，前n个自然数的和平方，与，平方和，之间的差值
pub fn square_of_sum(n: u32) -> u32 {
    let result: u32 = (1..n+1).sum();
    result * result
 }
 
 pub fn sum_of_squares(n: u32) -> u32 {
    (1..n+1).map(|i| i * i).collect::<Vec<u32>>().iter().sum()
 }
 
 pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
 }
 /*
 pub fn square_of_sum(n: u32) -> u32 {
   let sum = n * (n + 1) / 2;
   sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
   (0..n + 1).map(|x| x * x).fold(0, |accum, x| accum + x)
}

pub fn difference(n: u32) -> u32 {
   square_of_sum(n) - sum_of_squares(n)
}
 */

/* 11 倍数之和
给定一个数字,找出另外的特定数字的所有唯一倍数的总和,但不包括第一个数字.
如果我们列出20以下，3或5的倍数的所有自然数,我们得到3,5,6,9,10,12,15和18.
这些倍数的总和是78
*/
use std::collections::BTreeSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: BTreeSet<u32> = BTreeSet::new();

    for i in factors {
        let mut multiple = 2;
        let mut x = *i;
        while x < limit {
            multiples.insert(x);
            x = i * multiple;
            multiple += 1;
        }
    }

    multiples.iter().sum()
}

/* 12 谷物
    计算棋盘上的小麦粒数,假设每个方格的数量增加一倍.
    曾经有一位睿智的仆人拯救了王子的生命。国王承诺支付仆人梦寐以求的一切。知道国王喜欢国际象棋,仆人告诉国王他想吃小麦粒，在棋盘的第一个正方形上放一粒小麦。而接下来的方格是两粒.四粒小麦放在第三格,依此类推.
    棋盘上有 64 个方格.
    编写代码，用来显示:
        每个方格上有多少谷物,和
        谷物总数
*/
pub fn square(s: u32) -> u64 {
    let mut result: u64 = 1;
    for _ in 0..s-1 {
        result *= 2
    }
    result
}
pub fn total() -> u64 {
    (1..=64).map(|s| square(s)).fold(0, |accum, s| accum + s)
}
/*
pub fn square(s: u32) -> u64 {
   if s == 0 || s > 64 {
       panic!("Square must be between 1 and 64");
   }

   2u64.pow(s - 1)
}

pub fn total() -> u64 {
   (1..65).fold(0, |acc, s| acc + square(s))
}
*/

/* 13 勾股数
    请找出 a，b，c，恰好符合勾股定理, 而其中 a+b+c=1000.
    返回， a * b * c 值.
*/
pub fn find() -> Option<u32> {
    for a in 1..1000 {
        for b in (a + 1)..(1000 - a) {
            let c = 1000 - (a + b);
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}

/* 14 素数因子
    计算给定自然数的素因子
*/
pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut n = n;
    let mut index = 2;
    while n != 1 {
        for i in index.. {
            if n % i == 0 {
                n /= i;
                index = i;
                result.push(i);
                break;
            }
        }    
    }
    result
 }
 /*
 pub fn factors(n: u64) -> Vec<u64> {
   let mut val = n;
   let mut out: Vec<u64> = vec![];
   let mut possible: u64 = 2;
   while val > 1 {
       while val % possible == 0 {
           out.push(possible);
           val /= possible;
       }
       possible += 1;
   }
   out
}
*/

/* 15 系列
    给定一串数字,输出所有连续的n长度顺序子串。

    例如,字符串”49142”长度为3的子串系列:

    “491”
    “914”
    “142”
    以下长度为 4 的 系列:

    “4914”
    “9142”
    如果你要求一个 5 位数字，长度为 6 的系列,你应该得到原数字。
*/
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    if digits.len() < len {
        return out;
    }
    for i in 0..(digits.len()-len+1) {
        out.push(digits.get(i..(i+len)).unwrap().to_string());
    }
    out
} 
/*
pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        _ => digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|window| window.into_iter().collect::<String>())
            .collect::<Vec<String>>(),
    }
 }
*/

/* 16 水仙花数
    一个数字,它是自身每个单数字与数字量的幂，之和
    153 是水仙花数,因为:153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
    154 不是一个水仙花数,因为:154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
*/
pub fn is_armstrong_number(num: u32) -> bool {
    // let out: Vec<u32> = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let out = num.to_string();
    let len = out.len() as u32;
    // out.into_iter().fold(0, |accum, x| accum + x.pow(len)) == num
    out.chars()
        .map(|c| c.to_digit(10).unwrap().pow(len))
        .sum::<u32>() == num
}

/* 17 考拉兹猜想
    是指对于每一个正整数，如果它是奇数，则对它乘 3 再加 1，如果它是偶数，则对它除以 2，如此循环，最终都能够得到 1。

    给予一个数字n，那它到达 1 的步骤
*/
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut n = n;
    let mut step: u64 = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n = n * 3 + 1
        }
        step += 1;
    }
    Some(step)
}
/*
pub fn collatz_positive(n: u64) -> u64 {
   if n == 1 {
       0
   } else {
       1 + match n % 2 {
           0 => collatz_positive(n / 2),
           _ => collatz_positive(n * 3 + 1),
       }
   }
}

// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Option<u64> {
   if n < 1 {
       None
   } else {
       Some(collatz_positive(n))
   }
}
*/

// 18 迪菲-赫尔曼
use rand::{thread_rng, Rng};


/// Right-to-left modular exponentiation implementation
/// For more information see https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
   let mut result = 1;

   let mut e = exponent;

   let mut b = base;

   while e > 0 {
       if (e & 1) == 1 {
           result = (result * b) % modulus;
       }

       e >>= 1;

       b = (b * b) % modulus;
   }

   result
}

pub fn private_key(p: u64) -> u64 {
   let mut rng = thread_rng();
   rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
   modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
   modular_exponentiation(b_pub, a, p)
}