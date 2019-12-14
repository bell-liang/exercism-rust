mod simple;
use simple::*;

// 1
#[test]
fn test_hello_world() {
	assert_eq!("Hello, World!", hello());
}

// 2
use chrono::{Utc, TimeZone};

#[test]
fn test_date() {
   let start_date = Utc.ymd(2011, 4, 25).and_hms(0, 0, 0);

   assert_eq!(after(start_date), Utc.ymd(2043, 1, 1).and_hms(1, 46, 40));
}

#[test]
//#[ignore]
fn test_another_date() {
   let start_date = Utc.ymd(1977, 6, 13).and_hms(0, 0, 0);

   assert_eq!(after(start_date), Utc.ymd(2009, 2, 19).and_hms(1, 46, 40));
}

#[test]
//#[ignore]
fn test_third_date() {
   let start_date = Utc.ymd(1959, 7, 19).and_hms(0, 0, 0);

   assert_eq!(after(start_date), Utc.ymd(1991, 3, 27).and_hms(1, 46, 40));
}

#[test]
//#[ignore]
fn test_datetime() {
   let start_date = Utc.ymd(2015, 1, 24).and_hms(22, 0, 0);

   assert_eq!(after(start_date), Utc.ymd(2046, 10, 2).and_hms(23, 46, 40));
}

#[test]
//#[ignore]
fn test_another_datetime() {
   let start_date = Utc.ymd(2015, 1, 24).and_hms(23, 59, 59);

   assert_eq!(after(start_date), Utc.ymd(2046, 10, 3).and_hms(1, 46, 39));
}

// 3
#[test]
fn test_vanilla_leap_year() {
   assert_eq!(is_leap_year(1996), true);
}

#[test]
//#[ignore]
fn test_any_old_year() {
   assert_eq!(is_leap_year(1997), false);
}

#[test]
//#[ignore]
fn test_century() {
   assert_eq!(is_leap_year(1700), false);
   assert_eq!(is_leap_year(1800), false);
   assert_eq!(is_leap_year(1900), false);
}

#[test]
//#[ignore]
fn test_exceptional_centuries() {
   assert_eq!(is_leap_year(1600), true);
   assert_eq!(is_leap_year(2000), true);
   assert_eq!(is_leap_year(2400), true);
}

#[test]
//#[ignore]
fn test_years_1600_to_1699() {
   let incorrect_years = (1600..1700)
       .filter(|&year| is_leap_year(year) != (year % 4 == 0))
       .collect::<Vec<_>>();

   if !incorrect_years.is_empty() {
       panic!("incorrect result for years: {:?}", incorrect_years);
   }
}

// 4
#[test]
fn test_1() {
   assert_eq!("1", raindrops(1));
}

#[test]
//#[ignore]
fn test_3() {
   assert_eq!("Pling", raindrops(3));
}

#[test]
//#[ignore]
fn test_5() {
   assert_eq!("Plang", raindrops(5));
}

#[test]
//#[ignore]
fn test_7() {
   assert_eq!("Plong", raindrops(7));
}

#[test]
//#[ignore]
fn test_6() {
   assert_eq!("Pling", raindrops(6));
}

#[test]
//#[ignore]
fn test_8() {
   assert_eq!("8", raindrops(8));
}

#[test]
//#[ignore]
fn test_9() {
   assert_eq!("Pling", raindrops(9));
}

#[test]
//#[ignore]
fn test_10() {
   assert_eq!("Plang", raindrops(10));
}

#[test]
//#[ignore]
fn test_14() {
   assert_eq!("Plong", raindrops(14));
}

#[test]
//#[ignore]
fn test_15() {
   assert_eq!("PlingPlang", raindrops(15));
}

#[test]
//#[ignore]
fn test_21() {
   assert_eq!("PlingPlong", raindrops(21));
}

#[test]
//#[ignore]
fn test_25() {
   assert_eq!("Plang", raindrops(25));
}

#[test]
//#[ignore]
fn test_27() {
   assert_eq!("Pling", raindrops(27));
}

#[test]
//#[ignore]
fn test_35() {
   assert_eq!("PlangPlong", raindrops(35));
}

#[test]
//#[ignore]
fn test_49() {
   assert_eq!("Plong", raindrops(49));
}

#[test]
//#[ignore]
fn test_52() {
   assert_eq!("52", raindrops(52));
}

#[test]
//#[ignore]
fn test_105() {
   assert_eq!("PlingPlangPlong", raindrops(105));
}

#[test]
//#[ignore]
fn test_3125() {
   assert_eq!("Plang", raindrops(3125));
}

#[test]
//#[ignore]
fn test_12121() {
   assert_eq!("12121", raindrops(12121));
}

// 4
fn process_reverse_case(input: &str, expected: &str) {
    assert_eq!(&reverse(input), expected)
 }
 
 #[test]
 /// empty string
 fn test_empty_string() {
    process_reverse_case("", "");
 }
 
 #[test]
 //#[ignore]
 /// a word
 fn test_a_word() {
    process_reverse_case("robot", "tobor");
 }
 
 #[test]
 //#[ignore]
 /// a capitalized word
 fn test_a_capitalized_word() {
    process_reverse_case("Ramen", "nemaR");
 }
 
 #[test]
 //#[ignore]
 /// a sentence with punctuation
 fn test_a_sentence_with_punctuation() {
    process_reverse_case("I'm hungry!", "!yrgnuh m'I");
 }
 
 #[test]
 //#[ignore]
 /// a palindrome
 fn test_a_palindrome() {
    process_reverse_case("racecar", "racecar");
 }
 
 #[test]
 //#[ignore]
 /// wide characters
 fn test_wide_characters() {
    process_reverse_case("子猫", "猫子");
 }
 
 #[test]
 //#[ignore]
 #[cfg(feature = "grapheme")]
 /// grapheme clusters
 fn test_grapheme_clusters() {
    process_reverse_case("uüu", "uüu");
 }

 // 6
 #[test]
fn test_first_prime() {
   assert_eq!(nth(0), 2);
}

#[test]
//#[ignore]
fn test_second_prime() {
   assert_eq!(nth(1), 3);
}

#[test]
//#[ignore]
fn test_sixth_prime() {
   assert_eq!(nth(5), 13);
}

#[test]
//#[ignore]
fn test_big_prime() {
   assert_eq!(nth(100), 104743);
}

// 7
#[test]
fn test_stating_something() {
   assert_eq!("Whatever.", reply("Tom-ay-to, tom-aaaah-to."));
}

#[test]
//#[ignore]
fn test_shouting() {
   assert_eq!("Whoa, chill out!", reply("WATCH OUT!"));
}

#[test]
//#[ignore]
fn test_shouting_gibberish() {
   assert_eq!("Whoa, chill out!", reply("FCECDFCAAB"));
}

#[test]
//#[ignore]
fn test_asking() {
   assert_eq!(
       "Sure.",
       reply("Does this cryogenic chamber make me look fat?")
   );
}

#[test]
//#[ignore]
fn test_ask_numeric_question() {
   assert_eq!("Sure.", reply("You are, what, like 15?"));
}

#[test]
//#[ignore]
fn test_asking_gibberish() {
   assert_eq!("Sure.", reply("fffbbcbeab?"));
}

#[test]
//#[ignore]
fn test_exclaiming() {
   assert_eq!("Whatever.", reply("Let's go make out behind the gym!"));
}

#[test]
//#[ignore]
fn test_using_acronyms_in_regular_speech() {
   assert_eq!(
       "Whatever.",
       reply("It's OK if you don't want to go to the DMV.")
   );
}

#[test]
//#[ignore]
fn test_forceful_question() {
   assert_eq!(
       "Calm down, I know what I'm doing!",
       reply("WHAT THE HELL WERE YOU THINKING?")
   );
}

#[test]
//#[ignore]
fn test_shouting_numbers() {
   assert_eq!("Whoa, chill out!", reply("1, 2, 3 GO!"));
}

#[test]
//#[ignore]
fn test_only_numbers() {
   assert_eq!("Whatever.", reply("1, 2, 3"));
}

#[test]
//#[ignore]
fn test_question_with_only_numbers() {
   assert_eq!("Sure.", reply("4?"));
}

#[test]
//#[ignore]
fn test_shouting_with_special_characters() {
   assert_eq!(
       "Whoa, chill out!",
       reply("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!")
   );
}

#[test]
//#[ignore]
fn test_shouting_with_no_exclamation_mark() {
   assert_eq!("Whoa, chill out!", reply("I HATE YOU"));
}

#[test]
//#[ignore]
fn test_statement_containing_question_mark() {
   assert_eq!("Whatever.", reply("Ending with ? means a question."));
}

#[test]
//#[ignore]
fn test_non_letters_with_question() {
   assert_eq!("Sure.", reply(":) ?"));
}

#[test]
//#[ignore]
fn test_prattling_on() {
   assert_eq!("Sure.", reply("Wait! Hang on. Are you going to be OK?"));
}

#[test]
//#[ignore]
fn test_silence() {
   assert_eq!("Fine. Be that way!", reply(""));
}

#[test]
//#[ignore]
fn test_prolonged_silence() {
   assert_eq!("Fine. Be that way!", reply("          "));
}

#[test]
//#[ignore]
fn test_alternate_silence() {
   assert_eq!("Fine. Be that way!", reply("\t\t\t\t\t\t\t\t\t\t"));
}

#[test]
//#[ignore]
fn test_multiple_line_question() {
   assert_eq!(
       "Whatever.",
       reply("\nDoes this cryogenic chamber make me look fat?\nno")
   );
}

#[test]
//#[ignore]
fn test_starting_with_whitespace() {
   assert_eq!("Whatever.", reply("         hmmmmmmm..."));
}

#[test]
//#[ignore]
fn test_ending_with_whitespace() {
   assert_eq!("Sure.", reply("Okay if like my  spacebar  quite a bit?   "));
}

#[test]
//#[ignore]
fn test_other_whitespace() {
   assert_eq!("Fine. Be that way!", reply("\n\r \t"));
}

#[test]
//#[ignore]
fn test_non_question_ending_with_whitespace() {
   assert_eq!(
       "Whatever.",
       reply("This is a statement ending with whitespace      ")
   );
}

// 8
#[test]
fn test_verse_0() {
   assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_verse_1() {
   assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_verse_2() {
   assert_eq!(verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_verse_8() {
   assert_eq!(verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_song_8_6() {
   assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}

#[test]
//#[ignore]
fn test_song_3_0() {
   assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

// 9
#[test]
fn test_two_pieces() {
   let input = vec!["nail", "shoe"];
   let expected = vec![
       "For want of a nail the shoe was lost.",
       "And all for the want of a nail.",
   ]
   .join("\n");
   assert_eq!(build_proverb(input), expected);
}

// Notice the change in the last line at three pieces.
#[test]
//#[ignore]
fn test_three_pieces() {
   let input = vec!["nail", "shoe", "horse"];
   let expected = vec![
       "For want of a nail the shoe was lost.",
       "For want of a shoe the horse was lost.",
       "And all for the want of a nail.",
   ]
   .join("\n");
   assert_eq!(build_proverb(input), expected);
}

#[test]
//#[ignore]
fn test_one_piece() {
   let input = vec!["nail"];
   let expected = String::from("And all for the want of a nail.");
   assert_eq!(build_proverb(input), expected);
}

#[test]
//#[ignore]
fn test_zero_pieces() {
   let input: Vec<&str> = vec![];
   let expected = String::new();
   assert_eq!(build_proverb(input), expected);
}

#[test]
//#[ignore]
fn test_full() {
   let input = vec![
       "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
   ];
   let expected = vec![
       "For want of a nail the shoe was lost.",
       "For want of a shoe the horse was lost.",
       "For want of a horse the rider was lost.",
       "For want of a rider the message was lost.",
       "For want of a message the battle was lost.",
       "For want of a battle the kingdom was lost.",
       "And all for the want of a nail.",
   ]
   .join("\n");
   assert_eq!(build_proverb(input), expected);
}

#[test]
//#[ignore]
fn test_three_pieces_modernized() {
   let input = vec!["pin", "gun", "soldier", "battle"];
   let expected = vec![
       "For want of a pin the gun was lost.",
       "For want of a gun the soldier was lost.",
       "For want of a soldier the battle was lost.",
       "And all for the want of a pin.",
   ]
   .join("\n");
   assert_eq!(build_proverb(input), expected);
}

// 10
#[test]
fn test_square_of_sum_1() {
   assert_eq!(1, square_of_sum(1));
}

#[test]
//#[ignore]
fn test_square_of_sum_5() {
   assert_eq!(225, square_of_sum(5));
}

#[test]
//#[ignore]
fn test_square_of_sum_100() {
   assert_eq!(25502500, square_of_sum(100));
}

#[test]
//#[ignore]
fn test_sum_of_squares_1() {
   assert_eq!(1, sum_of_squares(1));
}

#[test]
//#[ignore]
fn test_sum_of_squares_5() {
   assert_eq!(55, sum_of_squares(5));
}

#[test]
//#[ignore]
fn test_sum_of_squares_100() {
   assert_eq!(338350, sum_of_squares(100));
}

#[test]
//#[ignore]
fn test_difference_1() {
   assert_eq!(0, difference(1));
}

#[test]
//#[ignore]
fn test_difference_5() {
   assert_eq!(170, difference(5));
}

#[test]
//#[ignore]
fn test_difference_100() {
   assert_eq!(25164150, difference(100));
}

// 11
#[test]
fn multiples_one() {
   assert_eq!(0, sum_of_multiples(1, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_two() {
   assert_eq!(3, sum_of_multiples(4, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_three() {
   assert_eq!(23, sum_of_multiples(10, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_four() {
   assert_eq!(2318, sum_of_multiples(100, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_five() {
   assert_eq!(233168, sum_of_multiples(1000, &[3, 5]))
}

#[test]
//#[ignore]
fn multiples_six() {
   assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]))
}

#[test]
//#[ignore]
fn multiples_seven() {
   assert_eq!(30, sum_of_multiples(15, &[4, 6]))
}

#[test]
//#[ignore]
fn multiples_eight() {
   assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
}

#[test]
//#[ignore]
fn multiples_nine() {
   assert_eq!(275, sum_of_multiples(51, &[5, 25]))
}

#[test]
//#[ignore]
fn multiples_ten() {
   assert_eq!(2203160, sum_of_multiples(10000, &[43, 47]))
}

#[test]
//#[ignore]
fn multiples_eleven() {
   assert_eq!(4950, sum_of_multiples(100, &[1]))
}

#[test]
//#[ignore]
fn multiples_twelve() {
   assert_eq!(0, sum_of_multiples(10000, &[]))
}

// 12
#[test]
fn square_one() {
   assert_eq!(square(1), 1);
}

#[test]
//#[ignore]
fn square_two() {
   assert_eq!(square(2), 2);
}

#[test]
//#[ignore]
fn square_three() {
   assert_eq!(square(3), 4);
}

#[test]
//#[ignore]
fn square_four() {
   assert_eq!(square(4), 8);
}

#[test]
//#[ignore]
fn square_sixteen() {
   assert_eq!(square(16), 32_768);
}

#[test]
//#[ignore]
fn square_thirty_two() {
   assert_eq!(square(32), 2_147_483_648);
}

#[test]
//#[ignore]
fn square_sixty_four() {
   assert_eq!(square(64), 9_223_372_036_854_775_808);
}

#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_zero_panics() {
   square(0);
}

#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_sixty_five_panics() {
   square(65);
}

#[test]
//#[ignore]
fn total_sums_all_squares() {
   assert_eq!(total(), 18_446_744_073_709_551_615);
}