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