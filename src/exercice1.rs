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

pub fn average_rank_alt(songs: &Vec<Song>) -> f64 {
    if songs.is_empty() {
        panic!("No songs provided!");
    }

    // Avec fold :
    //songs.iter().fold(0, |acc, s| acc + s.rank) as f64 / songs.len() as f64
    // Avec map + sum :
    songs.iter().map(|s| s.rank).sum::<u32>() as f64 / songs.len() as f64
}

/// Filtre les morceaux dans `songs` et ne garde que ceux dont la note est
/// strictement supérieure à `rank_min`.
pub fn filter_songs(songs: Vec<Song>, rank_min: u32) -> Vec<Song> {
    let mut best: Vec<Song> = Vec::new();
    for song in songs {
        if song.rank > rank_min {
            best.push(song)
        }
    }
    best
}

pub fn filter_songs_alt(songs: Vec<Song>, rank_min: u32) -> Vec<Song> {
    songs.into_iter().filter(|s| s.rank > rank_min).collect()
}

/// Filtre les morceaux dans `songs` pour ne conserver que ceux dont la note
/// est strictement supérieure à la moyenne.
pub fn good_songs(songs: Vec<Song>) -> Vec<Song> {
    let avg: f64 = average_rank_alt(&songs);
    filter_songs_alt(songs, avg.trunc() as u32)
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
        const MEAN: f64 = 4.0;

        let m = average_rank(&sgs);
        assert_eq!(m, MEAN);

        let m_alt = average_rank_alt(&sgs);
        assert_eq!(m_alt, MEAN);
    }

    #[test]
    fn moyenne_plusieurs_morceau() {
        let sgs = example();
        const EPSILON: f64 = 1E-8;
        const MEAN: f64 = 3.666666666;

        let m = average_rank(&sgs);
        assert!((m - MEAN).abs() < EPSILON);

        let m_alt = average_rank_alt(&sgs);
        assert!((m_alt - MEAN).abs() < EPSILON);
    }

    #[test]
    fn filtre() {
        let sgs = example();
        let f = filter_songs(sgs, 4);
        assert!(f.len() == 1);
        assert_eq!(f[0].rank, 5);
        assert_eq!(f[0].title, "Nigerian Marketplace");

        let sgs = example();
        let f_alt = filter_songs_alt(sgs, 4);
        assert!(f_alt.len() == 1);
        assert_eq!(f_alt[0].rank, 5);
        assert_eq!(f_alt[0].title, "Nigerian Marketplace");
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
