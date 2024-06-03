use uom::si::Unit;

pub trait SensorData {
    async fn update(&mut self);
    fn inc_dependents(&mut self);
    fn dec_dependents(&mut self) -> bool;
}

pub trait SensorData1Axis<U: Unit>: SensorData {
}

pub trait SensorData3Axis<U: Unit>: SensorData {
    
}

pub struct MagnetometerData: 
