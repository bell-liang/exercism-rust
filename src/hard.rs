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