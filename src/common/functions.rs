// Copyright (C) 2023 Afonso Barracha
//
// Rust Keyword Extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rust Keyword Extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Rust Keyword Extraction. If not, see <http://www.gnu.org/licenses/>.

use std::{
    cmp::Ordering,
    collections::{hash_map::RandomState, HashMap},
};

fn sort_ranked_map<'a>(map: &'a HashMap<String, f32, RandomState>) -> Vec<(&'a String, &'a f32)> {
    let mut map_values = map.iter().collect::<Vec<(&'a String, &'a f32)>>();
    map_values.sort_by(|a, b| {
        let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

        if order == Ordering::Equal {
            return a.0.cmp(b.0);
        }

        order
    });
    map_values
}

pub fn get_ranked_strings(map: &HashMap<String, f32, RandomState>, n: usize) -> Vec<String> {
    sort_ranked_map(map)
        .iter()
        .take(n)
        .map(|(word, _)| word.to_string())
        .collect::<Vec<String>>()
}

pub fn get_ranked_scores(map: &HashMap<String, f32, RandomState>, n: usize) -> Vec<(String, f32)> {
    sort_ranked_map(map)
        .iter()
        .take(n)
        .map(|(w, v)| (w.to_string(), **v))
        .collect::<Vec<(String, f32)>>()
}