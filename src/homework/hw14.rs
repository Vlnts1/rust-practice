use std::collections::HashSet;

fn is_valid(m: u8, u: u8, x: u8, a1: u8, s: u8, l: u8, o: u8, n: u8) -> bool {
    let muxa = 1000 * m as u32 + 100 * u as u32 + 10 * x as u32 + a1 as u32;
    let slon = 1000 * s as u32 + 100 * l as u32 + 10 * o as u32 + n as u32;
    let a2 = a1;

    muxa * a2 as u32 == slon
}

fn solve() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let mut results = vec![];

    for m in 1..10 {
        for u in 0..10 {
            for x in 0..10 {
                for a1 in 1..10 {
                    for s in 1..10 {
                        for l in 0..10 {
                            for o in 0..10 {
                                for n in 0..10 {
                                    let digits = [m, u, x, a1, s, l, o, n];
                                    let unique: HashSet<_> = digits.iter().cloned().collect();
                                    if unique.len() != digits.len() {
                                        continue;
                                    }

                                    if is_valid(m, u, x, a1, s, l, o, n) {
                                        results.push((m, u, x, a1, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    results
}

fn main() {
    let solutions = solve();
    println!("Знайдено рішень: {}", solutions.len());

    for (m, u, x, a, s, l, o, n) in solutions {
        let muxa = format!("{}{}{}{}", m, u, x, a);
        let a_val = a.to_string();
        let slon = format!("{}{}{}{}", s, l, o, n);

        println!("  {}", muxa);
        println!("x    {}", a_val);
        println!("------");
        println!("  {}", slon);
        println!();
    }
}
