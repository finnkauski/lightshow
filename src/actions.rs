use lighthouse::hue::HueBridge;

pub trait Action {
    fn execute_on(bridge: HueBridge);
}

pub struct Wait {
    duration_s: u16,
}

pub struct Blink {
    speed: u16,
    pause: u16,
    color: String,
}
