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

 // test 19
 #[test]
fn test_encode_yes() {
   assert_eq!("bvh", encode_19("yes"));
}

#[test]
//#[ignore]
fn test_encode_no() {
   assert_eq!("ml", encode_19("no"));
}

#[test]
//#[ignore]
fn test_encode_omg() {
   assert_eq!("lnt", encode_19("OMG"));
}

#[test]
//#[ignore]
fn test_encode_spaces() {
   assert_eq!("lnt", encode_19("O M G"));
}

#[test]
//#[ignore]
fn test_encode_mindblowingly() {
   assert_eq!("nrmwy oldrm tob", encode_19("mindblowingly"));
}

#[test]
//#[ignore]
fn test_encode_numbers() {
   assert_eq!("gvhgr mt123 gvhgr mt", encode_19("Testing,1 2 3, testing."));
}

#[test]
//#[ignore]
fn test_encode_deep_thought() {
   assert_eq!("gifgs rhurx grlm", encode_19("Truth is fiction."));
}

#[test]
//#[ignore]
fn test_encode_all_the_letters() {
   assert_eq!(
       "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt",
       encode_19("The quick brown fox jumps over the lazy dog.")
   );
}

#[test]
//#[ignore]
fn test_encode_ignores_non_ascii() {
   assert_eq!("mlmzh xrrrt mlivw", encode_19("non ascii éignored"));
}

#[test]
//#[ignore]
fn test_decode_exercism() {
   assert_eq!("exercism", decode_19("vcvix rhn"));
}

#[test]
//#[ignore]
fn test_decode_a_sentence() {
   assert_eq!(
       "anobstacleisoftenasteppingstone",
       decode_19("zmlyh gzxov rhlug vmzhg vkkrm thglm v")
   );
}

#[test]
//#[ignore]
fn test_decode_numbers() {
   assert_eq!("testing123testing", decode_19("gvhgr mt123 gvhgr mt"));
}

#[test]
//#[ignore]
fn test_decode_all_the_letters() {
   assert_eq!(
       "thequickbrownfoxjumpsoverthelazydog",
       decode_19("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt")
   );
}

// 20
fn test(input: &str, output: &str) {
    assert_eq!(&encrypt(input), output);
 }
 
 #[test]
 fn test_empty_input() {
    test("", "")
 }
 
 #[test]
 //#[ignore]
 fn test_encrypt_also_decrypts_square() {
    // note that you only get the exact input back if:
    // 1. no punctuation
    // 2. even spacing
    // 3. all lowercase
    // 4. square input
    let example = "lime anda coco anut";
    assert_eq!(example, &encrypt(&encrypt(example)));
 }
 
 #[test]
 //#[ignore]
 fn test_example() {
    test(
        "If man was meant to stay on the ground, god would have given us roots.",
        "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau ",
    )
 }
 
 #[test]
 //#[ignore]
 fn test_empty_last_line() {
    test("congratulate", "crl oaa ntt gue")
 }
 
 #[test]
 //#[ignore]
 fn test_spaces_are_reorganized() {
    test("abet", "ae bt");
    test("a bet", "ae bt");
    test("     a  b     e      t             ", "ae bt");
 }
 
 #[test]
 //#[ignore]
 fn test_everything_becomes_lowercase() {
    test("caSe", "cs ae");
    test("cAsE", "cs ae");
    test("CASE", "cs ae");
 }
 
 #[test]
 //#[ignore]
 fn test_long() {
    test(
        r#"
 We choose to go to the moon.
 
 We choose to go to the moon in this decade and do the other things,
 not because they are easy, but because they are hard, because that
 goal will serve to organize and measure the best of our energies and
 skills, because that challenge is one that we are willing to accept,
 one we are unwilling to postpone, and one which we intend to win,
 and the others, too.
 
 -- John F. Kennedy, 12 September 1962
        "#,
        &(String::from("womdbudlmecsgwdwob enooetbsenaotioihe ")
            + "cwotcbeeaeunolnnnr henhaecrsrsealeaf1 ocieucavugetciwnk9 "
            + "ohnosauerithcnhde6 sotteusteehaegitn2 eohhtseotsatptchn  "
            + "tsiehetohatwtohee  oesrethrenceopwod  gtdtyhagbdhanoety  "
            + "ooehaetaesaresih1  tgcirygnsklewtne2  ooaneaoitilweptrs  "
            + "ttdgerazoleiaoese  hoesaeleflnlrnntp  etanshwaosgleedot  "
            + "mhnoyainubeiuatoe  oedtbrldreinnnojm "),
    )
 }

 // 21
 #[test]
fn rotate_a_1() {
   assert_eq!("b", rotate("a", 1));
}

#[test]
//#[ignore]
fn rotate_a_26() {
   assert_eq!("a", rotate("a", 26));
}

#[test]
//#[ignore]
fn rotate_a_0() {
   assert_eq!("a", rotate("a", 0));
}

#[test]
//#[ignore]
fn rotate_m_13() {
   assert_eq!("z", rotate("m", 13));
}

#[test]
//#[ignore]
fn rotate_n_13_with_wrap() {
   assert_eq!("a", rotate("n", 13));
}

#[test]
//#[ignore]
fn rotate_caps() {
   assert_eq!("TRL", rotate("OMG", 5));
}

#[test]
//#[ignore]
fn rotate_spaces() {
   assert_eq!("T R L", rotate("O M G", 5));
}

#[test]
//#[ignore]
fn rotate_numbers() {
   assert_eq!("Xiwxmrk 1 2 3 xiwxmrk", rotate("Testing 1 2 3 testing", 4));
}

#[test]
//#[ignore]
fn rotate_punctuation() {
   assert_eq!("Gzo\'n zvo, Bmviyhv!", rotate("Let\'s eat, Grandma!", 21));
}

#[test]
//#[ignore]
fn rotate_all_the_letters() {
   assert_eq!(
       "Gur dhvpx oebja sbk whzcf bire gur ynml qbt.",
       rotate("The quick brown fox jumps over the lazy dog.", 13)
   );
}

// 22
use std::collections::HashSet;

const PLAIN_TEXT: &str = "thisismysecret";
const KEY: &str = "abcdefghij";

#[test]
fn cipher_can_encode_with_given_key() {
   assert_eq!(encode_22(KEY, "aaaaaaaaaa"), Some(KEY.to_string()));
}

#[test]
//#[ignore]
fn cipher_can_decode_with_given_key() {
   assert_eq!(decode_22(KEY, "abcdefghij"), Some("aaaaaaaaaa".to_string()));
}

#[test]
//#[ignore]
fn cipher_is_reversible_given_key() {
   assert_eq!(
       decode_22(KEY, &encode_22(KEY, PLAIN_TEXT).unwrap()),
       Some("thisismyse".to_string())
   );
}

#[test]
//#[ignore]
fn cipher_can_double_shift_encode() {
   let plain_text = "iamapandabear";
   assert_eq!(
       encode_22(plain_text, plain_text),
       Some("qayaeaagaciai".to_string())
   );
}

#[test]
//#[ignore]
fn cipher_can_wrap_encode() {
   assert_eq!(encode_22(KEY, "zzzzzzzzzz"), Some("zabcdefghi".to_string()));
}

#[test]
//#[ignore]
fn cipher_can_encode_a_message_that_is_shorter_than_the_key() {
   assert_eq!(encode_22(KEY, "aaaaa"), Some("abcde".to_string()));
}

#[test]
//#[ignore]
fn cipher_can_decode_a_message_that_is_shorter_than_the_key() {
   assert_eq!(decode_22(KEY, "abcde"), Some("aaaaa".to_string()));
}

#[test]
//#[ignore]
fn encode_returns_none_with_an_all_caps_key() {
   let key = "ABCDEF";
   assert_eq!(encode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn encode_returns_none_with_an_any_caps_key() {
   let key = "abcdEFg";
   assert_eq!(encode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn encode_returns_none_with_numeric_key() {
   let key = "12345";
   assert_eq!(encode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn encode_returns_none_with_any_numeric_key() {
   let key = "abcd345ef";
   assert_eq!(encode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn encode_returns_none_with_empty_key() {
   let key = "";
   assert_eq!(encode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn decode_returns_none_with_an_all_caps_key() {
   let key = "ABCDEF";
   assert_eq!(decode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn decode_returns_none_with_an_any_caps_key() {
   let key = "abcdEFg";
   assert_eq!(decode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn decode_returns_none_with_numeric_key() {
   let key = "12345";
   assert_eq!(decode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn decode_returns_none_with_any_numeric_key() {
   let key = "abcd345ef";
   assert_eq!(decode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn decode_returns_none_with_empty_key() {
   let key = "";
   assert_eq!(decode_22(key, PLAIN_TEXT), None);
}

#[test]
//#[ignore]
fn encode_random_uses_key_made_of_letters() {
   let (k, _) = encode_random(PLAIN_TEXT);
   assert!(k.chars().all(|c| c.is_ascii_lowercase()));
}

#[test]
//#[ignore]
fn encode_random_uses_key_of_100_characters_or_more() {
   let (k, _) = encode_random(PLAIN_TEXT);
   assert!(k.len() >= 100);
}

#[test]
//#[ignore]
fn encode_random_uses_randomly_generated_key() {
   let mut keys = HashSet::new();
   let trials = 100;
   for _ in 0..trials {
       keys.insert(encode_random(PLAIN_TEXT).0);
   }
   assert_eq!(keys.len(), trials);
}

#[test]
//#[ignore]
fn encode_random_can_encode() {
   let (k, encoded) = encode_random("aaaaaaaaaa");
   assert_eq!(encoded, k.split_at(10).0);
}

#[test]
//#[ignore]
fn encode_random_can_decode() {
   let (k, _) = encode_random("aaaaaaaaaa");
   assert_eq!(decode_22(&k, k.split_at(10).0), Some("aaaaaaaaaa".to_string()));
}

#[test]
//#[ignore]
fn encode_random_is_reversible() {
   let (k, encoded) = encode_random(PLAIN_TEXT);
   assert_eq!(decode_22(&k, &encoded), Some(PLAIN_TEXT.to_string()));
}

// 23
fn process_encode_case(input: &str, rails: u32, expected: &str) {
   let rail_fence = RailFence::new(rails);
   assert_eq!(rail_fence.encode(input), expected);
}

/// Process a single test case for the property `decode`
///
/// All cases for the `decode` property are implemented
/// in terms of this function.
fn process_decode_case(input: &str, rails: u32, expected: &str) {
   let rail_fence = RailFence::new(rails);
   assert_eq!(rail_fence.decode(input), expected);
}

// encode

#[test]
/// encode with two rails
fn test_encode_with_two_rails() {
   process_encode_case("XOXOXOXOXOXOXOXOXO", 2, "XXXXXXXXXOOOOOOOOO");
}

#[test]
//#[ignore]
/// encode with three rails
fn test_encode_with_three_rails() {
   process_encode_case("WEAREDISCOVEREDFLEEATONCE", 3, "WECRLTEERDSOEEFEAOCAIVDEN");
}

#[test]
//#[ignore]
/// encode with ending in the middle
fn test_encode_with_ending_in_the_middle() {
   process_encode_case("EXERCISES", 4, "ESXIEECSR");
}

// decode

#[test]
//#[ignore]
/// decode with three rails
fn test_decode_with_three_rails() {
   process_decode_case("TEITELHDVLSNHDTISEIIEA", 3, "THEDEVILISINTHEDETAILS");
}

#[test]
//#[ignore]
/// decode with five rails
fn test_decode_with_five_rails() {
   process_decode_case("EIEXMSMESAORIWSCE", 5, "EXERCISMISAWESOME");
}

#[test]
//#[ignore]
/// decode with six rails
fn test_decode_with_six_rails() {
   process_decode_case(
       "133714114238148966225439541018335470986172518171757571896261",
       6,
       "112358132134558914423337761098715972584418167651094617711286",
   );
}

#[test]
//#[ignore]
/// encode wide characters
///
/// normally unicode is not part of exercism exercises, but in an exercise
/// specifically oriented around shuffling characters, it seems worth ensuring
/// that wide characters are handled properly
///
/// this text is possibly one of the most famous haiku of all time, by
/// Matsuo Bashō (松尾芭蕉)
fn test_encode_wide_characters() {
   process_encode_case(
       "古池 蛙飛び込む 水の音",
       3,
       "古飛 池蛙びむ水音 込の",
   );
}

// 24
use std::collections::BTreeMap;
#[test]
fn test_transform_one_value() {
   let input = input_from(&[(1, vec!['A'])]);

   let expected = expected_from(&[('a', 1)]);

   assert_eq!(expected, transform(&input));
}

#[test]
//#[ignore]
fn test_transform_more_values() {
   let input = input_from(&[(1, vec!['A', 'E', 'I', 'O', 'U'])]);

   let expected = expected_from(&[('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]);

   assert_eq!(expected, transform(&input));
}

#[test]
//#[ignore]
fn test_more_keys() {
   let input = input_from(&[(1, vec!['A', 'E']), (2, vec!['D', 'G'])]);

   let expected = expected_from(&[('a', 1), ('e', 1), ('d', 2), ('g', 2)]);

   assert_eq!(expected, transform(&input));
}

#[test]
//#[ignore]
fn test_full_dataset() {
   let input = input_from(&[
       (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
       (2, vec!['D', 'G']),
       (3, vec!['B', 'C', 'M', 'P']),
       (4, vec!['F', 'H', 'V', 'W', 'Y']),
       (5, vec!['K']),
       (8, vec!['J', 'X']),
       (10, vec!['Q', 'Z']),
   ]);

   let expected = expected_from(&[
       ('a', 1),
       ('b', 3),
       ('c', 3),
       ('d', 2),
       ('e', 1),
       ('f', 4),
       ('g', 2),
       ('h', 4),
       ('i', 1),
       ('j', 8),
       ('k', 5),
       ('l', 1),
       ('m', 3),
       ('n', 1),
       ('o', 1),
       ('p', 3),
       ('q', 10),
       ('r', 1),
       ('s', 1),
       ('t', 1),
       ('u', 1),
       ('v', 4),
       ('w', 4),
       ('x', 8),
       ('y', 4),
       ('z', 10),
   ]);

   assert_eq!(expected, transform(&input));
}

fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
   v.iter().cloned().collect()
}

fn expected_from(v: &[(char, i32)]) -> BTreeMap<char, i32> {
   v.iter().cloned().collect()
}

// 25
fn square(x: i32) -> i32 {
   x * x
}

#[test]
fn func_single() {
   let input = vec![2];
   let expected = vec![4];
   assert_eq!(map(input, square), expected);
}

#[test]
//#[ignore]
fn func_multi() {
   let input = vec![2, 3, 4, 5];
   let expected = vec![4, 9, 16, 25];
   assert_eq!(map(input, square), expected);
}

#[test]
//#[ignore]
fn closure() {
   let input = vec![2, 3, 4, 5];
   let expected = vec![4, 9, 16, 25];
   assert_eq!(map(input, |x| x * x), expected);
}

#[test]
//#[ignore]
fn closure_floats() {
   let input = vec![2.0, 3.0, 4.0, 5.0];
   let expected = vec![4.0, 9.0, 16.0, 25.0];
   assert_eq!(map(input, |x| x * x), expected);
}

#[test]
//#[ignore]
fn strings() {
   let input = vec!["1".to_string(), "2".into(), "3".into()];
   let expected = vec!["11".to_string(), "22".into(), "33".into()];
   assert_eq!(map(input, |s| s.repeat(2)), expected);
}

#[test]
//#[ignore]
fn change_in_type() {
   let input: Vec<&str> = vec!["1", "2", "3"];
   let expected: Vec<String> = vec!["1".into(), "2".into(), "3".into()];
   assert_eq!(map(input, |s| s.to_string()), expected);
}

#[test]
//#[ignore]
fn mutating_closure() {
   let mut counter = 0;
   let input = vec![-2, 3, 4, -5];
   let expected = vec![2, 3, 4, 5];
   let result = map(input, |x: i64| {
       counter += 1;
       x.abs()
   });
   assert_eq!(result, expected);
   assert_eq!(counter, 4);
}

#[test]
//#[ignore]
fn minimal_bounds_on_input_and_output() {
   // must be able to accept arbitrary input and output types
   struct Foo;
   struct Bar;
   map(vec![Foo], |_| Bar);
}

// 26
#[test]
fn empty() {
   assert_eq!(abbreviate(""), "");
}

#[test]
//#[ignore]
fn basic1() {
   assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
}

#[test]
//#[ignore]
fn lowercase_words() {
   assert_eq!(abbreviate("Ruby on Rails"), "ROR");
}

#[test]
//#[ignore]
fn camelcase() {
   assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
}

#[test]
//#[ignore]
fn punctuation() {
   assert_eq!(abbreviate("First In, First Out"), "FIFO");
}

#[test]
//#[ignore]
fn all_caps_words() {
   assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
}

#[test]
//#[ignore]
fn non_acronym_all_caps_word() {
   assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
}

#[test]
//#[ignore]
fn hyphenated() {
   assert_eq!(
       abbreviate("Complementary metal-oxide semiconductor"),
       "CMOS"
   );
}

// 27
#[test]
fn limit_lower_than_the_first_prime() {
   assert_eq!(primes_up_to(1), []);
}

#[test]
//#[ignore]
fn limit_is_the_first_prime() {
   assert_eq!(primes_up_to(2), [2]);
}

#[test]
//#[ignore]
fn primes_up_to_10() {
   assert_eq!(primes_up_to(10), [2, 3, 5, 7]);
}

#[test]
//#[ignore]
fn limit_is_prime() {
   assert_eq!(primes_up_to(13), [2, 3, 5, 7, 11, 13]);
}

#[test]
//#[ignore]
fn limit_of_1000() {
   let expected = vec![
       2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
       97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
       191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
       283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
       401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
       509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
       631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
       751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
       877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
   ];
   assert_eq!(primes_up_to(1000), expected);
}

// 28
#[test]
fn test_valid_dna_input() {
   assert!(DNA::new("GCTA").is_ok());
}

#[test]
//#[ignore]
fn test_valid_rna_input() {
   assert!(RNA::new("CGAU").is_ok());
}

#[test]
//#[ignore]
fn test_invalid_dna_input() {
   // Invalid character
   assert_eq!(DNA::new("X").err(), Some(0));
   // Valid nucleotide, but invalid in context
   assert_eq!(DNA::new("U").err(), Some(0));
   // Longer string with contained errors
   assert_eq!(DNA::new("ACGTUXXCTTAA").err(), Some(4));
}

#[test]
//#[ignore]
fn test_invalid_rna_input() {
   // Invalid character
   assert!(RNA::new("X").is_err());
   // Valid nucleotide, but invalid in context
   assert!(RNA::new("T").is_err());
   // Longer string with contained errors
   assert!(RNA::new("ACGUTTXCUUAA").is_err());
}

#[test]
//#[ignore]
fn test_acid_equals_acid() {
   assert_eq!(DNA::new("CGA").unwrap(), DNA::new("CGA").unwrap());
   assert_ne!(DNA::new("CGA").unwrap(), DNA::new("AGC").unwrap());
   assert_eq!(RNA::new("CGA").unwrap(), RNA::new("CGA").unwrap());
   assert_ne!(RNA::new("CGA").unwrap(), RNA::new("AGC").unwrap());
}

#[test]
//#[ignore]
fn test_transcribes_cytosine_guanine() {
   assert_eq!(RNA::new("G").unwrap(), DNA::new("C").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_guanine_cytosine() {
   assert_eq!(RNA::new("C").unwrap(), DNA::new("G").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_adenine_uracil() {
   assert_eq!(RNA::new("U").unwrap(), DNA::new("A").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_thymine_to_adenine() {
   assert_eq!(RNA::new("A").unwrap(), DNA::new("T").unwrap().to_rna());
}

#[test]
//#[ignore]
fn test_transcribes_all_dna_to_rna() {
   assert_eq!(
       RNA::new("UGCACCAGAAUU").unwrap(),
       DNA::new("ACGTGGTCTTAA").unwrap().to_rna()
   )
}

// 29
#[test]
fn positive_length_sides_are_ok() {
   let sides = [2, 2, 2];
   let triangle = Triangle::build(sides);
   assert!(triangle.is_some());
}

#[test]
//#[ignore]
fn zero_length_sides_are_illegal() {
   let sides = [0, 0, 0];
   let triangle = Triangle::build(sides);
   assert!(triangle.is_none());
}

#[test]
//#[ignore]
fn equilateral_triangles_have_equal_sides() {
   let sides = [2, 2, 2];
   let triangle = Triangle::build(sides).unwrap();
   assert!(triangle.is_equilateral());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
fn larger_equilateral_triangles_have_equal_sides() {
   let sides = [10, 10, 10];
   let triangle = Triangle::build(sides).unwrap();
   assert!(triangle.is_equilateral());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
fn isosceles_triangles_have_two_equal_sides_one() {
   let sides = [3, 4, 4];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(triangle.is_isosceles());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
fn isosceles_triangles_have_two_equal_sides_two() {
   let sides = [4, 4, 3];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(triangle.is_isosceles());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
fn isosceles_triangles_have_two_equal_sides_three() {
   let sides = [4, 3, 4];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(triangle.is_isosceles());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
fn isosceles_triangles_have_two_equal_sides_four() {
   let sides = [4, 7, 4];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(triangle.is_isosceles());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
fn scalene_triangle_has_no_equal_sides_one() {
   let sides = [3, 4, 5];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(!triangle.is_isosceles());
   assert!(triangle.is_scalene());
}

#[test]
//#[ignore]
fn scalene_triangle_has_no_equal_sides_two() {
   let sides = [5, 4, 6];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(!triangle.is_isosceles());
   assert!(triangle.is_scalene());
}

#[test]
//#[ignore]
fn scalene_triangle_has_no_equal_sides_three() {
   let sides = [10, 11, 12];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(!triangle.is_isosceles());
   assert!(triangle.is_scalene());
}

#[test]
//#[ignore]
fn scalene_triangle_has_no_equal_sides_four() {
   let sides = [5, 4, 2];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(!triangle.is_isosceles());
   assert!(triangle.is_scalene());
}

#[test]
//#[ignore]
fn sum_of_two_sides_must_equal_or_exceed_the_remaining_side_one() {
   let sides = [7, 3, 2];
   let triangle = Triangle::build(sides);
   assert!(triangle.is_none());
}

#[test]
//#[ignore]
fn sum_of_two_sides_must_equal_or_exceed_the_remaining_side_two() {
   let sides = [1, 1, 3];
   let triangle = Triangle::build(sides);
   assert!(triangle.is_none());
}

#[test]
//#[ignore]
#[cfg(feature = "generic")]
fn scalene_triangle_with_floating_point_sides() {
   let sides = [0.4, 0.6, 0.3];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(!triangle.is_isosceles());
   assert!(triangle.is_scalene());
}

#[test]
//#[ignore]
#[cfg(feature = "generic")]
fn equilateral_triangles_with_floating_point_sides() {
   let sides = [0.2, 0.2, 0.2];
   let triangle = Triangle::build(sides).unwrap();
   assert!(triangle.is_equilateral());
   assert!(!triangle.is_scalene());
}

#[test]
//#[ignore]
#[cfg(feature = "generic")]
fn isosceles_triangle_with_floating_point_sides() {
   let sides = [0.3, 0.4, 0.4];
   let triangle = Triangle::build(sides).unwrap();
   assert!(!triangle.is_equilateral());
   assert!(triangle.is_isosceles());
   assert!(!triangle.is_scalene());
}

// 30
#[test]
fn test_one30() {
   assert_eq!("I", Roman::from(1).to_string());
}

#[test]
//#[ignore]
fn test_two() {
   assert_eq!("II", Roman::from(2).to_string());
}

#[test]
//#[ignore]
fn test_three() {
   assert_eq!("III", Roman::from(3).to_string());
}

#[test]
//#[ignore]
fn test_four() {
   assert_eq!("IV", Roman::from(4).to_string());
}

#[test]
//#[ignore]
fn test_five() {
   assert_eq!("V", Roman::from(5).to_string());
}

#[test]
//#[ignore]
fn test_six() {
   assert_eq!("VI", Roman::from(6).to_string());
}

#[test]
//#[ignore]
fn test_nine() {
   assert_eq!("IX", Roman::from(9).to_string());
}

#[test]
//#[ignore]
fn test_twenty_seven() {
   assert_eq!("XXVII", Roman::from(27).to_string());
}

#[test]
//#[ignore]
fn test_forty_eight() {
   assert_eq!("XLVIII", Roman::from(48).to_string());
}

#[test]
//#[ignore]
fn test_fifty_nine() {
   assert_eq!("LIX", Roman::from(59).to_string());
}

#[test]
//#[ignore]
fn test_ninety_three() {
   assert_eq!("XCIII", Roman::from(93).to_string());
}

#[test]
//#[ignore]
fn test_141() {
   assert_eq!("CXLI", Roman::from(141).to_string());
}

#[test]
//#[ignore]
fn test_163() {
   assert_eq!("CLXIII", Roman::from(163).to_string());
}

#[test]
//#[ignore]
fn test_402() {
   assert_eq!("CDII", Roman::from(402).to_string());
}

#[test]
//#[ignore]
fn test_575() {
   assert_eq!("DLXXV", Roman::from(575).to_string());
}

#[test]
//#[ignore]
fn test_911() {
   assert_eq!("CMXI", Roman::from(911).to_string());
}

#[test]
//#[ignore]
fn test_1024() {
   assert_eq!("MXXIV", Roman::from(1024).to_string());
}

#[test]
//#[ignore]
fn test_3000() {
   assert_eq!("MMM", Roman::from(3000).to_string());
}

// 31
#[test]
fn single_bit_one_to_decimal() {
   let input_base = 2;
   let input_digits = &[1];
   let output_base = 10;
   let output_digits = vec![1];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn binary_to_single_decimal() {
   let input_base = 2;
   let input_digits = &[1, 0, 1];
   let output_base = 10;
   let output_digits = vec![5];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn single_decimal_to_binary() {
   let input_base = 10;
   let input_digits = &[5];
   let output_base = 2;
   let output_digits = vec![1, 0, 1];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn binary_to_multiple_decimal() {
   let input_base = 2;
   let input_digits = &[1, 0, 1, 0, 1, 0];
   let output_base = 10;
   let output_digits = vec![4, 2];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn decimal_to_binary() {
   let input_base = 10;
   let input_digits = &[4, 2];
   let output_base = 2;
   let output_digits = vec![1, 0, 1, 0, 1, 0];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn trinary_to_hexadecimal() {
   let input_base = 3;
   let input_digits = &[1, 1, 2, 0];
   let output_base = 16;
   let output_digits = vec![2, 10];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn hexadecimal_to_trinary() {
   let input_base = 16;
   let input_digits = &[2, 10];
   let output_base = 3;
   let output_digits = vec![1, 1, 2, 0];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn fifteen_bit_integer() {
   let input_base = 97;
   let input_digits = &[3, 46, 60];
   let output_base = 73;
   let output_digits = vec![6, 10, 45];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn empty_list() {
   let input_base = 2;
   let input_digits = &[];
   let output_base = 10;
   let output_digits = vec![];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn single_zero() {
   let input_base = 10;
   let input_digits = &[0];
   let output_base = 2;
   let output_digits = vec![];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn multiple_zeros() {
   let input_base = 10;
   let input_digits = &[0, 0, 0];
   let output_base = 2;
   let output_digits = vec![];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn leading_zeros() {
   let input_base = 7;
   let input_digits = &[0, 6, 0];
   let output_base = 10;
   let output_digits = vec![4, 2];
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Ok(output_digits)
   );
}

#[test]
//#[ignore]
fn invalid_positive_digit() {
   let input_base = 2;
   let input_digits = &[1, 2, 1, 0, 1, 0];
   let output_base = 10;
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Err(Error31::InvalidDigit(2))
   );
}

#[test]
//#[ignore]
fn input_base_is_one() {
   let input_base = 1;
   let input_digits = &[];
   let output_base = 10;
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Err(Error31::InvalidInputBase)
   );
}

#[test]
//#[ignore]
fn output_base_is_one() {
   let input_base = 2;
   let input_digits = &[1, 0, 1, 0, 1, 0];
   let output_base = 1;
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Err(Error31::InvalidOutputBase)
   );
}

#[test]
//#[ignore]
fn input_base_is_zero() {
   let input_base = 0;
   let input_digits = &[];
   let output_base = 10;
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Err(Error31::InvalidInputBase)
   );
}

#[test]
//#[ignore]
fn output_base_is_zero() {
   let input_base = 10;
   let input_digits = &[7];
   let output_base = 0;
   assert_eq!(
       convert(input_digits, input_base, output_base),
       Err(Error31::InvalidOutputBase)
   );
}

// 32
fn some_strings(v: &[&str]) -> Option<Vec<String>> {
    Some(v.iter().map(|s| s.to_string()).collect())
 }
 
 #[test]
 fn test_grades_for_empty_school() {
    let s = School::new();
    assert_eq!(s.grades(), vec![]);
 }
 
 #[test]
 //#[ignore]
 fn test_grades_for_one_student() {
    let mut s = School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grades(), vec![2]);
 }
 
 #[test]
 //#[ignore]
 fn test_grades_for_several_students_are_sorted() {
    let mut s = School::new();
    s.add(2, "Aimee");
    s.add(7, "Logan");
    s.add(4, "Blair");
    assert_eq!(s.grades(), vec![2, 4, 7]);
 }
 
 #[test]
 //#[ignore]
 fn test_grades_when_several_students_have_the_same_grade() {
    let mut s = School::new();
    s.add(2, "Aimee");
    s.add(2, "Logan");
    s.add(2, "Blair");
    assert_eq!(s.grades(), vec![2]);
 }
 
 #[test]
 //#[ignore]
 fn test_grade_for_empty_school() {
    let s = School::new();
    assert_eq!(s.grade(1), None);
 }
 
 #[test]
 //#[ignore]
 fn test_grade_when_no_students_have_that_grade() {
    let mut s = School::new();
    s.add(7, "Logan");
    assert_eq!(s.grade(1), None);
 }
 
 #[test]
 //#[ignore]
 fn test_grade_for_one_student() {
    let mut s = School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grade(2), some_strings(&["Aimee"]));
 }
 
 #[test]
 //#[ignore]
 fn test_grade_returns_students_sorted_by_name() {
    let mut s = School::new();
    s.add(2, "James");
    s.add(2, "Blair");
    s.add(2, "Paul");
    assert_eq!(s.grade(2), some_strings(&["Blair", "James", "Paul"]));
 }
 
 #[test]
 //#[ignore]
 fn test_add_students_to_different_grades() {
    let mut s = School::new();
    s.add(3, "Chelsea");
    s.add(7, "Logan");
    assert_eq!(s.grades(), vec![3, 7]);
    assert_eq!(s.grade(3), some_strings(&["Chelsea"]));
    assert_eq!(s.grade(7), some_strings(&["Logan"]));
 }

 // 33
 #[test]
 fn finds_a_value_in_an_array_with_one_element() {
    assert_eq!(find(&[6], 6), Some(0));
 }
 
 #[test]
 //#[ignore]
 fn finds_first_value_in_an_array_with_two_element() {
    assert_eq!(find(&[1, 2], 1), Some(0));
 }
 
 #[test]
 //#[ignore]
 fn finds_second_value_in_an_array_with_two_element() {
    assert_eq!(find(&[1, 2], 2), Some(1));
 }
 
 #[test]
 //#[ignore]
 fn finds_a_value_in_the_middle_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
 }
 
 #[test]
 //#[ignore]
 fn finds_a_value_at_the_beginning_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
 }
 
 #[test]
 //#[ignore]
 fn finds_a_value_at_the_end_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
 }
 
 #[test]
 //#[ignore]
 fn finds_a_value_in_an_array_of_odd_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
        Some(9)
    );
 }
 
 #[test]
 //#[ignore]
 fn finds_a_value_in_an_array_of_even_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
        Some(5)
    );
 }
 
 #[test]
 //#[ignore]
 fn identifies_that_a_value_is_not_included_in_the_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 7), None);
 }
 
 #[test]
 //#[ignore]
 fn a_value_smaller_than_the_arrays_smallest_value_is_not_included() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 0), None);
 }
 
 #[test]
 //#[ignore]
 fn a_value_larger_than_the_arrays_largest_value_is_not_included() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 13), None);
 }
 
 #[test]
 //#[ignore]
 fn nothing_is_included_in_an_empty_array() {
    assert_eq!(find(&[], 1), None);
 }
 
 #[test]
 //#[ignore]
 #[cfg(feature = "generic")]
 fn works_for_arrays() {
    assert_eq!(find([6], 6), Some(0));
 }
 
 #[test]
 //#[ignore]
 #[cfg(feature = "generic")]
 fn works_for_vec() {
    let vector = vec![6];
    assert_eq!(find(&vector, 6), Some(0));
    assert_eq!(find(vector, 6), Some(0));
 }
 
 #[test]
 //#[ignore]
 #[cfg(feature = "generic")]
 fn works_for_str_elements() {
    assert_eq!(find(["a"], "a"), Some(0));
    assert_eq!(find(["a", "b"], "b"), Some(1));
 }

// 34
#[test]
fn robots_are_created_with_position_and_direction() {
   let robot = Robot::new(0, 0, Direction::North);
   assert_eq!((0, 0), robot.position());
   assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn positions_can_be_negative() {
   let robot = Robot::new(-1, -1, Direction::South);
   assert_eq!((-1, -1), robot.position());
   assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_does_not_change_position() {
   let robot = Robot::new(0, 0, Direction::North).turn_right();
   assert_eq!((0, 0), robot.position());
}

#[test]
//#[ignore]
fn turning_right_from_north_points_the_robot_east() {
   let robot = Robot::new(0, 0, Direction::North).turn_right();
   assert_eq!(&Direction::East, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_from_east_points_the_robot_south() {
   let robot = Robot::new(0, 0, Direction::East).turn_right();
   assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_from_south_points_the_robot_west() {
   let robot = Robot::new(0, 0, Direction::South).turn_right();
   assert_eq!(&Direction::West, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_from_west_points_the_robot_north() {
   let robot = Robot::new(0, 0, Direction::West).turn_right();
   assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_does_not_change_position() {
   let robot = Robot::new(0, 0, Direction::North).turn_left();
   assert_eq!((0, 0), robot.position());
}

#[test]
//#[ignore]
fn turning_left_from_north_points_the_robot_west() {
   let robot = Robot::new(0, 0, Direction::North).turn_left();
   assert_eq!(&Direction::West, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_from_west_points_the_robot_south() {
   let robot = Robot::new(0, 0, Direction::West).turn_left();
   assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_from_south_points_the_robot_east() {
   let robot = Robot::new(0, 0, Direction::South).turn_left();
   assert_eq!(&Direction::East, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_from_east_points_the_robot_north() {
   let robot = Robot::new(0, 0, Direction::East).turn_left();
   assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn advance_does_not_change_the_direction() {
   let robot = Robot::new(0, 0, Direction::North).advance();
   assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn advance_increases_the_y_coordinate_by_one_when_facing_north() {
   let robot = Robot::new(0, 0, Direction::North).advance();
   assert_eq!((0, 1), robot.position());
}

#[test]
//#[ignore]
fn advance_decreases_the_y_coordinate_by_one_when_facing_south() {
   let robot = Robot::new(0, 0, Direction::South).advance();
   assert_eq!((0, -1), robot.position());
}

#[test]
//#[ignore]
fn advance_increases_the_x_coordinate_by_one_when_facing_east() {
   let robot = Robot::new(0, 0, Direction::East).advance();
   assert_eq!((1, 0), robot.position());
}

#[test]
//#[ignore]
fn advance_decreases_the_x_coordinate_by_one_when_facing_west() {
   let robot = Robot::new(0, 0, Direction::West).advance();
   assert_eq!((-1, 0), robot.position());
}

#[test]
//#[ignore]
fn follow_instructions_to_move_west_and_north() {
   let robot = Robot::new(0, 0, Direction::North).instructions("LAAARALA");
   assert_eq!((-4, 1), robot.position());
   assert_eq!(&Direction::West, robot.direction());
}

#[test]
//#[ignore]
fn follow_instructions_to_move_west_and_south() {
   let robot = Robot::new(2, -7, Direction::East).instructions("RRAAAAALA");
   assert_eq!((-3, -8), robot.position());
   assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn follow_instructions_to_move_east_and_north() {
   let robot = Robot::new(8, 4, Direction::South).instructions("LAAARRRALLLL");
   assert_eq!((11, 5), robot.position());
   assert_eq!(&Direction::North, robot.direction());
}

// 35
#[test]
fn paired_square_brackets() {
   assert!(brackets_are_balanced("[]"));
}

#[test]
//#[ignore]
fn empty_string35() {
   assert!(brackets_are_balanced(""));
}

#[test]
//#[ignore]
fn unpaired_brackets() {
   assert!(!brackets_are_balanced("[["));
}

#[test]
//#[ignore]
fn wrong_ordered_brackets() {
   assert!(!brackets_are_balanced("}{"));
}

#[test]
//#[ignore]
fn wrong_closing_bracket() {
   assert!(!brackets_are_balanced("{]"));
}

#[test]
//#[ignore]
fn paired_with_whitespace() {
   assert!(brackets_are_balanced("{ }"));
}

#[test]
//#[ignore]
fn simple_nested_brackets() {
   assert!(brackets_are_balanced("{[]}"));
}

#[test]
//#[ignore]
fn several_paired_brackets() {
   assert!(brackets_are_balanced("{}[]"));
}

#[test]
//#[ignore]
fn paired_and_nested_brackets() {
   assert!(brackets_are_balanced("([{}({}[])])"));
}

#[test]
//#[ignore]
fn unopened_closing_brackets() {
   assert!(!brackets_are_balanced("{[)][]}"));
}

#[test]
//#[ignore]
fn unpaired_and_nested_brackets() {
   assert!(!brackets_are_balanced("([{])"));
}

#[test]
//#[ignore]
fn paired_and_wrong_nested_brackets() {
   assert!(!brackets_are_balanced("[({]})"));
}

#[test]
//#[ignore]
fn math_expression() {
   assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}

#[test]
//#[ignore]
fn complex_latex_expression() {
   let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                \\end{array}\\right)";
   assert!(brackets_are_balanced(input));
}

// 37
#[test]
fn chess_position_on_the_board_is_some() {
   assert!(ChessPosition::new(2, 4).is_some());
}

#[test]
//#[ignore]
fn chess_position_off_the_board_is_none() {
   assert!(ChessPosition::new(-1, 2).is_none());

   assert!(ChessPosition::new(8, 2).is_none());

   assert!(ChessPosition::new(5, -1).is_none());

   assert!(ChessPosition::new(5, 8).is_none());
}

#[test]
//#[ignore]
fn queen_is_created_with_a_valid_position() {
   Queen::new(ChessPosition::new(2, 4).unwrap());
}

#[test]
//#[ignore]
fn queens_that_can_not_attack() {
   let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
   let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());

   assert!(!white_queen.can_attack(&black_queen));
}

#[test]
//#[ignore]
fn queens_on_the_same_rank_can_attack() {
   let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
   let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());

   assert!(white_queen.can_attack(&black_queen));
}

#[test]
//#[ignore]
fn queens_on_the_same_file_can_attack() {
   let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
   let black_queen = Queen::new(ChessPosition::new(3, 5).unwrap());

   assert!(white_queen.can_attack(&black_queen));
}

#[test]
//#[ignore]
fn queens_on_the_same_diagonal_can_attack_one() {
   let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
   let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

   assert!(white_queen.can_attack(&black_queen));
}

#[test]
//#[ignore]
fn queens_on_the_same_diagonal_can_attack_two() {
   let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
   let black_queen = Queen::new(ChessPosition::new(3, 1).unwrap());

   assert!(white_queen.can_attack(&black_queen));
}

#[test]
//#[ignore]
fn queens_on_the_same_diagonal_can_attack_three() {
   let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
   let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());

   assert!(white_queen.can_attack(&black_queen));
}

#[test]
//#[ignore]
fn queens_on_the_same_diagonal_can_attack_four() {
   let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
   let black_queen = Queen::new(ChessPosition::new(5, 5).unwrap());

   assert!(white_queen.can_attack(&black_queen));
}

// 39
#[test]
fn empty_equals_empty() {
   let v: &[u32] = &[];

   assert_eq!(Comparison::Equal, sublist(&v, &v));
}

#[test]
//#[ignore]
fn test_empty_is_a_sublist_of_anything() {
   assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
}

#[test]
//#[ignore]
fn test_anything_is_a_superlist_of_empty() {
   assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
}

#[test]
//#[ignore]
fn test_1_is_not_2() {
   assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
}

#[test]
//#[ignore]
fn test_compare_larger_equal_lists() {
   use std::iter::repeat;

   let v: Vec<char> = repeat('x').take(1000).collect();

   assert_eq!(Comparison::Equal, sublist(&v, &v));
}

#[test]
//#[ignore]
fn test_sublist_at_start() {
   assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
}

#[test]
//#[ignore]
fn sublist_in_middle() {
   assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
}

#[test]
//#[ignore]
fn sublist_at_end() {
   assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
}

#[test]
//#[ignore]
fn partially_matching_sublist_at_start() {
   assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
}

#[test]
//#[ignore]
fn sublist_early_in_huge_list() {
   let huge: Vec<u32> = (1..1000000).collect();

   assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
}

#[test]
//#[ignore]
fn huge_sublist_not_in_huge_list() {
   let v1: Vec<u64> = (10..1000001).collect();
   let v2: Vec<u64> = (1..1000000).collect();

   assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
}

#[test]
//#[ignore]
fn superlist_at_start() {
   assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
}

#[test]
//#[ignore]
fn superlist_in_middle() {
   assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
}

#[test]
//#[ignore]
fn superlist_at_end() {
   assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]));
}

#[test]
//#[ignore]
fn superlist_early_in_huge_list() {
   let huge: Vec<u32> = (1..1000000).collect();

   assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
}

#[test]
//#[ignore]
fn recurring_values_sublist() {
   assert_eq!(
       Comparison::Sublist,
       sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
   );
}

#[test]
//#[ignore]
fn recurring_values_unequal() {
   assert_eq!(
       Comparison::Unequal,
       sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
   );
}

// 40
fn assert_in_delta(expected: f64, actual: f64) {
   let diff: f64 = (expected - actual).abs();
   let delta: f64 = 0.01;
   if diff > delta {
       panic!(
           "Your result of {} should be within {} of the expected result {}",
           actual, delta, expected
       )
   }
}

#[test]
fn earth_age() {
   let duration = Duration::from(1_000_000_000);
   assert_in_delta(31.69, Earth::years_during(&duration));
}

#[test]
//#[ignore]
fn mercury_age() {
   let duration = Duration::from(2_134_835_688);
   assert_in_delta(280.88, Mercury::years_during(&duration));
}

#[test]
//#[ignore]
fn venus_age() {
   let duration = Duration::from(189_839_836);
   assert_in_delta(9.78, Venus::years_during(&duration));
}

#[test]
//#[ignore]
fn mars_age() {
   let duration = Duration::from(2_329_871_239);
   assert_in_delta(39.25, Mars::years_during(&duration));
}

#[test]
//#[ignore]
fn jupiter_age() {
   let duration = Duration::from(901_876_382);
   assert_in_delta(2.41, Jupiter::years_during(&duration));
}

#[test]
//#[ignore]
fn saturn_age() {
   let duration = Duration::from(3_000_000_000);
   assert_in_delta(3.23, Saturn::years_during(&duration));
}

#[test]
//#[ignore]
fn uranus_age() {
   let duration = Duration::from(3_210_123_456);
   assert_in_delta(1.21, Uranus::years_during(&duration));
}

#[test]
//#[ignore]
fn neptune_age() {
   let duration = Duration::from(8_210_123_456);
   assert_in_delta(1.58, Neptune::years_during(&duration));
}

// 42

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

#[test]
fn test_empty() {
   let expected: HashMap<u32, u32> = HashMap::new();
   let computed: HashMap<u32, u32> = hashmap!();
   assert_eq!(computed, expected);
}

#[test]
//#[ignore]
fn test_no_trailing_comma() {
   let mut expected = HashMap::new();
   expected.insert(1, "one");
   expected.insert(2, "two");
   assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
}

#[test]
//#[ignore]
fn test_trailing_comma() {
   let mut expected = HashMap::new();
   expected.insert('h', 89);
   expected.insert('a', 1);
   expected.insert('s', 19);
   expected.insert('h', 8);
   assert_eq!(
       hashmap!(
           'h' => 89,
           'a' => 1,
           's' => 19,
           'h' => 8,
       ),
       expected
   );
}

#[test]
//#[ignore]
fn test_nested() {
   let mut expected = HashMap::new();
   expected.insert("non-empty", {
       let mut subhashmap = HashMap::new();
       subhashmap.insert(23, 623);
       subhashmap.insert(34, 21);
       subhashmap
   });
   expected.insert("empty", HashMap::new());
   assert_eq!(
       hashmap!(
           "non-empty" => hashmap!(
               23 => 623,
               34 => 21
           ),
           "empty" => hashmap!()
       ),
       expected
   );
}

mod test {
   #[test]
   //#[ignore]
   fn type_not_in_scope() {
       let _expected: ::std::collections::HashMap<(), ()> = hashmap!();
   }
}

// 43
fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
   for element in expected {
       if !actual.contains(element) {
           panic!(
               "Allergen missing\n  {:?} should be in {:?}",
               element, actual
           );
       }
   }

   if actual.len() != expected.len() {
       panic!(
           "Allergy vectors are of different lengths\n  expected {:?}\n  got {:?}",
           expected, actual
       );
   }
}

#[test]
fn is_not_allergic_to_anything() {
   let allergies = Allergies::new(0);
   assert!(!allergies.is_allergic_to(&Allergen::Peanuts));
   assert!(!allergies.is_allergic_to(&Allergen::Cats));
   assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
//#[ignore]
fn is_allergic_to_eggs() {
   assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
}

#[test]
//#[ignore]
fn is_allergic_to_egg_shellfish_and_strawberries() {
   let allergies = Allergies::new(5);
   assert!(allergies.is_allergic_to(&Allergen::Eggs));
   assert!(allergies.is_allergic_to(&Allergen::Shellfish));
   assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
//#[ignore]
fn no_allergies_at_all() {
   let expected = &[];
   let allergies = Allergies::new(0).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_just_eggs() {
   let expected = &[Allergen::Eggs];
   let allergies = Allergies::new(1).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_just_peanuts() {
   let expected = &[Allergen::Peanuts];
   let allergies = Allergies::new(2).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_just_strawberries() {
   let expected = &[Allergen::Strawberries];
   let allergies = Allergies::new(8).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_eggs_and_peanuts() {
   let expected = &[Allergen::Eggs, Allergen::Peanuts];
   let allergies = Allergies::new(3).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_eggs_and_shellfish() {
   let expected = &[Allergen::Eggs, Allergen::Shellfish];
   let allergies = Allergies::new(5).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_many_things() {
   let expected = &[
       Allergen::Strawberries,
       Allergen::Tomatoes,
       Allergen::Chocolate,
       Allergen::Pollen,
       Allergen::Cats,
   ];
   let allergies = Allergies::new(248).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn allergic_to_everything() {
   let expected = &[
       Allergen::Eggs,
       Allergen::Peanuts,
       Allergen::Shellfish,
       Allergen::Strawberries,
       Allergen::Tomatoes,
       Allergen::Chocolate,
       Allergen::Pollen,
       Allergen::Cats,
   ];
   let allergies = Allergies::new(255).allergies();

   compare_allergy_vectors(expected, &allergies);
}

#[test]
//#[ignore]
fn scores_over_255_do_not_trigger_false_positives() {
   let expected = &[
       Allergen::Eggs,
       Allergen::Shellfish,
       Allergen::Strawberries,
       Allergen::Tomatoes,
       Allergen::Chocolate,
       Allergen::Pollen,
       Allergen::Cats,
   ];
   let allergies = Allergies::new(509).allergies();

   compare_allergy_vectors(expected, &allergies);
}

// 44
#[test]
fn to_single_byte() {
   assert_eq!(&[0x00], to_bytes(&[0x00]).as_slice());
   assert_eq!(&[0x40], to_bytes(&[0x40]).as_slice());
   assert_eq!(&[0x7f], to_bytes(&[0x7f]).as_slice());
}

#[test]
//#[ignore]
fn to_double_byte() {
   assert_eq!(&[0x81, 0x00], to_bytes(&[0x80]).as_slice());
   assert_eq!(&[0xc0, 0x00], to_bytes(&[0x2000]).as_slice());
   assert_eq!(&[0xff, 0x7f], to_bytes(&[0x3fff]).as_slice());
}

#[test]
//#[ignore]
fn to_triple_byte() {
   assert_eq!(&[0x81, 0x80, 0x00], to_bytes(&[0x4000]).as_slice());
   assert_eq!(&[0xc0, 0x80, 0x00], to_bytes(&[0x10_0000]).as_slice());
   assert_eq!(&[0xff, 0xff, 0x7f], to_bytes(&[0x1f_ffff]).as_slice());
}

#[test]
//#[ignore]
fn to_quadruple_byte() {
   assert_eq!(&[0x81, 0x80, 0x80, 0x00], to_bytes(&[0x20_0000]).as_slice());
   assert_eq!(
       &[0xc0, 0x80, 0x80, 0x00],
       to_bytes(&[0x0800_0000]).as_slice()
   );
   assert_eq!(
       &[0xff, 0xff, 0xff, 0x7f],
       to_bytes(&[0x0fff_ffff]).as_slice()
   );
}

#[test]
//#[ignore]
fn to_quintuple_byte() {
   assert_eq!(
       &[0x81, 0x80, 0x80, 0x80, 0x00],
       to_bytes(&[0x1000_0000]).as_slice()
   );
   assert_eq!(
       &[0x8f, 0xf8, 0x80, 0x80, 0x00],
       to_bytes(&[0xff00_0000]).as_slice()
   );
   assert_eq!(
       &[0x8f, 0xff, 0xff, 0xff, 0x7f],
       to_bytes(&[0xffff_ffff]).as_slice()
   );
}

#[test]
//#[ignore]
fn from_bytes_test() {
   assert_eq!(Ok(vec![0x7f]), from_bytes(&[0x7f]));
   assert_eq!(Ok(vec![0x2000]), from_bytes(&[0xc0, 0x00]));
   assert_eq!(Ok(vec![0x1f_ffff]), from_bytes(&[0xff, 0xff, 0x7f]));
   assert_eq!(Ok(vec![0x20_0000]), from_bytes(&[0x81, 0x80, 0x80, 0x00]));
   assert_eq!(
       Ok(vec![0xffff_ffff]),
       from_bytes(&[0x8f, 0xff, 0xff, 0xff, 0x7f])
   );
}

#[test]
//#[ignore]
fn to_bytes_multiple_values() {
   assert_eq!(&[0x40, 0x7f], to_bytes(&[0x40, 0x7f]).as_slice());
   assert_eq!(
       &[0x81, 0x80, 0x00, 0xc8, 0xe8, 0x56],
       to_bytes(&[0x4000, 0x12_3456]).as_slice()
   );
   assert_eq!(
       &[
           0xc0, 0x00, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x00, 0xff, 0x7f, 0x81, 0x80,
           0x00
       ],
       to_bytes(&[0x2000, 0x12_3456, 0x0fff_ffff, 0x00, 0x3fff, 0x4000]).as_slice()
   );
}

#[test]
//#[ignore]
fn from_bytes_multiple_values() {
   assert_eq!(
       Ok(vec![0x2000, 0x12_3456, 0x0fff_ffff, 0x00, 0x3fff, 0x4000]),
       from_bytes(&[
           0xc0, 0x00, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x00, 0xff, 0x7f, 0x81, 0x80,
           0x00,
       ])
   );
}

#[test]
//#[ignore]
fn incomplete_byte_sequence() {
   assert_eq!(Err(Error44::IncompleteNumber), from_bytes(&[0xff]));
}

#[test]
//#[ignore]
fn zero_incomplete_byte_sequence() {
   assert_eq!(Err(Error44::IncompleteNumber), from_bytes(&[0x80]));
}

#[test]
//#[ignore]
fn overflow_u32() {
   assert_eq!(
       Err(Error44::Overflow),
       from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x7f])
   );
}

#[test]
//#[ignore]
fn chained_execution_is_identity() {
   let test = &[0xf2, 0xf6, 0x96, 0x9c, 0x3b, 0x39, 0x2e, 0x30, 0xb3, 0x24];
   assert_eq!(Ok(Vec::from(&test[..])), from_bytes(&to_bytes(test)));
}

// 45
fn to_some_string(s: &str) -> Option<String> {
   Some(s.to_string())
}

#[test]
fn test_cleans_the_number() {
   assert_eq!(number("(223) 456-7890"), to_some_string("2234567890"));
}

#[test]
//#[ignore]
fn test_cleans_numbers_with_dots() {
   assert_eq!(number("223.456.7890"), to_some_string("2234567890"));
}

#[test]
//#[ignore]
fn test_cleans_numbers_with_multiple_spaces() {
   assert_eq!(number("223 456   7890   "), to_some_string("2234567890"));
}

#[test]
//#[ignore]
fn test_invalid_when_9_digits() {
   assert_eq!(number("123456789"), None);
}

#[test]
//#[ignore]
fn test_invalid_when_11_digits_does_not_start_with_a_1() {
   assert_eq!(number("22234567890"), None);
}

#[test]
//#[ignore]
fn test_valid_when_11_digits_and_starting_with_1() {
   assert_eq!(number("12234567890"), to_some_string("2234567890"));
}

#[test]
//#[ignore]
fn test_valid_when_11_digits_and_starting_with_1_even_with_punctuation() {
   assert_eq!(number("+1 (223) 456-7890"), to_some_string("2234567890"));
}

#[test]
//#[ignore]
fn test_invalid_when_more_than_11_digits() {
   assert_eq!(number("321234567890"), None);
}

#[test]
//#[ignore]
fn test_invalid_with_letters() {
   assert_eq!(number("123-abc-7890"), None);
}

#[test]
//#[ignore]
fn test_invalid_with_punctuations() {
   assert_eq!(number("123-@:!-7890"), None);
}

#[test]
//#[ignore]
fn test_invalid_if_area_code_does_not_start_with_2_9() {
   assert_eq!(number("(123) 456-7890"), None);
}

#[test]
//#[ignore]
fn test_invalid_if_exchange_code_does_not_start_with_2_9() {
   assert_eq!(number("(223) 056-7890"), None);
}

// 46
#[test]
fn addition() {
   let command = "What is 1 plus 1?";
   assert_eq!(Some(2), answer(command));
}

#[test]
//#[ignore]
fn more_addition() {
   let command = "What is 53 plus 2?";
   assert_eq!(Some(55), answer(command));
}

#[test]
//#[ignore]
fn addition_with_negative_numbers() {
   let command = "What is -1 plus -10?";
   assert_eq!(Some(-11), answer(command));
}

#[test]
//#[ignore]
fn large_addition() {
   let command = "What is 123 plus 45678?";
   assert_eq!(Some(45801), answer(command));
}

#[test]
//#[ignore]
fn subtraction() {
   let command = "What is 4 minus -12?";
   assert_eq!(Some(16), answer(command));
}

#[test]
//#[ignore]
fn multiplication() {
   let command = "What is -3 multiplied by 25?";
   assert_eq!(Some(-75), answer(command));
}

#[test]
//#[ignore]
fn division() {
   let command = "What is 33 divided by -3?";
   assert_eq!(Some(-11), answer(command));
}

#[test]
//#[ignore]
fn multiple_additions() {
   let command = "What is 1 plus 1 plus 1?";
   assert_eq!(Some(3), answer(command));
}

#[test]
//#[ignore]
fn addition_and_subtraction() {
   let command = "What is 1 plus 5 minus -2?";
   assert_eq!(Some(8), answer(command));
}

#[test]
//#[ignore]
fn multiple_subtraction() {
   let command = "What is 20 minus 4 minus 13?";
   assert_eq!(Some(3), answer(command));
}

#[test]
//#[ignore]
fn subtraction_then_addition() {
   let command = "What is 17 minus 6 plus 3?";
   assert_eq!(Some(14), answer(command));
}

#[test]
//#[ignore]
fn multiple_multiplications() {
   let command = "What is 2 multiplied by -2 multiplied by 3?";
   assert_eq!(Some(-12), answer(command));
}

#[test]
//#[ignore]
fn addition_and_multiplication() {
   let command = "What is -3 plus 7 multiplied by -2?";
   assert_eq!(Some(-8), answer(command));
}

#[test]
//#[ignore]
fn multiple_divisions() {
   let command = "What is -12 divided by 2 divided by -3?";
   assert_eq!(Some(2), answer(command));
}

#[test]
//#[ignore]
fn unknown_operation() {
   let command = "What is 52 cubed?";
   assert!(answer(command).is_none());
}

#[test]
//#[ignore]
fn non_math_question() {
   let command = "Who is the President of the United States?";
   assert!(answer(command).is_none());
}

// 47
#[test]
fn just_the_header_if_no_input() {
   let input = "";
   let expected = "Team                           | MP |  W |  D |  L |  P";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn a_win_is_three_points_a_loss_is_zero_points() {
   let input = "Allegoric Alaskans;Blithering Badgers;win";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
       + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn a_win_can_also_be_expressed_as_a_loss() {
   let input = "Blithering Badgers;Allegoric Alaskans;loss";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
       + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn a_different_team_can_win() {
   let input = "Blithering Badgers;Allegoric Alaskans;win";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Blithering Badgers             |  1 |  1 |  0 |  0 |  3\n"
       + "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn a_draw_is_one_point_each() {
   let input = "Allegoric Alaskans;Blithering Badgers;draw";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  1 |  0 |  1 |  0 |  1\n"
       + "Blithering Badgers             |  1 |  0 |  1 |  0 |  1";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn there_can_be_more_than_one_match() {
   let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
       + "Allegoric Alaskans;Blithering Badgers;win";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
       + "Blithering Badgers             |  2 |  0 |  0 |  2 |  0";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn there_can_be_more_than_one_winner() {
   let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
       + "Allegoric Alaskans;Blithering Badgers;win";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3\n"
       + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn there_can_be_more_than_two_teams() {
   let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
       + "Blithering Badgers;Courageous Californians;win\n"
       + "Courageous Californians;Allegoric Alaskans;loss";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
       + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3\n"
       + "Courageous Californians        |  2 |  0 |  0 |  2 |  0";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn typical_input() {
   let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
       + "Devastating Donkeys;Courageous Californians;draw\n"
       + "Devastating Donkeys;Allegoric Alaskans;win\n"
       + "Courageous Californians;Blithering Badgers;loss\n"
       + "Blithering Badgers;Devastating Donkeys;loss\n"
       + "Allegoric Alaskans;Courageous Californians;win";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n"
       + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
       + "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n"
       + "Courageous Californians        |  3 |  0 |  1 |  2 |  1";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn incomplete_competition_not_all_pairs_have_played() {
   let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
       + "Devastating Donkeys;Allegoric Alaskans;loss\n"
       + "Courageous Californians;Blithering Badgers;draw\n"
       + "Allegoric Alaskans;Courageous Californians;win";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
       + "Blithering Badgers             |  2 |  1 |  1 |  0 |  4\n"
       + "Courageous Californians        |  2 |  0 |  1 |  1 |  1\n"
       + "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0";

   assert_eq!(tally(&input), expected);
}

#[test]
//#[ignore]
fn ties_broken_alphabetically() {
   let input = "Courageous Californians;Devastating Donkeys;win\n".to_string()
       + "Allegoric Alaskans;Blithering Badgers;win\n"
       + "Devastating Donkeys;Allegoric Alaskans;loss\n"
       + "Courageous Californians;Blithering Badgers;win\n"
       + "Blithering Badgers;Devastating Donkeys;draw\n"
       + "Allegoric Alaskans;Courageous Californians;draw";
   let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string()
       + "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7\n"
       + "Courageous Californians        |  3 |  2 |  1 |  0 |  7\n"
       + "Blithering Badgers             |  3 |  0 |  1 |  2 |  1\n"
       + "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1";

   assert_eq!(tally(&input), expected);
}