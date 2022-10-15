pub trait Sensor: std::any::Any + Send + Sync {
    fn sample(&self) -> Result<Vec<SensorOutput>, std::io::Error>;
}

#[derive(Clone, Debug)]
pub enum SensorOutput {
    /// Process sensing output
    Process { pid: u32, stat: ProcessStats },
    /// Mount point sensing output
    MountPoint { name: String, size: u64, free: u64 },
}

#[derive(Clone, Debug)]
pub enum ProcessStats {
    /// Reports an error from the sensor with a description.
    Error(String),
    /// Contents of the Stat
    Stat {
        /// The filename of the executable, in parentheses.
        ///
        /// This is visible whether or not the executable is swapped out.
        ///
        /// Note that if the actual comm field contains invalid UTF-8 characters, they will be replaced
        /// here by the U+FFFD replacement character.
        comm: String,
        /// Process State.
        ///
        /// See [state()](#method.state) to get the process state as an enum.
        state: char,
        /// The PID of the parent of this process.
        ppid: i32,
        /// The process group ID of the process.
        pgrp: i32,
        /// The session ID of the process.
        session: i32,
        /// The controlling terminal of the process.
        ///
        /// The minor device number is contained in the combination of bits 31 to 20 and  7  to  0;
        /// the major device number is in bits 15 to 8.
        ///
        /// See [tty_nr()](#method.tty_nr) to get this value decoded into a (major, minor) tuple
        tty_nr: i32,
        /// The ID of the foreground process group of the controlling terminal of the process.
        tpgid: i32,
        /// The kernel flags  word of the process.
        ///
        /// For bit meanings, see the PF_* defines in  the  Linux  kernel  source  file
        /// [`include/linux/sched.h`](https://github.com/torvalds/linux/blob/master/include/linux/sched.h).
        ///
        /// See [flags()](#method.flags) to get a [`StatFlags`](struct.StatFlags.html) bitfield object.
        flags: u32,
        /// The number of minor faults the process has made which have not required loading a memory
        /// page from disk.
        minflt: u64,
        /// The number of minor faults that the process's waited-for children have made.
        cminflt: u64,
        /// The number of major faults the process has made which have required loading a memory page
        /// from disk.
        majflt: u64,
        /// The number of major faults that the process's waited-for children have made.
        cmajflt: u64,
        /// Amount of time that this process has been scheduled in user mode, measured in clock ticks
        /// (divide by [`ticks_per_second()`](crate::ticks_per_second).
        ///
        /// This includes guest time, guest_time (time spent running a virtual CPU, see below), so that
        /// applications that are not aware of the guest time field  do not lose that time from their
        /// calculations.
        utime: u64,
        /// Amount of time that this process has been scheduled in kernel mode, measured in clock ticks
        /// (divide by [`ticks_per_second()`](crate::ticks_per_second)).
        stime: u64,

        /// Amount  of  time  that  this  process's  waited-for  children  have  been  scheduled  in
        /// user  mode,  measured  in clock ticks (divide by [`ticks_per_second()`](crate::ticks_per_second)).
        ///
        /// This includes guest time, cguest_time (time spent running a virtual CPU, see below).
        cutime: i64,

        /// Amount of time that this process's waited-for  children  have  been  scheduled  in  kernel
        /// mode,  measured  in  clock  ticks  (divide  by [`ticks_per_second()`](crate::ticks_per_second)).
        cstime: i64,
        /// For processes running a real-time scheduling policy (policy below; see sched_setscheduler(2)),
        /// this is the negated scheduling priority, minus one;
        ///
        /// That is, a number in the range -2 to -100,
        /// corresponding to real-time priority 1 to 99.  For processes running under a non-real-time
        /// scheduling policy, this is the raw nice value (setpriority(2)) as represented in the kernel.
        /// The kernel stores nice values as numbers in the range 0 (high) to 39  (low),  corresponding
        /// to the user-visible nice range of -20 to 19.
        /// (This explanation is for Linux 2.6)
        ///
        /// Before Linux 2.6, this was a scaled value based on the scheduler weighting given to this process.
        priority: i64,
        /// The nice value (see `setpriority(2)`), a value in the range 19 (low priority) to -20 (high priority).
        nice: i64,
        /// Number  of  threads in this process (since Linux 2.6).  Before kernel 2.6, this field was
        /// hard coded to 0 as a placeholder for an earlier removed field.
        num_threads: i64,
        /// The time in jiffies before the next SIGALRM is sent to the process due to an interval
        /// timer.
        ///
        /// Since kernel 2.6.17, this  field is no longer maintained, and is hard coded as 0.
        itrealvalue: i64,
        /// The time the process started after system boot.
        ///
        /// In kernels before Linux 2.6, this value was expressed in  jiffies.  Since  Linux 2.6, the
        /// value is expressed in clock ticks (divide by `sysconf(_SC_CLK_TCK)`).
        ///
        starttime: u64,
        /// Virtual memory size in bytes.
        vsize: u64,
        /// Resident Set Size: number of pages the process has in real memory.
        ///
        /// This is just the pages which count toward text,  data,  or stack space.
        /// This does not include pages which have not been demand-loaded in, or which are swapped out.
        rss: i64,
        /// Current soft limit in bytes on the rss of the process; see the description of RLIMIT_RSS in
        /// getrlimit(2).
        rsslim: u64,
        /// The address above which program text can run.
        startcode: u64,
        /// The address below which program text can run.
        endcode: u64,
        /// The address of the start (i.e., bottom) of the stack.
        startstack: u64,
        /// The current value of ESP (stack pointer), as found in the kernel stack page for the
        /// process.
        kstkesp: u64,
        /// The current EIP (instruction pointer).
        kstkeip: u64,
        /// The  bitmap of pending signals, displayed as a decimal number.  Obsolete, because it does
        /// not provide information on real-time signals; use `/proc/<pid>/status` instead.
        signal: u64,
        /// The bitmap of blocked signals, displayed as a decimal number.  Obsolete, because it does
        /// not provide information on  real-time signals; use `/proc/<pid>/status` instead.
        blocked: u64,
        /// The  bitmap of ignored signals, displayed as a decimal number.  Obsolete, because it does
        /// not provide information on real-time signals; use `/proc/<pid>/status` instead.
        sigignore: u64,
        /// The bitmap of caught signals, displayed as a decimal number.  Obsolete, because it does not
        /// provide information  on  real-time signals; use `/proc/<pid>/status` instead.
        sigcatch: u64,
        /// This  is  the  "channel"  in which the process is waiting.  It is the address of a location
        /// in the kernel where the process is sleeping.  The corresponding symbolic name can be found in
        /// `/proc/<pid>/wchan`.
        wchan: u64,
        /// Number of pages swapped **(not maintained)**.
        nswap: u64,
        /// Cumulative nswap for child processes **(not maintained)**.
        cnswap: u64,
        /// Signal to be sent to parent when we die.
        ///
        /// (since Linux 2.1.22)
        exit_signal: Option<i32>,
        /// CPU number last executed on.
        ///
        /// (since Linux 2.2.8)
        processor: Option<i32>,
        /// Real-time scheduling priority
        ///
        ///  Real-time scheduling priority, a number in the range 1 to 99 for processes scheduled under a real-time policy, or 0, for non-real-time processes
        ///
        /// (since Linux 2.5.19)
        rt_priority: Option<u32>,
        /// Scheduling policy (see sched_setscheduler(2)).
        ///
        /// Decode using the `SCHED_*` constants in `linux/sched.h`.
        ///
        /// (since Linux 2.5.19)
        policy: Option<u32>,
        /// Aggregated block I/O delays, measured in clock ticks (centiseconds).
        ///
        /// (since Linux 2.6.18)
        delayacct_blkio_ticks: Option<u64>,
        /// Guest time of the process (time spent running a virtual CPU for a guest operating system),
        /// measured in clock ticks (divide by [`ticks_per_second()`](crate::ticks_per_second))
        ///
        /// (since Linux 2.6.24)
        guest_time: Option<u64>,
        /// Guest time of the process's children, measured in clock ticks (divide by
        /// [`ticks_per_second()`](crate::ticks_per_second)).
        ///
        /// (since Linux 2.6.24)
        cguest_time: Option<i64>,
        /// Address above which program initialized and uninitialized (BSS) data are placed.
        ///
        /// (since Linux 3.3)
        start_data: Option<u64>,
        /// Address below which program initialized and uninitialized (BSS) data are placed.
        ///
        /// (since Linux 3.3)
        end_data: Option<u64>,
        /// Address above which program heap can be expanded with brk(2).
        ///
        /// (since Linux 3.3)
        start_brk: Option<u64>,
        /// Address above which program command-line arguments (argv) are placed.
        ///
        /// (since Linux 3.5)
        arg_start: Option<u64>,
        /// Address below program command-line arguments (argv) are placed.
        ///
        /// (since Linux 3.5)
        arg_end: Option<u64>,
        /// Address above which program environment is placed.
        ///
        /// (since Linux 3.5)
        env_start: Option<u64>,
        /// Address below which program environment is placed.
        ///
        /// (since Linux 3.5)
        env_end: Option<u64>,
        /// The thread's exit status in the form reported by waitpid(2).
        ///
        /// (since Linux 3.5)
        exit_code: Option<i32>,
    },
}

unsafe impl Send for SensorOutput {}
