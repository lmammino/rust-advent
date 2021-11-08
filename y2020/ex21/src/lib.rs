use std::collections::{HashMap, HashSet};

struct AllergensIngredients<'a> {
    mapping: HashMap<&'a str, HashSet<&'a str>>,
    ingredients: Vec<&'a str>,
}

impl<'a> From<&'a str> for AllergensIngredients<'a> {
    fn from(input: &'a str) -> Self {
        let mut mapping: HashMap<&'a str, HashSet<&'a str>> = Default::default();
        let mut ingredients: Vec<&str> = Default::default();

        input.lines().for_each(|line| {
            let (ing, alle) = line[..(line.len() - 1)].split_once(" (contains ").unwrap();
            let ings: HashSet<&str> = ing.split(' ').collect();
            let alles: HashSet<&str> = alle.split(", ").collect();
            ingredients.extend(ings.iter());
            for allergen in alles {
                let values = mapping.entry(allergen).or_insert_with(|| ings.clone());
                let new_values: HashSet<&str> = values.intersection(&ings).cloned().collect();
                mapping.insert(allergen, new_values);
            }
        });

        Self {
            mapping,
            ingredients,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let allergens_ingredients: AllergensIngredients = input.into();

    let bad_ingredients_list: HashSet<&str> = allergens_ingredients
        .mapping
        .values()
        .flat_map(|x| x.iter())
        .cloned()
        .collect();

    allergens_ingredients
        .ingredients
        .into_iter()
        .filter(|x| !bad_ingredients_list.contains(x))
        .count()
}

pub fn part2(input: &str) -> String {
    let mut allergens_ingredients: AllergensIngredients = input.into();
    let mut sure_mappings: HashMap<&str, &str> = Default::default();

    while sure_mappings.len() != allergens_ingredients.mapping.len() {
        for (all, ings) in &allergens_ingredients.mapping {
            if ings.len() == 1 {
                sure_mappings.insert(all, ings.iter().next().unwrap());
            }
        }
        sure_mappings.iter().for_each(|(_, ing)| {
            for (_, ings_set) in allergens_ingredients.mapping.iter_mut() {
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
