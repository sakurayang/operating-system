# 时间片轮转 (Round Robin)

> [source](https://www.geeksforgeeks.org/program-round-robin-scheduling-set-1/)

时间片轮转算法是一种 CPU 调度算法，他给每个进程分配一个固定的时间来运行。

## 例子

| 进程  | 持续时间 | 顺序  | 到达时间 |
|-----|------|-----|------|
| P1  | 3    | 1   | 0    |
| P2  | 4    | 2   | 0    |
| P3  | 3    | 3   | 0    |

假设时间片是一单位时间

```text
| 1 | 2 | 3 | 1 | 2 | 3 | 1 | 2 | 3 | 2 |
0                                       10
```

P1 等待时间：4

P2 等待时间：6

P3 等待时间：6

平均等待时间 (AWT) ： `(4+6+6)/3=5.33`

## 衡量标准

对于该算法，有以下性能衡量标准

1. 完成时间 *(Completion Time)*：进程完成执行的时间
2. 周转时间 *(Turn around Time)*：完成时间 - 到达时间
3. 等待时间 *(Waiting Time)*：周转时间 - 运行时间 *(Brust Time)*

### 计算进程等待时间

1. 创建一个 `rem_brust_time[]` 来追踪进程剩余的运行时间。这个数组的初始状态是 `brust_time[]`
2. 创建一个 `wait_time[]` 来储存等待时间，初始化全为0。
3. 初始化时间 `t = 0`
4. 重复以下步骤若所有进程未完成。
    1. 如果 `rem_brust_time[i] > quantum`
        1. `t += quantum`
        2. `rem_brust_time[i] -= quantum`
    2. 否则这就是这个进程的最后一个周期
        1. `t += rem_brust_time[i]`
        2. `wait_time = t - brust_time[i]`
        3. `rem_brust_time[i] = 0`

### 实现

```python
# Python3 program for implementation of
# RR scheduling

# Function to find the waiting time
# for all processes
def find_waiting_time(processes, n, bt, wt, quantum):
    rem_bt = [0] * n

    # Copy the burst time into rt[]
    for i in range(n):
        rem_bt[i] = bt[i]
    t = 0  # Current time

    # Keep traversing processes in round-robin
    # manner until all of them are
    # not done.
    while True:
        done = True

        # Traverse all processes one by
        # one repeatedly
        for i in range(n):
            # If burst time of a process is greater
            # than 0 then only need to process further
            if rem_bt[i] > 0:
                done = False  # There is a pending process

                if rem_bt[i] > quantum:

                    # Increase the value of t i.e. shows
                    # how much time a process has been processed
                    t += quantum

                    # Decrease the burst_time of current
                    # process by quantum
                    rem_bt[i] -= quantum

                # If burst time is smaller than or equal 
                # to quantum. Last cycle for this process
                else:

                    # Increase the value of t i.e. shows
                    # how much time a process has been processed
                    t = t + rem_bt[i]

                    # Waiting time is current time minus
                    # time used by this process
                    wt[i] = t - bt[i]

                    # As the process gets fully executed
                    # make its remaining burst time = 0
                    rem_bt[i] = 0

        # If all processes are done
        if done:
            break


# Function to calculate turn around time
def find_turn_around_time(processes, n, bt, wt, tat):
    # Calculating turnaround time
    for i in range(n):
        tat[i] = bt[i] + wt[i]


# Function to calculate average waiting
# and turn-around times.
def find_avg_time(processes, n, bt, quantum):
    wt = [0] * n
    tat = [0] * n

    # Function to find waiting time
    # of all processes
    find_waiting_time(processes, n, bt, wt, quantum)
    # Function to find turn around time
    # for all processes
    find_turn_around_time(processes, n, bt, wt, tat)

    # Display processes along with all details
    print("Processes    Burst Time     Waiting",
          "Time    Turn-Around Time")
    total_wt = 0
    total_tat = 0
    for i in range(n):
        total_wt = total_wt + wt[i]
        total_tat = total_tat + tat[i]
        print(" ", i + 1, "\t\t", bt[i],
              "\t\t", wt[i], "\t\t", tat[i])

    print("\nAverage waiting time = %.5f " % (total_wt / n))
    print("Average turn around time = %.5f " % (total_tat / n))


# Driver code
if __name__ == "__main__":
    # Process id's
    proc = [1, 2, 3]
    n = 3

    # Burst time of all processes
    burst_time = [10, 5, 8]

    # Time quantum
    quantum = 2
    find_avg_time(proc, n, burst_time, quantum)

# This code is contributed by
# Shubham Singh(SHUBHAMSINGH10)
```

输出：

```text
    Processes  Burst time  Waiting time  Turn around time
     1        10     13         23
     2        5     10         15
     3        8     13         21
    Average waiting time = 12
    Average turn around time = 19.6667
```