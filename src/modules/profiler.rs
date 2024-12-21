use std::{collections::HashMap, fmt::{Display, Formatter}, sync::{RwLock, RwLockReadGuard}};

/// Represents a snapshot of a profiled section of code.
#[derive(Debug, Clone)]
pub struct Snap {
    pub duration: f32
}

/// Facilitates recording of a Snap.
/// 
/// This guard is used to record the duration of a Snap.
/// It keeps track of when it was created, and when it was dropped.
/// It uses this information to deduce how long the Snap was active.
pub struct SnapRecordingGuard<'a> {
    snap: &'a mut Snap,
    start: std::time::Instant,
}

impl<'a> SnapRecordingGuard<'a> {
    pub fn new(snap: &'a mut Snap) -> Self {
        SnapRecordingGuard {
            snap,
            start: std::time::Instant::now(),
        }
    }
}

impl<'a> Drop for SnapRecordingGuard<'a> {
    fn drop(&mut self) {
        self.snap.duration = self.start.elapsed().as_secs_f32();
    }
}

impl Snap {
    pub fn new() -> Self {
        Snap {
            duration: 0.0
        }
    }

    pub fn record(&mut self) -> SnapRecordingGuard {
        SnapRecordingGuard::new(self)
    }
}

impl Display for Snap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Snap: {:.2}ms", self.duration * 1000.0)
    }
}

#[cfg(test)]
mod snap_unit_tests {
    use super::*;

    #[test]
    fn test_snap_record() {
        let mut snap = Snap::new();
        {
            let guard = snap.record();
            assert!(guard.start.elapsed().as_secs_f32() > 0.0);
        }
        assert!(snap.duration > 0.0);
    }
}

/// Represents a monitor for a section of code.
/// 
/// Monitors can be used to track the performance of a section of code.
/// 
/// # Example
/// 
pub struct Monitor {
    pub name: String,
    pub snaps: RwLock<Vec<Snap>>,
    /// To prevent multiple recordings at the same time being submitted,
    /// we use this flag to prevent it.
    pub create_new_snap: bool,
    pub max_snapshots: usize
}

pub struct MonitorIterator<'a> {
    index: usize,
    snaps: RwLockReadGuard<'a, Vec<Snap>>
}

impl<'a> Iterator for MonitorIterator<'a> {
    type Item = Snap;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.snaps.len() {
            let snap = &self.snaps[self.index];
            self.index += 1;
            Some(snap.clone())
        } else {
            None
        }
    }
}

/// Facilitates recording of a Snap within a Monitor.
/// 
/// This guard is used to record the duration of a Snap associated with a Monitor.
/// It keeps track of when it was created, and when it was dropped.
/// It uses this information to deduce how long the Snap was active.
/// Afterwards, it submits the Snap to the Monitor.
/// 
/// If multiple SnapRecordingGuards are created at the same time, only one new Snap will be created.
/// However, all SnapRecordingGuards will instead combine the time they were active into the same Snap.
pub struct MonitorRecordingGuard<'a> {
    monitor: &'a mut Monitor,
    snap: Snap
}

impl<'a> MonitorRecordingGuard<'a> {
    pub fn new(monitor: &'a mut Monitor, snap: Snap) -> Self {
        MonitorRecordingGuard {
            monitor,
            snap
        }
    }
}

impl<'a> Drop for MonitorRecordingGuard<'a> {
    fn drop(&mut self) {
        let snaps = &mut self.monitor.snaps.write().unwrap();
        match self.monitor.create_new_snap {
            true => {
                snaps.push(self.snap.clone());
            },
            false => {
                let last = snaps.len() - 1;
                let last_snap = &mut snaps[last];
                last_snap.duration += self.snap.duration;
            }
        }
    }
}

impl<'a> Monitor {
    const DEFAULT_MAX_SNAPSHOTS: usize = 100;

    pub fn new(name: &str) -> Self {
        Monitor {
            name: name.to_string(),
            snaps: RwLock::new(Vec::new()),
            create_new_snap: false,
            max_snapshots: Self::DEFAULT_MAX_SNAPSHOTS
        }
    }

    pub fn record(&mut self) -> Option<MonitorRecordingGuard> {
        match self.create_new_snap {
            true => None,
            false => {
                self.create_new_snap = true;
                Some(MonitorRecordingGuard::new(self, Snap::new()))
            }
        }
    }

    pub fn average(&self) -> f32 {
        let snaps = self.snaps.read().unwrap();
        let mut total = 0.0;
        for snap in snaps.iter() {
            total += snap.duration;
        }

        total / snaps.len() as f32
    }

    pub fn percentile(&self, percentile: f32) -> f32 {
        let snaps = self.snaps.read().unwrap();
        let mut sorted = snaps.clone();
        sorted.sort_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap());
        let index = (percentile * snaps.len() as f32) as usize;
        sorted[index].duration
    }

    pub fn iter(&self) -> MonitorIterator{
        return MonitorIterator {
            index: 0,
            snaps: self.snaps.read().unwrap()
        }
    }

    pub fn len(&self) -> usize {
        let snaps = self.snaps.read().unwrap();
        snaps.len()
    }

    pub fn latest(&self) -> Option<Snap> {
        let snaps = self.snaps.read().unwrap();
        if snaps.len() > 0 {
            Some(snaps[snaps.len() - 1].clone())
        } else {
            None
        }
    }

    pub fn cull(&mut self, max_snapshots: usize) {
        let mut snaps = self.snaps.write().unwrap();
        if snaps.len() > max_snapshots {
            snaps.truncate(max_snapshots);
        }
    }

    pub fn clear(&mut self) {
        let mut snaps = self.snaps.write().unwrap();
        snaps.clear();
    }

    pub fn new_frame(&mut self) {
        self.create_new_snap = false;
    }
}

impl Display for Monitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let snaps = self.snaps.read().unwrap();
        write!(f, "Monitor: ({} snaps, avg: {:.2}ms, 1%: {:.2}ms)", snaps.len(), self.average() * 1000.0, self.percentile(0.01) * 1000.0)
    }
}

#[cfg(test)]
mod monitor_unit_tests {
    use super::*;

    #[test]
    fn test_monitor_record() {
        let mut monitor = Monitor::new("test");
        let guard = monitor.record();
        assert!(guard.is_some());
    }

    #[test]
    fn test_monitor_latest() {
        let mut monitor = Monitor::new("test");
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        let last = monitor.latest();
        assert!(last.is_some());
    }

    #[test]
    fn test_monitor_cull() {
        let mut monitor = Monitor::new("test");
        monitor.max_snapshots = 1;
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        monitor.cull(1);
        assert_eq!(monitor.snaps.read().unwrap().len(), 1);
    }

    #[test]
    fn test_monitor_iter() {
        let mut monitor = Monitor::new("test");
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        let iter = monitor.iter();
        assert!(iter.count() > 0);
    }

    #[test]
    fn test_monitor_average() {
        
        let mut monitor = Monitor::new("test");
        let avg: f32;
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        {
            // Calculate the average of the two snapshots.
            avg = monitor.iter().map(|snap| snap.duration).sum::<f32>() / 2.0;
        }
        assert_eq!(monitor.average(), avg);
    }

    #[test]
    fn test_monitor_clear() {
        let mut monitor = Monitor::new("test");
        {
            let guard = monitor.record();
            assert!(guard.is_some());
        }
        monitor.clear();
        assert_eq!(monitor.len(), 0);
    }
}

pub struct Profiler {
    pub monitors: HashMap<String, Monitor>
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            monitors: HashMap::new()
        }
    }

    pub fn monitor(&mut self, name: &str) -> &mut Monitor {
        if !self.monitors.contains_key(name) {
            self.monitors.insert(name.to_string(), Monitor::new(name));
        }

        self.monitors.get_mut(name).unwrap()
    }

    pub fn clear(&mut self) {
        for (_, monitor) in self.monitors.iter_mut() {
            monitor.clear();
        }
    }
}

#[cfg(test)]
mod profiler_unit_tests {
    use super::*;

    #[test]
    fn test_profiler_monitor() {
        let mut profiler = Profiler::new();
        let monitor = profiler.monitor("test");
        assert_eq!(monitor.name, "test");
    }
}