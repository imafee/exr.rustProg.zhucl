# std::collections(rust 标准库)

提供了 4 种通用的**容器**类型，其中包括了 8 种数据结构

- Sequences: 线性序列
  - Vec 连续存储的可变长数组
  - VecDeque 连续存储的可变长双端队列
  - LinkedList 非连续存储的双向链表
- Maps: 键值对
  - HashMap 基于哈希表的无序键值对
  - BTreeMap 基于 B 树的有序键值对，按 key 排序
- Sets: 集合
  - HashSet 基于哈希表的无序集合
  - BTreeSet 基于 B 树的有序集合
- Misc:
  - BinaryHeap 基于二叉堆的优先队列
