pub trait ReorderItem {
    type Type;

    fn reorder_item(self, pred: impl Fn(&Self::Type) -> bool) -> Self;
}

impl<T> ReorderItem for Option<&mut [T]> {
    type Type = T;

    fn reorder_item(self, pred: impl Fn(&Self::Type) -> bool) -> Self {
        if let Some(items) = self {
            if let Some((idx, _)) = items.iter().enumerate().find(|(_, item)| pred(item)) {
                items.swap(0, idx);
                Some(&mut items[1..])
            } else {
                None
            }
        } else {
            None
        }
    }
}

macro_rules! entities_ordered_by_type {
    ([$($entity:expr),* $(,)?], $($query:expr),* $(,)?) => {{
        use crate::utils::ReorderItem;
        let mut entities = [$($entity),*];
        let unresolved = Some(&mut entities[..]);
        $(
            let unresolved = unresolved.reorder_item(|&entity| $query.get(entity).is_ok())
        );*;
        if unresolved.is_some() {
            Some(entities)
        } else {
            None
        }
    }}
}
pub(crate) use entities_ordered_by_type;

macro_rules! some_or {
    ($option:expr; $($if_none:tt)*) => {{
        if let Some(some) = $option {
            some
        } else {
            $($if_none)*
        }
    }}
}

pub(crate) use some_or;

// macro_rules! ok_or {
// ($option:expr; $($if_none:tt)*) => {{
// if let Ok(some) = $option {
// some
// } else {
// $($if_none)*
// }
// }}
// }

// pub(crate) use ok_or;
