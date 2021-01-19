const STARTING_MISSILES: u8 = 8;
const READY_AMOUNT: u8 = 2;

fn main() {
    // let mut missiles: u8 = STARTING_MISSILES;
    // let ready: u8 = READY_AMOUNT;
    let (missiles, ready): (u8, u8) = (STARTING_MISSILES, READY_AMOUNT);
    // let unused: i32 = 1; // #[warn(unused_variables)]
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    // let missiles = missiles - ready;
    // println!("{} missiles left", missiles);    
    println!("{} missiles left", missiles - ready);
    // READY_AMOUNT = 8; // cannot assign to this expression
}
