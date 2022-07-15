fn main() {
    // Normal case
    let ints1:Vec<u32> = vec![1112121211, 2334232346];
    println!("Total value is :{:?}", sum_u_ints(&ints1));
    // overflow case
    let ints2:Vec<u32> = vec![1112121211, 2334232346, 1112121211, 23342323];
    println!("Wrong value is :{:?}", sum_u_ints(&ints2));
}

// The code for the second question
//-----------------------------------------------------
fn sum_u_ints(items: &[u32]) -> Option<u32> {
    // init the total with zero first
    let mut total = 0_u32;
    // for loop all items of slice
    for u in items {
        // add each value for total
        match total.checked_add(*u) {
            //Set the new value to total if it is not overflow
            Some(v) => {
                total = v;
            }
            //if it is overflow,then print error and return None
            None => {
                println!("overflow!");
                return None
            }
        }
    }
    // return the total value if not overflow
    Some(total)
}
//-----------------------------------------------------
