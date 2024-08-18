pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// models can contain other models which contain a set of functions
mod front_of_house {
    mod hosting {
        fn add_to_wait_list(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
    }

}