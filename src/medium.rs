/* 1 鞍点
    检测矩阵中的鞍点(saddle point).

    所以说你有一个像这样的矩阵:

        0  1  2
    |---------
    0 | 9  8  7
    1 | 5  3  2     <--- saddle point at (1,0)
    2 | 6  6  7
    它在(1,0)处有一个鞍点.

    它被称为”鞍点”,因为它是该 行 最大数,也是该 列 的最小数。

    矩阵可以具有零个或多个鞍点.

    您的代码应该能够为任何给定矩阵提供所有鞍点的(可能为空)列表。

    矩阵可以具有不同数量的行和列(非正方形)
*/
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = vec![];
    let row = input.len();
    let column = input[0].len();
    for i in 0..row {
        for j in 0..column {
            let i_max = input[i].iter().max().unwrap();
            let j_min = input.iter().map(|x| x[j]).min().unwrap();
            if input[i][j] == *i_max && input[i][j] == j_min {
                out.push((i, j))
            }
        }
    }
    out
}
/* 2 等值线
    在等值线(又称为”无定形的 Word 是一个字或短语”)没有重复字母的单词或短语的逻辑术语，但允许连字符和空格出现多次
*/
pub fn check(candidate: &str) -> bool {
    let mut result = true;
    if candidate.len() == 0 {
        return true;
    }
    let candidate = candidate.to_lowercase().chars().filter(|c| *c != '-' && *c != ' ').collect::<Vec<char>>();
    for i in 0..(candidate.len()-1) {
        if candidate.get((i+1)..).unwrap().contains(candidate.get(i).unwrap()){
            result = false;
            break;
        }
    }
    result
}
/*
pub fn check(word: &str) -> bool {
   // Filter all non-Alphabetic character out and collect them in a new String
   let normalized_string: String = word.to_lowercase()
       .chars()
       .filter(|c| c.is_alphabetic())
       .collect();

   /* Find the char element from back and front and compare the index.
      If it is the same unique char the index will be the same.*/
   let is_unique = |x: char, word: &str| word.find(x).unwrap() == word.rfind(x).unwrap();

   // Length should be the same if it is a isogram
   normalized_string.len()
       == normalized_string
           .chars()
           .filter(|&x| is_unique(x, normalized_string.as_str()))
           .count()
}
*/

// 3 说
// 提供一个 0 到 999,999,999,999 之间的数字,用英语拼出这个数字
pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let digit_to_string = |x| {
        match x {
            0 => "",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => "",
        }
    };
    let bit = |x| digit_to_string(x % 10).to_string();
    let ten_bit = |n| {
        match n / 10 {
            0 => bit(n).to_string(),
            1 => match n % 10 {
                0 => "ten".to_string(),
                1 => "eleven".to_string(),
                2 => "twelve".to_string(),
                3 => "thirteen".to_string(),
                4 => "fourteen".to_string(),
                5 => "fifteen".to_string(),
                6 => "sixteen".to_string(),
                7 => "seventeen".to_string(),
                8 => "eighteen".to_string(),
                9 => "nineteen".to_string(),
                _ => "".to_string(),
            },
            2 => match n % 10 {
                0 => "twenty".to_string(),
                _ => format!("twenty-{}", bit(n)),
            },
            3 => match n % 10 {
                0 => "thirty".to_string(),
                _ => format!("thirty-{}", bit(n)),
            },
            4 => match n % 10 {
                0 => "fourty".to_string(),
                _ => format!("forty-{}", bit(n)),
            },
            5 => match n % 10 {
                0 => "fifty".to_string(),
                _ => format!("fifty-{}", bit(n)),
            },
            6 => match n % 10 {
                0 => "sixty".to_string(),
                _ => format!("sixty-{}", bit(n)),
            },
            7 => match n % 10 {
                0 => "seventy".to_string(),
                _ => format!("seventy-{}", bit(n)),
            },
            8 => match n % 10 {
                0 => "eighty".to_string(),
                _ => format!("eighty-{}", bit(n)),
            },
            9 => match n % 10 {
                0 => "ninety".to_string(),
                _ => format!("ninety-{}", bit(n)),
            },
            _ => "".to_string(),
        }
    };
    let hundred_bit = |x| {
        match x {
            0 => "".to_string(),
            _ => format!("{} hundred", digit_to_string(x)).to_string(),
        }
    };
    let mut n_list: Vec<u64> = vec![];
    let mut n = n;
    while n / 1000 != 0 {
        n_list.push(n % 1000);
        n = n / 1000;
    };
    n_list.push(n);
    let mut n_string_list: Vec<String> = vec![];
    for x in n_list.into_iter() {
        n_string_list.push(format!("{} {}", hundred_bit(x / 100), ten_bit(x % 100)));
    };
    let unit_list = vec!["", "thousand", "million", "billion"];
    let mut n_string_unit_list: Vec<String> = vec![];
    for i in 0..n_string_list.len() {
        if n_string_list[i] == " ".to_string() {
            continue;
        }
        n_string_unit_list.push(format!("{} {}", n_string_list[i].trim(), unit_list[i]));
    };
    n_string_unit_list.reverse();
    format!("{}", n_string_unit_list.join(" ").trim())
}
/*
const SMALL: &'static [&'static str] = &[
   "zero",
   "one",
   "two",
   "three",
   "four",
   "five",
   "six",
   "seven",
   "eight",
   "nine",
   "ten",
   "eleven",
   "twelve",
   "thirteen",
   "fourteen",
   "fifteen",
   "sixteen",
   "seventeen",
   "eighteen",
   "nineteen",
];

const TENS: &'static [&'static str] = &[
   "ones", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALE: &'static [&'static str] = &[
   "",
   "thousand",
   "million",
   "billion",
   "trillion",
   "quadrillion",
   "quintillion",
];

pub fn encode(n: u64) -> String {
   if n < 20 {
       SMALL[n as usize].to_string()
   } else if n < 100 {
       let small = n % 10;
       let mut out = String::from(TENS[n as usize / 10]);
       if small > 0 {
           out.push('-');
           out.push_str(SMALL[small as usize]);
       }
       out
   } else if n < 1000 {
       let mut out = String::from(SMALL[n as usize / 100]);
       out.push_str(" hundred");
       let ones = n % 100;
       if ones > 0 {
           out.push(' ');
           out.push_str(&encode(ones));
       }
       out
   } else {
       let mut sets: Vec<u64> = Vec::new();
       let mut val = n;
       while val >= 1 {
           sets.push(val % 1000);
           val /= 1000;
       }
       let mut out = String::new();
       while let Some(modu) = sets.pop() {
           let len = sets.len();
           if modu == 0 {
               continue;
           }
           if out.len() > 0 {
               out.push(' ');
           }
           out.push_str(&encode(modu));
           if len > 0 {
               out.push(' ');
               out.push_str(SCALE[len]);
           }
       }
       out
   }
}
*/

/* 4 运行游程编码
实现编码和解码.
游程编码(RLE)是一种简单的数据压缩形式，其中运行的(连续数据元素)仅由一个数据值和计数代替。
例如,我们可以只用 13 字节，就可以 代表原始的 53 个字符.
"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"  ->  "12WB12W3B24WB"
RLE 允许从压缩数据中，完美地重建原始数据,这使其成为无损数据压缩.
"AABCCCDEEEE"  ->  "2AB3CD4E"  ->  "AABCCCDEEEE"
为简单起见,您可以假设未编码的字符串，仅包含字母 A 到 Z(小写或大写)和空格。这样,要编码的数据将永远 不包含 任何数字,并且要解码的数据内的数字始终表示后续字符的计数
*/
pub fn encode_3(source: &str) -> String {
    if source == "" {
        return "".to_string();
    }
    let source = source.chars().collect::<Vec<char>>();
    let mut out = String::from("");
    let mut len = 0;
    let mut c = source[0];
    for i in source.iter() {
        if *i == c {
            len += 1;
        } else {
            if len > 1 {
                out.push_str(&len.to_string());
            }
            out.push(c);
            c = *i;
            len = 1;
        }
    }
    if len > 1 {
        out.push_str(&len.to_string());
    }
    out.push(c);
    out.to_string()
}
pub fn decode(source: &str) -> String {
    let source= source.chars().collect::<Vec<char>>();
    let mut num_list: Vec<u32> = vec![];
    let mut char_list: Vec<char> = vec![];
    let mut num: u32 = 1;
    let mut flag = false;
    for i in 0..(source.len()) {
        if source[i].is_ascii_digit() {
            if flag {
                num = num * 10 + source[i].to_digit(10).unwrap();
            } else {
                num = source[i].to_digit(10).unwrap();
            }
            flag = true;
        } else {
            flag = false;
            num_list.push(num);
            num = 1;
            char_list.push(source[i]);
        }
    }
    let mut out = String::from("");
    for i in 0..(num_list.len()) {
        for _ in 0..(num_list[i]) {
            out.push(char_list[i]);
        }
    }
    out.to_string()
}
/*
use std::cmp;

pub fn encode(input: &str) -> String {
   input
       .chars()
       .fold(
           (String::new(), ' ', 0, 1),
           |(mut acc, last, last_n, pos), c| {
               // acc = where answer is accumulated
               // last = last character read
               // last_n = accum count for last
               if c == last {
                   if pos == input.len() {
                       // end of string
                       acc += (last_n + 1).to_string().as_str();
                       acc.push(c);
                   }
                   (acc, last, last_n + 1, pos + 1)
               } else {
                   if last_n > 1 {
                       acc += last_n.to_string().as_str();
                   }
                   if last_n > 0 {
                       // ignore initial last (single whitespace)
                       acc.push(last);
                   }
                   if pos == input.len() {
                       // end of string
                       acc.push(c);
                   }
                   (acc, c, 1, pos + 1)
               }
           },
       )
       .0
}

pub fn decode(input: &str) -> String {
   input
       .chars()
       .fold((String::new(), 0), |(mut acc, last_n), c| {
           if let Some(d) = c.to_digit(10) {
               (acc, 10 * last_n + d)
           } else {
               acc += c.to_string().repeat(cmp::max(last_n, 1) as usize).as_str();
               (acc, 0)
           }
       })
       .0
}
*/

/* 5 ISBN 检测器
ISBN-10 格式是 9 位数字(0 到 9)加上一个校验字符(一个数字或一个 X)。在校验字符为 X 的情况下,这表示值”10”。这些可以与连字符(不管有没有)通信,并且可以通过以下公式检查它们的有效性:
(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0
如果结果是 0,那么它是一个有效的 ISBN-10,否则它是无效的.

给定一个字符串,程序应该检查所提供的字符串是否是有效的 ISBN-10。为了实现这一点，需要在计算 ISBN 的校验位数之前，考虑字符串的预处理/解析.

该程序应该能够验证 ISBN-10 的破折号(不管有没有).
*/
pub fn is_valid_isbn(isbn: &str) -> bool {
    let out = isbn.chars().filter(|c| *c != '-').collect::<Vec<char>>();
    let mut sum = 0;
    let len = out.len();
    for i in 0..(len-1) {
        if out[i].is_ascii_digit() {
            sum += out[i].to_digit(10).unwrap() * (10 - i as u32);
        } else {
            return false;
        }
    }
    let last_char = out[len-1];
    if !last_char.is_ascii_digit() && last_char != 'X' {
        return false;
    }
    if last_char == 'X' {
        sum += 10;
    } else {
        sum += last_char.to_digit(10).unwrap();
    }
    if sum % 11 == 0 {
        true
    } else {
        false
    }
}
/*
/// An ISBN type
#[derive(PartialEq, Eq)]
enum IsbnType {
   Isbn10,
   Isbn13,
}

/// Checks if an 'X' is valid at the given position for the given ISBN type
#[allow(non_snake_case)]
fn is_X_valid(position: &usize, isbn_type: &IsbnType) -> bool {
   (isbn_type == &IsbnType::Isbn10 && position == &9)
       || (isbn_type == &IsbnType::Isbn13 && position == &12)
}

/// Checks if a '-' is valid at the given position for the given ISBN type
fn is_dash_valid(position: &usize, isbn_type: &IsbnType) -> bool {
   isbn_type == &IsbnType::Isbn13 && (position == &1 || position == &5 || position == &11)
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
   let isbn_type = match isbn.len() {
       10 => IsbnType::Isbn10,
       13 => IsbnType::Isbn13,
       _ => return false,
   };

   let mut checksum = 0;
   let mut coefficient = 10;
   for (position, c) in isbn.char_indices() {
       let digit_value = match c {
           '0'...'9' => c.to_digit(10).unwrap(),
           'X' if is_X_valid(&position, &isbn_type) => 10,
           '-' if is_dash_valid(&position, &isbn_type) => continue,
           _ => return false,
       };

       checksum += coefficient * digit_value;
       coefficient -= 1;
   }

   checksum % 11 == 0
}
*/

/* 6 完全数
根据 Nicomachus’(60-120CE)的自然数分类方案，确定一个数是- Perfect(完全), Abundant(过剩数), Deficient(亏数)

希腊数学家Nicomachus设计了一种自然数的分类方案,将每一个数识别归类为 perfect, abundant, or deficient ，方案基于aliquot sum（等值和）。 等值和定义为不包括数字本身的约数(可除出整数)的总和。例如,15 的等值和是(1+3+5)=9.

Perfect(完全): aliquot sum = number
6 ，因为 (1 + 2 + 3) = 6
28 ，因为 (1 + 2 + 4 + 7 + 14) = 28
Abundant(过剩数): aliquot sum > number
12 , 因为 (1 + 2 + 3 + 4 + 6) = 16
24 , 因为 (1 + 2 + 3 + 4 + 6 + 8 + 12) = 36
Deficient(亏数): aliquot sum < number
8 ， 因为 (1 + 2 + 4) = 7
素数 都是 deficient
实现一种方法来确定给定的数字是否为Perfect。 根据您的语言轨迹,您还可能需要实现一种方法来确定给定的数字是否为Abundant或Deficient.

资源
*/
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
   Abundant,
   Perfect,
   Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let len = num / 2 + 1;
    let sum = (1..len).fold(0, |accm, i| {
        if num % i == 0 {
            accm + i
        } else {
            accm
        }
    });
    if sum == num {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}
/*
pub fn classify(num: u64) -> Option<Classification> {
   if num == 0 {
       return None;
   }
   let sum: u64 = (1..num).filter(|i| num % i == 0).sum();
   if sum == num {
       Some(Classification::Perfect)
   } else if sum < num {
       Some(Classification::Deficient)
   } else {
       Some(Classification::Abundant)
   }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
   Abundant,
   Perfect,
   Deficient,
}
*/