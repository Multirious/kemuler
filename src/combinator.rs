use crate::simulate::{SimulateAbsoluteValue, SimulateRelativeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct And<A, B>(pub A, pub B);

pub trait Combine {
    fn and<Rhs>(self, rhs: Rhs) -> And<Self, Rhs>
    where
        Self: Sized,
    {
        And(self, rhs)
    }
}

impl<A, B> Combine for And<A, B> {}

impl<V, A, B> SimulateAbsoluteValue for And<A, B>
where
    A: SimulateAbsoluteValue<Value = V>,
    B: SimulateAbsoluteValue<Value = V>,
    V: Clone,
{
    type Value = V;

    fn change_to(&mut self, to: Self::Value) -> &mut Self {
        self.0.change_to(to.clone());
        self.1.change_to(to);
        self
    }
}

impl<V, A, B> SimulateRelativeValue for And<A, B>
where
    A: SimulateRelativeValue<Value = V>,
    B: SimulateRelativeValue<Value = V>,
    V: Clone,
{
    type Value = V;

    fn change_by(&mut self, by: Self::Value) -> &mut Self {
        self.0.change_by(by.clone());
        self.1.change_by(by);
        self
    }
}
