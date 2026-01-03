struct FreshRange(u64, u64);
struct Ingredient(u64, bool);

pub fn determine_fresh_ingredients() {
    let (ranges_str, ingredients_str) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(a, b)| FreshRange(a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<FreshRange>>();
    let mut ingredients = ingredients_str
        .lines()
        .map(|line| Ingredient(line.parse().unwrap(), false))
        .collect::<Vec<Ingredient>>();

    ingredients.iter_mut().for_each(|ing| {
        ranges.iter().for_each(|rng| {
            if rng.0 <= ing.0 && rng.1 >= ing.0 {
                ing.1 = true;
            }
        });
    });

    println!(
        "Number of fresh ingredients: {}",
        ingredients.iter().filter(|ing| ing.1).count()
    );
}
