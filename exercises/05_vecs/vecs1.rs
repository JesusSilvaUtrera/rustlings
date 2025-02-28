fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro (vec!). Vector are stored in the heap (dynamic memory), so it can
    // change its size, arrays are store in the stack (size must be known at compile time).
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
