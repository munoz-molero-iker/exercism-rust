pub fn is_armstrong_number(num: u32) -> bool {
    
    let num_vector: Vec<u32> = n_to_vector(num);
    let mut total_sum: u32 = 0;
    for digit in &num_vector {

        total_sum += digit.pow(num_vector.len() as u32)

    }
    println!("{}", total_sum);
    num == total_sum

}

fn n_to_vector(n: u32) -> Vec<u32>{

    n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()

}