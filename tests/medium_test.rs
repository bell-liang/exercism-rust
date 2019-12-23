extern crate exercism_rust;
use exercism_rust::medium::*;

// 1
fn find_sorted_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
   let mut result = find_saddle_points(input);
   println!("{:?}", result);
   result.sort();
   result
}

#[test]
fn identify_single_saddle_point() {
   let input = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
   assert_eq!(vec![(1, 0)], find_saddle_points(&input));
}

#[test]
//#[ignore]
fn identify_empty_matrix() {
   let input = vec![vec![], vec![], vec![]];
   let expected: Vec<(usize, usize)> = Vec::new();
   assert_eq!(expected, find_saddle_points(&input));
}

#[test]
//#[ignore]
fn identify_lack_of_saddle_point() {
   let input = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
   let expected: Vec<(usize, usize)> = Vec::new();
   assert_eq!(expected, find_saddle_points(&input));
}

#[test]
//#[ignore]
fn multiple_saddle_points_in_col() {
   let input = vec![vec![4, 5, 4], vec![3, 5, 5], vec![1, 5, 4]];
   assert_eq!(
      vec![(0, 1), (1, 1), (2, 1)],
      find_sorted_saddle_points(&input)
   );
}

#[test]
//#[ignore]
fn multiple_saddle_points_in_row() {
   let input = vec![vec![6, 7, 8], vec![5, 5, 5], vec![7, 5, 6]];
   assert_eq!(
      vec![(1, 0), (1, 1), (1, 2)],
      find_sorted_saddle_points(&input)
   );
}

#[test]
//#[ignore]
fn identify_bottom_right_saddle_point() {
   let input = vec![vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
   assert_eq!(vec![(2, 2)], find_saddle_points(&input));
}

// track specific as of v1.3
#[test]
//#[ignore]
fn non_square_matrix_high() {
   let input = vec![vec![1, 5], vec![3, 6], vec![2, 7], vec![3, 8]];
   assert_eq!(vec![(0, 1)], find_saddle_points(&input));
}

#[test]
//#[ignore]
fn non_square_matrix_wide() {
   let input = vec![vec![3, 1, 3], vec![3, 2, 4]];
   assert_eq!(vec![(0, 0), (0, 2)], find_sorted_saddle_points(&input));
}

#[test]
//#[ignore]
fn single_column_matrix() {
   let input = vec![vec![2], vec![1], vec![4], vec![1]];
   assert_eq!(vec![(1, 0), (3, 0)], find_saddle_points(&input));
}

#[test]
//#[ignore]
fn single_row_matrix() {
   let input = vec![vec![2, 5, 3, 5]];
   assert_eq!(vec![(0, 1), (0, 3)], find_saddle_points(&input));
}

// 2
#[test]
fn empty_string() {
   assert_eq!(check(""), true, "An empty string should be an isogram.")
}

#[test]
//#[ignore]
fn only_lower_case_characters() {
   assert_eq!(check("isogram"), true, "\"isogram\" should be an isogram.")
}

#[test]
//#[ignore]
fn one_duplicated_character() {
   assert_eq!(
       check("eleven"),
       false,
       "\"eleven\" has more than one \'e\', therefore it is no isogram."
   )
}

#[test]
//#[ignore]
fn longest_reported_english_isogram() {
   assert_eq!(
       check("subdermatoglyphic"),
       true,
       "\"subdermatoglyphic\" should be an isogram."
   )
}

#[test]
//#[ignore]
fn one_duplicated_character_mixed_case() {
   assert_eq!(
       check("Alphabet"),
       false,
       "\"Alphabet\" has more than one \'a\', therefore it is no isogram."
   )
}

#[test]
//#[ignore]
fn hypothetical_isogramic_word_with_hyphen() {
   assert_eq!(
       check("thumbscrew-japingly"),
       true,
       "\"thumbscrew-japingly\" should be an isogram."
   )
}

#[test]
//#[ignore]
fn isogram_with_duplicated_hyphen() {
   assert_eq!(
       check("six-year-old"),
       true,
       "\"six-year-old\" should be an isogram."
   )
}

#[test]
//#[ignore]
fn made_up_name_that_is_an_isogram() {
   assert_eq!(
       check("Emily Jung Schwartzkopf"),
       true,
       "\"Emily Jung Schwartzkopf\" should be an isogram."
   )
}

#[test]
//#[ignore]
fn duplicated_character_in_the_middle() {
   assert_eq!(
       check("accentor"),
       false,
       "\"accentor\" has more than one \'c\', therefore it is no isogram."
   )
}

// 3
// Note: No tests created using 'and' with numbers.
// Apparently Most American English does not use the 'and' with numbers,
// where it is common in British English to use the 'and'.

#[test]
fn test_zero() {
   assert_eq!(encode(0), String::from("zero"));
}

//
// If the below test is uncommented, it should not compile.
//
/*
//#[ignore]
fn test_negative() {
   assert_eq!(encode(-1), String::from("won't compile"));
}
*/

#[test]
//#[ignore]
fn test_one() {
   assert_eq!(encode(1), String::from("one"));
}

#[test]
//#[ignore]
fn test_fourteen() {
   assert_eq!(encode(14), String::from("fourteen"));
}

#[test]
//#[ignore]
fn test_twenty() {
   assert_eq!(encode(20), String::from("twenty"));
}

#[test]
//#[ignore]
fn test_twenty_two() {
   assert_eq!(encode(22), String::from("twenty-two"));
}

#[test]
//#[ignore]
fn test_one_hundred() {
   assert_eq!(encode(100), String::from("one hundred"));
}

// note, using American style with no and
#[test]
//#[ignore]
fn test_one_hundred_twenty() {
   assert_eq!(encode(120), String::from("one hundred twenty"));
}

#[test]
//#[ignore]
fn test_one_hundred_twenty_three() {
   assert_eq!(encode(123), String::from("one hundred twenty-three"));
}

#[test]
//#[ignore]
fn test_one_thousand() {
   assert_eq!(encode(1000), String::from("one thousand"));
}

#[test]
//#[ignore]
fn test_one_thousand_two_hundred_thirty_four() {
   assert_eq!(
       encode(1234),
       String::from("one thousand two hundred thirty-four")
   );
}

// note, using American style with no and
#[test]
//#[ignore]
fn test_eight_hundred_and_ten_thousand() {
   assert_eq!(encode(810_000), String::from("eight hundred ten thousand"));
}

#[test]
//#[ignore]
fn test_one_million() {
   assert_eq!(encode(1_000_000), String::from("one million"));
}

// note, using American style with no and
#[test]
//#[ignore]
fn test_one_million_two() {
   assert_eq!(encode(1_000_002), String::from("one million two"));
}

#[test]
//#[ignore]
fn test_1002345() {
   assert_eq!(
       encode(1_002_345),
       String::from("one million two thousand three hundred forty-five")
   );
}

#[test]
//#[ignore]
fn test_one_billion() {
   assert_eq!(encode(1_000_000_000), String::from("one billion"));
}

#[test]
//#[ignore]
fn test_987654321123() {
   assert_eq!(
       encode(987_654_321_123),
       String::from(
           "nine hundred eighty-seven billion \
            six hundred fifty-four million \
            three hundred twenty-one thousand \
            one hundred twenty-three"
       )
   );
}

/*
 These tests are only if you implemented full parsing for u64 type.
*/
#[test]
//#[ignore]
fn test_max_i64() {
   assert_eq!(
       encode(9_223_372_036_854_775_807),
       String::from(
           "nine quintillion two hundred twenty-three \
            quadrillion three hundred seventy-two trillion \
            thirty-six billion eight hundred fifty-four million \
            seven hundred seventy-five thousand eight hundred seven"
       )
   );
}

#[test]
//#[ignore]
fn test_max_u64() {
   assert_eq!(
       encode(18_446_744_073_709_551_615),
       String::from(
           "eighteen quintillion four hundred forty-six \
            quadrillion seven hundred forty-four trillion \
            seventy-three billion seven hundred nine million \
            five hundred fifty-one thousand six hundred fifteen"
       )
   );
}