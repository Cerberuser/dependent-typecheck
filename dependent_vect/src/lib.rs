use dependent::*;
use dependent_nat::*;
use std::marker::PhantomData;
use std::ops::Index;

#[derive(Clone)]
pub struct Vect<T, N: Nat>(Vec<T>, PhantomData<N>);
impl<Item: Clone, N: Nat> Dependent for Vect<Item, N> {
    type Native = Vec<Item>;
    type Frozen = [Item];
    fn freeze(&self) -> &Self::Frozen {
        self.0.as_slice()
    }
    fn freeze_mut(&mut self) -> &mut Self::Frozen {
        self.0.as_mut_slice()
    }
    fn into_native(self) -> Self::Native {
        self.0
    }
    fn as_native(&self) -> &Self::Native {
        &self.0
    }
}

pub fn collect<Item: Clone, N: Nat, N2: Nat + From<N>, I: IntoIterator<Item = Item>>(
    iter: I,
) -> (N, Vect<Item, N>) {
    let inner: Vec<_> = iter.into_iter().collect();
    (expect_nat(inner.len()), Vect(inner, PhantomData))
}

#[macro_export]
macro_rules! vect {
    ($data:expr) => {
        ::dependent_nat::with_n! {
            let v = $data;
            $crate::collect::<_, _, N, _>(v)
        }
    };
}

impl<Item: Clone, N: Nat> Vect<Item, N> {
    pub fn retag<New: Nat>(self, _proof: Equiv<N, New>) -> Vect<Item, New> {
        Vect(self.0, PhantomData)
    }
}

impl<Item: Clone, N: Nat> Index<Fin<N>> for Vect<Item, N> {
    type Output = Item;
    fn index(&self, index: Fin<N>) -> &Item {
        unsafe { self.0.get_unchecked(index.as_usize()) }
    }
}
