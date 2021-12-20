//! # 银行家算法 (Banker's Algorithm)
//!
//! > [source](https://www.geeksforgeeks.org/bankers-algorithm-in-operating-system-2/)
//!
//! 银行家算法是一种资源分配与死锁避免算法，它通过模拟资源分配的动作来测试安全性。
//! 然后进行 "s-state "检查以测试可能的活动，然后再决定是否应允许继续分配。
//!
//! ## 算法原理
//!
//! ### 资源结构说明
//!
//! > 假设 `n` 为系统中的进程数量，`m` 为资源种类的数量。
//!
//! **可用 _(Available)_**
//! - 这是一个大小为 `m` 的一维数组，表示系统中每种资源的数量
//! - `Available[j] = k` 表示种类为 `R[j]` 的资源有 `k` 个
//!
//! **最大 _(Max)_**
//! - 这是一个大小为 `n*m` 的二维数组，表示了每个进程所需求的每种资源的最大数量
//! - `Max[i, j] = k` 表示进程 `P[i]` 需要最多 `k` 个种类为 `R[j]` 的资源
//!
//! **分配 _(Allocation)_**
//! - 这是一个大小为 `n*m` 的二维数组，表示了每个进程现在已分配的每种资源的数量
//! - `Allocation[i, j] = k` 表示进程 `P[i]` 现在已经被分配了 `k` 个种类为 `R[j]` 的资源
//!
//! **需求 _(Need)_**
//! - 这是一个大小为 `n*m` 的二维数组，表示了每个进程还需要的每种资源的数量
//! - `Need[i, j] = k` 表示进程 `P[i]` 现在需要 `k` 个种类为 `R[j]` 的资源
//! - `Need[i, j] = Max[i, j] - Allocation[i, j]`
//!
//! ### 安全性判断算法
//!
//! > 假定用 `Work` 表示目前占用的资源
//!
//! 1. 令 `Work` 和 `Finish` 分别为长度 `m` 和 `n` 的数组。
//!    初始化，令 `Work = Available` ，
//!    `Finish[i] = false` *(i = 1, 2, 4..., n)*
//! 2. 对每一个可能的 `i`， 判断其是否符合以下两个条件，若不符合，则跳到步骤 4
//!    1. `Finish[i] = false`
//!    2. `Need[i] <= Work`
//! 3. 令 `Work = Work + Allocation[i]` 以及 `Finish[i] = true` 后，转到步骤 2
//! 4. 如果对于所有可能的 `i` 都有 `Finish[i] = true`，则系统处于安全状态
//!
//! ### 资源分配算法
//!
//! 1. 如果 `Request[i] <= Need[i]`，转到步骤 2 。否则抛出异常：进程请求资源数超过最大资源数
//! 2. 如果 `Request[i] <= Available`，转到步骤 3 。否则进程 `P[i]` 等待，因为资源目前不可用。
//! 3. 令系统假定已为 `P[i]` 分配了其所请求的所有资源，修改状态如下：
//!    - `Available = Available - Request[i]`
//!    - `Allocation[i] = Allocation[i] + Request[i]`
//!    - `Need[i] = Need[i] - Request[i]`
//!
//! ## 伪代码
//!
//! - P - 进程的集合
//! - Mp - 进程p的最大的请求数目
//! - Cp - 进程p当前被分配的资源
//! - A - 当前可用的资源
//!
//! ```r
//! while (P != ∅) {
//!     found = FALSE;
//!     foreach (p ∈ P) {
//!         if (Mp − Cp ≤ A) {
//! /* p可以獲得他所需的資源。假設他得到資源後執行；執行終止，並釋放所擁有的資源。*/
//!              A = A + Cp ;
//!              P = P − {p};
//!              found = TRUE;
//!         }
//!     }
//!     if (! found) return FAIL;
//! }
//! return OK;
//! ```
//!

use crate::typedef::{Available, Process, Sequence};

pub fn is_safe(available: Available, processes: Vec<Process>) -> bool {
    // 令 Work 和 Finish 分别为长度 m 和 n 的数组。
    // 初始化，令 Work = Available
    let mut work = available;
    // Finish[i] = false (i = 1, 2, 4..., n)
    let process_count = processes.len();
    let mut finish: Vec<bool> = vec![false; process_count];
    let mut visit_count: Vec<usize> = vec![0; process_count];

    // 若 finish 有 false，循环
    while finish.iter().any(|x| !*x) {
        // 若某个进程被访问的次数超过总进程数，那说明无法找到安全序列
        if visit_count.iter().any(|x| x >= &(process_count + 1)) { return false; }
        // 遍历所有的进程
        for i in 0..process_count {
            let process = &processes[i];
            // 访问次数 + 1
            visit_count[i] += 1;
            // 判断其是否完成，以及其需求资源是否不超过可用资源
            if !finish[i] && process.need <= work {
                // 若否，则假装分配完了，将其已分配的资源释放
                for j in 0..work.len() {
                    work[j] += &process.allocation[j];
                }
                // 标记为完成
                finish[i] = true;
            }
        }
    }
    return true;
}


/// 这个函数大部分和 is_safe 相同，除了有注释的部分
/// TODO: 精简重复的部分
/// 其实有想法，但是懒得做，直接复制黏贴了
pub fn alloc_calc(available: Available, processes: Vec<Process>) -> Result<Sequence, u8> {
    if !is_safe(available.clone(), processes.clone()) { return Err(0); }
    let mut work = available;
    let process_count = processes.len();
    let mut finish: Vec<bool> = vec![false; process_count];
    let mut visit_count: Vec<usize> = vec![0; process_count];

    // 这里不同
    let mut sequence: Sequence = Sequence::new();

    while finish.iter().any(|x| !*x) {
        for i in 0..process_count {
            let process = &processes[i];
            visit_count[i] += 1;
            if !finish[i] && process.need <= work {
                for j in 0..work.len() {
                    work[j] += &process.allocation[j];
                }
                // 这里不同
                sequence.push(i);
                finish[i] = true;
            }
        }
    }
    return Ok(sequence);
}

