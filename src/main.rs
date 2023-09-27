use std::cmp::max;

fn main() {
    let timestamp = std::time::Instant::now();

    let primes = sieve_eratosthenes(0, 1000000);
    let mut i = 0;
    for (n, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            println!("{}", n);
            i += 1;
        }
    }
    println!("{} primes found", i);
    println!("sieve_eratosthenes took {}ms", timestamp.elapsed().as_millis());
}

// Source: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn sieve_eratosthenes(first_number: u32, number_count: u32) -> Vec<bool> {
    let m = max(2, first_number);
    let n = first_number + number_count;
    let sqrtn = (n as f64).sqrt() as u32;
    let mut sieve = vec![true; n as usize];

    sieve[0] = false;
    sieve[1] = false;

    for i in m..sqrtn {
        for j in (i*i..n).step_by(i as usize) {
            if sieve[i as usize] == true {
                sieve[j as usize] = false;
            }
        }
    }

    return sieve;
}

// Source: https://www.geeksforgeeks.org/sieve-eratosthenes-0n-time-complexity/
fn sieve_eratosthenes_fast(first_number: u32, number_count: u32) -> Vec<bool> {
    let m = max(2, first_number);
    let n = first_number + number_count;
    let mut sieve = vec![true; n as usize];
    let mut spf = vec![0; n as usize];

    sieve[0] = false;
    sieve[1] = false;

    for i in m..n {
        if sieve[i as usize] == true {
            spf[i as usize] = i;
        }

        for j in m..n {
            if i * j >= n || j > spf[i as usize] {
                break;
            }

            sieve[(i * j) as usize] = false;
            spf[(i * j) as usize] = j;
        }
    }

    return sieve;
}


