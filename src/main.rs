fn verse(n: i32) -> String {
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

fn main() {
    let string = "robot";
    let result = string.chars().rev().collect::<Vec<char>>();
    println!("{:?}", &result);
    println!("Hello, world!");
    for i in 2..2 {
        println!("{}", i);
    }
    println!("{:?}", verse(4));
    println!("{:?}", (2..5)
                        .rev()
                        .map(|n| verse(n))
                        .collect::<Vec<_>>()
                        .join("\n"));
    let mut out: Vec<String> = vec![];
    let digits = "92017";
    let len = 2;
    for i in 0..(digits.len()-len+1) {
        out.push(digits.get(i..(i+len)).unwrap().to_string());
    }
    println!("{:?}", out);

    let num = 5_u32;
    let out: Vec<u32> = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let len = out.len() as u32;
    println!("{:?}, {}", out, len);
}
