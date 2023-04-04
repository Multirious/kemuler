use crate::control_flow::IntoDuration;

pub trait EmulateBinaryState: EmulateAbsoluteValue<Value = bool> {
    fn activate(&mut self) -> &mut Self {
        self.change_to(true);
        self
    }

    fn deactivate(&mut self) -> &mut Self {
        self.change_to(false);
        self
    }

    fn pulse(&mut self) -> &mut Self {
        self.activate();
        self.deactivate();
        self
    }

    fn pulse_for<D: IntoDuration>(&mut self, duration: D) -> &mut Self {
        self.activate();
        std::thread::sleep(duration.into_duration());
        self.deactivate();
        self
    }
}

impl<T: EmulateAbsoluteValue<Value = bool>> EmulateBinaryState for T {}

pub trait EmulateAbsoluteValue {
    type Value;
    fn change_to(&mut self, to: Self::Value) -> &mut Self;
}

pub trait EmulateRelativeValue {
    type Value;
    fn change_by(&mut self, by: Self::Value) -> &mut Self;
}
