//! # 高响应比优先 (Highest Response Ratio Next, HRRN)
//!
//! 周转时间 = 作业开始时间 - 到达时间 + 需求时间
//!
//! 优先权 = 周转时间 / 需求时间
//!
//! 带权周转时间 = 周转时间 / 需求时间
//!
//! $$T_{turnaround} = T_{begin} - T_{arrive} + T_{need}$$
//!
//! $$priority = \frac{T_{need} + T_{turnaround}}{T_{need}}$$
//!
//! $$T_{weighted turnaround} = \frac{T_{turnaround}} {T_{need}}$$
//!
//! 步骤：
//! 1. 根据队列中的作业进入时间排序
//! 2. 计算队列中所有作业的优先权，并排序，选出优先权最高者
//! 3. 将优先权最高者插入运行队列，并计算调度时间，将调度时间作为下一作业的开始时间
//! 4. 重复 2-3 直至队列完成
//! 5. 输出队列
//!
//! Steps:
//! 1. Sort by arrive time of job
//! 2. Calculate all priority for all jobs in array and choose the highest one
//! 3. Insert the highest one into running array and calculate schedule time, use it as begin time
//!    for next job
//! 4. Redo 3 and 3 till the array finish
//! 5. Output
//!

use std::collections::HashMap;
use ulid::Ulid;

#[derive(Copy, Clone, Debug)]
pub struct Time {
    /// 到达时间
    pub arrive: f64,
    /// 开始时间
    pub begin: f64,
    /// 结束时间
    end: f64,
    /// 周转时间
    pub turnaround: f64,
    /// 带权周转时间
    pub weighted_turnaround: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Resource {
    /// 所需运行时间
    pub time: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub id: Ulid,
    pub time: Time,
    pub resource: Resource,
    priority: f64,
}

impl Job {
    pub fn new(arrive_time: f64, need_time: f64) -> Job {
        let id = Ulid::new();
        return Job {
            id,
            time: Time {
                arrive: arrive_time,
                begin: 0.0,
                end: 0.0,
                turnaround: 0.0,
                weighted_turnaround: 0.0,
            },
            resource: Resource { time: need_time },
            priority: 0.0,
        };
    }
}

/// use [Gnome sort](https://wikipedia.org/wiki/Gnome_sort)
/// ```{r, tidy=FALSE, eval=FALSE, highlight=FALSE }
/// procedure gnomeSort(a[]):
///     pos := 0
///     while pos > length(a):
///         if (pos == 0 or a[pos] >= a[pos-1]):
///             pos := pos + 1
///         else:
///             swap a[pos] and a[pos-1]
///             pos := pos - 1
/// ```

fn sort_by_arrive_time(jobs: Vec<Job>) -> Vec<Job> {
    let mut sorted: Vec<Job> = Vec::new();
    sorted.extend(jobs);
    //[[], jobs].concat();

    let mut pos: usize = 0;
    while pos < sorted.len() {
        if pos == 0 || sorted[pos].time.arrive > sorted[pos - 1].time.arrive {
            pos += 1;
        } else {
            sorted.swap(pos, pos - 1);
            pos -= 1;
        }
    }

    return sorted;
}

fn calc_priority(begin_time: f64, arrive_time: f64, need_time: f64) -> f64 {
    let turnaround_time = begin_time - arrive_time;
    let priority = turnaround_time / need_time + 1.0;
    // println!("({} - {} + {}) / {} = {}", begin_time, arrive_time, need_time, need_time, priority);
    return priority;
}

fn get_highest_priority_job_index(jobs: &Vec<Job>, last_job_end_time: f64) -> usize {
    // index -> priority
    let mut priority_map: HashMap<usize, f64> = HashMap::new();
    for index in 0..jobs.len() {
        let job = &jobs[index];
        let mut begin_time = job.time.arrive;

        if begin_time > last_job_end_time {
            if jobs.len() > 1 {
                continue;
            } else {
                priority_map.insert(
                    index,
                    calc_priority(begin_time, job.time.arrive, job.resource.time),
                );
            }
        } else {
            begin_time = last_job_end_time;
            priority_map.insert(
                index,
                calc_priority(begin_time, job.time.arrive, job.resource.time),
            );
        }
    }

    let mut pos: usize = 0;
    let mut key_list = Vec::from_iter(priority_map.keys());

    while pos < key_list.len() {
        if pos == 0
            || priority_map.get(key_list[pos]).unwrap()
                < priority_map.get(key_list[pos - 1]).unwrap()
        {
            pos += 1;
        } else {
            key_list.swap(pos, pos - 1);
            pos -= 1;
        }
    }

    // println!("{:?} - {:?}\n{:?}\n{:?}", last_job_end_time, jobs, priority_map, key_list);
    if key_list.is_empty() {
        0
    } else {
        *key_list[0]
    }
}

pub fn run(jobs: Vec<Job>) -> Vec<Job> {
    if jobs.len() == 0 {
        return Vec::new();
    }
    let mut sorted: Vec<Job> = Vec::new();
    // 1
    let mut run_array = sort_by_arrive_time(jobs);
    let mut last_end_time: f64 = run_array[0].time.arrive;

    while run_array.len() != 0 {
        // 2
        let highest_priority_job_index = get_highest_priority_job_index(&run_array, last_end_time);
        let highest_priority_job = run_array[highest_priority_job_index];
        // println!("{} - {:?}", highest_priority_job_index, highest_priority_job);

        let job_begin_time: f64 = if highest_priority_job.time.arrive > last_end_time {
            highest_priority_job.time.arrive
        } else {
            last_end_time
        };
        let job_end_time = job_begin_time + highest_priority_job.resource.time;
        let id = highest_priority_job.id;
        // 3
        let job_turnaround_time = job_end_time - highest_priority_job.time.arrive;
        let job_weighted_turnaround = job_turnaround_time / highest_priority_job.resource.time;
        let temp = Job {
            id,
            time: Time {
                arrive: highest_priority_job.time.arrive,
                begin: job_begin_time,
                end: job_end_time,
                turnaround: job_turnaround_time,
                weighted_turnaround: job_weighted_turnaround,
            },
            resource: Resource {
                time: highest_priority_job.resource.time,
            },
            priority: highest_priority_job.priority,
        };
        // 3
        last_end_time = job_end_time;
        sorted.push(temp);
        run_array.remove(highest_priority_job_index);
    }

    // 5
    return sorted;
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn test_create() {
        let create = Job::new(0.0, 3.6);
        assert_eq!(create.resource.time, 3.6);
        assert_eq!(create.time.arrive, 0.0);
    }

    #[test]
    fn test_run() {
        let a = Job::new(0.0, 4.0);
        let b = Job::new(1.0, 3.0);
        let c = Job::new(2.0, 5.0);
        let e = Job::new(4.0, 4.0);
        let d = Job::new(3.0, 2.0);

        let mut jobs_array: Vec<Job> = vec![a, b, c, e, d];
        let sorted: Vec<Job> = vec![a, b, d, c, e];

        let mut r = thread_rng();
        jobs_array.shuffle(&mut r);

        let sorted_jobs_array = run(jobs_array);

        println!("{:?} \n{:?}", sorted, sorted_jobs_array);
        for i in 0..sorted.len() {
            println!("now is {}", i);
            println!("comp\n{:?} \n{:?}", sorted[i], sorted_jobs_array[i]);
            assert_eq!(sorted[i].id, sorted_jobs_array[i].id);
        }
    }
}
