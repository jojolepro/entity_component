#[doc(hidden)]
#[macro_export]
macro_rules! gen_bitset {
    ($bitset:ident;) => {};
    ($bitset:ident; &mut $st:ident $($tail:tt)*) => {
        *std::rc::Rc::get_mut(&mut $bitset).unwrap() = $st.bitset().clone();
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; &$st:ident $($tail:tt)*) => {
        *std::rc::Rc::get_mut(&mut $bitset).unwrap() = $st.bitset().clone();
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; !&$st:ident $($tail:tt)*) => {
        let mut cloned = $st.bitset().clone();
        cloned.bit_not();
        *std::rc::Rc::get_mut(&mut $bitset).unwrap() = cloned;
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; && &mut $st:ident $($tail:tt)*) => {
        std::rc::Rc::get_mut(&mut $bitset).unwrap().bit_and($st.bitset());
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; && &$st:ident $($tail:tt)*) => {
        std::rc::Rc::get_mut(&mut $bitset).unwrap().bit_and($st.bitset());
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; && !&$st:ident $($tail:tt)*) => {
        std::rc::Rc::get_mut(&mut $bitset).unwrap().bit_andnot($st.bitset());
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; || &mut $st:ident $($tail:tt)*) => {
        std::rc::Rc::get_mut(&mut $bitset).unwrap().bit_or($st.bitset());
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; || &$st:ident $($tail:tt)*) => {
        std::rc::Rc::get_mut(&mut $bitset).unwrap().bit_or($st.bitset());
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; || !&$st:ident $($tail:tt)*) => {
        std::rc::Rc::get_mut(&mut $bitset).unwrap().bit_or($st.bitset().clone().bit_not());
        gen_bitset!($bitset; $($tail)*);
    };
    // scopes
    /*($bitset:ident; && ($($inner:tt)*) $($tail:tt)*) => {
        $bitset.bit_and({gen_bitset!($bitset; $($inner:tt)*); $bitset});
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; && !($($inner:tt)*) $($tail:tt)*) => {
        $bitset.bit_andnot({gen_bitset!($bitset; $($inner:tt)*); $bitset});
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; || ($($inner:tt)*) $($tail:tt)*) => {
        $bitset.bit_or({gen_bitset!($bitset; $($inner:tt)*); $bitset});
        gen_bitset!($bitset; $($tail)*);
    };
    ($bitset:ident; || !($($inner:tt)*) $($tail:tt)*) => {
        $bitset.bit_or({gen_bitset!($bitset; $($inner:tt)*); $bitset}.bit_not());
        gen_bitset!($bitset; $($tail)*);
    };*/
}

#[doc(hidden)]
#[macro_export]
macro_rules! iter_bitset {
    ($bitset:ident ; $(,)?$($idents:block),* ;) => {izip!($($idents),*)};
    ($bitset:ident ; $(,)?$($idents:block),* ; &mut $st:ident $($tail:tt)*) => {
        iter_bitset!($bitset; $($idents),* , {$st.iter_mut_with_bitset($bitset.clone())} ; $($tail)*)
    };
    ($bitset:ident ; $(,)?$($idents:block),* ; &$st:ident $($tail:tt)*) => {
        iter_bitset!($bitset; $($idents),* , {$st.iter_with_bitset($bitset.clone())} ; $($tail)*)
    };
    ($bitset:ident ; $(,)?$($idents:block),* ; !&$st:ident $($tail:tt)*) => {
        iter_bitset!($bitset; $($idents),* , {$st.iter_with_bitset($bitset.clone())} ; $($tail)*)
    };
    ($bitset:ident ; $(,)?$($idents:block),* ; && $($tail:tt)*) => {
        iter_bitset!($bitset; $($idents),* ; $($tail)*)
    };
    ($bitset:ident ; $(,)?$($idents:block),* ; || $($tail:tt)*) => {
        iter_bitset!($bitset; $($idents),* ; $($tail)*)
    };
    /*($bitset:ident; $(,)?$($idents:block),*; ($($tail:tt)*)) => {
        iter_bitset!($bitset; $($idents)*; $($tail)*)
    };
    ($bitset:ident; $(,)?$($idents:block),*; !($($tail:tt)*)) => {
        iter_bitset!($bitset; $($idents)*; $($tail)*)
    };*/
}

/// The join macro makes it very easy to iterate over multiple
/// components of the same `Entity` at once.
///
/// There are two ways to use this macro:
/// With a single component and with multiple.
///
/// When joining over a single component, simply provide the name of the
/// `Components<T>` instance as an immutable or mutable reference.
/// An iterator over the components will be returned.
/// The iterator will be of type `&T` or `&mut T` elements.
///
/// Joining over multiple components offers a complete syntax to decide which
/// components should or should not be matched.
/// Here is an example:
/// ```rust,ignore
/// let iter = join!(&storage1 && &mut storage2 || &mut storage3 && !&storage4);
/// ```
///
/// Here, we first provide a bitset. This is due to a limitation with rust
/// macros where creating variables inside of the macro and returning them
/// is not allowed.
///
/// Then, we tell join to join over all entities that have:
/// - A component in storage1
/// - A component in either storage2 or storage3
/// - No component in storage4
///
/// We also specify that storage2 and storage3 should be accessed mutably.
///
/// Finally, we can iterate:
/// ```rust,ignore
/// iter.for_each(|(component1, mut component2, mut component3, _)| {});
/// ```
/// This iterator will be of type `(Option<&T1>, Option<&mut T2>, ...)`.
#[macro_export]
macro_rules! join {
    (&$st:ident) => {
        $st.iter()
    };
    (&mut $st:ident) => {
        $st.iter_mut()
    };
    ($($complex:tt)*) => {
        {
            // TODO find a way to avoid having this first vec allocation.
            let mut bitset = std::rc::Rc::new(vec![]);
            gen_bitset!(bitset; $($complex)*);
            let iter = iter_bitset!(bitset ; ; $($complex)*);
            iter
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn join_components() {
        struct A;
        struct B;
        let comp1 = Components::<A>::default();
        let comp2 = Components::<B>::default();
        join!(&comp1 && &comp2).for_each(|_| {});
    }

    #[test]
    fn complex_join() {
        struct A;
        struct B;
        struct C;
        let mut storage1 = Components::<A>::default();
        let storage2 = Components::<B>::default();
        let storage3 = Components::<C>::default();
        let mut count = 0;
        join!(&mut storage1 && &storage2 || !&storage3).for_each(|(_a, _b, _c)| {
            count += 1;
        });
        assert_eq!(count, 0);
    }

    #[test]
    fn start_with_not() {
        struct A;
        struct B;
        let comp1 = Components::<A>::default();
        let comp2 = Components::<B>::default();
        join!(!&comp1 && &comp2).for_each(|_| {});
    }
}

