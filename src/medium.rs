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

/* 7 时钟
实现一个处理没有日期的时间的 clock.

您应该可以添加和减去,分钟数.

代表相同时间的两个时钟应该彼此相等.
*/
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: Clock::caculate_hours(hours, minutes),
            minutes: Clock::caculate_minutes(minutes),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        let tmp_minutes = self.minutes + minutes;
        Clock {
            hours: Clock::caculate_hours(self.hours, tmp_minutes),
            minutes: Clock::caculate_minutes(tmp_minutes),
        }
    }

    fn caculate_hours(hours: i32, minutes: i32) -> i32 {
        let temp_hours = (hours + minutes / 60) % 24;
        let temp_minutes = minutes % 60;
        if temp_hours <= 0 {
            if temp_minutes < 0 {
                24 + temp_hours - 1
            } else {
                (24 + temp_hours) % 24
            }
        } else if temp_minutes < 0{
            temp_hours - 1
        } else {
            temp_hours
        }
    }

    fn caculate_minutes(minutes: i32) -> i32 {
        let temp_minutes = minutes % 60;
        if temp_minutes < 0 {
            60 + temp_minutes
        } else {
            temp_minutes
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
/*
use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
   minutes: i32,
}

impl fmt::Display for Clock {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let hours = self.minutes / 60;
       let mins = self.minutes % 60;
       write!(f, "{:02}:{:02}", hours, mins)
   }
}

impl Clock {
   pub fn new(hour: i32, minute: i32) -> Self {
       Clock::build(hour * 60 + minute)
   }

   fn build(minutes: i32) -> Self {
       let mut mins = minutes;
       while mins < 0 {
           mins += 1440;
       }
       Clock {
           minutes: mins % 1440,
       }
   }

   pub fn add_minutes(&mut self, minutes: i32) -> Self {
       Clock::build(self.minutes + minutes)
   }
}
*/

/* 8 DOT DSL

编写类似于 Graphviz 点语言的特定领域语言.

一个Domain Specific Language (DSL)是针对特定域优化的小语言.

比如说DOT 语言允许您编写图形的文本描述，然后通过Graphviz中其中一个图形工具(如dot)转换为图像，一个简单的图形如下所示:


    graph {
        graph [bgcolor="yellow"]
        a [color="red"]
        b [color="blue"]
        a -- b [color="green"]
    }
把它放在一个example.dot文件中，并运行dot example.dot -T png -o example.png，就会创建一个图像example.png，其中黄色背景上的绿线连接的红色和蓝色圆圈。

创建类似于点语言的 DSL.

本练习希望您通过使用builder pattern模式，构建多个结构。简而言之,此模式允许您将包含大量参数的结构的构造函数，拆分为多个单独的函数。这种方法为您提供了实现紧凑，但高度灵活的结构构造和配置的方法
*/
pub mod graph {
    use crate::medium::graph::graph_items::node::Node;
    use crate::medium::graph::graph_items::edge::Edge;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.clone().to_vec();

            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.clone().to_vec();

            self
        }
        pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            attrs.iter().for_each(|&(name, value)| {
                self.attrs.insert(name.to_string(), value.to_string());
            });

            self
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().filter(|n| n.name == name).nth(0)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                src: String,
                dst: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Edge {
                        src: src.to_string(),
                        dst: dst.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|&(name, value)| {
                        self.attrs.insert(name.to_string(), value.to_string());
                    });

                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|&(name, value)| {
                        self.attrs.insert(name.to_string(), value.to_string());
                    });

                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|v| v.as_ref())
                }
            }
        }
    }
}
/* 9 汉明窗
给 2 个长度为 n 的 DNA 序列，求汉明距离差异为多少。

通过比较两条 DNA 链,并计算其中有多少核苷酸与其他序列中的同等核苷酸不同.


GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT
^ ^ ^ ^ ^ ^^
这两条 DNA 链之间的 汉明距离为 7.
*/
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    let mut len = 0;
    for i in 0..(s1.len()) {
        if s1[i] != s2[i] {
            len += 1;
        }
    }
    Some(len)
}
/*
pub fn hamming_distance(a: &str, b: &str) -> Option<usize> {
   if a.len() != b.len() {
       return None;
   }

   Some(a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count())
}
*/

/* 10 简单链表
编写一个使用元素(Elements)和列表(List)的简单链表实现.

链表是计算机科学中的一种基本数据结构，常用于其他数据结构的实现。它们在函数式编程语言(如 Clojure、Erlang 或 Haskell)中很普遍，但是在命令式语言(如 Ruby 或 Python)中很少见。

最简单的链表是单链表。列表中的每个元素，包含数据和一个”next”字段，指向元素列表中的下一个元素。

链接列表的这种变体通常用于表示序列，或推/取堆栈(也称为 LIFO 堆栈;后进先出)。

作为第一步,让我们创建一个包含范围(1..10)的单一链接列表,并提供反转链接列表，和转换为数组，或从数组转换的函数.

在使用内置链表的语言实现这一点时,实现自己的抽象数据类型.
*/
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn push(&mut self, _element: T) {
        let node = Box::new(Node { data: _element, next: self.head.take()});
        self.head = Some(node);
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => self.head.take().map(|node| {
                let node = *node;
                self.head = node.next;
                self.len -= 1;
                node.data
            })
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut next = self.head.as_ref().map(|node| &**node);
        while let Some(node) = next {
            rev_list.push(node.data.clone());
            next = node.next.as_ref().map(|node| &**node);
        }
        rev_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _item.iter() {
            list.push(i.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = vec![];
        while let Some(data) = self.pop(){
            vec.push(data)
        }
        vec.reverse();
        vec
    }
}

/* 11 杨辉三角形
计算给定的行数的 杨辉三角形。

在 杨辉 三角形中, 可以看到，一个数由它头顶左右两个数相加得到的。
    1
   1 1
  1 2 1
 1 3 3 1
1 4 6 4 1
# ... etc
*/
pub struct PascalsTriangle {
    row: u32,
}
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row: row_count,
        }
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row {
            0 => vec![],
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            _ => {
                let mut out = vec![vec![1], vec![1, 1]];
                for _ in 1..(self.row-1) {
                    let mut temp = vec![1];
                    let last = out.last().unwrap();
                    for j in 0..(last.len()-1) {
                        temp.push(last[j] + last[j+1]);
                    }
                    temp.push(1);
                    out.push(temp);
                }
                out
            }
        }
    }
}
/*
pub struct PascalsTriangle {
   row_count: u32,
}

impl PascalsTriangle {
   pub fn new(row_count: u32) -> Self {
       PascalsTriangle {
           row_count: row_count,
       }
   }

   pub fn rows(&self) -> Vec<Vec<u32>> {
       (0..self.row_count)
           .map(|row| PascalsTriangle::row(row))
           .collect()
   }

   pub fn row(number: u32) -> Vec<u32> {
       let mut r = vec![1];

       for p in 1..(number + 1) {
           if let Some(&last) = r.last() {
               r.push((last * (number + 1 - p)) / p)
           }
       }
       r
   }
}
*/

/* 12 拼字母的分数
给出一个单词,计算该单词的字母的分数.

字母价值
你需要这些:

26个英文字母                           对应有多少分
A, E, I, O, U, L, N, R, S, T       1
D, G                               2
B, C, M, P                         3
F, H, V, W, Y                      4
K                                  5
J, X                               8
Q, Z                               10
例子
“cabbage”的得分值应为 14 分:
C , 就得 3 分
A , 就得 1 分,两次
B , 就得 3 分,两次
G , 就得 2 分
E , 就得 1 分
总计:
3 + 2*1 + 2*3 + 2 + 1
= 3 + 2 + 6 + 3
= 5 + 9
= 14
*/
pub fn score(word: &str) -> u64 {
    let char_to_score = |c: char| {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' |  'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        }
    };
    word.to_lowercase().chars().map(|c| char_to_score(c)).sum()
}
/*
use std::collections::HashMap;

pub fn score(word: &str) -> u16 {
   let values = dictionary();
   word.to_lowercase()
       .chars()
       .map(|c| values.get(&c).clone())
       .fold(0, |score, v| score + v.unwrap_or(&0))
}

fn dictionary() -> HashMap<char, u16> {
   let mut values = HashMap::new();
   values.insert('a', 1);
   values.insert('b', 3);
   values.insert('c', 3);
   values.insert('d', 2);
   values.insert('e', 1);
   values.insert('f', 4);
   values.insert('g', 2);
   values.insert('h', 4);
   values.insert('i', 1);
   values.insert('j', 8);
   values.insert('k', 5);
   values.insert('l', 1);
   values.insert('m', 3);
   values.insert('n', 1);
   values.insert('o', 1);
   values.insert('p', 3);
   values.insert('q', 10);
   values.insert('r', 1);
   values.insert('s', 1);
   values.insert('t', 1);
   values.insert('u', 1);
   values.insert('v', 4);
   values.insert('w', 4);
   values.insert('x', 8);
   values.insert('y', 4);
   values.insert('z', 10);
   values
}
*/

/* 13 全字母句
判断句子是否是全字母句。全字母句(希腊语:παγρμμα,pan 语法,”每个字母”)是一个使用字母表中每个字母，至少一次的句子。最著名的英语是 全字母句:

The quick brown fox jumps over the lazy dog.

字母表由 ASCII 字母a到z的全部组成，并且不区分大小写。输入不能包含非 ASCII 符号.
*/

use std::collections::HashSet;
pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len() == 26
}
/*
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub fn is_pangram(sentence: &str) -> bool {
   sentence
       .to_lowercase()
       .chars()
       .filter(|c| c.is_alphabetic())
       .filter(|c| c.is_ascii())
       .collect::<BTreeSet<char>>() == english_letter_set()
}

fn english_letter_set() -> BTreeSet<char> {
   BTreeSet::from_iter(ENGLISH_ALPHABET.chars())
}

const ENGLISH_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
*/

/* 15 核苷酸计数
给定一条单 DNA 链 ,计算每个核苷酸在字符串中出现的次数.

地球上每一生物的遗传语言都是 DNA.DNA 是一种大分子,它是由一系列叫做核苷酸的单个元素组成。DNA 中存在 4 种类型,它们仅略有不同,并且可用以下符号表示:’A’表示腺嘌呤,’C’表示胞嘧啶,’G’表示鸟嘌呤,’T’表示胸腺嘧啶.
*/
use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'G' && nucleotide != 'C' && nucleotide != 'T' {
        return Err(nucleotide);
    }
    let mut num = 0;
    for c in dna.chars() {
        if c != 'A' && c != 'G' && c != 'C' && c != 'T' {
            return Err(c);
        }
        if c == nucleotide {
            num += 1;
        }
    }
    Ok(num)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_hashmap = HashMap::new();
    dna_hashmap.insert('A', 0);
    dna_hashmap.insert('T', 0);
    dna_hashmap.insert('C', 0);
    dna_hashmap.insert('G', 0);
    for c in dna.chars() {
        if c != 'A' && c != 'G' && c != 'C' && c != 'T' {
            return Err(c);
        }
        if let Some(x) = dna_hashmap.get_mut(&c) {
            *x += 1;
        }
    }
    Ok(dna_hashmap)
}
/*
use std::collections::HashMap;

static VALID_NUCLEOTIDES: &'static str = "ACGT";

fn valid(c: char) -> Result<char, char> {
   if VALID_NUCLEOTIDES.contains(c) {
       Ok(c)
   } else {
       Err(c)
   }
}

pub fn count(nucleotide: char, input: &str) -> Result<usize, char> {
   valid(nucleotide)?;
   let mut count = 0;
   for c in input.chars() {
       if valid(c)? == nucleotide {
           count += 1;
       }
   }
   Ok(count)
}

pub fn nucleotide_counts(input: &str) -> Result<HashMap<char, usize>, char> {
   let mut map: HashMap<char, usize> = VALID_NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
   for nucleotide in input.chars() {
       if let Some(n) = map.get_mut(&nucleotide) {
           *n += 1;
       } else {
           return Err(nucleotide);
       }
   }
   Ok(map)
}
*/

/* 16 模 10 算法
给定一个数,判定它是否有效 Luhn 公式.

Luhn 算法，也称为“模 10”算法

这个Luhn 算法是一个简单的校验和公式,用于验证各种身份号码,如信用卡号码和加拿大社会保险号码.

任务是检查给定字符串是否有效.

验证一个数
长度为 1 或更小的字符串无效。在输入中允许使用空格，但在检查前，应清除空格。所有其他非数字字符都是不允许的.

例子 1:有效信用卡号码

4539 1488 0343 6467
LuHN 算法的第一步是，从右边开始每第二个数字加倍。我们要加倍的位


4_3_ 1_8_ 0_4_ 6_6_
如果加倍的数字值导致大于 9 的数字,则将此值减去 9。我们加倍的结果:


8569 2478 0383 3437
1_8_ => 2_7_ , 8*2 - 9 = 7

然后把所有数字加起来:


8+5+6+9+2+4+7+8+0+3+8+3+3+4+3+7 = 80
如果总和可被 10 整除,则数字是有效的.这个号码是有效的!
*/
pub fn is_valid(code: &str) -> bool {
    let char_list = code.chars().filter(|c| *c != ' ').collect::<Vec<char>>();
    if char_list.len() == 1 {
        return false;
    }
    let mut accm = 0;
    for (index, c) in char_list.iter().rev().enumerate() {
        let mut temp_date;
        if c.is_ascii_digit() {
            temp_date = c.to_digit(10).unwrap();
        } else {
            return false;
        }
        if index % 2 != 0 {
            temp_date *= 2;
        }
        if temp_date > 9 {
            accm += temp_date - 9;
        } else {
            accm += temp_date;
        }
    }
    accm % 10 == 0
}
/*
pub fn is_valid(candidate: &str) -> bool {
   if candidate.chars().filter(|c| c.is_digit(10)).take(2).count() <= 1
       || candidate.chars().any(|c| !c.is_digit(10) && c != ' ')
   {
       return false;
   }

   candidate
       .chars()
       .filter_map(|c| c.to_digit(10))
       .rev()
       .enumerate()
       .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
       .map(|digit| if digit > 9 { digit - 9 } else { digit })
       .sum::<u32>() % 10 == 0
}
*/

/* 17 最大系列乘积
给定一个数字串,计算长度为 n 的连续子串的最大乘积.

例如,对于输入'1027839564' ， 3 位数系列的最大乘积是 270 (9 * 5 * 6), 5 位数系列的最大乘积为 7560 (7 * 8 * 3 * 9 * 5).

注意这些系列数字字符，在输入中，只要求相邻位置，不需要连续数值(123456..).

对于输入'73167176531330624919225119674426574742355349194934'一系列 6 位数的最大乘积是 23520.
*/
#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let char_list = string_digits.chars().collect::<Vec<char>>();
    if span == 0 {
        return Ok(1);
    }
    if span > char_list.len() {
        return Err(Error::SpanTooLong);
    }
    let mut num_list: Vec<u32> = vec![];
    for i in 0..(char_list.len()-span+1) {
        let mut temp_mul = 1;
        for j in i..(i+span) {
            if let Some(date) = char_list[j].to_digit(10) {
                temp_mul *= date;
            } else {
                return Err(Error::InvalidDigit(char_list[j]));
            }
        }
        num_list.push(temp_mul);
    }
    Ok(*num_list.iter().max().unwrap() as u64)
}
/*
#[derive(Debug, PartialEq)]
pub enum Error {
   SpanTooLong,
   InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
   if span == 0 {
       return Ok(1);
   }

   if let Some(invalid) = string_digits.chars().find(|c| !c.is_digit(10)) {
       return Err(Error::InvalidDigit(invalid));
   }

   let products: Vec<u64> = string_digits
       .chars()
       .map(|c| c.to_digit(10).unwrap() as u64)
       .collect::<Vec<u64>>()
       .windows(span)
       .map(|w| w.into_iter().product())
       .collect();

   if let Some(&x) = products.iter().max() {
       Ok(x)
   } else {
       Err(Error::SpanTooLong)
   }
}
*/

/* 18 单词计数
给定一个短语,计算该短语中，每个单词的出现次数.

例如输入"olly olly in come free"


olly: 2
in: 1
come: 1
free: 1
*/
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let words = words.to_lowercase();
    let word_list = words.split_whitespace().collect::<Vec<&str>>();
    let mut word_hashmap: HashMap<String, u32> = HashMap::new();
    for i in 0..word_list.len() {
        let temp_word = word_list[i].trim_matches(|c: char| !c.is_ascii_alphanumeric()).trim().to_string();
        if temp_word == "".to_string() {
            continue;
        }
        if word_hashmap.contains_key(&temp_word) {
            if let Some(count) = word_hashmap.get_mut(&temp_word) {
                *count += 1;
            }
        } else {
            word_hashmap.insert(temp_word, 1);
        }
    }
    word_hashmap
}
/*
use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
   let mut map: HashMap<String, u32> = HashMap::new();
   let lower = input.to_lowercase();
   let slice: &str = lower.as_ref();
   for word in slice
       .split(|c: char| !c.is_alphanumeric())
       .filter(|s| !s.is_empty())
   {
       *map.entry(word.to_string()).or_insert(0) += 1;
   }
   map
}
*/