/**
 * Some utility functions
*/


/// Return the lowest 2-power number less than or equal n
/// ## Parameters
/// * `n` - number whose lower 2-power bound we want to compute
/// ## Return
/// highghest 2-power number lower or equal than n
pub fn nearest_lower_2_power(n : usize) -> usize {
    (2 as usize).pow((n as f32).log2() as u32)
}


/// Return the lowest 2-power number greater than or equal n
/// ## Parameters
/// * `n` - number whose upper 2-power bound we want to compute
/// ## Return
/// highghest 2-power number greater or equal than n
pub fn nearest_upper_2_power(n : usize) -> usize {
    let log = (n as f32).log2();
    let log_int = log as u32;
    let two : usize = 2;

    if log.fract() == 0.0 { two.pow(log_int) } else { two.pow(log_int + 1)}
}