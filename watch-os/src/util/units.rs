enum Time {
    Ticks(u32),
    Milliseconds(f32),
    Seconds(f32),
}

impl Time {
    fn to_seconds(self) -> Time {
        Time::Seconds(match self {
            Time::Ticks(t) => todo!("AAAAAAAAAAAAAAAAAAAAAAAAAAA conversertiojn"),
            Time::Milliseconds(t) => t / 1000.0,
            Time::Seconds(t) => t,
        })
    }
}
            
