use std::{collections::HashSet, fs::File, io::Read};

use arrayvec::ArrayVec;
use itertools::Itertools;

mod tile;

const HAINUM: usize = 14;

fn main() -> std::io::Result<()> {
    let mut hai_sets_general: Vec<ArrayVec<u8, HAINUM>> = {
        let mut reader_general = File::open("patterns_general_three.dat")?;
        let fsize_general = reader_general.metadata()?.len() as usize;
        let mut buffer_general = Vec::with_capacity(fsize_general);
        let size_read_general = reader_general.read_to_end(&mut buffer_general)?;
        assert_eq!(size_read_general, fsize_general);
        buffer_general
            .into_iter()
            .chunks(HAINUM)
            .into_iter()
            .map(|c| c.collect())
            .collect()
    };

    let hai_gen_original = hai_sets_general.clone();
    hai_sets_general.sort();
    hai_sets_general.dedup();
    println!(
        "General differs after dedup: {}, len from {} to {}",
        hai_gen_original != hai_sets_general,
        hai_gen_original.len(),
        hai_sets_general.len(),
    );
    let hai_sets_general: HashSet<_> = hai_sets_general.into_iter().collect();

    let mut hai_sets_rust: Vec<ArrayVec<u8, HAINUM>> = {
        let mut reader_rust = File::open("patterns_rust_three.dat")?;
        let fsize_rust = reader_rust.metadata()?.len() as usize;
        let mut buffer_rust = Vec::with_capacity(fsize_rust);
        let size_read_rust = reader_rust.read_to_end(&mut buffer_rust)?;
        assert_eq!(size_read_rust, fsize_rust);
        buffer_rust
            .into_iter()
            .chunks(HAINUM)
            .into_iter()
            .map(|c| c.collect())
            .collect()
    };

    let hai_rust_original = hai_sets_rust.clone();
    hai_sets_rust.sort();
    hai_sets_rust.dedup();
    println!(
        "Rust differs after dedup: {}, len from {} to {}",
        hai_rust_original != hai_sets_rust,
        hai_rust_original.len(),
        hai_sets_rust.len(),
    );
    let hai_sets_rust: HashSet<_> = hai_sets_rust.into_iter().collect();

    let general_only = hai_sets_general.difference(&hai_sets_rust);
    println!("Sets only in general version: ");
    general_only.for_each(|g| println!("{:?}", g));
    let rust_only = hai_sets_rust.difference(&hai_sets_general);
    println!("Sets only in rust version: ");
    rust_only.for_each(|r| println!("{:?}", r));

    println!("Dup patterns:");
    hai_rust_original
        .windows(2)
        .filter_map(|w| {
            if w[0] == w[1] {
                Some(w[0].clone())
            } else {
                None
            }
        })
        .for_each(|p| {
            p.into_iter()
                .for_each(|c| print!("{:?},", tile::Tile::try_from(c as char).unwrap()));
            println!("");
        });

    Ok(())
}
