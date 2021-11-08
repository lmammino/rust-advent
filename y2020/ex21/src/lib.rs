use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut allergens_ingredients: HashMap<&str, HashSet<&str>> = Default::default();
    let mut ingredients: Vec<&str> = Default::default();

    for line in input.lines() {
        let (ing, alle) = line[..(line.len() - 1)].split_once(" (contains ").unwrap();
        let ings: HashSet<&str> = ing.split(' ').collect();
        let alles: HashSet<&str> = alle.split(", ").collect();
        ingredients.extend(ings.iter());
        for allergen in alles {
            let values = allergens_ingredients
                .entry(allergen)
                .or_insert_with(|| ings.clone());
            let new_values: HashSet<&str> = values.intersection(&ings).cloned().collect();
            allergens_ingredients.insert(allergen, new_values);
        }
    }

    let bad_ingredients_list: HashSet<&str> = allergens_ingredients
        .values()
        .flat_map(|x| x.iter())
        .cloned()
        .collect();

    ingredients
        .into_iter()
        .filter(|x| !bad_ingredients_list.contains(x))
        .count()
}

pub fn part2(input: &str) -> String {
    let mut allergens_ingredients: HashMap<&str, HashSet<&str>> = Default::default();

    for line in input.lines() {
        let (ing, alle) = line[..(line.len() - 1)].split_once(" (contains ").unwrap();
        let ings: HashSet<&str> = ing.split(' ').collect();
        let alles: HashSet<&str> = alle.split(", ").collect();
        for allergen in alles {
            let values = allergens_ingredients
                .entry(allergen)
                .or_insert_with(|| ings.clone());
            let new_values: HashSet<&str> = values.intersection(&ings).cloned().collect();
            allergens_ingredients.insert(allergen, new_values);
        }
    }

    let mut sure_mappings: HashMap<&str, &str> = Default::default();

    while sure_mappings.len() != allergens_ingredients.len() {
        for (all, ings) in &allergens_ingredients {
            if ings.len() == 1 {
                sure_mappings.insert(all, ings.iter().next().unwrap());
            }
        }
        sure_mappings.iter().for_each(|(_, ing)| {
            for (_, ings_set) in allergens_ingredients.iter_mut() {
                if ings_set.len() > 1 {
                    ings_set.remove(ing);
                }
            }
        });
    }

    let mut ret: Vec<(&str, &str)> = sure_mappings.into_iter().collect();
    ret.sort_by_key(|x| x.0);

    ret.iter().map(|x| x.1).collect::<Vec<&str>>().join(",")
}

#[cfg(test)]
mod ex21_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2556);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(
            part2(input),
            String::from("vcckp,hjz,nhvprqb,jhtfzk,mgkhhc,qbgbmc,bzcrknb,zmh")
        );
    }
}
