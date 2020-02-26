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

/* 19 Atbash 加密
创建 Atbash 密码的实现,这是在中东创建的古老加密系统.

Atbash 密码是一种简单的替换密码,它依赖于转置字母表中的所有字母,使得生成的字母表向后。

第一个字母替换为最后一个字母,
第二个字母替换为倒数第二个字母,
依此类推.
拉丁字母的 Atbash 密码如下:


明文:  abcdefghijklmnopqrstuvwxyz
加密:  zyxwvutsrqponmlkjihgfedcba
它是一个非常弱的加密方式，因为它只有一个加密可能性，且为一个简单的单字母替换密码。但是,这并不是现在’加密游戏-练习时间’的问题.

密文以固定长度的组写出，传统的组大小为 5 个字母，并且不包括标点符号。这是为了使单词边界，更难猜测
*/
fn assic(c: char) -> u8 {
    c as u8
}
fn transport(c: char) -> char {
    if c.is_digit(10) {
        c
    } else {
        (assic('z') - assic(c) + assic('a')) as char
    }
}

pub fn encode_19(plain: &str) -> String {
    plain.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii())
        .filter(|c| c.is_alphanumeric())
        .map(|c| transport(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|ch| ch.into_iter().clone().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
pub fn decode_19(cipher: &str) -> String {
    cipher.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| transport(c))
        .collect::<String>()
}

/* 20 密码矩形
实现，用于组成称为方形代码的加密信息的经典方法.

给定英文文本,输出该文本的加密编码版本。

首先,输入被规范化:

从英文文本中删除空格和标点符号,并且消息是朝下的.
然后,

规范化字符被分成行。当使用插入的换行符打印时,这些行自自然然形成类似矩形的样子。
例如,句子


"If man was meant to stay on the ground, god would have given us roots."
规范化为:


"ifmanwasmeanttostayonthegroundgodwouldhavegivenusroots"
明文应该组织成一个矩形。矩形的大小(r x c)应该根据消息的长度来决定c >= r和c - r <= 1,这里的c是列数和r是行数.

我们的标准化文本长度为 54 个字符，用c = 8和r = 7指示矩形:


"ifmanwas"
"meanttos"
"tayonthe"
"groundgo"
"dwouldha"
"vegivenu"
"sroots  "
通过向下(第一行第一个,拼接第二行第一个)，读取从左到右的列来获得编码消息.

上面的消息编码为:


"imtgdvsfearwermayoogoanouuiontnnlvtwttddesaohghnsseoau"
根据输出的，矩形块编码文本的大小(r X c)，表明有c块r长度的编码字串，以空格分隔。对于那些n位字符，但少于规定的长度的，每个尾添一个空格。


"imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
请注意,如果我们要堆叠这些,我们可以直观地，将密文解码回原始消息: (第一行第一个，拼接第二行第一个...)


"imtgdvs"
"fearwer"
"mayoogo"
"anouuio"
"ntnnlvt"
"wttddes"
"aohghn "
"sseoau "
*/
pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string()
    }
    let input = input.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<Vec<char>>();
    println!("{}", (input.len() as f32).sqrt() as u32);
    let input = input.chunks(({
                        let len = (input.len() as f32).sqrt() as u32 as usize;
                        if input.len() == len*len {
                            len
                        } else {
                            len + 1
                        }
                    }) as usize)
        .map(|ch| ch.into_iter().clone().collect::<String>())
        .collect::<Vec<String>>();
    println!("{:?}", input);
    let mut output = vec![];
    for i in 0..input[0].len() {
        let mut temp = vec![];
        for j in 0..input.len() {
            temp.push({
                if let Some(row) = input.get(j) {
                    if let Some(clumn) = row.get(i..i+1) {
                        let c: u8 = clumn.as_bytes()[0];
                        c as char
                    } else {
                        ' '
                    }
                } else {
                    ' '
                }
            })
        }
        output.push(temp.into_iter().collect::<String>());
    }
    output.join(" ")
}

/* 21 旋转密码
创建旋转密码的实现,有时也称为 Caesar 密码.

Caesar 密码是一个简单的移位密码，它依赖于使用0到26整数(key)，在字母表中转置所有字母。由于模运算，使用0要么26，总是会产生相同的输出。将字母移动为与 key 值一样多的值。

旋转密码的一般表示法是ROT + <key>。最常用的旋转密码是ROT13.

一个拉丁字母表的ROT13加密如下:


原文:  abcdefghijklmnopqrstuvwxyz
密文:  nopqrstuvwxyzabcdefghijklm
它比 Atbash 密码更强大,因为它有 27 个可能性 key，和 25 个可用的密文.

密文会与输入相同的格式写出,包括空格和标点符号.
*/
pub fn rotate(input: &str, key: i8) -> String {
    input.chars()
        .map(|c: char| {
            if c.is_ascii_alphabetic() {
                let key = key as u8;
                if c.is_ascii_lowercase() {
                    if c as u8 + key > 'z' as u8 {
                        ('a' as u8 + c as u8 + key - 'z' as u8 - 1) as u8 as char
                    } else {
                        (c as u8 + key) as u8 as char
                    }
                } else {
                    if c as u8 + key > 'Z' as u8 {
                        ('A' as u8 + c as u8 + key - 'Z' as u8 - 1) as u8 as char
                    } else {
                        (c as u8 + key) as u8 as char
                    }
                }
            } else {
                c
            }
        })
        .collect::<String>()
}
/*
pub fn rotate(text: &str, shift_key: u8) -> String {
   text.chars()
       .map(|c| {
           let case = if c.is_uppercase() { 'A' } else { 'a' } as u8;
           if c.is_alphabetic() {
               (((c as u8 - case + shift_key) % 26) + case) as char
           } else {
               c
           }
       })
       .collect::<String>()
}
*/

/* 22 简单密码
实现一个简单的移位密码,像 Caesar 和，一个更安全的替换密码.

步骤 1
“如果他有什么秘密要说的话，他就是用密码写的，也就是说，通过改变字母表的字母顺序，一个字也说不出来。如果有人想破译这些，并理解它们的意思，他必须用字母表中的第四个字母，即 D，代替 A，以及其他的字母。 —Suetonius, Life of Julius Caesar

密码是非常直截了当的算法，使文本不可读，同时仍容易允许破译。他们容易受到许多形式的密码分析，但我们幸运的是，我们的小姐妹通常不是密码学家。

用 Caesar密码，加密来自 Julius Caesar 的消息。现在 Caesar 知道加密方式不是很好，但他还是有个好处: 几乎没有人能读对。所以对于一对夫妇的信件已经足够了，让人们无法识别他们所知道的几个字。

你的任务是创建一个简单的移位密码，就像 Caesar 密码一样。这个图像是 Caesar 密码的一个很好的例子:

Caesar Cipher

例如:

将”iamapandabear”作为输入到 encode 函数返回密码”ldpdsdqgdehdu”。(虽不足以让我们的信息在运输过程中，确保高保密性).

当将”ldpdsdqgdehdu”放入decode函数时，它将返回原始的”iamapandabear”，让您的朋友阅读您的原始消息.

步骤 2
移位密码是没有乐趣，直到你的妹妹算了出来后！尝试修改代码,允许我们指定一个密钥(key)，并用作移位距离。这称为代换密码.

下面是一个例子:

给定密钥”aaaaaaaaaaaaaaaaaaa”，对字符串”iamapandabar”进行编码将返回原来的”iamapandable”.

给定密钥”ddddddddddddd”，编码我们的字符串”iamapandabore”会返回疑惑的”ldpdsdqgdhdu”.

在上面的示例中,我们为键值设置了 a=0。因此,当明文添加到密钥时,我们最终得到相同的消息。所以”aaaa”不是一个理想的密钥。但是,如果我们把密钥设置为dddd,我们将得到与 caesar 密码相同的东西.

步骤 3
任何密码中最薄弱的环节都是人。让我们通过提供随机性，并确保密钥仅包含小写字母，来使替换密码更具容错性.

如果有人根本不提交密钥,则生成一个长度至少为 100 个字符的真正随机密钥.

如果提交的密钥不只由小写字母组成,那解决方案应该以适当提醒的方式处理错误.
*/
pub fn encode_22(key: &str, s: &str) -> Option<String> {
    if !key.chars().all(|c: char| c.is_ascii_lowercase()) || key.is_empty() {
        return None;
    }
    Some(s.chars()
        .zip(key.chars())
        .map(|(s1, k1)| ((s1 as u8 - 'a' as u8 + k1 as u8 - 'a' as u8) % 26 + 'a' as u8) as char)
        .collect::<String>())
}
pub fn decode_22(key: &str, s: &str) -> Option<String> {
    if !key.chars().all(|c: char| c.is_ascii_lowercase()) || key.is_empty() {
        return None;
    }
    Some(s.chars()
        .zip(key.chars())
        .map(|(s1, k1)| ('z' as i8 - ('z' as i8 - (s1 as i8 - (k1 as i8 - 'a' as i8))) % 26) as u8 as char)
        .collect::<String>())
}
use rand::Rng;
pub fn encode_random(s: &str) -> (String, String) {
    let mut r = rand::thread_rng();
    let mut key = String::new();
    for _ in 0..100 {
        key.push(char::from('a' as u8 + r.gen_range(0, 26)));
    }
    let encoded = encode_22(&key, s);
    (key, encoded.unwrap())
}
/*
extern crate rand;
use rand::Rng;

pub fn encode_random(s: &str) -> (String, String) {
   let mut r = rand::thread_rng();
   let mut key = String::new();
   for _ in 0..100 {
       key.push(char::from('a' as u8 + r.gen_range(0, 26)));
   }
   let encoded = encode(&key, s);
   (key, encoded.unwrap())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
   shift(key, s, 1)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
   shift(key, s, -1)
}

fn shift(key: &str, s: &str, dir: i8) -> Option<String> {
   if key.is_empty() {
       return None;
   }
   let mut o = String::new();
   let mut i = 0;
   let mut key_arr = Vec::new();
   for c in key.chars() {
       if !c.is_ascii_lowercase() {
           return None;
       }
       key_arr.push(c);
   }
   for c in s.chars() {
       let shift = key_arr[i % key_arr.len()] as i8 - 'a' as i8;
       let n = ((c as i8 - 'a' as i8 + dir * shift) % 26 + 26) % 26;
       o.push(char::from('a' as u8 + n as u8));
       i += 1;
   }
   Some(o)
}
*/

/* 23 栅栏密码
实现篱笆密码法的编/解码.

篱笆密码法是一种置换式密码,它从编码的方式得到它的名字。它早被古希腊人所使用。

在 栅栏 密码中,信息向下写在虚构的篱笆的连续”rail”上，然后当我们到达底部时，又向上移动(像锯齿形)。最后,消息以行读取.

例如,使用三个”Rails(栅栏)”，和加密”WE ARE DISCOVERED FLEE AT ONCE”信息，密文写道:


W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
然后读出:


WECRLTEERDSOEEFEAOCAIVDEN
要解密一条消息,你要采用锯齿形读法，而密文则是一行一行看。


? . . . ? . . . ? . . . ? . . . ? . . . ? . . . ?
. ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
第一行有七个点,可以用”WECRRLTE”填充.


W . . . E . . . C . . . R . . . L . . . T . . . E
. ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
现在第二行为”ERDSOEEFEAOC”.


W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
最后一排”AIVDEN”.


W(1) . . . E . . . C . . . R . . . L . . . T . . . E
. E(2) . R . D . S . O . E . E . F . E . A . O . C .
. . A(3) . . . I . . . V . . . D . . . E . . . N . .
1,2,3只是为了方便理解，不存在任何地方

如果你现在锯齿形阅读,你可以阅读原始消息.
*/
pub struct RailFence(u32);
impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }
    pub fn encode(&self, text: &str) -> String {
        let mut out: Vec<String> = Vec::with_capacity(self.0 as usize);
        for _ in 0..self.0 {
            out.push("".to_string());
        }
        let mut index = 0;
        let mut down = true;
        let mut text_iter = text.chars();
        let mut count = 0;
        while count < text.len() {
            if down {
                if let Some(c) = text_iter.next() {
                    out[index].push(c);
                }
                count += 1;
                if index == (self.0-1) as usize {
                    down = false;
                    index -= 1;
                } else {
                    index += 1;
                }
            } else {
                if index == 0 {
                    down = true;
                } else {
                    if let Some(c) = text_iter.next() {
                        out[index].push(c);
                    }
                    index -= 1;
                    count += 1;
                }
            }
        }
        out.join("")
    }
    pub fn decode(&self, cipher: &str) -> String {
        let mut count = Vec::with_capacity(self.0 as usize);
        for _ in 0..self.0 {
            count.push(0);
        }
        let mut index = 0;
        let mut down = true;
        let mut text_iter = cipher.chars();
        while let Some(_) = text_iter.next() {
            if down {
                count[index] += 1;
                index += 1;
                if index == self.0 as usize {
                    down = false;
                    index -= 2;
                }
            } else {
                count[index] += 1;
                index -= 1;
                if index == 0 {
                    down = true;
                }
            }
        }
        for i in 1..count.len() {
            count[i] += count[i-1];
        }
        count.insert(0, 0);
        let mut out: Vec<String> = Vec::with_capacity(self.0 as usize);
        for _ in 0..count.len()-1 {
            out.push("".to_string());
        }
        for i in 0..(count.len()-1) {
            out[i].push_str(&cipher.get(count[i]..count[i+1]).unwrap().to_owned());
        }
        let mut out_string = "".to_string();
        let mut id = 0;
        let mut medium = vec![];
        for i in 0..out.len() {
            medium.push(out[i].chars());
        }
        while id < cipher.len() {
            for i in 0..medium.len() {
                if let Some(c) = medium[i].next() {
                    out_string.push(c);
                }
                id += 1;
            }
            for i in (1..medium.len()-1).rev() {
                if let Some(c) = medium[i].next() {
                    out_string.push(c);
                }
                id += 1;
            }
        }
        out_string
    }
}
/*
pub struct RailFence(u32);

fn uncons(s: &str) -> (&str, &str) {
   s.split_at(s.chars().next().map_or(0, |c| c.len_utf8()))
}

impl RailFence {
   pub fn new(rails: u32) -> RailFence {
       RailFence(rails)
   }

   fn next(&self, down: &mut bool, rail: &mut usize) {
       if *down {
           if *rail + 1 < self.0 as usize {
               *rail += 1;
           } else {
               *down = false;
               *rail -= 1;
           }
       } else {
           if *rail > 0 {
               *rail -= 1;
           } else {
               *down = true;
               *rail += 1;
           }
       }
   }

   pub fn encode(&self, text: &str) -> String {
       let mut rails =
           vec![String::with_capacity(1 + (text.len() / self.0 as usize)); self.0 as usize];
       let mut down = true;
       let mut rail = 0;

       for ch in text.chars() {
           rails[rail].push(ch);
           self.next(&mut down, &mut rail);
       }

       rails.join("")
   }

   pub fn decode(&self, cipher: &str) -> String {
       let mut rail_caps = vec![0; self.0 as usize];
       let mut down = true;
       let mut rail = 0;

       for _ in cipher.chars() {
           rail_caps[rail] += 1;
           self.next(&mut down, &mut rail);
       }

       // this vector owns the text of each rail
       let mut rails_own = Vec::with_capacity(self.0 as usize);
       let mut skip = 0;

       for &cap in rail_caps.iter() {
           rails_own.push(
               cipher
                   .chars()
                   .skip(skip)
                   .enumerate()
                   .take_while(|&(i, _)| i < cap)
                   .map(|(_, c)| c)
                   .collect::<String>(),
           );
           skip += cap;
       }

       // this vector holds string slices viewing into rails_own
       let mut rails: Vec<&str> = rails_own.iter().map(|r| r.as_ref()).collect();

       let mut out = String::with_capacity(cipher.len());
       down = true;
       rail = 0;

       while rails.iter().any(|r: &&str| r.len() > 0) {
           let (head, t_rail) = uncons(rails[rail]);
           rails[rail] = t_rail;
           self.next(&mut down, &mut rail);
           out.push_str(head);
       }

       out
   }
}
*/

/* 24 ETL
ETL，是英文 Extract-Transform-Load 的缩写，用来描述将数据从来源端经过萃取、转置、加载至目的端的过程

ETL
提取转换负载(ETL)是一种很有意思的说法,”我们在这个系统中有一些遗留的遗留数据,现在我们需要在这个闪亮的新系统中使用,所以我们将迁移它.”

(通常情况下,接下来是，”我们只需要运行一次就好啦。”之后，通常会有很多怒拍额头,并抱怨自身有多么愚蠢。

目标
我们将从遗留系统中，提取一些拼字游戏分数.

旧的系统存储每一个字母的列表:

1 分:”A”,”E”,”I”,”O”,”U”,”L”,”N”,”R”,”S”,”T”,
2 分:”D”,”G”,
3 分:”B”、”C”、”M”、”P”,
4 分:”F”、”H”、”V”、”W”、”Y”,
5 分:”K”,
8 分:”J”,”X”,
10 分:”Q”,”Z”,
闪亮的新拼写系统存储每个字母的分数，这使得计算一个单词的分数更快、更容易。它也把字母存为小写字母，而不考虑输入字母的情况:

“a” 值 1 分.
“b” 值 3 分.
“c” 值 3 分.
“d” 值 2 分.
等.
你的任务,你应该选择接受它, 应将遗留数据格式，转换成闪亮的新格式。
*/
use std::collections::BTreeMap;
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out: BTreeMap<char, i32> = BTreeMap::new();
    for (key, value) in h.iter() {
        for c in value.iter() {
            out.insert(c.to_ascii_lowercase(), *key);
        }
    }
    out
}
/*
use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
   input
       .iter()
       .flat_map(|(&n, vec)| vec.iter().map(move |c| (c.to_ascii_lowercase(), n)))
       .collect()
}
*/

/* 25 集合操作
实现accumulate操作, 给出一个集合，和一个操作行为，该行为会影响到集合中的每个值，并返回一个新的，包含影响结果值的集合

如:给出数字的集合:

1,2,3,4,5
和一个平方操作:

平方它(x => x * x)
您的代码应该能够生成原集合的平方集合:

1,4,9,16,25
查看测试套件，以查看预期的函数命名.
*/
pub fn map<T, F, U>(input: Vec<T>, mut f: F) -> Vec<U>
    where F: FnMut(T) -> U,
    {
        let mut out = Vec::with_capacity(input.len());
        for i in input {
            out.push(f(i));
        }
        out
}

/* 26 缩写
将短语转换为，其首字母缩写词。

技术人员都喜欢他们的 TLA(三字母缩略语(Three Letter Acronyms)显得高大上)!

通过编写将诸如 Portable Network Graphics 之类的长名称，转换为其首字母缩略词(PNG)的程序,帮助生成一些术语。
*/
pub fn abbreviate(phrase: &str) -> String {
    let str_list = phrase.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let mut out = "".to_string();
    for s in str_list {
        if s.chars().all(|c: char| c.is_lowercase()) || s.chars().all(|c: char| c.is_uppercase()) {
            out.push(s.chars().next().unwrap());
        } else {
            for c in s.chars() {
                if c.is_uppercase() {
                    out.push(c);
                }
            }
        }
    }
    out.to_uppercase()
}
/*
pub fn abbreviate(phrase: &str) -> String {
   phrase
       .split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
       .flat_map(|word| split_camel(word))
       .filter_map(|word| word.chars().next())
       .collect::<String>()
       .to_uppercase()
}

fn split_camel(phrase: &str) -> Vec<String> {
   let chars: Vec<char> = phrase.chars().collect();
   let mut words: Vec<String> = Vec::new();
   let mut word_start: usize = 0;
   for (i, c) in chars.iter().enumerate() {
       if i == chars.len() - 1 || c.is_lowercase() && chars[i + 1].is_uppercase() {
           words.push(chars[word_start..i + 1].iter().cloned().collect());
           word_start = i + 1;
       }
   }
   words
}
*/

/* 27 素数筛
使用 Eratosthenes 的 Sieve 查找从 2 到给定数字的所有素数.

这是一种简单且历史悠久的筛法，用来找出一定范围内所有的素数。

所使用的原理是从 2 开始，将每个素数的各个倍数，标记成合数。一个素数的各个倍数，是一个差为此素数本身的等差数列。此为这个筛法和试除法不同的关键之处，后者是以素数来测试每个待测数能否被整除。

埃拉托斯特尼筛法是列出所有小素数最有效的方法之一，其名字来自于古希腊数学家埃拉托斯特尼，并且被描述在另一位古希腊数学家尼科马库斯所著的《算术入门》中。

维基百科文章有一个有用的图解解释算法:

请注意,这是一个非常具体的算法,并且测试不会检查您是否实现了算法,只要您已经提出了正确的素数列表.https://zh.wikipedia.org/wiki/埃拉托斯特尼筛法

一个好的第一个测试是，检查你不使用除法或余数运算(div, /, mod or % 具体语言所具有的)
*/
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound == 1 {
        return vec![];
    }
    let mut all: Vec<u64> = (1..upper_bound).map(|x| x + 1).collect::<Vec<u64>>();
    let mut index = 0;
    let mut p = all[index];
    while *all.last().unwrap() >= p * p {
        all.retain(|&x| (x == p) || (x % p != 0));
        index += 1;
        p = all[index];
        println!("{}, {:?}", p, all);
    }
    all
}
/*
pub fn primes_up_to(limit: i32) -> Vec<i32> {
   let mut integers = (1..limit).map(|x| x + 1).collect::<Vec<i32>>();
   let mut p = Some(2);

   while let Some(y) = p {
       integers.retain(|&x| (x == y) || (x % y != 0));
       p = integers.clone().into_iter().find(|x| *x > y);
   }
   integers
}
*/
// 28 RNA 转录
/*
给定 DNA 链,返回其 RNA 补体(每个 RNA 转录).

DNA 和 RNA 链都是核苷酸序列.

DNA 中发现的四个核苷酸是腺嘌呤(A),胞嘧啶(C),鸟嘌呤(G)和胸腺嘧啶(T).

RNA 中发现的四个核苷酸是腺嘌呤(A),胞嘧啶(C),鸟嘌呤(G)和尿嘧啶(T).

给定 DNA 链,其转录的 RNA 链，通过用其互补物替换每个核苷酸而形成:

G- >C
C- >G
T- >A
A- >U
*/
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}
impl Nucleotide {
    fn from_char(ch: char) -> Option<Nucleotide> {
        Some(match ch {
            'A' => Nucleotide::Adenine,
            'C' => Nucleotide::Cytosine,
            'G' => Nucleotide::Guanine,
            'T' => Nucleotide::Thymine,
            'U' => Nucleotide::Uracil,
            _ => {
                return None;    
            }
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DNA(Vec<Nucleotide>);
impl DNA {
    pub fn new(input: &str) -> Result<DNA, usize> {
        let mut out = Vec::new();
        for (index, ch) in input.chars().enumerate() {
            match Nucleotide::from_char(ch) {
                Some(Nucleotide::Uracil) | None  => {
                    return Err(index);
                },                                                   
                Some(n) => {
                    out.push(n);
                },
            }
        }
        Ok(DNA(out))
    }
    pub fn to_rna(mut self) -> RNA {
        for nuc in self.0.iter_mut() {
            *nuc = match *nuc {
                Nucleotide::Adenine => Nucleotide::Uracil,
                Nucleotide::Cytosine => Nucleotide::Guanine,
                Nucleotide::Guanine => Nucleotide::Cytosine,
                Nucleotide::Thymine => Nucleotide::Adenine,
                Nucleotide::Uracil => unreachable!(),
            }
        }
        RNA(self.0)
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RNA(Vec<Nucleotide>);
impl RNA {
    pub fn new(input: &str) -> Result<RNA, usize> {
        let mut out = Vec::new();
        for (index, ch) in input.chars().enumerate() {
            match Nucleotide::from_char(ch) {
                Some(Nucleotide::Thymine) | None => {
                    return Err(index);
                },
                Some(n) => {
                    out.push(n);
                },
            }
        }
        Ok(RNA(out))
    }
}

// 29 三角形
/*
确定三角形是等边、等腰还是不等边三角形.

一个等边的三角形，三条边都有相同的长度。

一个等腰的三角形，至少两边相同的长度。(有时它被指定为两边长度完全相同,但是为了这个练习的目的，我们的说法是，至少两边。)

一不等边的三角形的两边各有不同的长度。
*/
pub struct Triangle {
    sides: [u64; 3],
}
impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] + sides[1] > sides[2] &&
            sides[0] + sides[2] > sides[1] && 
            sides[1] + sides[2] > sides[0] {
                Some(Triangle {
                    sides: sides,
                })
            } else {
                None
            }
    }
    pub fn count(&self) -> i8 {
        [(0, 1), (0, 2), (1, 2)].iter()
            .map(|(a, b)| if self.sides.get(*a) == self.sides.get(*b) { 1 } else { 0 })
            .sum()
    }
    pub fn is_equilateral(&self) -> bool {
        self.count() == 3
    }
    pub fn is_scalene(&self) -> bool {
        self.count() == 0
    }
    pub fn is_isosceles(&self) -> bool {
        self.count() == 1
    }
}
/*
use std::ops::Add;

pub struct Triangle<T> {
   sides: [T; 3],
}

impl<T> Triangle<T>
where
   T: Copy + PartialEq + PartialOrd + Add<Output = T> + Default,
{
   fn valid_sides(&self) -> bool {
       (self.sides.iter().all(|&s| s > T::default()))
           && (self.sides[0] + self.sides[1] >= self.sides[2])
           && (self.sides[1] + self.sides[2] >= self.sides[0])
           && (self.sides[2] + self.sides[0] >= self.sides[1])
   }

   fn count_distinct_pairs(&self) -> usize {
       [(0, 1), (0, 2), (1, 2)]
           .iter()
           .map(|&(a, b)| if self.sides[a] != self.sides[b] { 1 } else { 0 })
           .sum()
   }

   pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
       let t = Triangle { sides: sides };

       if t.valid_sides() {
           Some(t)
       } else {
           None
       }
   }

   pub fn is_equilateral(&self) -> bool {
       self.count_distinct_pairs() == 0
   }

   pub fn is_isosceles(&self) -> bool {
       self.count_distinct_pairs() == 2
   }

   pub fn is_scalene(&self) -> bool {
       self.count_distinct_pairs() == 3
   }
}
*/

// 30 罗马数字
/*
写一个函数,从普通数字，转换成罗马数字.

罗马人是一群聪明的人。他们征服了欧洲大部分国家,统治了几百年。他们发明了混凝土和直路,甚至 Bikinis 夜店。他们从来没有发现过的一件事就是数字零。这使得写作和约会他们的功绩的广泛历史稍有挑战性，但他们提出的数字系统仍在使用。例如,英国广播公司使用罗马数字制定他们的节目。

罗马人用字母 I（1）、V（5）、X（10）、L（50）、C（100）、D（500）和 M（1000） 写数字(注意这些字母有很多直线,因此很容易侵入石碑)。


 1  => I
10  => X
 7  => VII
不需要能 转换 超过 3000 的罗马数字

在较大的罗马数字的右边记上较小的罗马数字，表示大数字加小数字。 在较大的罗马数字的左边记上较小的罗马数字，表示大数字减小数字。

要在实践中看到这一点,请考虑 1990 的例子.

在罗马数字中,1990 是 MCMXC:

1000=M 900=CM 90=XC

CM = 1000 - 100 = 900

2008 被写成 MMVIII:

2000=MM 8=Ⅷ
*/
use std::fmt;

static ROMAN_MAP: [(usize, &'static str); 13] = [
   (1, "I"),
   (4, "IV"),
   (5, "V"),
   (9, "IX"),
   (10, "X"),
   (40, "XL"),
   (50, "L"),
   (90, "XC"),
   (100, "C"),
   (400, "CD"),
   (500, "D"),
   (900, "CM"),
   (1000, "M"),
];

pub struct Roman {
    num: usize,
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = "".to_string();
        let mut num = self.num;
        for &(unit, attr) in ROMAN_MAP.iter().rev() {
            while num >= unit {
                out.push_str(attr);
                num -= unit;
            }
        }
        write!(f, "{}", out)
    }
}

impl Roman {
    pub fn new(num: usize) -> Roman {
        Roman {
            num,
        }
    }
}

impl From<usize> for Roman {
    fn from(i: usize) -> Self {
        Roman::new(i)
    }
}
// 31 数制转换
/*
将一个数字，表示为一个基数中的数字序列，并转为其他基本

实施通用基本转换。给出一个a参数，表示为数字序列，将其转换为基数b。

在位置表示法中，以数字表示b可以被理解为权力的线性组合b.

数字 42，基本为 10，意思是:

(4 * 10^1) + (2 * 10^0)

数字 101010，基本为 2，意思是:

(1 * 2^5) + (0 * 2^4) + (1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (0 * 2^0)

号码 1120，基本为 3，意思是:

(1 * 3^3) + (1 * 3^2) + (2 * 3^1) + (0 * 3^0)

我想你明白了!

是。上面这三个数字完全一样。恭喜!
*/
#[derive(Debug, PartialEq)]
pub enum Error31 {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error31> {
    if from_base < 2 {
        return Err(Error31::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error31::InvalidOutputBase);
    }
    for n in number {
        if *n >= from_base {
            return Err(Error31::InvalidDigit(*n));
        }
    }
    let mut sum = number
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, n)| {
            let n = *n;
            acc + n * from_base.pow(index as u32)
        });
    println!("{}", sum);
    let mut result = Vec::new();
    while sum > 0 {
        result.push(sum % to_base);
        sum /= to_base;
        println!("{}", sum);
    }
    result.reverse();
    Ok(result)
}

// 32 学册
/*
根据学生的姓名以及他们所处的年级，为学校创建一个名册.

最后，你应该能够:

将学生的姓名添加到年级名册
“Add Jim to grade 2.”
“OK.”
获取所有注册年级的学生列表
“哪个学生在二年级?”
“We’ve only got Jim just now.”
获取所有年级所有学生的排序列表。年级应分为 1，2，3 级，年级中的学生应按名称按字母顺序排序。
“谁现在都在学校就读?”
“Grade 1: Anna, Barb, and Charlie. Grade 2: Alex, Peter, and Zoe. Grade 3…”
请注意，我们所有学生只有一个名字.(这是一个小镇，你想要什么?)
*/

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grades.entry(grade).or_insert(vec![]);
        students.push(student.to_string());
        students.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut g = self.grades.keys().cloned().collect::<Vec<u32>>();
        g.sort();
        g
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|v| v.iter().cloned().collect())
    }
}

// 33 二分查找
/*
实现二分查找算法.

搜索已排序的集合是一项常见任务。字典是定义单词的排序列表。有了一个词，就可以找到它的定义。电话簿是人员姓名，地址和电话号码的分类列表。知道某人的姓名可以让他们快速找到他们的电话号码和地址。

如果要搜索的列表包含多个项目(比如十几个)，则二分查找将比线性搜索需要更少的比较，但它强制要求对列表进行排序。

在计算机科学中，二分查找或半间隔搜索算法在按键值排序的数组中，查找指定输入值(搜索”关键字”)的位置。

在每个步骤中，算法将搜索关键字值与数组中间元素的值进行比较。

如果匹配，则找到匹配元素，并返回其索引或位置。

否则，如果搜索关键字小于中间元素的键，则算法在中间元素左侧的子阵列上重复其操作，或者如果搜索关键字更大，则在右侧的子阵列上重复其操作。

如果要搜索的剩余阵列为空，则在阵列中找不到该键值，并返回特殊的”not found”指示。

二分查找将每次迭代检查的项目数减半，因此定位项目(或确定其不存在)需要对数时间。二分搜索是一种对半分，并渐进的搜索算法。
*/
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut a = 0;
    let mut b = array.len();
    if array.is_empty() {
        return None;
    }
    if array[a] == key {
        return Some(a);
    }
    if array[b-1] == key {
        return Some(b-1);
    }
    let mut m = b -  a;
    while m > 1 {
        let x = a + m / 2;
        println!("{}, {}, {:?}", a, b, array[x]);
        if array[x] == key {
            return Some(x);
        } else if array[x] > key {
            b = x;
        } else {
            a = x;
        }
        m = b - a;
    }
    None
}

// 34 机器人模拟器
/*
编写机器人模拟器。

机器人工厂的测试设施需要一个程序，来验证机器人的运动。

机器人有三种可能的运动:

右转
左转
前进
机器人被放置在一个假设的无限网格上，以一组{x，y}坐标，例如{3，8}面向特定方向(北、东、南或西)，能向北和东前进。

然后，机器人接收许多指令，测试设备验证机器人的新位置以及指向哪个方向。

字母串”RAALAR”的意思是:
右转
前两次
向左拐
前一次
再次左转
假设一个机器人从{ 7， 3 }向北开始，然后运行这个指令流，它应该就放在面向西方的{ 9, 4 }上。
*/
#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Debug, Clone)]
pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot::build(x, y, d)
    }
    pub fn build(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }
    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => {
                Robot::build(self.position.0, self.position.1, Direction::East)
            },
            Direction::East => {
                Robot::build(self.position.0, self.position.1, Direction::South)
            },
            Direction::South => {
                Robot::build(self.position.0, self.position.1, Direction::West)
            },
            Direction::West => {
                Robot::build(self.position.0, self.position.1, Direction::North)
            },
        }
    }
    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => {
                Robot::build(self.position.0, self.position.1, Direction::West)
            },
            Direction::East => {
                Robot::build(self.position.0, self.position.1, Direction::North)
            },
            Direction::South => {
                Robot::build(self.position.0, self.position.1, Direction::East)
            },
            Direction::West => {
                Robot::build(self.position.0, self.position.1, Direction::South)
            },
        }
    }
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => {
                Robot::build(self.position.0, self.position.1 + 1, self.direction)
            },
            Direction::East => {
                Robot::build(self.position.0 + 1, self.position.1, self.direction)
            },
            Direction::South => {
                Robot::build(self.position.0, self.position.1 - 1, self.direction)
            },
            Direction::West => {
                Robot::build(self.position.0 - 1, self.position.1, self.direction)
            },
        }
    }
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self.clone(), |robot, instruction| {
                match instruction {
                    'R' => robot.turn_right(),
                    'L' => robot.turn_left(),
                    'A' => robot.advance(),
                    _ => robot,
                }
            })
    }
    pub fn position(&self) -> (i32, i32) {
        self.position
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
/*
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
   North,
   East,
   South,
   West,
}

impl Direction {
   pub fn previous_clockwise(&self) -> Self {
       match *self {
           Direction::North => Direction::West,
           Direction::East => Direction::North,
           Direction::South => Direction::East,
           Direction::West => Direction::South,
       }
   }

   pub fn next_clockwise(&self) -> Self {
       match *self {
           Direction::North => Direction::East,
           Direction::East => Direction::South,
           Direction::South => Direction::West,
           Direction::West => Direction::North,
       }
   }
}

#[derive(Clone, Copy)]
struct Position {
   x: i32,
   y: i32,
}

impl Position {
   fn new(x: i32, y: i32) -> Self {
       Position { x: x, y: y }
   }

   fn advance(&self, direction: &Direction) -> Self {
       match *direction {
           Direction::North => Self::new(self.x, self.y + 1),
           Direction::South => Self::new(self.x, self.y - 1),
           Direction::East => Self::new(self.x + 1, self.y),
           Direction::West => Self::new(self.x - 1, self.y),
       }
   }
}

#[derive(Clone)]
pub struct Robot {
   position: Position,
   direction: Direction,
}

impl Robot {
   pub fn new(x: i32, y: i32, d: Direction) -> Self {
       Robot::build(Position::new(x, y), d)
   }

   fn build(position: Position, direction: Direction) -> Self {
       Robot {
           position: position,
           direction: direction,
       }
   }

   pub fn turn_right(&self) -> Self {
       Self::build(self.position, self.direction.next_clockwise())
   }

   pub fn turn_left(&self) -> Self {
       Self::build(self.position, self.direction.previous_clockwise())
   }

   pub fn advance(&self) -> Self {
       Self::build(self.position.advance(&self.direction), self.direction)
   }

   pub fn instructions(&self, instructions: &str) -> Self {
       instructions
           .chars()
           .fold(self.clone(), |robot, instruction| {
               robot.execute(instruction)
           })
   }

   pub fn position(&self) -> (i32, i32) {
       (self.position.x, self.position.y)
   }

   pub fn direction(&self) -> &Direction {
       &self.direction
   }

   fn execute(self, command: char) -> Self {
       match command {
           'R' => self.turn_right(),
           'L' => self.turn_left(),
           'A' => self.advance(),
           _ => self,
       }
   }
}
*/

// 35 括号配套
/*
给定包含括号[]大括号{}括号()或它们的任何组合的字符串，验证任何和所有对，都被正确地匹配和嵌套
*/
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![' '];
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => { brackets.push(c);},
            '}' => {
                if *brackets.last().unwrap() == '{' {
                    brackets.pop();
                } else {
                    return false;
                }},
            ']' => {
                if *brackets.last().unwrap() == '[' {
                    brackets.pop();
                } else {
                    return false;
                }},
            ')' => {
                if *brackets.last().unwrap() == '(' {
                    brackets.pop();
                } else {
                    return false;
                }},
            _ => {},
        }
    }
    brackets.len() == 1
}
/*
use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
   Brackets::from(string).are_balanced()
}

struct Brackets {
   raw_brackets: Vec<char>,
   pairs: MatchingBrackets,
}

impl<'a> From<&'a str> for Brackets {
   fn from(i: &str) -> Self {
       Brackets::new(String::from(i), None)
   }
}

impl Brackets {
   fn new(s: String, pairs: Option<Vec<(char, char)>>) -> Self {
       let p = match pairs {
           Some(x) => MatchingBrackets::from(x),
           None => MatchingBrackets::from(vec![('[', ']'), ('{', '}'), ('(', ')')]),
       };

       Brackets {
           raw_brackets: s.chars().filter(|c| p.contains(&c)).collect::<Vec<char>>(),
           pairs: p,
       }
   }

   fn are_balanced(&self) -> bool {
       let mut unclosed: Vec<char> = Vec::new();

       for &bracket in self.raw_brackets.iter() {
           if let Some(last_unclosed) = unclosed.pop() {
               unclosed.extend(self.pairs.unmatched(last_unclosed, bracket));
           } else {
               unclosed.push(bracket);
           }
       }

       unclosed.is_empty()
   }
}

struct MatchingBrackets {
   collection: HashMap<char, char>,
}

impl From<Vec<(char, char)>> for MatchingBrackets {
   fn from(v: Vec<(char, char)>) -> Self {
       MatchingBrackets {
           collection: v.into_iter().collect::<HashMap<char, char>>(),
       }
   }
}

impl MatchingBrackets {
   fn contains(&self, other: &char) -> bool {
       let known = self.collection
           .keys()
           .chain(self.collection.values())
           .collect::<Vec<_>>();
       known.contains(&other)
   }

   fn closer_for(&self, k: &char) -> Option<&char> {
       self.collection.get(k)
   }

   fn closed_by(&self, l: char, r: char) -> bool {
       match self.closer_for(&l) {
           Some(&x) => r == x,
           None => false,
       }
   }

   fn unmatched(&self, open: char, potential_close: char) -> Vec<char> {
       let mut ret: Vec<char> = Vec::new();

       if !self.closed_by(open, potential_close) {
           ret.push(open);
           ret.push(potential_close);
       }

       ret
   }
}
*/

// 37 皇后攻击
/*
给定棋盘上的两个皇后的位置，指示在各自位置的它们，是否能互相攻击。

在象棋游戏中，女王可以攻击同一行、列或对角线上的棋子。

棋盘可以用 8 乘 8 的数组来表示。

所以如果你告诉白皇后在(2， 3)和黑皇后在(5， 6)，那么设定像这样:


_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ W _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ B _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
你也可以回答女王是否可以互相攻击。而在这种情况下，答案是肯定的，他们可以，因为这两个部分共用一个对角线。
*/
#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition {
                x: rank,
                y: file,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (x, y) = (self.position.x, self.position.y);
        let (other_x, other_y) = (other.position.x, other.position.y);
        if x == other_x || y == other_y || (x-other_x).abs() == (y-other_y).abs() {
            true
        } else {
            false
        }
    }
}
/*
#[derive(Debug)]
pub struct Queen {
   position: ChessPosition,
}

pub trait ChessPiece {
   fn position(&self) -> &ChessPosition;
   fn can_attack<T: ChessPiece>(&self, other: &T) -> bool;
}

impl ChessPiece for Queen {
   fn position(&self) -> &ChessPosition {
       &self.position
   }

   fn can_attack<T: ChessPiece>(&self, piece: &T) -> bool {
       self.position.horizontal_from(&piece.position())
           || self.position.vertical_from(&piece.position())
           || self.position.diagonal_from(&piece.position())
   }
}

impl Queen {
   pub fn new(position: ChessPosition) -> Queen {
       Queen { position: position }
   }
}

#[derive(Debug)]
pub struct ChessPosition {
   pub rank: i8,
   pub file: i8,
}

impl ChessPosition {
   pub fn new(rank: i8, file: i8) -> Option<Self> {
       let position = ChessPosition {
           rank: rank,
           file: file,
       };

       if position.is_valid() {
           Some(position)
       } else {
           None
       }
   }

   fn is_valid(&self) -> bool {
       self.rank >= 0 && self.rank <= 7 && self.file >= 0 && self.file <= 7
   }

   fn horizontal_from(&self, other: &ChessPosition) -> bool {
       self.rank == other.rank
   }

   fn vertical_from(&self, other: &ChessPosition) -> bool {
       self.file == other.file
   }

   fn diagonal_from(&self, other: &ChessPosition) -> bool {
       self.sum() == other.sum() || self.difference() == other.difference()
   }

   fn sum(&self) -> i8 {
       self.rank + self.file
   }

   fn difference(&self) -> i8 {
       self.rank - self.file
   }
}
*/

// 39 子列表
/*
给定两个列表确定第一个列表是否包含在第二个列表中；第二个列表是否包含在第一个列表中；两个列表是否包含在彼此，或者这些都不是真的。

具体来说，列表 A 是列表 B 的子列表，在于是否 B 的前面删除 0 个或更多元素和 B 的后面删除 0 个或更多元素，则得到完全等于 A 的列表。

例子:

A = [1,2,3]，B = [1,2,3,4,5]，A 是 B 的子列表
A = [3,4,5]，B = [1,2,3,4,5]，A 是 B 的子列表
A = [3,4]，B = [1,2,3,4,5]，A 是 B 的子列表
A = [1,2,3]，B = [1,2,3]，A 等于 B。
A = [1,2,3,4,5]，B = [2,3,4]，A 是 B 的父列表
A = [1,2,4]，B = [1,2,3,4,5]，A 不是 B 的父列表，子列表，或等于 B 的列表
*/
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if compare(first_list, second_list) {
        Comparison::Sublist
    } else if compare(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn compare<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    if short.len() > long.len() {
        return false;
    }
    if long.starts_with(short) {
        return true;
    }

    compare(short, &long[1..])
}

// 41 地球年
/*
给定年龄(以秒为单位)，计算某人的年龄:

地球：轨道周期 365.25 地球日,或 31557600 秒
水星：轨道周期 0.2408467 地球年
金星：轨道周期 0.61519726 地球年
火星：轨道周期 1.8808158 地球年
木星：轨道周期 11.862615 地球年
土星：轨道周期 29.447498 地球年
天王星：轨道周期 84.016846 地球年
海王星：轨道周期 164.79132 地球年
因此，如果你被告知某人的年龄为 1,000,000,000 秒，你应该可以说它们的年龄为 31.69 地球年.
*/
#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            seconds: s as f64,
        }
    }
}

impl From<f64> for Duration {
    fn from(s: f64) -> Self {
        Duration {
            seconds: s,
        }
    }
}

pub trait Planet {
    fn trans_duration() -> Duration;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::trans_duration().seconds
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn trans_duration() -> Duration {
        Duration::from(7600543.81992)
    }
}

impl Planet for Venus {
    fn trans_duration() -> Duration {
        Duration::from(19414149.052176)
    }
}

impl Planet for Earth {
    fn trans_duration() -> Duration {
        Duration::from(31557600)
    }
}

impl Planet for Mars {
    fn trans_duration() -> Duration {
        Duration::from(59354032.69008)
    }
}

impl Planet for Jupiter {
    fn trans_duration() -> Duration {
        Duration::from(374355659.124)
    }
}

impl Planet for Saturn {
    fn trans_duration() -> Duration {
        Duration::from(929292362.8848)
    }
}

impl Planet for Uranus {
    fn trans_duration() -> Duration {
        Duration::from(2651370019.3296)
    }
}

impl Planet for Neptune {
    fn trans_duration() -> Duration {
        Duration::from(5200418560.032)
    }
}

// 42 宏
/*
宏是 Rust 程序员工具箱中的一个强大工具，要想对它有个简单认知，可以看看宏例子。让我们写一个!

问题陈述
你可以用vec![]内联宏，生产一个任意长度的Vec。然而,Rust 并没有生产HashMap的内联宏hashmap!()。

例如，您的库的用户可能会编写hashmap!('a' => 3, 'b' => 11, 'z' => 32)。这应该扩展到以下代码:



{
   let mut hm = HashMap::new();
   hm.insert('a', 3);
   hm.insert('b', 11);
   hm.insert('z', 32);
   hm
}
*/
#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $value: expr),* $(,)*) => {
       {
          let mut _map = ::std::collections::HashMap::new();
          $(
             _map.insert($key, $value);
          )*
          _map
       }
    };
 }

// 43 过敏源
/*
给出一个人的过敏分，确定他们是否对某一物品过敏，以及他们的过敏列表.

过敏测试产生单个数字分数，其中包含有关该人所有过敏的信息(他们进行了测试)。

测试的物品列表(及其值)为:

鸡蛋(1)
花生(2)
贝类(4)
草莓(8)
西红柿(16)
巧克力(32)
花粉(64)
猫(128)
因此，如果汤姆对花生和巧克力过敏，他会得到 34 分.

现在，只要得到 34 分，你的程序应该说:

汤姆是否对上面列出的任何一种过敏原过敏.
汤姆所有过敏原。
注意：给出的分数可能不包括上面列出的过敏原(即分数为 256，512，1024 等的过敏原)。您的程序应忽略这些组成部分。例如，如果过敏分数是 257，您的程序应该只报告鸡蛋(1)过敏
*/
pub struct Allergies(u32);

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let index = Allergies::allergen()
            .iter()
            .position(|a| a == allergen)
            .unwrap();
        self.0 & 1 << index as u32 != 0
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        Allergies::allergen()
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
    fn allergen() -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}

// 44 可变长度数量
/*
实现可变长度数量编码和解码.

这项工作的目标是实现VLQ的编码/解码。

简而言之，此编码的目标是以节省字节的方式，对整数值进行编码。只有每个字节的前 7 位是有效的(右对齐；有点像 ASCII 字节)。因此，如果您有 32 位值，则必须解开为一系列 7 位字节。当然，根据您的整数，您将拥有可变数量的字节。要指出哪个是系列的最后一个字节，请将 #7 位清零。而所有前面的字节中，您都要设置#7 位。

所以，如果一个整数介于0-127，它可以表示为一个字节。虽然 VLQ 可以处理任意大小的数字，但对于本练习，我们将仅限于适合 32 位无符号整数的数字。以下是整数作为 32 位值的示例，以及它们转换为的可变长度数量:


 NUMBER        VARIABLE QUANTITY
00000000              00
00000040              40
0000007F              7F
00000080             81 00
00002000             C0 00
00003FFF             FF 7F
00004000           81 80 00
00100000           C0 80 00
001FFFFF           FF FF 7F
00200000          81 80 80 00
08000000          C0 80 80 00
0FFFFFFF          FF FF FF 7F
*/
#[derive(Debug, PartialEq)]
pub enum Error44 {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = vec![];
    let mut temp;
    for value in values {
        let mut value = *value;
        temp = Vec::with_capacity(4);
        if value == 0 {
            temp = vec![0];
        } else {
            while value > 0 {
                let mut v = (value & 0x7f) as u8;
                if !temp.is_empty() {
                    v |= 0x80;
                }
                temp.push(v);
                value >>= 7;
            }
        }
        temp.reverse();
        result.append(& mut temp);
    }
    result
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error44> {
    let mut result = vec![];
    let mut temp: u32 = 0;
    for (i, v) in bytes.iter().enumerate() {
        if temp & 0xfe_00_00_00 > 0 {
            return Err(Error44::Overflow);
        }
        temp = (temp << 7) | (v & 0x7f) as u32;
        if v & 0x80 == 0 {
            result.push(temp);
            temp = 0;
        } else {
            if i + 1 == bytes.len() {
                return Err(Error44::IncompleteNumber);
            }
        }
    }
    Ok(result)
}

// 45 电话号码
/*
整理用户输入的电话号码，以便他们可以发送短信.

这个**北美编号计划(NANP)**是北美洲、加拿大或百慕大群岛等许多国家使用的电话号码系统。所有 NANP 国家共享相同的国际国家代码:1。

NANP 数字是十位数字，由三位编号——区域划分代码组成，俗称地区代码其次是一个七位数的本地号码。本地号码的前三位数字表示交换码，剩余是唯一的四位数字，这是用户号码。

格式通常表示为


(NXX)-NXX-XXXX
这里的N是从 2 到 9 的任何数字，而X是从 0 到 9 的任何数字.

您的任务是通过删除标点符号和国家代码(1)，整理不同格式的电话号码。

例如，输入

+1 (613)-995-0253
613-995-0253
1 613 995 0253
613.995.0253
都应该产出

6139950253

**注:**因为这个练习只涉及 NANP 国家使用的电话号码，只有 1 被认为是有效的国家代码
*/
pub fn number(user_number: &str) -> Option<String> {
    let number = user_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();
    let is_valid = |s: &[char]| {
        s[0] > '1' && s[0] <= '9' && s[3] > '1' && s[3] <= '9'
    };
    if number.len() == 11 && number[0] == '1' && is_valid(&number[1..]) {
        return Some(number[1..].iter().collect::<String>());
    }
    if number.len() == 10 && is_valid(&number) {
        return Some(number.iter().collect::<String>());
    }
    None
}
/*
pub fn number(s: &str) -> Option<String> {
    let digits: String = s.chars().filter(|&c| c.is_digit(10)).collect();
    match digits.len() {
        10 => match (digits.chars().nth(0), digits.chars().nth(3)) {
            (Some('0'), _) => None,
            (Some('1'), _) => None,
            (_, Some('0')) => None,
            (_, Some('1')) => None,
            _ => Some(digits),
        },
        11 => match digits.chars().nth(0) {
            Some('1') => number(&digits[1..]),
            _ => None,
        },
        _ => None,
    }
}
*/

// 46 计算
/*
解析并运算简单的数学单词问题，将答案作为整数返回。

迭代 1 - 加法
将两个数字相加

What is 5 plus 13?

运算为 18.

处理大数和负数。

迭代 2 - 减法，乘法和除法
现在，执行其他三个操作.

What is 7 minus 5?

2

What is 6 multiplied by 4?

24

What is 25 divided by 5?

五

迭代 3 - 多个操作
按顺序处理一组操作.

由于这些是口头语言问题，从左到右运算表达式，忽略了典型的操作顺序(乘法优先级比加法高)。

What is 5 plus 13 plus 6?

24

What is 3 plus 2 multiplied by 3?

15(不是 9)
*/
#[derive(Debug)]
pub struct WordProblem {
    op: Vec<char>,
    d: Vec<i32>,
}
impl WordProblem {
    pub fn new(command: &str) -> Self {
        let mut op = vec![];
        let mut d = vec![];
        for s in command
                    .trim_end_matches(|c| c == '?')
                    .split_ascii_whitespace() {
            match s {
                "plus" => op.push('+'),
                "multiplied"  => op.push('*'),
                "minus"  => op.push('-'),
                "divided" => op.push('/'),
                s if s.chars().nth(0).unwrap().is_ascii_digit() => {
                    d.push(s.chars()
                        .rev()
                        .enumerate()
                        .map(|(i, c)|
                            c.to_digit(10).unwrap() * 10_u32.pow(i as u32))
                        .sum::<u32>() as i32);
                },
                s if s.chars().nth(0).unwrap() == '-' => {
                    let temp = s[1..].chars()
                        .rev()
                        .enumerate()
                        .map(|(i, c)|
                            c.to_digit(10).unwrap() * 10_u32.pow(i as u32))
                        .sum::<u32>() as i32;
                    d.push(temp - 2 * temp);
                },
                _ => {},
            }
        }
        WordProblem {
            op,
            d,
        }
    }
    pub fn compute(&self) -> Option<i32> {
        if self.op.is_empty() {
            return None;
        }
        let mut res = self.d[0];
        for (i, s) in self.op.iter().enumerate() {
            let temp = self.d[i+1];
            match s {
                '+' => res += temp,
                '-' => res -= temp,
                '*' => res *= temp,
                '/' => res /= temp,
                _ => {},
        }
        }
        Some(res)
    }
}

pub fn answer(command: &str) -> Option<i32> {
    WordProblem::new(command).compute()
}
/*
struct Token<'a> {
   value: &'a str,
}

impl <'a> Token<'a> {
   fn is_valid(&self) -> bool {
       !self.value.is_empty() && (self.is_operand() || self.is_operator())
   }

   fn is_operand(&self) -> bool {
       self.value.chars().all(|c| c.is_numeric() || c == '-')
   }

   fn is_operator(&self) -> bool {
       self.value == "plus"
           || self.value == "minus"
           || self.value == "multiplied"
           || self.value == "divided"
   }
}

pub fn answer(c: &str) -> Option<i32> {
   let mut t = tokens(c);
   let mut result: i32 = 0;
   let mut opr = "plus";

   if t.len() <= 1 {
       None
   } else {
       while t.len() > 1 {
           result = evaluate(result, opr, operand(&t.remove(0)));
           opr = operator(&t.remove(0));
       }
       result = evaluate(result, opr, operand(&t.remove(0)));
       Some(result)
   }
}

fn evaluate(r: i32, operator: &str, operand: i32) -> i32 {
   match operator {
       "plus" => r + operand,
       "minus" => r - operand,
       "multiplied" => r * operand,
       "divided" => r / operand,
       _ => r,
   }
}

fn operand(t: &Token) -> i32 {
   t.value.parse().unwrap()
}

fn operator<'a>(t: &Token<'a>) -> &'a str {
   t.value
}

fn tokens<'a>(command: &'a str) -> Vec<Token<'a>> {
   command
       .split(|c: char| c.is_whitespace() || c == '?')
       .map(|w| Token {
           value: w,
       })
       .filter(|t| t.is_valid())
       .collect()
}
*/

// 47 足球比赛记录
/*
统计一场小型足球比赛的结果.

基于一个输入文件，它包含哪个队与哪个队进行比赛，结果是什么。用下面的表格创建出一个文件:


Team                           | MP |  W |  D |  L |  P
Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
Blithering Badgers             |  3 |  1 |  0 |  2 |  3
Courageous Californians        |  3 |  0 |  1 |  2 |  1
那些缩写是什么意思?

MP：赛次
W：比赛赢了
D：打平
L：比赛输了
P：分
一场胜利，3 分。打平 1 分。输了 0 分.

结果应该按分数下降排序。在平局的情况下，球队按字母顺序排列。

输入
你的梳理程序，将接收像这样的输入:


Allegoric Alaskans;Blithering Badgers;win
Devastating Donkeys;Courageous Californians;draw
Devastating Donkeys;Allegoric Alaskans;win
Courageous Californians;Blithering Badgers;loss
Blithering Badgers;Devastating Donkeys;loss
Allegoric Alaskans;Courageous Californians;win
一行中首个队伍名是主队。所以这一行


Allegoric Alaskans;Blithering Badgers;win
意味着，Allegoric Alaskans 打败了 Blithering Badgers。

这行:


Courageous Californians;Blithering Badgers;loss
意味着，Courageous Californians 输给了 Blithering Badgers

这行:


Devastating Donkeys;Courageous Californians;draw
意味着，Devastating Donkeys 与 Courageous Californians 打平
*/
use std::cmp::Ordering::Equal;

pub struct FootballGame {
    record: HashMap<String, Vec<u32>>,
}
impl FootballGame {
    pub fn new(input: &str) -> Self {
        if input.is_empty() {
            let mut res = HashMap::new();
            res.insert("".to_string(), vec![]);
            return FootballGame {
                record: res,
            };
        }
        let mut res = HashMap::new();
        let competes = input.split(|c| c == '\n').collect::<Vec<&str>>();
        for compete in competes {
            let temp = compete.split(|c| c == ';').collect::<Vec<&str>>();
            let first = res
                .entry(temp[0].to_string())
                .or_insert(vec![0, 0, 0, 0, 0]);
            first[0] += 1;
            match temp[2] {
                "win" => {
                    first[1] += 1;
                    first[4] += 3;
                }
                "draw" => {
                    first[2] += 1;
                    first[4] += 1;
                }
                "loss" => {
                    first[3] += 1;
                }
                _ => {}
            }
            let second = res
                .entry(temp[1].to_string())
                .or_insert(vec![0, 0, 0, 0, 0]);
            second[0] += 1;
            match temp[2] {
                "loss" => {
                    second[1] += 1;
                    second[4] += 3;
                }
                "draw" => {
                    second[2] += 1;
                    second[4] += 1;
                }
                "win" => second[3] += 1,
                _ => {}
            }
        }
        FootballGame { record: res }
    }
    pub fn show(&self) -> String {
        if self.record.contains_key("") {
            return format!("{:30} | MP |  W |  D |  L |  P", "Team");
        }
        let mut v: Vec<_> = self.record
            .iter()
            .map(|(team, r)| {
                (team, r[0], r[1], r[2], r[3], r[4])
            })
            .collect();
            v.sort_by(|a, b| match b.5.cmp(&(a.5)) {
                Equal=> a.0.cmp(&(b.0)),
                other=> other,
            });
        let mut res = vec![format!("{:30} | MP |  W |  D |  L |  P", "Team")];
        for (team, mp, w, d, l, p) in v {
            res.push(format!(
                "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                team, mp, w, d, l, p
            ));
        }
        res.join("\n")
    }
}

pub fn tally(input: &str) -> String {
    FootballGame::new(input).show()
}

// 48 自定义集合
/*
创建自定义 set 类型。

有时需要定义某种类型的自定义数据结构，例如一个集合(set)。在本练习中，您将定义自己的集合。它的内部工作原理无关紧要，只要它的行为就像一组唯一元素的集合
*/
#[derive(Debug)]
pub struct CustomSet<T> {
    date: Vec<T>,
}

impl<T: Ord + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.date.iter().all(|x| other.date.contains(x))
            && other.date.iter().all(|x| self.date.contains(x))
    }
}

impl<T: Ord + Clone> CustomSet<T> {

    pub fn new(input: &[T]) -> Self {
        let temp = input.iter().map(|d| d.clone()).collect::<Vec<_>>();
        CustomSet {
            date: temp,
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.date.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.date.contains(&element) {
            self.date.push(element.clone());
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.date.iter().all(|x| other.date.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.date.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.date.iter().any(|x| other.date.contains(x))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new(&self.date.iter()
            .cloned()
            .filter(|x| other.contains(x))
            .collect::<Vec<_>>())
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new(&self.date.iter()
            .cloned()
            .filter(|x| !other.contains(x))
            .collect::<Vec<_>>())
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet::new(&self.date.iter()
            .cloned()
            .chain(other.date.iter().cloned())
            .collect::<Vec<_>>())
    }
}

// 49 算术谜题
/*
写一个函数来解决字母谜题.

Alphametics是一个拼图，其中单词中的字母被数字替换.

例如SEND + MORE = MONEY:


  S E N D
  M O R E +
-----------
M O N E Y
用有效数字替换它们会给出:


  9 5 6 7
  1 0 8 5 +
-----------
1 0 6 5 2
这是正确的，因为每个字母都被不同的数字替换，之后单词会被翻译成数字，然后产生有效的总和。

每个字母必须代表不同的数字，并且多位数的首数字不能是 0 。

写一个函数来解决字母谜题.
*/
#[allow(dead_code)]
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let temp_str_list: Vec<&str> = input
        .split(|x| x == '+' || x == '=')
        .filter(|s| *s != "")
        .map(|s: &str| s.trim())
        .collect();
    let mut res: HashMap<char, u8> = HashMap::new();
    let mut first_big = vec![];
    for s in temp_str_list.clone() {
        for (idx, c) in s.chars().enumerate() {
            let d = res.entry(c.clone()).or_insert(0);
            if idx == 0 {
                first_big.push(c.clone());
                *d = 1;
            }
        }
    }
    let mut key_list = vec![];
    for c in res.keys() {
        key_list.push(c.clone());
    }
    key_list.sort();
    let mut left: u32;
    let mut right;
    loop {
        println!("{:?}", res);
        let d_list = temp_str_list
            .clone()
            .into_iter()
            .map(|s| {
                s.chars()
                    .fold(0, |sum, c| sum * 10 + *res.get(&c).unwrap() as u32)
            })
            .collect::<Vec<u32>>();
        println!("{:?}", temp_str_list);
        println!("{:?}", d_list);
        left = d_list[0..(d_list.len()-1)].iter().sum();
        right = d_list[d_list.len()-1];
        if left == right {
            break;
        }
        for i in 0..key_list.len() {
            let temp = res.get_mut(&key_list[i]).unwrap();
            if *temp == 9 {
                if i == key_list.len()-1 {
                    return None;
                } else {
                    if first_big.contains(&key_list[i]) {
                        *temp = 1;
                    } else {
                        *temp = 0;
                    }
                }
            } else {
                *temp += 1;
                break;
            }
        }
    }
    Some(res)
}

// 50 两个桶
/*
给定两个不同尺寸的桶，演示如何通过两个桶，策略性地传输液体来测量精确的升数.

由于这个数学问题，很容易受到解释/个体方法的影响，因此这些测试代码专门针对期望一个总体解决方案而编写.

为了提供帮助，测试代码首先为桶(1/2)装满。这意味着，若开始时，选择较大的桶装满，那就不会出现，较小的桶装满而较大的桶为空的情况。(也就是说，起点应该用较小桶)；不然这会破坏比较两种方法的目的!

您的程序将作为输入:

桶 1 的大小
桶 2 的大小
要达到的理想升数
首先要装满哪个桶，要么是桶 1 还是桶 2
您的计划应确定:

达到所需的升数，所需的”移动”总数，包括第一次装满
哪个桶应该以所需的升数结束(假设这是桶 A) - 要么桶 1 ，要么桶 2
另一个桶中，剩下多少升(桶 B)
注意:任何时候对其中一个或两个桶进行更改，都计为一(1)次移动。

示例：桶 1 最多可容纳 7 升，桶 2 最多可容纳 11 升。让我们说桶 1，第一步，持有 7 升，桶 2 持有 8 升(7，8)。如果您清空桶 1，并且不对桶 2 进行任何更改，则分别为 0 升和 8 升(0，8)，这将作为一个”移动”。相反，如果你已经把桶 1 倒入桶 2，直到桶 2 充满，那留下的，桶 1 中的 4 升和桶 2 中的 11 升(4，11)，这也算作只有一个”移动”。

总而言之，唯一有效的行动是:

从一个桶，倒到另一个桶
清空一个桶，对另一个什么都不做
装满一个桶，对另一个什么也不做
*/
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve_50(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let state = match start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };

    let mut next_search = VecDeque::new();
    let mut visited = HashSet::new();
    let mut res: HashMap<(u8, u8), ((u8, u8), &str)> = HashMap::new();
    let mut moves = 1;

    next_search.push_front(state);

    visited.insert((capacity_1, 0));
    visited.insert((0, capacity_2));

    let initial = state;

    loop {
        let mut current_search = next_search;
        next_search = VecDeque::new();

        for state in current_search.drain(0..) {
            let (bucket_1, bucket_2) = state;
            if bucket_1 == goal || bucket_2 == goal {
                let mut tmp = vec![(state, "end")];
                let mut val = res.get(&state).unwrap();
                while val.0 != initial {
                    tmp.push(*val);
                    val = res.get(&val.0).unwrap();
                }
                tmp.push((val.0, val.1));
                tmp.reverse();
                println!("first capacity {}, second capacity {}", capacity_1, capacity_2);
                println!("{:?}", tmp);
            }
            if bucket_1 == goal {
                return BucketStats {
                    moves,
                    goal_bucket: Bucket::One,
                    other_bucket: bucket_2,
                }
            } else if bucket_2 == goal {
                return BucketStats {
                    moves,
                    goal_bucket: Bucket::Two,
                    other_bucket: bucket_1,
                }
            }

            let empty_1 = (0, bucket_2);
            if !visited.contains(&empty_1) {
                next_search.push_front(empty_1);
                visited.insert(empty_1);
                res.insert(empty_1, (state, "empty_1"));
            }

            let empty_2 = (bucket_1, 0);
            if !visited.contains(&empty_2) {
                next_search.push_front(empty_2);
                visited.insert(empty_2);
                res.insert(empty_2, (state, "empty_2"));
            }

            let fill_1 = (capacity_1, bucket_2);
            if !visited.contains(&fill_1) {
                next_search.push_front(fill_1);
                visited.insert(fill_1);
                res.insert(fill_1, (state, "fill_1"));
            }

            let fill_2 = (bucket_1, capacity_2);
            if !visited.contains(&fill_2) {
                next_search.push_front(fill_2);
                visited.insert(fill_2);
                res.insert(fill_2, (state, "fill_2"));
            }

            let pour_1_into_2 = if bucket_1 + bucket_2 <= capacity_1 {
                (bucket_1 + bucket_2, 0)
            } else {
                (capacity_1, bucket_1 + bucket_2 - capacity_1)
            };
            if !visited.contains(&pour_1_into_2) {
                next_search.push_front(pour_1_into_2);
                visited.insert(pour_1_into_2);
                res.insert(pour_1_into_2, (state, "1<-2"));
            }

            let pour_2_into_1 = if bucket_1 + bucket_2 <= capacity_2 {
                (0, bucket_1 + bucket_2)
            } else {
                (bucket_1 + bucket_2 - capacity_2, capacity_2)
            };
            if !visited.contains(&pour_2_into_1) {
                next_search.push_front(pour_2_into_1);
                visited.insert(pour_2_into_1);
                res.insert(pour_2_into_1, (state, "1->2"));
            }
        }
        moves += 1;
    }
}

// 51 猪的拉丁文
/*
实现一个从英语翻译成猪拉丁语的程序.

猪拉丁语是一种拼凑的儿童语言，目的是使人困惑。它遵循一些简单的规则(下面)，但是当它说得很快时，对于非儿童(以及非母语者)来说真的很难理解.

规则 1如果一个单词以元音开头，在单词的末尾加上一个”ay”音。请注意，在单词开头的”xr”和”yt”会产生元音(例如 “xray” -> “xrayay”, “yttria” -> “yttriaay”）。
规则 2如果一个单词以辅音开头，把它移到单词的末尾，然后在单词的末尾加上一个”ay”音。辅音可以由多个辅音组成，例如辅音群(例如”chair” -> “airchay”).
规则 3如果一个单词以辅音开头，后面跟着”qu”，把它移动到单词的结尾，然后在单词的结尾加上”ay”音(例如，”square” -> “aresquay”).
规则 4如果一个单词在辅音群后面包含”y”，或者作为两个字母元音的单词的第二个字母(例如，”rhythm” -> “ythmrhay”, “my” -> “ymay”)。
边缘案例还有一些规则，也有区域性的变化.
*/
pub fn translate(input: &str) -> String {
    let s_list: Vec<&str> = input.split_ascii_whitespace().collect();
    let mut res = vec![];
    let y = "aeiou";
    for s in s_list {
        let mut tmp = "".to_string();
        if s.starts_with(|c| y.contains(c))
            || s.starts_with("xr")
            || s.starts_with("yt") {
                tmp.push_str(s);
                tmp.push_str("ay");
                res.push(tmp);
                continue;
        }
        if let Some(i) = s.find("qu") {
            match i {
                0 | 1 => {
                    tmp.push_str(s.get((i+2)..).unwrap());
                    tmp.push_str(s.get(0..(i+2)).unwrap());
                    tmp.push_str("ay");
                    res.push(tmp);
                    continue;
                },
                _ => {},
            }
        }
        if let Some(i) = s.find('y') {
            match i {
                i if i > 1 => {
                    if s.get(0..i).unwrap().chars().all(|c| !y.contains(c)) {
                        tmp.push_str(s.get(i..).unwrap());
                        tmp.push_str(s.get(0..i).unwrap());
                        tmp.push_str("ay");
                        res.push(tmp);
                        continue;
                    }
                }
                _ => {},
            }
        }
        if let Some(i) = s.find(|c| y.contains(c)) {
            tmp.push_str(s.get(i..).unwrap());
            tmp.push_str(s.get(0..i).unwrap());
            tmp.push_str("ay");
            res.push(tmp);
        }
    }
    res.join(" ")
}
/*
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

// Regular expressions from Python version of exercism

pub fn translate_word(word: &str) -> String {
   // Prevent creation and compilation at every call.
   // These are compiled exactly once
   lazy_static! {
       // Detects if it starts with a vowel
       static ref VOWEL: Regex = Regex::new(r"^([aeiou]|y[^aeiou]|xr)[a-z]*").unwrap();
       // Detects splits for initial consonants
       static ref CONSONANTS: Regex = Regex::new(r"^([^aeiou]?qu|[^aeiou][^aeiouy]*)([a-z]*)").unwrap();
   }

   if VOWEL.is_match(word) {
       String::from(word) + "ay"
   } else {
       let caps = CONSONANTS.captures(word).unwrap();
       String::from(&caps[2]) + &caps[1] + "ay"
   }
}

pub fn translate(text: &str) -> String {
   text.split(" ")
       .map(|w| translate_word(w))
       .collect::<Vec<_>>()
       .join(" ")
}
*/

// 52  钻石
/*
钻石 kata 将一个字母作为输入，并以菱形输出。给定一个字母，它会打印一个以’A’开头的钻石，并在最宽处提供所提供的字母。

要求
第一行包含一个’A’.
最后一行包含一个’A’.
除第一行和最后一行之外的所有行都有两个完全相同的字母.
所有行都具有与前导空格一样多的尾随空格.(这可能是 0).
钻石是水平对称的.
钻石是垂直对称的.
钻石具有方形(宽度等于高度).
字母形成菱形.
上半部分的字母按升序排列.
下半部分的字母按降序排列.
四个角(包含空格)是三角形.
例子
在以下示例中，空格表示为·字符.

字母’A’的钻石:


A
字母’C’的钻石:


··A··
·B·B·
C···C
·B·B·
··A··
字母’E’的钻石:


····A····
···B·B···
··C···C··
·D·····D·
E·······E
·D·····D·
··C···C··
···B·B···
····A····
*/
pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    static ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let length = ((c as u8 - 'A' as u8) * 2 + 1) as usize;
    let mut res: Vec<String> = Vec::with_capacity(length);
    let mut len = length / 2;
    let mut tmp = "".to_string();
    for _ in 0..len {
        tmp.push(' ');
    }
    tmp.push('A');
    for _ in 0..len {
        tmp.push(' ');
    }
    res.push(tmp);
    for i in 1..(length / 2 + 1) {
        len -= 1;
        tmp = "".to_string();
        for _ in 0..len {
            tmp.push(' ');
        }
        tmp.push_str(ABC.get(i..i + 1).unwrap());
        for _ in 0..(i * 2 - 1) {
            tmp.push(' ');
        }
        tmp.push_str(ABC.get(i..i + 1).unwrap());
        for _ in 0..len {
            tmp.push(' ');
        }
        res.push(tmp);
    }
    let mut right = res.get(0..length / 2).unwrap().to_vec();
    right.reverse();
    res.extend(right);
    res
}


// 53 螺旋矩阵
/*
给定大小，以螺旋顺序返回数字的方阵.

矩阵应填充自然数字，从左上角的 1​​ 开始，以向内，顺时针螺旋顺序增加，如下例所示:

大小为 3 的螺旋矩阵

1 2 3
8 9 4
7 6 5
尺寸为 4 的螺旋矩阵

 1  2  3 4
12 13 14 5
11 16 15 6
10  9  8 7
*/
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut res = vec![vec![0; size as usize]; size as usize];
    let mut left = 0 as usize;
    let mut right = size as usize;
    let mut up = 0 as usize;
    let mut down = size as usize;
    let mut count: u32 = 0;
    while count != size * size {
        for j in left..right {
            count += 1;
            res[up][j] = count;
        }
        up += 1;

        for i in up..down {
            count += 1;
            res[i][right - 1] = count;
        }
        right -= 1;

        for j in (left..right).rev() {
            count += 1;
            res[down - 1][j] = count;
        }
        down -= 1;

        for i in (up..down).rev() {
            count += 1;
            res[i][left] = count;
        }
        left += 1;
    }
    res
}

// 54 回文乘数
/*
在给定范围内，检测回文乘数.

回文数是指当数字倒过来时，保持不变的数。例如,121是回文数，但112不是。

给定一系列数字，找出最大和最小回文，这是该范围内数字的乘积.

您的解决方案应该返回最大和最小回文，以及范围内的每个因素。如果最大或最小回文在范围内，有多于一对的因素，则返回所有的对.

例 1
给定范围[1, 9](包含 1，9)

并给出在这个范围内的列表中，所有可能的乘数:[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16, 18, 15, 21, 24, 27, 20, 28, 32, 36, 25, 30, 35, 40, 45, 42, 48, 54, 49, 56, 63, 64, 72, 81]

回文乘数都是单数数字(在这种情况下):[1, 2, 3, 4, 5, 6, 7, 8, 9]

最小回文乘数是1。 其因素是(1, 1)。 最大回文乘数是9。 其因素是(1, 9)和(3, 3)。

例 2
给定范围[10, 99](包含)

最小回文乘数是121。 其因素是(11, 11)。 最大回文乘数是9009。 其因素是(91, 99)
*/
pub type Palindrome = u64;

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut res: Vec<Palindrome> = vec![];
    for x in min..(max + 1) {
        for y in x..(max + 1) {
            let m = x * y;
            if !res.contains(&m) {
                let tmp = m.to_string().into_bytes();
                if tmp.iter().zip(tmp.iter().rev()).all(|(d1, d2)| d1 == d2) {
                    res.push(m);
                }
            }
        }
    }
    res
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().map(|d| *d).min()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().map(|d| *d).max()
}

// 56 grep
/*
在文件中，搜索与正则表达式模式匹配的行。返回每个匹配行的行号和内容.

Unixgrep命令可用于搜索，与用户提供的搜索查询匹配的一个或多个文件中的行(称为模式).

该grep命令有三个参数:

用于匹配文件中的行的模式.
零个或多个标志以自定义匹配行为.
一个或多个要搜索匹配行的文件.
你的任务是实现grep函数，应该读取指定文件的内容，找到与指定模式匹配的行，然后将这些行输出为单个字符串。请注意，行应按其找到的顺序输出，第一个文件中的第一个匹配行首先输出。

例如，假设有一个名为”input.txt”的文件，其中包含以下内容:


hello
world
hello again
如果我们打电话grep "hello" input.txt，返回的字符串应该是:


hello
hello again
旗
如前所述，grep命令还应该支持以下标志:

-n打印每个匹配行的行号.
-l仅打印包含至少一个匹配行的文件的名称.
-i使用不区分大小写的比较匹配行.
-v反转程序 - 收集所有与模式不匹配的行.
-x仅匹配整行，而不是匹配包含匹配的行.
如果我们运行grep -n "hello" input.txt，-nflag 将要求匹配的行以其行号作为前缀:


1:hello
3:hello again
如果我们运行grep -i "HELLO" input.txt，我们将做一个不区分大小写的匹配，输出将是:


hello
hello again
该grep命令应该一次支持多个标志.

例如，运行grep -l -v "hello" file1.txt file2.txt应该打印不包含字符串”hello”的文件的名称.
*/
use std::{fs, path::Path};

#[derive(Debug)]
pub enum FileAccessError {
    FileNotFoundErr(String),
    FileReadError(String),
}

pub struct Flags {
    print_line_number: bool,
    print_file_name: bool,
    use_case_insensitive_comparison: bool,
    use_inverted_comparison: bool,
    match_entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            print_line_number: flags.contains(&"-n"),
            print_file_name: flags.contains(&"-l"),
            use_case_insensitive_comparison: flags.contains(&"-i"),
            use_inverted_comparison: flags.contains(&"-v"),
            match_entire_line: flags.contains(&"-x"),
        }
    }
}

pub fn get_to_lines(file_name: &str) -> Result<Vec<String>, FileAccessError> {
    let path = Path::new(file_name);

    if !path.exists() {
        return Err(FileAccessError::FileNotFoundErr(file_name.to_string()));
    }

    if let Ok(content) = fs::read_to_string(path) {
        Ok(content.split("\n").map(|line| line.to_string()).collect())
    } else {
        Err(FileAccessError::FileReadError(file_name.to_string()))
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, FileAccessError> {
    let is_multi_file = files.len() > 1;
    let mut res: Vec<String> = vec![];

    for file_name in files.to_vec() {
        let file_lines = get_to_lines(file_name)?;

        res.extend(
            file_lines
                .iter()
                .enumerate()
                .filter(|&(_, line)| {
                    let mut line = line.clone().to_string();
                    let mut pattern = pattern.to_string();

                    if flags.use_case_insensitive_comparison {
                        line = line.to_lowercase();
                        pattern = pattern.to_lowercase();
                    }
                    let mut is_filtered = false;
                    if line.contains(&pattern) {
                        is_filtered = true;
                    }
                    if flags.match_entire_line {
                        is_filtered = line == pattern;
                    }
                    if flags.use_inverted_comparison {
                        is_filtered = !line.contains(&pattern);
                    }
                    is_filtered
                })
                .filter(|(_, line)| !line.is_empty())
                .map(|(line_number, line)| {
                    let mut tmp = line.clone();

                    if flags.print_line_number {
                        tmp.insert_str(0, &format!("{}. ", line_number + 1));
                    }

                    if is_multi_file {
                        tmp.insert_str(0, &format!("{} ", file_name));
                    }

                    if flags.print_file_name {
                        tmp = file_name.clone().to_string();
                    }

                    tmp
                }),
        );
    }

    res.dedup_by(|a, b| (*a).eq(b));
    Ok(res)
}

// 58 十进制加减乘
/*
实现任意精度的Decimal类。

浮点数是计算中非整数实数的最常见表示，它们是由IEEE 754标准定义。它们非常灵活且通用，但它们确实有一些局限性。众所周知，在浮点运算中，0.1 + 0.2 != 0.3。

解决这一问题的方法是，寻找另一种无损的方法来模拟任意精度的非整数 实数。这可能在内存或处理速度方面，不如浮点数有效；但目标是提供准确的结果。

尽管Decimal作为一种自定义类型，我们仍然应该能够将它们视为数字: 而==，<，>，+，-和*操作符都应该按小数进行工作。只是权宜之计，你不需要执行除法，因为任意的精确除法很快就会失控。(如何表示任意精度1/3?)

在 Rust 中，将这些操作用于自定义类型的方法是，实现自定义对象的相关 trait。特别是，您至少需要实现.PartialEq，PartialOrd，Add，Sub和Mul。 严格地说，由于十进制数构成一个总排序，你也应该实现Eq和Ord，尽管这些 trait 并没有被这些测试所检验.
*/
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Decimal {
    symbol: bool,
    left: usize,
    right: Vec<i8>,
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.symbol, other.symbol) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (true, true) => {
                if self.left > other.left {
                    Ordering::Greater
                } else if self.left < other.left {
                    Ordering::Less
                } else if self.right > other.right {
                    Ordering::Greater
                } else if self.right < other.right {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
            (false, false) => {
                if self.left < other.left {
                    Ordering::Greater
                } else if self.left > other.left {
                    Ordering::Less
                } else if self.right < other.right {
                    Ordering::Greater
                } else if self.right > other.right {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.left == other.left && self.right == other.right
    }
}

impl Eq for Decimal {}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let right = self.right.iter().map(|d| d.to_string()).collect::<String>();
        if self.symbol {
            write!(f, "{}.{}", self.left, right)
        } else {
            write!(f, "-{}.{}", self.left, right)
        }
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Decimal {
        let mut symbol = true;
        let mut left: usize = 0;
        let mut right: Vec<i8> = vec![0];
        let mut tmp= input;

        if input.starts_with("-") {
            tmp = input.trim_start_matches("-");
            symbol = false;
        }

        tmp = tmp.trim_start_matches('0');

        if !input.contains(".") {
            if !tmp.is_empty() {
                left = tmp
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .fold(0, |sum, d| sum * 10 + d as usize);
            }
        } else {
            let inner_tmp: Vec<&str> = tmp.split('.').collect();
            if !inner_tmp[0].is_empty() {
                left = inner_tmp[0]
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .fold(0, |sum, d| (sum * 10 + d as usize) as usize);
            }
            let inner_right = inner_tmp[1].trim_end_matches('0');
            if !inner_right.is_empty() {
                right = inner_right
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect();
            }
        }

        Decimal {
            symbol,
            left,
            right,
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut symbol = true;
        let mut left: usize;
        let mut right: Vec<i8> = vec![0];

        match (self.symbol, other.symbol) {
            (true, true) | (false, false) => {
                let mut s_right = self.right;
                let mut o_right = other.right;

                if s_right.len() > o_right.len() {
                    for _ in 0..(s_right.len() - o_right.len()) {
                        o_right.push(0);
                    }
                } else if s_right.len() < o_right.len() {
                    for _ in 0..(o_right.len() - s_right.len()) {
                        s_right.push(0);
                    }
                }

                if !self.symbol {
                    symbol = false;
                }

                let mut is_carry = false;
                let len = s_right.len();

                let mut tmp;
                for i in (0..len).rev() {
                    tmp = s_right[i] + o_right[i];
                    if is_carry {
                        tmp += 1;
                        is_carry = false;
                    }
                    if tmp > 9 {
                        tmp %= 10;
                        is_carry = true;
                    }
                    right.push(tmp);
                }
                right.reverse();
                while right.ends_with(&[0]) {
                    right.pop();
                }
                if right.is_empty() {
                    right.push(0);
                }

                left = self.left + other.left;
                if is_carry {
                    left += 1;
                }
            }
            (true, false) => {
                let tmp = self
                    - Decimal {
                        symbol: true,
                        left: other.left,
                        right: other.right,
                    };
                symbol = tmp.symbol;
                left = tmp.left;
                right = tmp.right;
            }
            (false, true) => {
                let tmp = other
                    - Decimal {
                        symbol: true,
                        left: self.left,
                        right: self.right,
                    };
                symbol = tmp.symbol;
                left = tmp.left;
                right = tmp.right;
            }
        }

        Decimal {
            symbol,
            left,
            right,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut symbol = true;
        let mut left: usize;
        let mut right: Vec<i8> = vec![0];

        let inner_self = self.clone();
        let inner_other = other.clone();

        match (self.symbol, other.symbol) {
            (true, true) => {
                let mut s_right = self.right.clone();
                let mut o_right = other.right.clone();

                if s_right.len() > o_right.len() {
                    for _ in 0..(s_right.len() - o_right.len()) {
                        o_right.push(0);
                    }
                } else if s_right.len() < o_right.len() {
                    for _ in 0..(o_right.len() - s_right.len()) {
                        s_right.push(0);
                    }
                }

                let mut big_left = self.left;
                let mut big_right = s_right.clone();

                let mut small_left = other.left;
                let mut small_right = o_right.clone();

                if inner_self < inner_other {
                    big_left = other.left;
                    big_right = o_right.clone();

                    small_left = self.left;
                    small_right = s_right.clone();

                    symbol = false;
                }

                let mut is_borrow = false;
                let len = s_right.len();

                let mut tmp ;
                for i in (0..len).rev() {
                    tmp = big_right[i] - small_right[i];
                    if is_borrow {
                        tmp -= 1;
                        is_borrow = false;
                    }
                    if tmp < 0 {
                        tmp += 10;
                        is_borrow = true;
                    }
                    right.push(tmp);
                }
                right.reverse();
                while right.ends_with(&[0]) {
                    right.pop();
                }
                if right.is_empty() {
                    right.push(0);
                }

                left = big_left + small_left;
                if is_borrow {
                    left -= 1;
                }
            }
            (false, false) => {
                let mut s_right = self.right;
                let mut o_right = other.right;

                if s_right.len() > o_right.len() {
                    for _ in 0..(s_right.len() - o_right.len()) {
                        o_right.push(0);
                    }
                } else if s_right.len() < o_right.len() {
                    for _ in 0..(o_right.len() - s_right.len()) {
                        s_right.push(0);
                    }
                }

                let mut big_left = other.left.clone();
                let mut big_right = o_right.clone();

                let mut small_left = self.left.clone();
                let mut small_right = s_right.clone();

                if inner_self < inner_other {
                    big_left = self.left;
                    big_right = s_right.clone();

                    small_left = other.left;
                    small_right = o_right.clone();

                    symbol = false;
                }

                let mut is_borrow = false;
                let len = s_right.len();

                let mut tmp ;
                for i in (0..len).rev() {
                    tmp = big_right[i] - small_right[i];
                    if is_borrow {
                        tmp += 1;
                        is_borrow = false;
                    }
                    if tmp < 0 {
                        tmp += 10;
                        is_borrow = true;
                    }
                    right.push(tmp);
                }
                right.reverse();
                while right.ends_with(&[0]) {
                    right.pop();
                }
                if right.is_empty() {
                    right.push(0);
                }

                left = big_left + small_left;
                if is_borrow {
                    left -= 1;
                }
            }
            (true, false) => {
                let tmp = self
                    + Decimal {
                        symbol: true,
                        left: other.left,
                        right: other.right,
                    };
                symbol = true;
                left = tmp.left;
                right = tmp.right;
            }
            (false, true) => {
                let tmp = other
                    + Decimal {
                        symbol: true,
                        left: self.left,
                        right: self.right,
                    };
                symbol = false;
                left = tmp.left;
                right = tmp.right;
            }
        }

        Decimal {
            symbol,
            left,
            right,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut symbol = true;
        let left;
        let mut right: Vec<i8> = vec![0];
        
        let inner_self = self.right.iter().fold(self.left, |a, d| a * 10 + *d as usize);
        let inner_other = other.right.iter().fold(other.left, |a, d| a * 10 + *d as usize);
        let mut tmp = inner_self * inner_other;

        for _ in 0..(self.right.len() + other.right.len()) {
            right.push((tmp % 10) as i8);
            tmp /= 10;
        }
        left = tmp;
        right.reverse();
        while right.ends_with(&[0]) {
            right.pop();
        }

        match (self.symbol, other.symbol) {
            (true, false) | (false, true) => {
                symbol = false;
            },
            _ => {},
        }

        Decimal {
            symbol,
            left,
            right,
        }
    }
}

// 59 字谜
/*
给出一个单词和可能的字谜列表，选择正确的子列表.

给出"listen"和候选人名单一样"enlists" "google" "inlets" "banana"程序应该返回一个包含"inlets"的列表。
*/

pub fn anagrams_for<'a>(word: &str, input: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    input
        .iter()
        .filter(|s| {
            let s = (**s).to_lowercase();
            s != word &&
                s.chars().all(|c| word.contains(c))
        })
        .cloned()
        .collect()
}

// 60 蛋白质转译
/*
将 RNA 序列转译成蛋白质.

RNA 可以分解为三个称为密码子的核苷酸序列，然后转译成多肽，如下:

RNA:"AUGUUUUCU"=>转译成

密码子:"AUG"， "UUU"， "UCU"=>其成为具有以下序列的多肽=>

蛋白:"Methionine"， "Phenylalanine"， "Serine"

这有 64 个密码子，而这些密码子又相当于 20 个氨基酸；然而，在本练习中，所有密码子序列和所得氨基酸都不重要。如果它适用于一个密码子，该程序应该适用于所有这些密码子。但是，您可以随意扩展测试套件中的列表以包含它们.

还有三个终止密码子(也称为’STOP’密码子)；如果遇到任何这些密码子(通过核糖体)，那么所有转译结束，并终止蛋白质。

之后的所有后续密码子都会被忽略，如下所示:

RNA:"AUGUUUUCUUAAAUG"=>

密码:"AUG"， "UUU"， "UCU"， "UAA"， "AUG"=>

蛋白:"Methionine"， "Phenylalanine"， "Serine"

注意终止密码子"UAA"终止转译，最终的蛋氨酸，不会转译成蛋白质序列。

以下是本练习所需的密码子和产生的氨基酸。

密码子	蛋白
AUG	蛋氨酸
UUU，UUC	苯丙氨酸
UUA，UUG	亮氨酸
UCU，UCC，UCA，UCG	丝氨酸
UAU，UAC	酪氨酸
UGU，UGC	半胱氨酸
UGG	色氨酸
UAA，UAG，UGA	STOP
*/

pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).map(|pro| *pro)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|s| self.name_for(&s.iter().collect::<String>()))
            .take_while(|s| s.is_none() || s.unwrap() != "stop codon")
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codons: pairs.into_iter().collect()
    }
}

// 62 书店买书
/*
为了尝试鼓励，畅销 5 书系列书籍的销售，书店决定提供多书购买的折扣。

这五本书中的任何一本都要花 8 美元.

但是，如果您购买两本不同的书籍，那么这两本书将获得 5%的折扣。

如果您购买 3 本不同的书籍，您将获得 10%的折扣。

如果您购买 4 本不同的书籍，您将获得 20%的折扣。

如果您全部购买 5 件，即可获得 25%的折扣。

注意：如果您购买了 4 本书，但不同的书只有 3 本，那么您可以在 3 本书中获得 10%的折扣，但是第 4 本书的价格仍为 8 美元。

你的任务是写一段代码，来计算购物车(这 5 本书的任意搭配)的价格(只包含同一系列的书籍)，给予尽可能大的折扣。

例如，购物车若是下面的书，那价格是多少?

第一本书的 2 份
第二本书的 2 份
第三本书的 2 份
第四本书的 1 份
第五本书的 1 份
将这 8 本书分组的一种方法是:

第 1 组 5 本书 -> 25%折扣(第 1，第 2，第 3，第 4，第 5)
还有 1 组 3 本书 -> 10%折扣(第 1 名，第 2 名，第 3 名)
这将总共给出:

5 本书，25%的折扣
还有 3 本书可享受 10%的折扣
导致:

5 x(8 - 2.00)== 5 x 6.00 == $ 30.00
还有 3 x(8 - 0.80)== 3 x 7.20 == 21.60 美元
总计 51.60 美元

但是，将这 8 本书分组的另一种方法是:

第 1 组 4 本书 -> 20%折扣(第 1，第 2，第 3，第 4)
还有 1 组 4 本书 -> 20%折扣(第 1，第 2，第 3，第 5)
这将总共给出:

4 本书，20%的折扣
还有 4 本书，20%的折扣
导致:

4 x(8 - 1.60)== 4 x 6.40 == 25.60 美元
还有 4 x(8 - 1.60)== 4 x 6.40 == 25.60 美元
总计 51.20 美元

51.20 美元是最大折扣的价格.
*/

pub fn lowest_price(books: &[u32]) -> u32 {
    
    let mut book_price_orders: HashMap<usize, Vec<Vec<Vec<u32>>>> = HashMap::new(); // 哈希表，用以存储价格及对应分组表
    // 整理书籍类别及数量
    let mut book_num_hashmap: HashMap<u32, u32> = HashMap::new();
    for book in books {
        let num = book_num_hashmap.entry(*book).or_insert(0);
        *num += 1;
    }
    let mut book_num_list: Vec<(u32, u32)> = book_num_hashmap
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect();
    book_num_list.sort_by(|a, b| b.1.cmp(&a.1));
    println! {"1. {:?}", book_num_list};
    // 按照书籍数量以大小分组
    let mut book_will_buy_list: Vec<Vec<u32>> = vec![];
    let (book, num) = book_num_list[0];
    for _ in 0..num {
        book_will_buy_list.push(vec![book]);
    }
    for (book, num) in book_num_list[1..].to_vec() {
        for i in 0..(num as usize) {
            book_will_buy_list[i].push(book);
        }
    }
    println!("2. {:?}\n", book_will_buy_list);
    // 如果所买书籍数量少于 5
    if book_will_buy_list.len() < 2 {
        let score: usize = match book_will_buy_list[0].len() {
            2 => 152,
            3 => 216,
            4 => 256,
            5 => 300,
            _ => 80,
        };
        println!("{}, {:?}", score, book_will_buy_list);
        book_price_orders.insert(score, vec![book_will_buy_list]);
        return score as u32;
    }
    // 建立原始分组变量用以对比，结束遍历
    let mut compare = book_will_buy_list.clone();
    compare.reverse();
    let mut compare_nums: Vec<usize> = vec![];
    for list in compare.clone() {
        compare_nums.push(list.len());
    }
    let mut rev_compare_nums = compare_nums.clone();
    rev_compare_nums.reverse();
    if compare_nums == rev_compare_nums {
        let score: usize = compare_nums
            .iter()
            .map(|n| match n {
                2 => 152,
                3 => 216,
                4 => 256,
                5 => 300,
                _ => 80,
            })
            .sum();
        println!("{}, {:?}", score, book_will_buy_list);
        book_price_orders.insert(score, vec![book_will_buy_list]);
        return score as u32;
    }
    let mut start: usize = 0; // 索引，移出最后一本书籍转移到别的组里
    let mut end: usize = book_will_buy_list.clone().len(); // 索引，每次遍历的最后一组
    let mut book_will_buy_numbers_list: Vec<Vec<usize>> = vec![]; // 所买书籍的数量列表集，用以排除已采用的分组

    let mut old_book_will_buy_list = book_will_buy_list.clone(); // 每次遍历完的最终分组表
    let mut count = 0;
    let mut is_pop; // 根据上组是否包含所转移书籍决定是否删除上组最后一本书
    let mut now_nums = vec![0];
    while compare_nums != now_nums {
        println!("新的遍历");

        // 比较每次遍历的最终表与初始表所移出书籍组，若相同，则移出书籍组索引加一
        if old_book_will_buy_list[start] == compare[start] {
            start += 1;
            println!("start + 1");
            continue;
        }

        // 第一层循环，遍历
        for i in (start + 1)..end {
            let mut new_book_will_buy_list = old_book_will_buy_list.clone();
            println!("\n第一层循环 inner_i: {}, {:?}", i, new_book_will_buy_list);
            let changing_book = old_book_will_buy_list[i - 1].last().unwrap();
            if old_book_will_buy_list[i..]
                .iter()
                .any(|list| !list.contains(changing_book))
            {
                is_pop = true;
            } else {
                continue;
            }
            println!("book: {}\n", changing_book);
            // 第二层循环，遍历
            for j in i..end {
                // 若上组不包含待移出书籍，则移除上组书籍
                if is_pop {
                    new_book_will_buy_list[j - 1].pop();
                    println! {"pop"}
                }
                // 判断本组是否包含待移出书籍，如包含，则开始下一次循环
                println!("第二层循环 inner_j: {}", j);
                if old_book_will_buy_list[j].contains(changing_book) {
                    is_pop = false;
                    continue;
                }
                if j == end - 2 && old_book_will_buy_list[end - 1].contains(changing_book) {
                    is_pop = false;
                } else {
                    is_pop = true;
                }

                // 移入待移出书籍
                new_book_will_buy_list[j].push(*changing_book);

                println!("{:?}", new_book_will_buy_list);
                // 统计本组书籍数量
                let mut book_will_buy_numbers: Vec<usize> = vec![];
                for list in new_book_will_buy_list.clone() {
                    book_will_buy_numbers.push(list.clone().len());
                }
                now_nums = book_will_buy_numbers.clone();
                book_will_buy_numbers.sort();

                // 如分组数量表已存在，则开始下一次循环
                if book_will_buy_numbers_list.contains(&book_will_buy_numbers) {
                    continue;
                }
                // 将本组分组数量表加入已存在数量表集，用以比较
                book_will_buy_numbers_list.push(book_will_buy_numbers.clone());

                // 计算分组价格
                let score: usize = book_will_buy_numbers
                    .iter()
                    .map(|n| match n {
                        2 => 152,
                        3 => 216,
                        4 => 256,
                        5 => 300,
                        _ => 80,
                    })
                    .sum();
                // 将价格及对应分组存入哈希表
                let mut sort_new_book_will_buy_list: Vec<Vec<u32>> = vec![];
                for list in new_book_will_buy_list.clone() {
                    let mut temp = list.clone();
                    temp.sort();
                    sort_new_book_will_buy_list.push(temp);
                }
                sort_new_book_will_buy_list.sort_by(|a, b| b.len().cmp(&a.len()));
                if let Some(order) = book_price_orders.get_mut(&score) {
                    order.push(sort_new_book_will_buy_list);
                } else {
                    book_price_orders.insert(score, vec![sort_new_book_will_buy_list]);
                }
            }
            old_book_will_buy_list = new_book_will_buy_list.clone();
            let mut end_compare = new_book_will_buy_list[end - 1].clone();
            end_compare.sort();
            let mut compare_end = compare[end - 1].clone();
            compare_end.sort();
            if end_compare == compare_end {
                println!("{:?}, {:?}", end_compare, compare_end);
                println!("end - 1");
                end -= 1;
            }
        }
        println!("{}. {:?}", count, old_book_will_buy_list);
        count += 1;
        if count == 7 {
            break;
        }
    }
    let mut price_lists = vec![];
    for (price, order) in book_price_orders {
        price_lists.push(price);
        println!("{}, {:?}", price, order);
    }
    *price_lists.iter().min().unwrap() as u32
}