mod math;

#[cfg(test)]
mod tests {


    
    #[test]
    fn sum_two_int() {
        let a = 1;
        let b = 2;

        let res = super::math::add_two_nums(a, b);

        assert_eq!(res, 3);
    }
}