// 1 OCR
/*
给定一个 3 x 4 ，由竖线符号，下划线和空格组成的网格，确定代表哪个数字，或者是否是乱码。

第一步
首先，将简单的二进制字体，转换为包含 0 或 1 的字符串。

二进制字体使用竖线符号和下划线，四行高，三列宽.


     _   #
    | |  # zero.
    |_|  #
         # 第 4 行总空着
转换为”0”


         #
      |  # one.
      |  #
         # (4行空)
转换为”1”

如果输入的大小正确，但无法识别，则程序应返回”?”

如果输入的大小不正确，程序应该返回错误。

第二步
更新您的程序以识别多二进制字符串，用 ? 替换乱码。

第三步
更新程序，以识别所有数字 0 到 9，既可以单独识别，也可以作为更大字符串的一部分识别。


 _
 _|
|_
转换为”2”


      _  _     _  _  _  _  _  _  #
    | _| _||_||_ |_   ||_||_|| | # 十进制数.
    ||_  _|  | _||_|  ||_| _||_| #
                                 # 第 4 行空着
被转换为”1234567890”

第四步
更新程序以处理多个数字，每 4 行。转换多行时，请使用逗号连接行.


    _  _
  | _| _|
  ||_  _|

    _  _
|_||_ |_
  | _||_|

 _  _  _
  ||_||_|
  ||_| _|
被转换为”123，456，789”
*/
#[derive(Debug, PartialEq)]
pub enum Error {
   InvalidRowCount(usize),
   InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let input = input;
    let lines_lists = input
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|c_list| c_list.to_vec())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>()
        .chunks(4)
        .map(|line_list| line_list.to_vec())
        .collect::<Vec<Vec<Vec<Vec<char>>>>>();
    println!("{:?}", lines_lists);
    fn convert_character(input: &str) -> char {
        if input == " _ | ||_|   " {
            '0'
        } else if input == "     |  |   " {
            '1'
        } else if input == " _  _||_    " {
            '2'
        } else if input == " _  _| _|   " {
            '3'
        } else if input == "   |_|  |   " {
            '4'
        } else if input == " _ |_  _|   " {
            '5'
        } else if input == " _ |_ |_|   " {
            '6'
        } else if input == " _   |  |   " {
            '7'
        } else if input == " _ |_||_|   " {
            '8'
        } else if input == " _ |_| _|   " {
            '9'
        } else {
            '?'
        }
    }
    let mut res = "".to_string();
    let len = lines_lists.clone().get(0).unwrap().get(0).unwrap().len();
    for lines_list in lines_lists {
        let mut symbol_lists: Vec<Vec<Vec<char>>> = vec![vec![vec![' '; 3]; 4]; len];
        for (row, line_list) in lines_list.iter().enumerate() {
            for (column, c_list) in line_list.iter().enumerate() {
                symbol_lists[column][row] = c_list.to_vec()
            }
        }
        println!("{:?}\n", symbol_lists);
        for symbol_list in symbol_lists.iter() {
            let symbol = symbol_list.into_iter().flatten().collect::<String>();
            res.push(convert_character(&symbol));
            println!("{:?}", symbol);
        }
        res.push(',');
        println!("{:?}", res);
    }
    Ok(res)
}

// 2 扫雷
/*
将数字添加到扫雷板上.

扫雷器是一个流行的游戏，其中用户要用数字提示，来找到地雷，这些数字提示指示有多少地雷直接相邻(水平，垂直，对角，总 9 个格)，围成一个正方形.

在这个练习中，您必须创建一些代码，这些代码计算与正方形相邻的地雷数量，并且像这样转换地雷板(其中*表示地雷):


    +-----+
    | * * |
    |  *  |
    |  *  |
    |     |
    +-----+
变成:


    +-----+
    |1*3*1|
    |13*31|
    | 2*2 |
    | 111 |
    +-----+
*/
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let input = minefield;
    let chars_lists = input
        .iter()
        .map(|chars_list| chars_list.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("{:?}", chars_lists);
    let mut res = chars_lists.clone();
    let row_len = chars_lists.clone().len();
    let column_len = chars_lists[0].clone().len();

    let insert_chars = vec![' '; column_len + 2];
    let mut compare = chars_lists.clone();
    for row in 0..row_len {
        compare[row].insert(0, ' ');
        compare[row].push(' ');
    }
    compare.insert(0, insert_chars.clone());
    compare.push(insert_chars);
    fn to_char_increase(c: char) -> char {
        match c {
            ' ' => '1',
            _ => ((c as u8) + 1) as char,
        }
    }
    for row in 1..(row_len + 1) {
        for column in 1..(column_len + 1) {
            if compare[row][column] == '*' {
                continue;
            } else {
                for i in (row - 1)..(row + 2) {
                    for j in (column - 1)..(column + 2) {
                        if compare[i][j] == '*' {
                            let tmp = res[row - 1][column - 1].clone();
                            res[row - 1][column - 1] = to_char_increase(tmp);
                        }
                    }
                }
            }
        }
    }
    let res = res
        .iter()
        .map(|chars_list| chars_list.iter().collect::<String>())
        .collect::<Vec<String>>();
    for chars_list in res.clone() {
        println!("{:?}", chars_list);
    }
    res
 }

// 3 多米诺骨牌
/*
制作一个多米诺骨牌.

计算给出的多米诺骨牌的排序方法，使它们形成一个正确的多米诺骨牌链(石头的一半上的数值，与相邻石头的一半上的数值相匹配)，并且(第一块石头和最后一块石头)石头的一半上的数值没有邻居，且它们的数值匹配.

比如，给出石头[2|1]，[2|3]和[1|3]你应该计算一些类似[1|2] [2|3] [3|1]或[3|2] [2|1] [1|3]或[1|3] [3|2] [2|1]的东西等，其中第一个和最后一个数字是相同的。

对于以下石头[1|2]，[4|1]和[2|3]，它的结果骨牌链无效：[4|1] [1|2] [2|3]第一个和最后一个数字不一样。4!= 3

一些测试用例可以在一个骨牌链解决方案中，使用重复的石头，假设使用了多个骨牌集合
*/
use std::collections::HashMap;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut map: HashMap<u8, HashMap<u8, u8>> = HashMap::new();
    let mut edges_len = 0;
    for eg in input.iter() {
        let (left, right) = *eg;

        let left_edge = map.entry(left).or_insert(HashMap::new());
        let left_end_point = left_edge.entry(right).or_insert(0);
        *left_end_point += 1;

        if left != right {
            edges_len += 1;
            let right_edge = map.entry(right).or_insert(HashMap::new());
            let right_end_point = right_edge.entry(left).or_insert(0);
            *right_end_point += 1;
        }
    }
    let mut current_point = input[0].0;
    let mut res: Vec<(u8, u8)> = vec![];
    while edges_len > 0 {
        let mut new_map = map.clone();
        let next_points = new_map.get_mut(&current_point).unwrap();
        let mut list_points: Vec<(u8, usize)> = vec![];
        for key in next_points.keys() {
            list_points.push((*key, map.get(key).unwrap().len() - 1));
        }
        if list_points.is_empty() {
            return None;
        }
        list_points.sort_by(|a, b| b.1.cmp(&a.1));
        if !next_points.is_empty() {
            let tmp_point;
            let tmp_list_points = list_points.clone();
            if !tmp_list_points.iter().all(|point| point.0 != current_point) {
                let inner_list_points = list_points.clone();
                for (id, point) in inner_list_points.iter().enumerate() {
                    if point.0 == current_point {
                        for _ in 0..*(next_points.get(&current_point).unwrap()) {
                            res.push((current_point, current_point));
                        }
                        list_points.remove(id);
                        next_points.remove(&current_point);
                        break;
                    }
                }
            }
            tmp_point = list_points[0].0;
            let edge_count = next_points.get_mut(&tmp_point).unwrap();
            *edge_count -= 1;
            if *edge_count == 0 {
                next_points.remove(&tmp_point);
            }
            let other_edge_count = new_map
                .get_mut(&tmp_point)
                .unwrap()
                .get_mut(&current_point)
                .unwrap();
            *other_edge_count -= 1;
            if *other_edge_count == 0 {
                new_map.get_mut(&tmp_point).unwrap().remove(&current_point);
            }
            edges_len -= 1;
            res.push((current_point, tmp_point));
            current_point = tmp_point;
            map = new_map;
        } else {
            break;
        }
    }
    Some(res)
}

// 4 并发字母频率
/*
使用并行计算，文本中的字母频率。

并行性是并行的，也可以按顺序进行。一个常见的例子是计算字母的频率。创建一个函数，返回文本列表中每个字母的总频率，并使用并行性。
*/
use std::sync::mpsc::channel;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut groups: Vec<Vec<String>> = vec![Vec::new(); worker_count];
    for (id, tmp_str) in input.iter().enumerate() {
        let id = id % worker_count;
        groups[id].push(tmp_str.to_string());
    }
    fn count(group: Vec<String>) -> HashMap<char, usize> {
        let mut res: HashMap<char, usize> = HashMap::new();
        for tmp_str in group {
            for c in tmp_str.chars() {
                *res.entry(c).or_insert(0) += 1;
            }
        }
        res
    }
    let (tx, rx) = channel();
    for group in groups {
        let tx = tx.clone();
        thread::spawn(move || tx.send(count(group)).unwrap());
    }

    let mut res: HashMap<char, usize> = HashMap::new();
    for _ in 0..worker_count {
        let tmp_res = rx.recv().unwrap();
        for (c, n) in tmp_res {
            *res.entry(c).or_insert(0) += n;
        }
    }
    res
}

// 5 矩形
/*
计算 ASCII 图中的矩形的数目，如下所示.


   +--+
  ++  |
+-++--+
|  |  |
+--+--+
上面的图表包含 6 个矩形:


+-----+
|     |
+-----+

   +--+
   |  |
   |  |
   |  |
   +--+

   +--+
   |  |
   +--+

   +--+
   |  |
   +--+

+--+
|  |
+--+

  ++
  ++
你可以假设输入总是一个适当的矩形(即每行的长度，等于第一行的长度)。
*/
pub fn count(lines: &[&str]) -> u32 {
   let line_lists = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut res: u32 = 0;
    let row_len = line_lists.len();
    let column_len = line_lists[0].len();
    for row in 0..(row_len - 1) {
        for column in 0..(column_len - 1) {
            if line_lists[row][column] == '+' {
                println!("the first point: [{}, {}]\n", row, column);
                for j in (column + 1)..column_len {
                    if line_lists[row][j] == '+' {
                        println!("the second point: [{}, {}]\n", row, j);
                        for i in (row + 1)..row_len {
                            if line_lists[i][j] == '+' && line_lists[i][column] == '+' {
                                println!("the third point: [{}, {}]", i, j);
                                println!("the fourth point: [{}, {}]\n", i, column);
                                res += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    res
}