/// Représente un morceau de musique avec une note et un titre.
pub struct Song {
    rank: u32,
    title: String,
}

/// Retourne la note moyenne d'un ensemble de morceaux.
pub fn average_rank(songs: &Vec<Song>) -> f64 {
    if songs.is_empty() {
        panic!("No songs provided!");
    }

    let mut sum_ranks: u32 = 0;
    for song in songs {
        sum_ranks += song.rank;
    }

    sum_ranks as f64 / songs.len() as f64
}

/// Filtre les morceaux dans `songs` et ne garde que ceux dont la note est
/// strictement supérieure à `rank_min`.
pub fn filter_songs(songs: Vec<Song>, rank_min: u32) -> Vec<Song> {
    panic!("Not implemented!")
}

/// Filtre les morceaux dans `songs` pour ne conserver que ceux dont la note
/// est strictement supérieure à la moyenne.
pub fn good_songs(songs: Vec<Song>) -> Vec<Song> {
    panic!("Not implemented!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Song> {
        vec![
            Song {
                rank: 4,
                title: String::from("Stairway to Heaven"),
            },
            Song {
                rank: 2,
                title: String::from("Never Gonna Give You Up"),
            },
            Song {
                rank: 5,
                title: String::from("Nigerian Marketplace"),
            },
        ]
    }

    #[test]
    #[should_panic]
    fn empty() {
        let sgs = Vec::new();
        let _m = average_rank(&sgs);
    }

    #[test]
    fn moyenne_un_seul_morceau() {
        let sgs = vec![Song {
            rank: 4,
            title: String::from("Stairway to Heaven"),
        }];
        let m = average_rank(&sgs);
        assert_eq!(m, 4.0);
    }

    #[test]
    fn moyenne_plusieurs_morceau() {
        let sgs = example();
        const EPSILON: f64 = 1E-8;
        const AVERAGE: f64 = 3.666666666;
        let m = average_rank(&sgs);
        assert!((m - AVERAGE).abs() < EPSILON);
    }

    #[test]
    fn filtre() {
        let sgs = example();
        let f = filter_songs(sgs, 4);
        assert!(f.len() == 1);
        assert_eq!(f[0].rank, 5);
        assert_eq!(f[0].title, "Nigerian Marketplace");
    }

    #[test]
    fn meilleurs() {
        let sgs = example();
        let f = good_songs(sgs);
        assert!(f.len() == 2);
        for m in &f {
            assert!(m.rank >= 4);
        }
    }
}
