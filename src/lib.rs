pub trait Sensor: std::any::Any + Send+ Sync {
    fn sample(&self) -> Result<Vec<SensorOutput>, std::io::Error>;
}

#[derive(Clone, Debug)]
pub enum SensorOutput {
    Process {
        pid: u32
    }
}

unsafe impl Send for SensorOutput {

}
