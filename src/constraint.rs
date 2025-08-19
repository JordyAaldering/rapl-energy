use std::{fs::OpenOptions, io::{self, Read, Seek, SeekFrom, Write}};

#[derive(Debug)]
pub struct ConstraintHandle {
    rapl_root: String,
    id: u8,
}

#[derive(Debug)]
pub struct Constraint {
    handle: ConstraintHandle,
    /// constraint_X_name (ro) (optional)
    /// An optional name of the constraint
    pub name: Option<String>,
    /// constraint_X_power_limit_uw (rw) (required)
    /// Power limit in micro watts, which should be applicable for the time window specified by “constraint_X_time_window_us”.
    pub power_limit_uw: u64,
    /// constraint_X_time_window_us (rw) (required, but may be empty for peak_power)
    /// Time window in micro seconds.
    pub time_window_us: u64,
    /// constraint_X_min_power_uw (ro) (optional)
    /// Minimum allowed power in micro watts.
    pub min_power_uw: Option<u64>,
    /// constraint_X_max_power_uw (ro) (optional)
    /// Maximum allowed power in micro watts.
    pub max_power_uw: Option<u64>,
    /// constraint_X_min_time_window_us (ro) (optional)
    /// Minimum allowed time window in micro seconds.
    pub min_time_window_us: Option<u64>,
    /// constraint_X_max_time_window_us (ro) (optional)
    /// Maximum allowed time window in micro seconds.
    pub max_time_window_us: Option<u64>,
}

impl Constraint {
    /// rapl_path: e.g. `/sys/class/powercap/intel-rapl:0:0`
    pub fn now(rapl_root: &str, id: u8) -> Option<Self> {
        let handle = ConstraintHandle::new(rapl_root, id);
        Some(Self {
            power_limit_uw:     handle.read("power_limit_uw")?,
            time_window_us:     handle.read("time_window_us")?,
            name:               handle.read("name"),
            min_power_uw:       handle.read("min_power_uw"),
            max_power_uw:       handle.read("max_power_uw"),
            min_time_window_us: handle.read("min_time_window_us"),
            max_time_window_us: handle.read("max_time_window_us"),
            handle,
        })
    }

    pub fn set_power_limit_uw(&mut self, value: u64) -> Result<(), io::Error> {
        self.handle.write("power_limit_uw", value)
    }

    pub fn set_time_window_us(&mut self, value: u64) -> Result<(), io::Error> {
        self.handle.write("time_window_us", value)
    }
}

impl ConstraintHandle {
    fn new(rapl_root: &str, id: u8) -> Self {
        Self { rapl_root: rapl_root.to_string(), id }
    }

    fn read<T: Default + std::str::FromStr>(&self, field: &str) -> Option<T> where T::Err: std::fmt::Debug {
        let path = format!("{}/constraint_{}_{}", self.rapl_root, self.id, field);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;

        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            // This case occurs for constraint_2_time_window_us.
            // Constraint 2 is peak_power, which does not have a time window.
            return Some(T::default())
        }

        let buf = buf.trim();
        Some(buf.parse::<T>().expect(&format!("Could not parse {}", buf)))
    }

    fn write(&self, field: &str, value: u64) -> Result<(), io::Error> {
        let path = format!("{}/constraint_{}_{}", self.rapl_root, self.id, field);
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        //file.write(&value.to_ne_bytes())
        file.seek(SeekFrom::Start(0))?;
        write!(file, "{}", value)
    }
}
