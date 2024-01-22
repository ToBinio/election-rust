use crate::voting::Voting;
use iter_tools::Itertools;
use std::cmp::Ordering;

pub fn display(voting: Voting) {
    voting
        .candidates
        .iter()
        .sorted_by(|a, b| {
            let vote_dif = b.get_votes().cmp(&a.get_votes());
            if vote_dif != Ordering::Equal {
                vote_dif
            } else {
                b.get_first_votes().cmp(&a.get_first_votes())
            }
        })
        .for_each(|voting| {
            println!(
                "{} | {} - {}",
                voting.get_votes(),
                voting.get_first_votes(),
                voting.name,
            )
        });

    println!();
    println!("{}     - invalid", voting.invalid());
}
