#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(generic_associated_types)]

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        type Quat = GroupElt<QDef<f64>>;
        type SO<const N: usize> = GroupElt<SODef<f64, N>>;

        assert_eq!(2 + 2, 4);
    }
}

use aljabar::{Matrix, Vector, Quaternion, InnerSpace, MetricSpace, One, Real, RealInnerSpace, VectorSpace, Zero};
use std::ops::{Mul, Add};


pub trait GroupDef {
    type G;

    fn compose(lhs: Self::G, rhs: Self::G) -> Self::G;
    fn identity() -> Self::G;
    fn invert(g: Self::G) -> Self::G;
}

pub trait LieGroupDef: GroupDef {
    type Algebra;
    type Euclidean;

    fn vee(a: Self::Algebra) -> Self::Euclidean;
    fn hat(v: Self::Euclidean) -> Self::Algebra;

    fn exp(a: Self::Algebra) -> Self::G;
    fn log(g: Self::G) -> Self::Algebra;

    #[allow(non_snake_case)]
    fn Exp(v: Self::Euclidean) -> Self::G {
        Self::exp(Self::hat(v))
    }
    #[allow(non_snake_case)]
    fn Log(g: Self::G) -> Self::Euclidean {
        Self::vee(Self::log(g))
    }
}

pub struct GroupElt<Def> where
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
    pub fn compose(self, other: Self) -> Self {
        Self::new_from(Def::compose(self.value, other.value))
    }
    pub fn identity() -> Self {
        Self::new_from(Def::identity())
    }
    pub fn inverse(self) -> Self {
        Self::new_from(Def::invert(self.value))
    }
}

impl<Def> GroupElt<Def> where
    Def: LieGroupDef,
{

    pub fn vee(a: Def::Algebra) -> Def::Euclidean {
        Def::vee(a)
    }
    pub fn hat(v: Def::Euclidean) -> Def::Algebra {
        Def::hat(v)
    }
    pub fn exp(a: Def::Algebra) -> Self {
        Self::new_from(Def::exp(a))
    }
    pub fn log(self) -> Def::Algebra {
        Def::log(self.value)
    }
    #[allow(non_snake_case)]
    pub fn Exp(v: Def::Euclidean) -> Self {
        Self::new_from(Def::Exp(v))
    }
    #[allow(non_snake_case)]
    pub fn Log(self) -> Def::Euclidean {
        Def::Log(self.value)
    }
}

impl<Def> Add<Def::Euclidean> for GroupElt<Def> where
    Def: LieGroupDef,
{
    type Output = Self;

    fn add(self, other: Def::Euclidean) -> Self::Output {
        self.compose(Self::Exp(other))
    }
}

struct SODef<T, const N: usize> {
    phantom: std::marker::PhantomData<Matrix<T, N, N>>,
}

impl<T, const N: usize> GroupDef for SODef<T, N> where
    T: Clone + PartialEq + Add<T, Output = T> + Mul<T, Output = T> + One + Real + Zero,
    T: Add<T, Output = T> + Mul<T, Output = T> + Clone,
    Vector<T, { N }>: InnerSpace<Scalar = T>,
{
    type G = Matrix<T, N, N>;

    fn compose(lhs: Self::G, rhs: Self::G) -> Self::G {
        lhs * rhs
    }
    fn identity() -> Self::G {
        Matrix::<T, {N}, {N}>::one()
    }
    fn invert(g: Self::G) -> Self::G {
        g.transpose()
    }
}


/*impl<T, const N: usize> LieGroupDef for SODef<T, N> where
    SODef<T, N>: GroupDef,
{
    type Algebra = Matrix<T, N, N>;
    type Euclidean = Vector<T, N>;

    fn vee(a: Self::Algebra) -> Vector<T, N> {
        todo!()
    }
    fn hat(v: Vector<T, N>) -> Self::Algebra {
        todo!()
    }
    fn exp(a: Self::Algebra) -> Self::G {
        todo!()
    }
    fn log(g: Self::G) -> Self::Algebra {
        todo!()
    }
    fn Exp(v: Self::Euclidean) -> Self::G {
        todo!()
    }
    fn Log(g: Self::G) -> Self::Euclidean {
        todo!()
    }

}*/

struct QDef<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> GroupDef for QDef<T> where
    T: Clone + PartialEq + Add<T, Output = T> + Mul<T, Output = T> + One + Real + Zero,
{
    type G = Quaternion<T>;

    fn compose(lhs: Self::G, rhs: Self::G) -> Self::G {
        lhs * rhs
    }
    fn identity() -> Self::G {
        Quaternion::<T>::one()
    }
    fn invert(g: Self::G) -> Self::G {
        g.conjugate()
    }
}

impl<T> LieGroupDef for QDef<T> where
    T: Clone + Mul<Vector<T, 3>, Output = Vector<T, 3>> + One + Real,
    QDef<T>: GroupDef<G = Quaternion<T>>,
    Vector<T, 3>: InnerSpace + MetricSpace<Metric = T> + RealInnerSpace + VectorSpace<Scalar = T>,
{
    type Algebra = Vector<T, 3>;
    type Euclidean = Vector<T, 3>;

    fn vee(a: Self::Algebra) -> Self::Euclidean {
        T::one().div2() * a
    }
    fn hat(v: Self::Euclidean) -> Self::Algebra {
        T::one().mul2() * v
    }
    fn exp(a: Self::Algebra) -> Self::G {
        let phi = a.clone().magnitude();
        Quaternion::from_sv(T::cos(phi.clone()), T::sin(phi) * a.normalize())
    }
    fn log(g: Self::G) -> Self::Algebra {
        todo!()
    }
}

