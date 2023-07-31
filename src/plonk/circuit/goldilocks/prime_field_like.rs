use super::*;

use boojum::field::traits::field_like::PrimeFieldLike;
use boojum::field::goldilocks::GoldilocksField as GL;

#[derive(Derivative)]
#[derivative(Clone, Copy, Debug(bound = ""), Hash(bound = ""))]
pub struct GoldilocksAsFieldWrapper<E: Engine, CS: ConstraintSystem<E>> {
    inner: GoldilocksField<E>,
    #[derivative(Debug = "ignore", Hash = "ignore")]
    _marker: std::marker::PhantomData<fn() -> CS>,
}

impl<E: Engine, CS: ConstraintSystem<E>> From<GoldilocksField<E>> for GoldilocksAsFieldWrapper<E, CS> {
    fn from(value: GoldilocksField<E>) -> Self {
        Self {
            inner: value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E: Engine, CS: ConstraintSystem<E>> std::fmt::Display for GoldilocksAsFieldWrapper<E, CS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Num as PrimeFieldLike{{")?;
        writeln!(f, "inner num: {:?},", self.inner.inner)?;
        writeln!(f, "}}")
    }
}

impl<E: Engine, CS: ConstraintSystem<E>> PrimeFieldLike for GoldilocksAsFieldWrapper<E, CS>
where
    CS: 'static,
{
    type Base = GL;
    type Context = CS;

    // identities
    fn zero(_ctx: &mut Self::Context) -> Self {
        let inner = GoldilocksField::zero();
        inner.into()
    }

    fn one(_ctx: &mut Self::Context) -> Self {
        let inner = GoldilocksField::one();
        inner.into()
    }

    fn minus_one(_ctx: &mut Self::Context) -> Self {
        let inner = GoldilocksField::minus_one();
        inner.into()
    }

    // Arithmetics. Expressed in mutable way. It would not matter in after inlining
    fn add_assign(&'_ mut self, other: &Self, ctx: &mut Self::Context) -> &'_ mut Self {
        let new = self.inner.add(ctx, &other.inner).unwrap();
        *self = new.into();

        self
    }

    fn sub_assign(&'_ mut self, other: &Self, ctx: &mut Self::Context) -> &'_ mut Self {
        let mut other_negate = *other;
        other_negate.negate(ctx);
        self.add_assign(&other_negate, ctx);

        self
    }

    fn mul_assign(&'_ mut self, other: &Self, ctx: &mut Self::Context) -> &'_ mut Self {
        let new = self.inner.mul(ctx, &other.inner).unwrap();
        *self = new.into();

        self
    }

    fn square(&'_ mut self, ctx: &mut Self::Context) -> &'_ mut Self {
        let this = self.inner;
        let new = self.inner.mul(ctx, &this).unwrap();
        *self = new.into();

        self
    }

    fn negate(&'_ mut self, ctx: &mut Self::Context) -> &'_ mut Self {
        self.inner = self.inner.negate(ctx).unwrap();

        self
    }

    fn double(&'_ mut self, ctx: &mut Self::Context) -> &'_ mut Self {
        let this = self.inner;
        let new = self.inner.add(ctx, &this).unwrap();
        *self = new.into();

        self
    }

    // infallible inverse
    fn inverse(&self, ctx: &mut Self::Context) -> Self {
        self.inner.inverse(ctx).unwrap().into()
    }

    // constant creation
    fn constant(value: Self::Base, _ctx: &mut Self::Context) -> Self {
        GoldilocksField::constant_from_field(value).into()
    }
}


#[derive(Derivative)]
#[derivative(Clone, Copy, Debug(bound = ""), Hash(bound = ""))]
pub struct GoldilocksExtAsFieldWrapper<E: Engine, CS: ConstraintSystem<E>> {
    inner: GoldilocksFieldExt<E>,
    #[derivative(Debug = "ignore", Hash = "ignore")]
    _marker: std::marker::PhantomData<fn() -> CS>,
}

impl<E: Engine, CS: ConstraintSystem<E>> From<GoldilocksFieldExt<E>> for GoldilocksExtAsFieldWrapper<E, CS> {
    fn from(value: GoldilocksFieldExt<E>) -> Self {
        Self {
            inner: value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E: Engine, CS: ConstraintSystem<E>> std::fmt::Display for GoldilocksExtAsFieldWrapper<E, CS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Num as PrimeFieldLike{{")?;
        writeln!(f, "inner field coeffs: {:?},", self.inner)?;
        writeln!(f, "}}")
    }
}

impl<E: Engine, CS: ConstraintSystem<E>> PrimeFieldLike for GoldilocksExtAsFieldWrapper<E, CS>
where
    CS: 'static,
{
    type Base = GL;
    type Context = CS;

    // identities
    fn zero(_ctx: &mut Self::Context) -> Self {
        let inner = GoldilocksFieldExt::zero();
        inner.into()
    }

    fn one(_ctx: &mut Self::Context) -> Self {
        let inner = GoldilocksFieldExt::one();
        inner.into()
    }

    fn minus_one(_ctx: &mut Self::Context) -> Self {
        let inner = GoldilocksFieldExt::minus_one();
        inner.into()
    }

    // Arithmetics. Expressed in mutable way. It would not matter in after inlining
    fn add_assign(&'_ mut self, other: &Self, ctx: &mut Self::Context) -> &'_ mut Self {
        let new = self.inner.add(ctx, &other.inner).unwrap();
        *self = new.into();

        self
    }

    fn sub_assign(&'_ mut self, other: &Self, ctx: &mut Self::Context) -> &'_ mut Self {
        let mut other_negate = *other;
        other_negate.negate(ctx);
        self.add_assign(&other_negate, ctx);

        self
    }

    fn mul_assign(&'_ mut self, other: &Self, ctx: &mut Self::Context) -> &'_ mut Self {
        let new = self.inner.mul(ctx, &other.inner).unwrap();
        *self = new.into();

        self
    }

    fn square(&'_ mut self, ctx: &mut Self::Context) -> &'_ mut Self {
        let this = self.inner;
        let new = self.inner.mul(ctx, &this).unwrap();
        *self = new.into();

        self
    }

    fn negate(&'_ mut self, ctx: &mut Self::Context) -> &'_ mut Self {
        self.inner = self.inner.negate(ctx).unwrap();

        self
    }

    fn double(&'_ mut self, ctx: &mut Self::Context) -> &'_ mut Self {
        let this = self.inner;
        let new = self.inner.add(ctx, &this).unwrap();
        *self = new.into();

        self
    }

    // infallible inverse
    fn inverse(&self, ctx: &mut Self::Context) -> Self {
        self.inner.inverse(ctx).unwrap().into()
    }
    
    // constant creation
    fn constant(value: Self::Base, _ctx: &mut Self::Context) -> Self {
        GoldilocksFieldExt::constant_from_field(value).into()
    }
}
