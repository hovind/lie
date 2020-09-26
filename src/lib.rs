#![feature(const_generics)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use aljabar::{Matrix, Vector, Quaternion, InnerSpace, One, Real, Zero};
use std::ops::{Mul, Add};


pub trait GroupDef {
    type G;

    fn compose(lhs: &Self::G, rhs: &Self::G) -> Self::G;
    fn identity() -> Self::G;
    fn invert(g: &Self::G) -> Self::G;
}

pub trait LieGroupDef<T, const N: usize>: GroupDef {
    type Algebra;

    fn vee(a: &Self::Algebra) -> Vector<T, N>;
    fn hat(v: &Vector<T, N>) -> Self::Algebra;
}

struct GroupElt<Def> where
    Def: GroupDef,
{
    value: Def::G,
}

impl<Def> GroupElt<Def> where
    Def: GroupDef,
{
    pub fn new_from(g: Def::G) -> Self {
        GroupElt { value: g }
    }
    pub fn compose(&self, other: &Self) -> Self {
        Self::new_from(Def::compose(&self.value, &other.value))
    }
    pub fn identity() -> Self {
        Self::new_from(Def::identity())
    }
    pub fn inverse(&self) -> Self {
        Self::new_from(Def::invert(&self.value))
    }
}


struct SODef<T, const N: usize> {
    phantom: std::marker::PhantomData<Matrix<T, N, N>>,
}

impl<T, const N: usize> GroupDef for SODef<T, N> where
    T: Clone + PartialEq + Add<T, Output = T> + Mul<T, Output = T> + One + Real + Zero,
{
    type G = Matrix<T, N, N>;

    fn compose(lhs: &Self::G, rhs: &Self::G) -> Self::G {
        todo!()
        //Matrix::<T, N, N>::mul(lhs.clone(), rhs.clone())
    }
    fn identity() -> Self::G {
        Matrix::<T, {N}, {N}>::one()
    }
    fn invert(g: &Self::G) -> Self::G {
        g.clone().transpose()
    }
}


impl<T, const N: usize> LieGroupDef<T, N> for SODef<T, N> where
    SODef<T, N>: GroupDef,
{
    type Algebra = Matrix<T, N, N>;

    fn vee(a: &Self::Algebra) -> Vector<T, N> {
        todo!()
    }
    fn hat(v: &Vector<T, N>) -> Self::Algebra {
        todo!()
    }

}

struct QDef<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> GroupDef for QDef<T> where
    T: Clone + PartialEq + Add<T, Output = T> + Mul<T, Output = T> + One + Real + Zero,
{
    type G = Quaternion<T>;

    fn compose(lhs: &Self::G, rhs: &Self::G) -> Self::G {
        todo!()
        //Matrix::<T, N, N>::mul(lhs.clone(), rhs.clone())
    }
    fn identity() -> Self::G {
        Quaternion::<T>::one()
    }
    fn invert(g: &Self::G) -> Self::G {
        g.clone().conjugate()
    }
}

type Quat = GroupElt<QDef<f64>>;
