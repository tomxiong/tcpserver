fn main() {
    let ints: Vec<u32> = vec![1112121211, 2334232346, 1112121211, 23342323];
    let total_ints = sum_u_ints(&ints);
    println!("Total value is :{}", total_ints.unwrap());
}

// The code for the second question
//-----------------------------------------------------
fn sum_u_ints(items: &[u32]) -> Option<u32> {
    let mut total = 0_u32;
    for u in items {
        match total.checked_add(*u) {
            Some(v) => {
                total = v;
            }
            None => {
                println!("overflow!");
                ()
            }
        }
    }
    Some(total)
}
//-----------------------------------------------------
