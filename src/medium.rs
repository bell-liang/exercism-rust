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

/* 3 运行游程编码
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