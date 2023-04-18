use std::{collections::HashSet, ops::Index};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&'a str> = HashSet::new();

    let mut src: Vec<char> = word.to_lowercase().chars().collect();
    let src_dup = src.clone();
    src.sort();
    for s in possible_anagrams.iter() {
        let string = s.to_string();

        let mut flag = true;
        let mut pp: Vec<char> = string.to_lowercase().chars().collect();
        for (i, c) in src_dup.iter().enumerate() {
            if c != pp.index(i) {
                flag = false;
                break;
            };
        }

        if flag {
            continue;
        }

        pp.sort();
        

        if src.len() != pp.len() {
            continue;
        }

        let mut flag = false;
        for (i, c) in src.iter().enumerate() {
            if c != pp.index(i) {
                flag = true;
                break;
            };
        }

        if flag {
            continue;
        }

        res.insert(s);
        // res.insert(Box::leak(s.to_string().into_boxed_str()));
    }

    res
}
