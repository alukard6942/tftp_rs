/**
 * File: log.rs
 * Author: alukard <alukard6942@github>
 * Date: 11.06.2022
 * Last Modified Date: 11.06.2022
 */

#[macro_export]
macro_rules! log{
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

