/// Implémente le crible d'Eratosthène.
/// La fonction retourne un vecteur contenant les nombres premiers inférieurs ou égaux à `n`.
///
/// # Exemple
/// ```
/// let r = td_exercices_choisis::exercice2::sieve(7);
/// assert_eq!(r, vec![1, 2, 3, 5, 7])
/// ```
pub fn sieve(n: u32) -> Vec<u32> {
    let n = n as usize;
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    for i in 2..=n {
        if sieve[i] {
            for m in ((i * i)..=n).step_by(i) {
                sieve[m] = false
            }
        }
    }

    sieve.iter()
        .enumerate()
        .filter_map(|(i, b)| if *b { Some(i as u32) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn n_zero() {
        let m = sieve(0);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn n_thirty() {
        let m = sieve(30);
        assert_eq!(m, vec![1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
