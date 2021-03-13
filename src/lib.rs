use std::collections::HashMap;

pub fn fib_standard(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..n {
        let old = a;
        a = b;
        b += old;
    }
    b
}

pub fn fib_recursive(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => fib_recursive(n - 2) + fib_recursive(n - 1),
    }
}

pub fn fib_memoization(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let v = match n {
        0 | 1 => 1,
        _ => fib_memoization(n - 2, memo) + fib_memoization(n - 1, memo),
    };

    memo.insert(n, v);
    v
}

pub struct FibIterator {
    a: usize,
    b: usize,
}

impl Default for FibIterator {
    fn default() -> Self {
        FibIterator { a: 1, b: 1 }
    }
}

impl Iterator for FibIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.a;
        self.a = self.b;
        self.b = curr + self.a;

        Some(curr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const N: usize = 10;
    const V: usize = 89;

    #[test]
    fn standard() {
        assert_eq!(fib_standard(N), V);
    }

    #[test]
    fn recursion() {
        assert_eq!(fib_recursive(N), V);
    }

    #[test]
    fn memo() {
        assert_eq!(fib_memoization(N, &mut HashMap::new()), V);
    }

    #[test]
    fn iterator() {
        assert_eq!(FibIterator::default().nth(N).unwrap(), V);
    }
}
