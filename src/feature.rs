use crate::champi::{Chair, Champi, Couleur, Type};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Feature<T> {
    fn categorize(items: &[T]) -> (Vec<usize>, Vec<String>);
}

macro_rules! field_feature {
    ($type:ty, $field_type:ty, $field:ident, $name:ident) => {
        pub struct $name;

        impl Feature<$type> for $name
        where
            $field_type: Copy + Debug + Hash,
        {
            fn categorize(items: &[$type]) -> (Vec<usize>, Vec<String>) {
                let mut category_map = HashMap::<$field_type, usize>::new();
                let mut category_idx = Vec::new();
                let mut categories = Vec::new();
                for item in items {
                    let index = *category_map.entry(item.$field).or_insert_with(|| {
                        let idx = categories.len();
                        categories.push(format!("{} = {:?}", stringify!($field), &item.$field));
                        idx
                    });
                    category_idx.push(index);
                }
                (category_idx, categories)
            }
        }
    };
}

field_feature!(Champi, Type, r#type, ChampiType);
field_feature!(Champi, bool, anneau, ChampiAnneau);
field_feature!(Champi, bool, volve, ChampiVolve);
field_feature!(Champi, bool, lait, ChampiLait);
field_feature!(Champi, Option<Chair>, chair, ChampiChair);
field_feature!(Champi, Option<Couleur>, couleur_spores, ChampiCouleurSpores);
field_feature!(
    Champi,
    Option<Couleur>,
    couleur_chapeau,
    ChampiCouleurChapeau
);
