use generator::*;

pub fn permutations<T>(elements: &[T]) -> Generator<'_, (), (&T, &T)>
where
    T: Send + Sync,
{
    Gn::new_scoped(move |mut s| {
        for i in 0..elements.len() {
            for j in 0..elements.len() {
                if i != j {
                    let el1 = elements.get(i).unwrap();
                    let el2 = elements.get(j).unwrap();
                    s.yield_with((el1, el2));
                }
            }
        }
        done!();
    })
}
