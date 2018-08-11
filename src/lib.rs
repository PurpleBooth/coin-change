#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
extern crate core;
#[cfg(test)]
extern crate quickcheck;

use core::iter;
use std::ops::Add;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Coin(u32);

impl Coin {
    pub fn new(value: u32) -> Result<Coin, String> {
        if value == 0 {
            return Err("A zero value coin is not a coin".to_string());
        }

        Ok(Coin(value))
    }
}

impl std::ops::Div<Coin> for u32 {
    type Output = u32;

    fn div(self, rhs: Coin) -> u32 {
        self / rhs.0
    }
}

impl std::ops::Rem<Coin> for u32 {
    type Output = u32;

    fn rem(self, modulus: Coin) -> Self {
        self % modulus.0
    }
}

impl std::ops::Add<Coin> for u32 {
    type Output = u32;

    fn add(self, rhs: Coin) -> <Self as Add<Coin>>::Output {
        self + rhs.0
    }
}

#[cfg(test)]
mod test_coin {
    use quickcheck::TestResult;

    use Coin;

    #[test]
    fn test_that_coin_is_created() {
        assert_eq!(Coin::new(3), Ok(Coin(3)))
    }

    #[quickcheck]
    fn a_coin_can_have_any_value_except_0(data: u32) -> TestResult {
        TestResult::from_bool(Coin::new(data + 1).is_ok())
    }

    #[quickcheck]
    fn coin_contains_value(data: u32) -> TestResult {
        TestResult::from_bool(Coin::new(data + 1).unwrap().0 == data + 1)
    }

    #[quickcheck]
    fn coin_can_divide_u32(data: u32) -> TestResult {
        TestResult::from_bool((data + 1) / (data + 1) == (data + 1) / Coin::new(data + 1).unwrap())
    }

    #[quickcheck]
    fn coin_can_rem_u32(data: u32) -> TestResult {
        TestResult::from_bool((data + 1) % (data + 1) == (data + 1) % Coin::new(data + 1).unwrap())
    }

    #[quickcheck]
    fn coin_can_add_u32(data: u32) -> TestResult {
        TestResult::from_bool(data + data + 1 == data + Coin::new(data + 1).unwrap())
    }

    #[test]
    fn zero_value_coins_fail() {
        assert_eq!(
            Coin::new(0),
            Err("A zero value coin is not a coin".to_string())
        )
    }
}

pub struct CoinOptions(Vec<Coin>);

impl CoinOptions {
    pub fn new(value: Vec<Coin>) -> Result<CoinOptions, String> {
        let one_coin = Coin::new(1).unwrap();

        if !value.contains(&one_coin) {
            return Err("You must provide at least one 1 value coin".to_string());
        }

        let mut deduped_coins = value.clone();
        deduped_coins.sort();
        deduped_coins.dedup();

        if deduped_coins.len() < value.len() {
            return Err("There are repeated coins repeated, please do not repeat them".to_string());
        }

        Ok(CoinOptions(value))
    }
}

#[cfg(test)]
mod test_coin_options {
    use CoinOptions;
    use Coin;

    #[quickcheck]
    fn it_errs_if_there_is_not_a_1_coin(input: u32) -> bool {
        CoinOptions::new(vec![Coin(input + 2)]).is_err()
    }

    #[quickcheck]
    fn any_other_coins_are_allowed_if_there_is_a_1_coin(input: u32) -> bool {
        CoinOptions::new(vec![Coin(1), Coin(input + 2)]).is_ok()
    }

    #[quickcheck]
    fn repeated_coins_cause_a_error(input: u32) -> bool {
        CoinOptions::new(vec![Coin(1), Coin(input + 2), Coin(input + 2)]).is_err()
    }

    #[quickcheck]
    fn the_options_include_the_coins_provided(input: u32) -> bool {
        let coin_options = CoinOptions::new(vec![Coin(1), Coin(input + 2)]).unwrap();

        coin_options.0.contains(&Coin(1)) && coin_options.0.contains(&Coin(input + 2))
    }
}

fn coin_change(number: u32, coin_options: CoinOptions) -> Vec<Coin> {
    let mut coins: Vec<Coin> = vec![];
    let mut remaining = number;

    let mut sorted_coin_options = coin_options.0;
    sorted_coin_options.sort();
    sorted_coin_options.reverse();

    for coin_option in sorted_coin_options {
        iter::repeat(coin_option)
            .take((remaining / coin_option) as usize)
            .for_each(|coin| coins.push(coin));

        remaining = remaining % coin_option
    }

    coins
}

#[cfg(test)]
mod test_coin_change {
    use Coin;
    use coin_change;
    use CoinOptions;
    use quickcheck::TestResult;

    #[test]
    fn test_that_correct_change_given() {
        assert_eq!(
            coin_change(
                3,
                CoinOptions::new(
                    vec![1, 2]
                        .iter()
                        .map(Clone::clone)
                        .map(Coin::new)
                        .map(Result::unwrap)
                        .collect::<Vec<Coin>>()
                ).unwrap(),
            ),
            vec![2, 1]
                .iter()
                .map(Clone::clone)
                .map(Coin::new)
                .map(Result::unwrap)
                .collect::<Vec<Coin>>()
        )
    }

    #[quickcheck]
    fn change_must_add_up_to_total_given(data: u32) -> bool {
        let output = coin_change(
            data,
            CoinOptions::new(
                vec![1, 2, 3, 4, 5]
                    .iter()
                    .map(Clone::clone)
                    .map(Coin::new)
                    .map(Result::unwrap)
                    .collect::<Vec<Coin>>(),
            ).unwrap(),
        );

        output
            .iter()
            .map(Clone::clone)
            .fold(0u32, |sum, val| sum + val) == data
    }

    #[quickcheck]
    fn count_of_coins_should_be_input_if_coin_options_is_one(input: u32) -> bool {
        let output = coin_change(
            input,
            CoinOptions::new(
                vec![1]
                    .iter()
                    .map(Clone::clone)
                    .map(Coin::new)
                    .map(Result::unwrap)
                    .collect::<Vec<Coin>>(),
            ).unwrap(),
        );

        output.len() == input as usize
    }

    #[quickcheck]
    fn change_should_be_in_largest_coin_available(input: u32) -> TestResult {
        let mut output = coin_change(
            input + 3,
            CoinOptions::new(
                vec![1, input + 2, input + 3, input + 4]
                    .iter()
                    .map(Clone::clone)
                    .map(Coin::new)
                    .map(Result::unwrap)
                    .collect::<Vec<Coin>>(),
            ).unwrap(),
        );

        output.sort();

        TestResult::from_bool(
            output
                == vec![input + 3]
                    .iter()
                    .map(Clone::clone)
                    .map(Coin::new)
                    .map(Result::unwrap)
                    .collect::<Vec<Coin>>(),
        )
    }

    #[quickcheck]
    fn the_output_is_split_over_multiple_coins_if_it_will_not_fix_exactly(
        input: u32,
    ) -> TestResult {
        let output = coin_change(
            input + 3,
            CoinOptions::new(
                vec![1, input + 2]
                    .iter()
                    .map(Clone::clone)
                    .map(Coin::new)
                    .map(Result::unwrap)
                    .collect::<Vec<Coin>>(),
            ).unwrap(),
        );

        TestResult::from_bool(
            output
                == vec![input + 2, 1]
                    .iter()
                    .map(Clone::clone)
                    .map(Coin::new)
                    .map(Result::unwrap)
                    .collect::<Vec<Coin>>(),
        )
    }
}
