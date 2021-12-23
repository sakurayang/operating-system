use ulid::Ulid;

pub type Time = u32;
pub type TimeVec = Vec<Time>;
pub type Sequence = Vec<usize>;

#[derive(Copy, Clone, Debug)]
pub struct PCB {
    id: Ulid,
    /// 到达时间
    pub arrive: Time,
    /// 开始时间
    pub begin: Time,
    /// 结束时间
    pub end: Time,
    /// 剩余时间
    pub remain: Time,
    /// 需求时间
    need: Time,
}

impl PCB {
    pub fn new(need: Time) -> PCB {
        PCB {
            id: Ulid::new(),
            arrive: 0,
            begin: 0,
            end: 0,
            remain: need,
            need,
        }
    }
    pub fn id(self) -> Ulid {
        self.id
    }
    pub fn need(self) -> Time {
        self.need
    }
    pub fn run_time(self) -> Time {
        self.end - self.begin
    }
    pub fn wait_time(self) -> Time {
        self.begin - self.arrive
    }
    pub fn turnaround(self) -> Time {
        self.end - self.arrive
    }
    pub fn weighted_turnaround(self) -> f64 {
        self.turnaround() as f64 / self.need() as f64
    }
    /// return if it can run
    pub fn run(&mut self, current: Time) -> bool {
        let can_run = if self.need == self.remain {
            // first
            self.begin = current;
            true
        } else if self.remain == 1 {
            // last
            self.end = current;
            true
        } else if self.remain == 0 {
            false
        } else { true };
        if self.remain != 0 {
            self.remain -= 1;
        }
        return can_run;
    }
}
