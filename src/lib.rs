pub trait Sensor: std::any::Any + Send+ Sync {
    type Item;
    fn sample(&self) -> Result<Vec<Self::Item>, std::io::Error>;
}

pub struct Process {
    pub pid: u32
}
