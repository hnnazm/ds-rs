fn run(input: Vec<i32>, target: i32) -> i32 {
    println!("[Performing Binary Search]");
    println!("Searching for {:?}", target);

    let mut low = 0;
    let mut high = input.len() - 1;
    let mut loc = -1;

    for _ in input.iter() {
        let mid = (low + high) / 2;

        if input[mid] == target {
            loc = mid as i32;
            break;
        }

        if input[mid] > target {
            high = mid;
        }

        if input[mid] < target {
            low = mid + 1;
        }
    }

    loc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_location() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let target = 1;
        let result = run(input, target);

        let expected = 0;

        assert_eq!(expected, result);
    }

    #[test]
    fn last_location() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let target = 6;
        let result = run(input, target);

        let expected = 5;

        assert_eq!(expected, result);
    }

    #[test]
    fn mid_location() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let target = 3;
        let result = run(input, target);

        let expected = 2;

        assert_eq!(expected, result);
    }

    #[test]
    fn odd_size() {
        let input = vec![1, 2, 3, 4, 5];
        let target = 2;
        let result = run(input, target);

        let expected = 1;

        assert_eq!(expected, result);
    }
}
