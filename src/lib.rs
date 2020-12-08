pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

#[macro_export]
macro_rules! solutions {
    ($($part:ident => { $code:expr, $exp:expr }),*) => {
        #[cfg(test)]
        mod solutions {
            use super::*;
            $(
                #[test]
                fn $part() {
                    assert_eq!($code, $exp);
                }
            )*
        }
    };
}
