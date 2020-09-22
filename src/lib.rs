pub trait Sensor: std::any::Any + Send+ Sync {
    fn sample(&self) -> Result<Vec<SensorOutput>, std::io::Error>;
}

#[derive(Clone, Debug)]
pub enum SensorOutput {
    Process {
        pid: u32
    },
    MountPoint {
        name: String,
        size: u64,
        free: u64
    }
}

unsafe impl Send for SensorOutput {

}
