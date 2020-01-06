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


// 4
#[test]
fn test_encode_empty_string() {
   assert_eq!("", encode_3(""));
}

#[test]
//#[ignore]
fn test_encode_single_characters() {
   assert_eq!("XYZ", encode_3("XYZ"));
}

#[test]
//#[ignore]
fn test_encode_string_with_no_single_characters() {
   assert_eq!("2A3B4C", encode_3("AABBBCCCC"));
}

#[test]
//#[ignore]
fn test_encode_single_characters_mixed_with_repeated_characters() {
   assert_eq!(
       "12WB12W3B24WB",
       encode_3("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB")
   );
}

#[test]
//#[ignore]
fn test_encode_multiple_whitespace_mixed_in_string() {
   assert_eq!("2 hs2q q2w2 ", encode_3("  hsqq qww  "));
}

#[test]
//#[ignore]
fn test_encode_lowercase_characters() {
   assert_eq!("2a3b4c", encode_3("aabbbcccc"));
}

// decoding tests

#[test]
//#[ignore]
fn test_decode_empty_string() {
   assert_eq!("", decode(""));
}

#[test]
//#[ignore]
fn test_decode_single_characters_only() {
   assert_eq!("XYZ", decode("XYZ"));
}

#[test]
//#[ignore]
fn test_decode_string_with_no_single_characters() {
   assert_eq!("AABBBCCCC", decode("2A3B4C"));
}

#[test]
//#[ignore]
fn test_decode_single_characters_with_repeated_characters() {
   assert_eq!(
       "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB",
       decode("12WB12W3B24WB")
   );
}

#[test]
//#[ignore]
fn test_decode_multiple_whitespace_mixed_in_string() {
   assert_eq!("  hsqq qww  ", decode("2 hs2q q2w2 "));
}

#[test]
//#[ignore]
fn test_decode_lower_case_string() {
   assert_eq!("aabbbcccc", decode("2a3b4c"));
}

// consistency test

#[test]
//#[ignore]
fn test_consistency() {
   assert_eq!("zzz ZZ  zZ", decode(encode_3("zzz ZZ  zZ").as_str()));
}


// 5
#[test]
fn test_valid() {
   assert!(is_valid_isbn("3-598-21508-8"));
}

#[test]
//#[ignore]
fn test_invalid_check_digit() {
   assert!(!is_valid_isbn("3-598-21508-9"));
}

#[test]
//#[ignore]
fn test_valid_check_digit_of_10() {
   assert!(is_valid_isbn("3-598-21507-X"));
}

#[test]
//#[ignore]
fn test_invalid_character_as_check_digit() {
   assert!(!is_valid_isbn("3-598-21507-A"));
}

#[test]
//#[ignore]
fn test_invalid_character_in_isbn() {
   assert!(!is_valid_isbn("3-598-2K507-0"));
}

#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_invalid_isbn_with_invalid_X() {
   assert!(!is_valid_isbn("3-598-2X507-9"));
}

#[test]
//#[ignore]
fn test_valid_isbn_without_dashes() {
   assert!(is_valid_isbn("3598215088"));
}

#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_valid_isbn_without_dashes_and_X_as_check() {
   assert!(is_valid_isbn("359821507X"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_without_dashes_and_no_check_digit() {
   assert!(!is_valid_isbn("359821507"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_without_dashes_and_too_long() {
   assert!(!is_valid_isbn("3598215078X"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_without_check_digit() {
   assert!(!is_valid_isbn("3-598-21507"));
}

#[test]
//#[ignore]
fn test_invalid_isbn_too_long() {
   assert!(!is_valid_isbn("3-598-21507-XX"));
}

#[test]
//#[ignore]
fn test_valid_digits_invalid_length() {
   assert!(!is_valid_isbn("35982150881"));
}

#[test]
//#[ignore]
fn test_special_characters() {
   assert!(!is_valid_isbn("!@#%!@"));
}

#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_invalid_isbn_with_check_digit_X_instead_of_0() {
   assert!(!is_valid_isbn("3-598-21515-X"));
}

// 6
macro_rules! tests {
   ($property_test_func:ident {
       $( $(#[$attr:meta])* $test_name:ident( $( $param:expr ),* ); )+
   }) => {
       $(
           $(#[$attr])*
           #[test]
           fn $test_name() {
               $property_test_func($( $param ),* )
           }
       )+
   }
}

fn test_classification(num: u64, result: Classification) {
   assert_eq!(classify(num), Some(result));
}

#[test]
fn basic() {
   assert_eq!(classify(0), None);
}

tests! {
   test_classification {
       test_1(1, Classification::Deficient);
       test_2(2, Classification::Deficient);
       test_4(4, Classification::Deficient);
       test_6(6, Classification::Perfect);
       test_12(12, Classification::Abundant);
       test_28(28, Classification::Perfect);
       test_30(30, Classification::Abundant);
       test_32(32, Classification::Deficient);
       //test_33550335(33550335, Classification::Abundant);
       //test_33550336(33550336, Classification::Perfect);
       //test_33550337(33550337, Classification::Deficient);
   }
}

// 7
#[test]
fn test_on_the_hour() {
   assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}

#[test]
//#[ignore]
fn test_past_the_hour() {
   assert_eq!(Clock::new(11, 9).to_string(), "11:09");
}

#[test]
//#[ignore]
fn test_midnight_is_zero_hours() {
   assert_eq!(Clock::new(24, 0).to_string(), "00:00");
}

#[test]
//#[ignore]
fn test_hour_rolls_over() {
   assert_eq!(Clock::new(25, 0).to_string(), "01:00");
}

#[test]
//#[ignore]
fn test_hour_rolls_over_continuously() {
   assert_eq!(Clock::new(100, 0).to_string(), "04:00");
}

#[test]
//#[ignore]
fn test_sixty_minutes_is_next_hour() {
   assert_eq!(Clock::new(1, 60).to_string(), "02:00");
}

#[test]
//#[ignore]
fn test_minutes_roll_over() {
   assert_eq!(Clock::new(0, 160).to_string(), "02:40");
}

#[test]
//#[ignore]
fn test_minutes_roll_over_continuously() {
   assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
}

#[test]
//#[ignore]
fn test_hours_and_minutes_roll_over() {
   assert_eq!(Clock::new(25, 160).to_string(), "03:40");
}

#[test]
//#[ignore]
fn test_hours_and_minutes_roll_over_continuously() {
   assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
}

#[test]
//#[ignore]
fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
   assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
}

#[test]
//#[ignore]
fn test_negative_hour() {
   assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
}

#[test]
//#[ignore]
fn test_negative_hour_roll_over() {
   assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
}

#[test]
//#[ignore]
fn test_negative_hour_roll_over_continuously() {
   assert_eq!(Clock::new(-91, 00).to_string(), "05:00");
}

#[test]
//#[ignore]
fn test_negative_minutes() {
   assert_eq!(Clock::new(1, -40).to_string(), "00:20");
}

#[test]
//#[ignore]
fn test_negative_minutes_roll_over() {
   assert_eq!(Clock::new(1, -160).to_string(), "22:20");
}

#[test]
//#[ignore]
fn test_negative_minutes_roll_over_continuously() {
   assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
}

#[test]
//#[ignore]
fn test_negative_hour_and_minutes_both_roll_over() {
   assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
}

#[test]
//#[ignore]
fn test_negative_hour_and_minutes_both_roll_over_continuously() {
   assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
}

//
// Clock Math
//

#[test]
//#[ignore]
fn test_add_minutes() {
   let clock = Clock::new(10, 0).add_minutes(3);
   assert_eq!(clock.to_string(), "10:03");
}

#[test]
//#[ignore]
fn test_add_no_minutes() {
   let clock = Clock::new(6, 41).add_minutes(0);
   assert_eq!(clock.to_string(), "06:41");
}

#[test]
//#[ignore]
fn test_add_to_next_hour() {
   let clock = Clock::new(0, 45).add_minutes(40);
   assert_eq!(clock.to_string(), "01:25");
}

#[test]
//#[ignore]
fn test_add_more_than_one_hour() {
   let clock = Clock::new(10, 0).add_minutes(61);
   assert_eq!(clock.to_string(), "11:01");
}

#[test]
//#[ignore]
fn test_add_more_than_two_hours_with_carry() {
   let clock = Clock::new(0, 45).add_minutes(160);
   assert_eq!(clock.to_string(), "03:25");
}

#[test]
//#[ignore]
fn test_add_across_midnight() {
   let clock = Clock::new(23, 59).add_minutes(2);
   assert_eq!(clock.to_string(), "00:01");
}

#[test]
//#[ignore]
fn test_add_more_than_one_day() {
   let clock = Clock::new(5, 32).add_minutes(1500);
   assert_eq!(clock.to_string(), "06:32");
}

#[test]
//#[ignore]
fn test_add_more_than_two_days() {
   let clock = Clock::new(1, 1).add_minutes(3500);
   assert_eq!(clock.to_string(), "11:21");
}

#[test]
//#[ignore]
fn test_subtract_minutes() {
   let clock = Clock::new(10, 3).add_minutes(-3);
   assert_eq!(clock.to_string(), "10:00");
}

#[test]
//#[ignore]
fn test_subtract_to_previous_hour() {
   let clock = Clock::new(10, 3).add_minutes(-30);
   assert_eq!(clock.to_string(), "09:33");
}

#[test]
//#[ignore]
fn test_subtract_more_than_an_hour() {
   let clock = Clock::new(10, 3).add_minutes(-70);
   assert_eq!(clock.to_string(), "08:53");
}

#[test]
//#[ignore]
fn test_subtract_across_midnight() {
   let clock = Clock::new(0, 3).add_minutes(-4);
   assert_eq!(clock.to_string(), "23:59");
}

#[test]
//#[ignore]
fn test_subtract_more_than_two_hours() {
   let clock = Clock::new(0, 0).add_minutes(-160);
   assert_eq!(clock.to_string(), "21:20");
}

#[test]
//#[ignore]
fn test_subtract_more_than_two_hours_with_borrow() {
   let clock = Clock::new(6, 15).add_minutes(-160);
   assert_eq!(clock.to_string(), "03:35");
}

#[test]
//#[ignore]
fn test_subtract_more_than_one_day() {
   let clock = Clock::new(5, 32).add_minutes(-1500);
   assert_eq!(clock.to_string(), "04:32");
}

#[test]
//#[ignore]
fn test_subtract_mores_than_two_days() {
   let clock = Clock::new(2, 20).add_minutes(-3000);
   assert_eq!(clock.to_string(), "00:20");
}

//
// Test Equality
//

#[test]
//#[ignore]
fn test_compare_clocks_for_equality() {
   assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}

#[test]
//#[ignore]
fn test_compare_clocks_a_minute_apart() {
   assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
}

#[test]
//#[ignore]
fn test_compare_clocks_an_hour_apart() {
   assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_hour_overflow() {
   assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_hour_overflow_by_several_days() {
   assert_eq!(Clock::new(3, 11), Clock::new(99, 11));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hour() {
   assert_eq!(Clock::new(22, 40), Clock::new(-2, 40));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hour_that_wraps() {
   assert_eq!(Clock::new(17, 3), Clock::new(-31, 3));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hour_that_wraps_multiple_times() {
   assert_eq!(Clock::new(13, 49), Clock::new(-83, 49));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_minutes_overflow() {
   assert_eq!(Clock::new(0, 1), Clock::new(0, 1441));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_minutes_overflow_by_several_days() {
   assert_eq!(Clock::new(2, 2), Clock::new(2, 4322));
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_minute() {
   assert_eq!(Clock::new(2, 40), Clock::new(3, -20))
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_minute_that_wraps() {
   assert_eq!(Clock::new(4, 10), Clock::new(5, -1490))
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
   assert_eq!(Clock::new(6, 15), Clock::new(6, -4305))
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hours_and_minutes() {
   assert_eq!(Clock::new(7, 32), Clock::new(-12, -268))
}

#[test]
//#[ignore]
fn test_compare_clocks_with_negative_hours_and_minutes_that_wrap() {
   assert_eq!(Clock::new(18, 7), Clock::new(-54, -11513))
}

#[test]
//#[ignore]
fn test_compare_full_clock_and_zeroed_clock() {
   assert_eq!(Clock::new(24, 0), Clock::new(0, 0))
}

// 8
use maplit::hashmap;

use graph::graph_items::edge::Edge;
use graph::graph_items::node::Node;
use graph::Graph;

#[test]
fn test_empty_graph() {
   let graph = Graph::new();

   assert!(graph.nodes.is_empty());

   assert!(graph.edges.is_empty());

   assert!(graph.attrs.is_empty());
}

#[test]
//#[ignore]
fn test_graph_with_one_node() {
   let nodes = vec![Node::new("a")];

   let graph = Graph::new().with_nodes(&nodes);

   assert!(graph.edges.is_empty());

   assert!(graph.attrs.is_empty());

   assert_eq!(graph.nodes, vec![Node::new("a")]);
}

#[test]
//#[ignore]
fn test_graph_with_one_node_with_keywords() {
   let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];

   let graph = Graph::new().with_nodes(&nodes);

   assert!(graph.edges.is_empty());

   assert!(graph.attrs.is_empty());

   assert_eq!(
       graph.nodes,
       vec![Node::new("a").with_attrs(&[("color", "green")])]
   );
}

#[test]
//#[ignore]
fn test_graph_with_one_edge() {
   let edges = vec![Edge::new("a", "b")];

   let graph = Graph::new().with_edges(&edges);

   assert!(graph.nodes.is_empty());

   assert!(graph.attrs.is_empty());

   assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
}

#[test]
//#[ignore]
fn test_graph_with_one_attribute() {
   let graph = Graph::new().with_attrs(&[("foo", "1")]);

   let expected_attrs = hashmap!{
       "foo".to_string() => "1".to_string(),
   };

   assert!(graph.nodes.is_empty());

   assert!(graph.edges.is_empty());

   assert_eq!(graph.attrs, expected_attrs);
}

#[test]
//#[ignore]
fn test_graph_with_attributes() {
   let nodes = vec![
       Node::new("a").with_attrs(&[("color", "green")]),
       Node::new("c"),
       Node::new("b").with_attrs(&[("label", "Beta!")]),
   ];

   let edges = vec![
       Edge::new("b", "c"),
       Edge::new("a", "b").with_attrs(&[("color", "blue")]),
   ];

   let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

   let expected_attrs = hashmap! {
       "foo".to_string() => "1".to_string(),
       "title".to_string() => "Testing Attrs".to_string(),
       "bar".to_string() => "true".to_string(),
   };

   let graph = Graph::new()
       .with_nodes(&nodes)
       .with_edges(&edges)
       .with_attrs(&attrs);

   assert_eq!(
       graph.nodes,
       vec![
           Node::new("a").with_attrs(&[("color", "green")]),
           Node::new("c"),
           Node::new("b").with_attrs(&[("label", "Beta!")]),
       ]
   );

   assert_eq!(
       graph.edges,
       vec![
           Edge::new("b", "c"),
           Edge::new("a", "b").with_attrs(&[("color", "blue")]),
       ]
   );

   assert_eq!(graph.attrs, expected_attrs);
}

#[test]
//#[ignore]
fn test_graph_stores_attributes() {
   let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
   let graph = Graph::new().with_nodes(
       &['a', 'b', 'c']
           .iter()
           .enumerate()
           .map(|(i, n)| Node::new(&n.to_string()).with_attrs(&attributes[i..i + 1]))
           .collect::<Vec<_>>(),
   );

   assert_eq!(
       graph
           .get_node("c")
           .expect("node must be stored")
           .get_attr("bim"),
       Some("bef")
   );
}

// 9
#[test]
fn test_no_difference_between_empty_strands() {
   assert_eq!(hamming_distance("", ""), Some(0));
}

#[test]
//#[ignore]
fn test_no_difference_between_identical_strands() {
   assert_eq!(hamming_distance("GGACTGA", "GGACTGA"), Some(0));
}

#[test]
//#[ignore]
fn test_complete_hamming_distance_in_small_strand() {
   assert_eq!(hamming_distance("ACT", "GGA"), Some(3));
}

#[test]
//#[ignore]
fn test_small_hamming_distance_in_the_middle_somewhere() {
   assert_eq!(hamming_distance("GGACG", "GGTCG"), Some(1));
}

#[test]
//#[ignore]
fn test_larger_distance() {
   assert_eq!(hamming_distance("ACCAGGG", "ACTATGG"), Some(2));
}

#[test]
//#[ignore]
fn test_first_string_is_longer() {
   assert_eq!(hamming_distance("AAA", "AA"), None);
}

#[test]
//#[ignore]
fn test_second_string_is_longer() {
   assert_eq!(hamming_distance("A", "AA"), None);
}

// 10
#[test]
fn test_new_list_is_empty() {
   let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
   assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
//#[ignore]
fn test_push_increments_length() {
   let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
   list.push(1);
   assert_eq!(list.len(), 1, "list's length must be 1");
   list.push(2);
   assert_eq!(list.len(), 2, "list's length must be 2");
}

#[test]
//#[ignore]
fn test_pop_decrements_length() {
   let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
   list.push(1);
   list.push(2);
   list.pop();
   assert_eq!(list.len(), 1, "list's length must be 1");
   list.pop();
   assert_eq!(list.len(), 0, "list's length must be 0");
}

#[test]
//#[ignore]
fn test_pop_returns_last_added_element() {
   let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
   list.push(1);
   list.push(2);
   assert_eq!(list.pop(), Some(2), "Element must be 2");
   assert_eq!(list.pop(), Some(1), "Element must be 1");
   assert_eq!(list.pop(), None, "No element should be contained in list");
}

#[test]
//#[ignore]
fn test_peek_returns_head_element() {
   let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
   assert_eq!(list.peek(), None, "No element should be contained in list");
   list.push(2);
   assert_eq!(list.peek(), Some(&2), "Element must be 2");
   assert_eq!(list.peek(), Some(&2), "Element must be still 2");
}

#[test]
//#[ignore]
fn test_from_slice() {
   let array = ["1", "2", "3", "4"];
   let mut list = SimpleLinkedList::from(array.as_ref());
   assert_eq!(list.pop(), Some("4"));
   assert_eq!(list.pop(), Some("3"));
   assert_eq!(list.pop(), Some("2"));
   assert_eq!(list.pop(), Some("1"));
}

#[test]
//#[ignore]
fn test_reverse() {
   let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
   list.push(1);
   list.push(2);
   list.push(3);
   let mut rev_list = list.rev();
   assert_eq!(rev_list.pop(), Some(1));
   assert_eq!(rev_list.pop(), Some(2));
   assert_eq!(rev_list.pop(), Some(3));
   assert_eq!(rev_list.pop(), None);
}

#[test]
//#[ignore]
fn test_into_vector() {
   let mut v = Vec::new();
   let mut s = SimpleLinkedList::new();
   for i in 1..4 {
       v.push(i);
       s.push(i);
   }
   let s_as_vec: Vec<i32> = s.into();
   assert_eq!(v, s_as_vec);
}

// 12
#[test]
fn a_is_worth_one_point() {
   assert_eq!(score("a"), 1);
}

#[test]
//#[ignore]
fn scoring_is_case_insensitive() {
   assert_eq!(score("A"), 1);
}

#[test]
//#[ignore]
fn f_is_worth_four() {
   assert_eq!(score("f"), 4);
}

#[test]
//#[ignore]
fn two_one_point_letters_make_a_two_point_word() {
   assert_eq!(score("at"), 2);
}

#[test]
//#[ignore]
fn three_letter_word() {
   assert_eq!(score("zoo"), 12);
}

#[test]
//#[ignore]
fn medium_word() {
   assert_eq!(score("street"), 6);
}

#[test]
//#[ignore]
fn longer_words_with_valuable_letters() {
   assert_eq!(score("quirky"), 22);
}

#[test]
//#[ignore]
fn long_mixed_case_word() {
   assert_eq!(score("OxyphenButazone"), 41);
}

#[test]
//#[ignore]
fn non_english_scrabble_letters_do_not_score() {
   assert_eq!(score("pinata"), 8, "'n' should score 1");
   assert_eq!(score("piñata"), 7, "'ñ' should score 0");
}

#[test]
//#[ignore]
fn empty_words_are_worth_zero() {
   assert_eq!(score(""), 0);
}

#[test]
//#[ignore]
fn all_letters_work() {
   assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}

#[test]
//#[ignore]
fn german_letters_do_not_score() {
   assert_eq!(score("STRASSE"), 7, "\"SS\" should score 2");
   assert_eq!(score("STRAßE"), 5, "'ß' should score 0");
}

// 13
#[test]
fn empty_strings_are_not_pangrams() {
   let sentence = "";
   assert!(!is_pangram(&sentence));
}

#[test]
//#[ignore]
fn classic_pangram_is_a_pangram() {
   let sentence = "the quick brown fox jumps over the lazy dog";
   assert!(is_pangram(&sentence));
}

#[test]
//#[ignore]
fn pangrams_must_have_all_letters() {
   let sentence = "a quick movement of the enemy will jeopardize five gunboats";
   assert!(!is_pangram(&sentence));
}

#[test]
//#[ignore]
fn pangrams_must_have_all_letters_two() {
   let sentence = "the quick brown fish jumps over the lazy dog";
   assert!(!is_pangram(&sentence));
}

#[test]
//#[ignore]
fn pangrams_must_include_z() {
   let sentence = "the quick brown fox jumps over the lay dog";
   assert!(!is_pangram(&sentence));
}

#[test]
//#[ignore]
fn underscores_do_not_affect_pangrams() {
   let sentence = "the_quick_brown_fox_jumps_over_the_lazy_dog";
   assert!(is_pangram(&sentence));
}

#[test]
//#[ignore]
fn numbers_do_not_affect_pangrams() {
   let sentence = "the 1 quick brown fox jumps over the 2 lazy dogs";
   assert!(is_pangram(&sentence));
}

#[test]
//#[ignore]
fn numbers_can_not_replace_letters() {
   let sentence = "7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog";
   assert!(!is_pangram(&sentence));
}

#[test]
//#[ignore]
fn capitals_and_punctuation_can_be_in_pangrams() {
   let sentence = "\"Five quacking Zephyrs jolt my wax bed.\"";
   assert!(is_pangram(&sentence));
}

#[test]
//#[ignore]
fn non_ascii_characters_can_be_in_pangrams() {
   let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
   assert!(is_pangram(&sentence));
}

// 15
use std::collections::HashMap;
fn check_dna(s: &str, pairs: &[(char, usize)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<char, usize> = nucleotide_counts(s).unwrap();
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k)), (k, Some(v)));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&char, &usize)>>(), vec![]);
 }
 
 #[test]
 fn count_returns_result() {
    assert!(count('A', "").is_ok());
 }
 
 #[test]
 //#[ignore]
 fn test_count_empty() {
    assert_eq!(count('A', ""), Ok(0));
 }
 
 #[test]
 //#[ignore]
 fn count_invalid_nucleotide() {
    assert_eq!(count('X', "A"), Err('X'));
 }
 
 #[test]
 //#[ignore]
 fn count_invalid_dna() {
    assert_eq!(count('A', "AX"), Err('X'));
 }
 
 #[test]
 //#[ignore]
 fn test_count_repetitive_cytosine() {
    assert_eq!(count('C', "CCCCC"), Ok(5));
 }
 
 #[test]
 //#[ignore]
 fn test_count_only_thymine() {
    assert_eq!(count('T', "GGGGGTAACCCGG"), Ok(1));
 }
 
 #[test]
 //#[ignore]
 fn counts_returns_result() {
    assert!(nucleotide_counts("ACGT").is_ok());
 }
 
 #[test]
 //#[ignore]
 fn test_nucleotide_count_empty() {
    check_dna("", &[('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
 }
 
 #[test]
 //#[ignore]
 fn test_nucleotide_count_only_guanine() {
    check_dna("GGGGGGGG", &[('A', 0), ('T', 0), ('C', 0), ('G', 8)]);
 }
 
 #[test]
 //#[ignore]
 fn test_nucleotide_count_counts_all() {
    check_dna(
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAA\
         GAGTGTCTGATAGCAGC",
        &[('A', 20), ('T', 21), ('C', 12), ('G', 17)],
    );
 }
 
 #[test]
 //#[ignore]
 fn counts_invalid_nucleotide_results_in_err() {
    assert_eq!(nucleotide_counts("GGXXX"), Err('X'));
 }

 // 16
 #[test]
fn single_digit_string_is_invalid() {
   assert!(!is_valid("1"));
}

#[test]
//#[ignore]
fn single_zero_string_is_invalid() {
   assert!(!is_valid("0"));
}

#[test]
//#[ignore]
fn simple_valid_sin() {
   assert!(is_valid(" 5 9 "));
}

#[test]
//#[ignore]
fn valid_canadian_sin_is_valid() {
   assert!(is_valid("046 454 286"));
}

#[test]
//#[ignore]
fn invalid_canadian_sin_is_invalid() {
   assert!(!is_valid("046 454 287"));
}

#[test]
//#[ignore]
fn invalid_credit_card_is_invalid() {
   assert!(!is_valid("8273 1232 7352 0569"));
}

#[test]
//#[ignore]
fn strings_that_contain_non_digits_are_invalid() {
   assert!(!is_valid("046a 454 286"));
}

#[test]
//#[ignore]
fn punctuation_is_invalid() {
   assert!(!is_valid("055-444-285"));
}

#[test]
//#[ignore]
fn symbols_are_invalid() {
   assert!(!is_valid("055£ 444$ 285"));
}

#[test]
//#[ignore]
fn single_digit_with_space_is_invalid() {
   assert!(!is_valid(" 0"));
}

#[test]
//#[ignore]
fn lots_of_zeros_are_valid() {
   assert!(is_valid(" 00000"));
}

#[test]
//#[ignore]
fn another_valid_sin() {
   assert!(is_valid("055 444 285"));
}

#[test]
//#[ignore]
fn nine_doubled_is_nine() {
   assert!(is_valid("091"));
}

// 17
#[test]
fn return_is_a_result() {
   assert!(lsp("29", 2).is_ok());
}

#[test]
//#[ignore]
fn find_the_largest_product_when_span_equals_length() {
   assert_eq!(Ok(18), lsp("29", 2));
}

#[test]
//#[ignore]
fn find_the_largest_product_of_two_with_numbers_in_order() {
   assert_eq!(Ok(72), lsp("0123456789", 2));
}

#[test]
//#[ignore]
fn find_the_largest_product_of_two_with_numbers_not_in_order() {
   assert_eq!(Ok(48), lsp("576802143", 2));
}

#[test]
//#[ignore]
fn find_the_largest_product_of_three_with_numbers_in_order() {
   assert_eq!(Ok(504), lsp("0123456789", 3));
}

#[test]
//#[ignore]
fn find_the_largest_product_of_three_with_numbers_not_in_order() {
   assert_eq!(Ok(270), lsp("1027839564", 3));
}

#[test]
//#[ignore]
fn find_the_largest_product_of_five_with_numbers_in_order() {
   assert_eq!(Ok(15120), lsp("0123456789", 5));
}

#[test]
//#[ignore]
fn span_of_six_in_a_large_number() {
   assert_eq!(
       Ok(23520),
       lsp("73167176531330624919225119674426574742355349194934", 6)
   );
}

#[test]
//#[ignore]
fn returns_zero_if_number_is_zeros() {
   assert_eq!(Ok(0), lsp("0000", 2));
}

#[test]
//#[ignore]
fn returns_zero_if_all_products_are_zero() {
   assert_eq!(Ok(0), lsp("99099", 3));
}

#[test]
//#[ignore]
fn a_span_is_longer_than_number_is_an_error() {
   assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
}

// There may be some confusion about whether this should be 1 or error.
// The reasoning for it being 1 is this:
// There is one 0-character string contained in the empty string.
// That's the empty string itself.
// The empty product is 1 (the identity for multiplication).
// Therefore LSP('', 0) is 1.
// It's NOT the case that LSP('', 0) takes max of an empty list.
// So there is no error.
// Compare against LSP('123', 4):
// There are zero 4-character strings in '123'.
// So LSP('123', 4) really DOES take the max of an empty list.
// So LSP('123', 4) errors and LSP('', 0) does NOT.
#[test]
//#[ignore]
fn an_empty_string_and_no_span_returns_one() {
   assert_eq!(Ok(1), lsp("", 0));
}

#[test]
//#[ignore]
fn a_non_empty_string_and_no_span_returns_one() {
   assert_eq!(Ok(1), lsp("123", 0));
}

#[test]
//#[ignore]
fn empty_string_and_non_zero_span_is_an_error() {
   assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
}

#[test]
//#[ignore]
fn a_string_with_non_digits_is_an_error() {
   assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
}

// 18
fn check_word_count(s: &str, pairs: &[(&str, u32)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<String, u32> = word_count(s);
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k.to_string()).unwrap_or(0)), (k, v));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
 }
 
 #[test]
 fn test_count_one_word() {
    check_word_count("word", &[("word", 1)]);
 }
 
 #[test]
 //#[ignore]
 fn test_count_one_of_each() {
    check_word_count("one of each", &[("one", 1), ("of", 1), ("each", 1)]);
 }
 
 #[test]
 //#[ignore]
 fn test_count_multiple_occurrences() {
    check_word_count(
        "one fish two fish red fish blue fish",
        &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
    );
 }
 
 #[test]
 //#[ignore]
 fn test_ignore_punctuation() {
    check_word_count(
        "car : carpet as java : javascript!!&@$%^&",
        &[
            ("car", 1),
            ("carpet", 1),
            ("as", 1),
            ("java", 1),
            ("javascript", 1),
        ],
    );
 }
 
 #[test]
 //#[ignore]
 fn test_include_numbers() {
    check_word_count(
        "testing, 1, 2 testing",
        &[("testing", 2), ("1", 1), ("2", 1)],
    );
 }
 
 #[test]
 //#[ignore]
 fn test_normalize_case() {
    check_word_count("go Go GO Stop stop", &[("go", 3), ("stop", 2)]);
 }