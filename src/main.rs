use rand::Rng;

fn main() {
    let p = get_prime_number(5, 13);
    let mut q = get_prime_number(5, 13);
    while p == q {
        q = get_prime_number(5, 13);
    }
    let N = p * q;
    let L = lcm(p-1, q-1);

    let E = get_E_number(L);
    let D = get_D_number(E, L);

    println!("p:{}, q:{}, N:{}, L:{}, E:{}, D:{}", p, q, N, L, E, D);
}

fn get_E_number(L: u64) -> u64 {
    let v = get_relatively_primes(L);
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0, v.len()-1);
    v[idx]
}

fn get_D_number(E: u64, L: u64) -> u64 {
    let mut v = Vec::new();
    for i in 2..L {
        if (E * i) % L == 1 {
            v.push(i);
        }
    }
    if v.len() < 2 {
        return v[0];
    }

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0, v.len()-1);
    v[idx]
}

fn get_prime_number(min: u64, max:u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen_range(min, max);
    let mut ret = false;
    while is_prime(n) != true {
        n = rng.gen_range(min, max);
    }
    n
}

fn get_relatively_primes(num: u64) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 2..(num+1) {
        if is_relatively_prime(i, num) {
            v.push(i);
        }
    }
    v
}

fn is_relatively_prime(a: u64, b: u64) -> bool {
    return gcd(a, b) == 1;
}

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    let max = number / 2;
    for i in 2..(max+1) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

// calc great common divider
fn gcd(a: u64, b:u64) -> u64 {
    let mut d: u64 = a;
    let mut q: u64 = b;
    let mut r: u64 = 1;
    if a < b {
        d = b;
        q = a;
    }
    while 0 < r {
        r = d % q;
        d = q;
        q = r;
    }
    d
}

// calc least common multiplier
fn lcm(a: u64, b: u64) -> u64 {
    let gcd = gcd(a, b);
    a * b / gcd
}