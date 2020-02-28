extern crate exercism_rust;
use exercism_rust::hard::*;

// 1
/*
#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn input_with_lines_not_multiple_of_four_is_error() {
   let input = " _ \n".to_string() +
               "| |\n" +
               "   ";

   assert_eq!(Err(Error::InvalidRowCount(3)), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn input_with_columns_not_multiple_of_three_is_error() {
   let input = "    \n".to_string() +
               "   |\n" +
               "   |\n" +
               "    ";

   assert_eq!(Err(Error::InvalidColumnCount(4)), convert(&input));
}
*/
#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn unrecognized_characters_return_question_mark() {
   let input = "   \n".to_string() +
               "  _\n" +
               "  |\n" +
               "   ";

   assert_eq!(Ok("?".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_0() {
   let input = " _ \n".to_string() +
               "| |\n" +
               "|_|\n" +
               "   ";

   assert_eq!(Ok("0".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_1() {
   let input = "   \n".to_string() +
               "  |\n" +
               "  |\n" +
               "   ";

   assert_eq!(Ok("1".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_2() {
   let input = " _ \n".to_string() +
               " _|\n" +
               "|_ \n" +
               "   ";

   assert_eq!(Ok("2".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_3() {
   let input = " _ \n".to_string() +
               " _|\n" +
               " _|\n" +
               "   ";

   assert_eq!(Ok("3".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_4() {
   let input = "   \n".to_string() +
               "|_|\n" +
               "  |\n" +
               "   ";

   assert_eq!(Ok("4".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_5() {
   let input = " _ \n".to_string() +
               "|_ \n" +
               " _|\n" +
               "   ";

   assert_eq!(Ok("5".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_6() {
   let input = " _ \n".to_string() +
               "|_ \n" +
               "|_|\n" +
               "   ";

   assert_eq!(Ok("6".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_7() {
   let input = " _ \n".to_string() +
               "  |\n" +
               "  |\n" +
               "   ";

   assert_eq!(Ok("7".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_8() {
   let input = " _ \n".to_string() +
               "|_|\n" +
               "|_|\n" +
               "   ";

   assert_eq!(Ok("8".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_9() {
   let input = " _ \n".to_string() +
               "|_|\n" +
               " _|\n" +
               "   ";

   assert_eq!(Ok("9".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_110101100() {
   let input = "       _     _        _  _ \n".to_string() +
               "  |  || |  || |  |  || || |\n" +
               "  |  ||_|  ||_|  |  ||_||_|\n" +
               "                           ";

   assert_eq!(Ok("110101100".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn replaces_only_garbled_numbers_with_question_mark() {
   let input = "       _     _           _ \n".to_string() +
               "  |  || |  || |     || || |\n" +
               "  |  | _|  ||_|  |  ||_||_|\n" +
               "                           ";

   assert_eq!(Ok("11?10?1?0".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_string_of_decimal_numbers() {
   let input = "    _  _     _  _  _  _  _  _ \n".to_string() +
               "  | _| _||_||_ |_   ||_||_|| |\n" +
               "  ||_  _|  | _||_|  ||_| _||_|\n" +
               "                              ";

   assert_eq!(Ok("1234567890".to_string()), convert(&input));
}

#[test]
//#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn numbers_across_multiple_lines_are_joined_by_commas() {
   let input = "    _  _ \n".to_string() +
               "  | _| _|\n" +
               "  ||_  _|\n" +
               "         \n" +
               "    _  _ \n" +
               "|_||_ |_ \n" +
               "  | _||_|\n" +
               "         \n" +
               " _  _  _ \n" +
               "  ||_||_|\n" +
               "  ||_| _|\n" +
               "         ";
   assert_eq!(Ok("123,456,789".to_string()), convert(&input));
}

// 2
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
 }
 
 fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
 }
 
 fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
 }
 
 #[test]
 fn no_rows() {
    run_test(&[]);
 }
 
 #[test]
 //#[ignore]
 fn no_columns() {
    run_test(&[""]);
 }
 
 #[test]
 //#[ignore]
 fn no_mines() {
    run_test(&["   ", "   ", "   "]);
 }
 
 #[test]
 //#[ignore]
 fn board_with_only_mines() {
    run_test(&["***", "***", "***"]);
 }
 
 #[test]
 //#[ignore]
 fn mine_surrounded_by_spaces() {
    run_test(&["111", "1*1", "111"]);
 }
 
 #[test]
 //#[ignore]
 fn space_surrounded_by_mines() {
    run_test(&["***", "*8*", "***"]);
 }
 
 #[test]
 //#[ignore]
 fn horizontal_line() {
    run_test(&["1*2*1"]);
 }
 
 #[test]
 //#[ignore]
 fn horizontal_line_mines_at_edges() {
    run_test(&["*1 1*"]);
 }
 
 #[test]
 //#[ignore]
 fn vertical_line() {
    run_test(&["1", "*", "2", "*", "1"]);
 }
 
 #[test]
 //#[ignore]
 fn vertical_line_mines_at_edges() {
    run_test(&["*", "1", " ", "1", "*"]);
 }
 
 #[test]
 //#[ignore]
 fn cross() {
    run_test(&[" 2*2 ", "25*52", "*****", "25*52", " 2*2 "]);
 }
 
 #[test]
 //#[ignore]
 fn large_board() {
    run_test(&["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"]);
 }