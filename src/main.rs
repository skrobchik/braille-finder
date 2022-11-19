use rayon::prelude::*;
/// Author: Robert Serrano Kobylyansky
/// 
/// Date: 2022-11-03
/// 
/// This small program was writen to design an actuator mechanism for an
/// 8-dot braille cell. There are two actuators, one for each column
/// of dots. Each column has 4 dots, that is the `pattern_len`. The actuator
/// works by moving a rail that is engraved in such a way so that
/// it contains all the possible permutations of dots. The rail is moved
/// by a stepper motor to the desired position, resulting in the corresponding
/// dots being raised.
/// ```
///     pattern: 0b0101      0      1       0      1
///                                NN             NN    <- pins (dots)
///                                NN             NN
///                         NN     NN      NN     NN
///        /----\  /----\   NN   /----\    NN   /----\  /----\  
/// ______/      \/      \__NN__/      \_ _NN__/      \/      \_______  <- rail [0b011 0101 10]
/// ```
/// The goal is to find the shortest rail which can be used to represent all
/// the dot patterns. `rayon` crate is used to parallelize the search, and speed things up.
fn main() {
    let max_length = 64;
    let pattern_len = 5;
    if let Some((length, answers)) = (pattern_len..=max_length)
        .map(|length| {
            println!("length: {}", length);
            (length, (0..2_u64.pow(length)).into_par_iter()
                .find_any(|n| {
                    (0..2_u64.pow(pattern_len)).into_par_iter().all(|p| {
                        (0..(length - pattern_len + 1))
                            .any(|i| (0..pattern_len).all(|j| (n >> (i + j)) & 1 == (p >> j) & 1))
                    })
                }))
        })
        .find_map(|(length, v)| v.map(|v| (length, v))) {
            println!("Found answer!)");
            println!("{:0width$b}", answers, width = length.try_into().unwrap());
        } else {
            println!("No answers found with max length {}", max_length);
        }
}

// For anyone curious the answer is:
// 001110000010001100101011011111010011