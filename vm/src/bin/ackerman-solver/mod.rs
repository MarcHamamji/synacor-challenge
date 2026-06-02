use rayon::prelude::*;
use std::{collections::HashMap};

fn ackerman(m: u16, n: u16, r7: u16, map: &mut HashMap<u32, u16>) -> u16 {
    let key = (m as u32) << 16 | (n as u32);
    if let Some(value) = map.get(&key) {
        return *value;
    }

    if m == 0 {
        return n.overflowing_add(1).0;
    } else if n == 0 {
        let value = ackerman(m - 1, r7, r7, map);
        map.insert(key, value);
        return value;
    } else {
        let value = ackerman(m - 1, ackerman(m, n - 1, r7, map), r7, map);
        map.insert(key, value);
        return value;
    }
}

fn main() {
    (0..=32767).into_par_iter().for_each(|r7| {
        let mut map: HashMap<u32, u16> = HashMap::new();
        map.clear();

        let value = ackerman(4, 1, r7, &mut map);
        // println!(
        //     "r7 = {} [{:.0}%]: {}",
        //     r7,
        //     (r7 as f32) / 32767.0 * 100.0,
        //     value
        // );
        if value == 6 {
            println!("FOUND VALID r7: {}", r7);
        }
    });
}
