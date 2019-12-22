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
        }
    };
    let bit = |x| digit_to_string(x % 10);
    let ten_bit = |n| {
        match n / 10 {
            0 => bit(n),
            1 => match n % 10 {
                0 => "ten",
                1 => "eleven",
                2 => "twelve",
                3 => "thirteen",
                4 => "fourteen",
                5 => "fifteen",
                6 => "sixteen",
                7 => "seventeen",
                8 => "eighteen",
                9 => "nineteen",
            },
            2 => match n % 10 {
                0 => "twenty",
                _ => &format!("twenty-{}", bit(n)),
            },
            3 => match n % 10 {
                0 => "thirty",
                _ => &format!("thirty-{}", bit(n)),
            },
            4 => match n % 10 {
                0 => "fourty",
                _ => &format!("fourty-{}", bit(n)),
            },
            5 => match n % 10 {
                0 => "fifty",
                _ => &format!("fifty-{}", bit(n)),
            },
            6 => match n % 10 {
                0 => "sixty",
                _ => &format!("sixty-{}", bit(n)),
            },
            7 => match n % 10 {
                0 => "seventy",
                _ => &format!("seventy-{}", bit(n)),
            },
            8 => match n % 10 {
                0 => "eighty",
                _ => &format!("eighty-{}", bit(n)),
            },
            9 => match n % 10 {
                0 => "ninety",
                _ => &format!("ninety-{}", bit(n)),
            },
        }
    };
    let hundred_bit = |x| {
        match x {
            0 => "",
            _ => &format!("{} hundred", digit_to_string(x)),
        }
    };
    let n_list: Vec<u64> = vec![];
    while n / 1000 != 0 {
        n_list.push(n % 1000);
        n = n / 1000;
    };
    let n_string_list: Vec<&str> = vec![];
    for x in n_list.into_iter() {
        n_string_list.push(&format!("{} {}", hundred_bit(x / 100), ten_bit(x % 100)));
    };
    let unit_list = vec!["", "thousand", "million", "billion"];
    let n_string_unit_list: Vec<&str> = vec![];
    for i in 0..n_string_list.len() {
        n_string_unit_list.push(&format!("{} {}", n_string_list[i], unit_list[i]));
    };
    let out = n_string_unit_list.join(" ");
    format!("{}", n_string_unit_list.join(" "))
}
