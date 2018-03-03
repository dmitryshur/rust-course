pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes = vec![true; n as usize];
    let mut new_vec = Vec::new();

    for i in 2..(n as f64).sqrt() as i32 + 1 {
        if primes[i as usize] == true {
            for j in SimpleStepRange(i.pow(2) as isize, n as isize, i as isize) {
                primes[j as usize] = false;
            }
        }
    }

    for (index, value) in primes.iter().enumerate() {
        if *value == true  && index > 1 {
           new_vec.push(index as u32);
        }
    }
    new_vec
}

struct SimpleStepRange(isize, isize, isize); // start, end, and step

impl Iterator for SimpleStepRange {
    type Item = isize;

    #[inline]
    fn next(&mut self) -> Option<isize> {
        if self.0 < self.1 {
            let v = self.0;
            self.0 = v + self.2;
            Some(v)
        } else {
            None
        }
    }
}
