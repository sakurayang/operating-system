# 页面置换算法 (Page Replacement)

> [source](https://www.geeksforgeeks.org/page-replacement-algorithms-in-operating-systems/) [wiki](https://www.wikiwand.com/en/Page_replacement_algorithm)

## 先进先出 (FIFO / First In First Out)

先入先出算法是最简单的页面置换算法。在这种算法中，操作系统在一个队列中跟踪内存中的所有页面，最老的页面在队列的前面。当一个页面需要被替换时，队列前面的页面就会被选择删除。

例子： 考虑要寻找的页为 1, 3, 0, 3, 5, 6, 3，页面空间为 3 ，计算页面错误次数 （miss）。

```text
 1   3   0   3   5   6   3
[ ] [ ] [0] [0] [0] [0] [3]
[ ] [3] [3] [3] [3] [6] [6]
[1] [1] [1] [1] [5] [5] [5]
Mis Mis Mis Hit Mis Mis Mis
```

最初所有的槽都是空的，所以当需要1、3、0时，它们会被分配到空的槽中。->3错误

当需要3时，已在内存中了 。-> 0错误。

然后需要5时，内存不可用，取代最旧的页槽 。-> 1错误。

需要6时，内存不可用，取代最旧的页槽。->1错误。

最后，需要3时，不可用的，取代0 -> 1错误。

(其实我觉得不用例程)

```python
# Python program to demonstrate
# working of FIFO
# using Queue interface in Java

q = []

# Adds elements {0, 1, 2, 3, 4} to queue
for i in range(5):
    q.append(i)

# Display contents of the queue.
print("Elements of queue-", q)

# To remove the head of queue.
# In this the oldest element '0' will be removed
removedele = q.pop(0)
print("removed element-", removedele)

print(q)

# To view the head of queue
head = q[0]
print("head of queue-", head)

# Rest all methods of collection interface,
# Like size and contains can be used with this
# implementation.
size = len(q)
print("Size of queue-", size)

# This code is contributed by patel2127.
```

输出：

```text
    Elements of queue-[0, 1, 2, 3, 4]
    removed element-0
    [1, 2, 3, 4]
    head of queue-1
    Size of queue-4
```

## 最少访问 (LRU / Least Recently Used)

这个算法将会取代最少使用的页。

例子： 考虑需要寻找的页为 7, 0, 1, 2, 0, 3, 0, 4, 2, 3, 0, 3, 2, 1 ，页面空间为 4

```text
 7   0   1   2   0   3   0   4   2   3   0   3   2   1
[ ] [ ] [ ] [2] [2] [2] [2] [2] [2] [2] [2] [2] [2] [2]
[ ] [ ] [1] [1] [1] [1] [1] [4] [4] [4] [4] [4] [4] [1]
[ ] [0] [0] [0] [0] [0] [0] [0] [0] [0] [0] [0] [0] [0]
[7] [7] [7] [7] [7] [3] [3] [3] [3] [3] [3] [3] [3] [3]
Mis Mis Mis Mis Hit Mis Hit Mis Hit Hit Hit Hit Hit Mis
```

设 `capacity`为内存可容纳的页数。`set` 为当前的页面集合。

1. 遍历页面集合。
    1. 如果 `set.length < capacity`
        1. 逐个插入页面到集合中，直到 `set` 的大小达到 `capacity` 或所有页面请求已被处理
        2. 同时维护最近发生的索引的 Map `indexes`
        3. 页面缺失计数增加
    2. 否则，如果当前页存在于 `set` 中，则不做任何事情。否则
        1. 在 `set` 中找到最近使用次数最少的页面。我们使用数组索引找到它。 基本上我们需要替换最小的索引
        2. 用当前页替换找到的页
        3. 页面缺失计数增加
        4. 更新当前页的索引
2. 返回页面故障。

```javascript
function pageFaults(pages, n, capacity) {
    // To represent set of current pages. We use
    // an unordered_set so that we quickly check
    // if a page is present in set or not
    let s = new Set();
    // To store least recently used indexes
    // of pages.
    let indexes = new Map();
    // Start from initial page
    let page_faults = 0;
    for (let i = 0; i < n; i++) {
        // Check if the set can hold more pages
        if (s.size < capacity) {
            // Insert it into set if not present
            // already which represents page fault
            if (!s.has(pages[i])) {
                s.add(pages[i]);
                // increment page fault
                page_faults++;
            }
            // Store the recently used index of
            // each page
            indexes.set(pages[i], i);
        }
                // If the set is full then need to perform lru
                // i.e. remove the least recently used page
        // and insert the current page
        else {
            // Check if current page is not already
            // present in the set
            if (!s.has(pages[i])) {
                // Find the least recently used pages
                // that is present in the set
                let lru = Number.MAX_VALUE, val = Number.MIN_VALUE;
                for (let itr of s.values()) {
                    let temp = itr;
                    if (indexes[temp] < lru) {
                        lru = indexes[temp];
                        val = temp;
                    }
                }
                // Remove the indexes page
                s.delete(val);
                //remove lru from hashmap
                indexes.delete(val);
                // insert the current page
                s.add(pages[i]);
                // Increment page faults
                page_faults++;
            }
            // Update the current page index
            indexes.set(pages[i], i);
        }
    }
    return page_faults;
}

// Driver method
let pages = [7, 0, 1, 2, 0, 3, 0, 4, 2, 3, 0, 3, 2];
let capacity = 4;
document.write(pageFaults(pages, pages.length, capacity));
```

```python
# Python3 program for page replacement algorithm

# Driver code
capacity = 4
processList = [7, 0, 1, 2, 0, 3, 0,
               4, 2, 3, 0, 3, 2]

# List of current pages in Main Memory
s = []

pageFaults = 0
# pageHits = 0

for i in processList:

    # If i is not present in currentPages list
    if i not in s:

        # Check if the list can hold equal pages
        if len(s) == capacity:
            s.remove(s[0])
            s.append(i)

        else:
            s.append(i)

        # Increment Page faults
        pageFaults += 1

    # If page is already there in
    # currentPages i.e in Main
    else:

        # Remove previous index of current page
        s.remove(i)

        # Now append it, at last index
        s.append(i)

print("%d" % pageFaults)

# This code is contributed by mahi_07
```

输出：6