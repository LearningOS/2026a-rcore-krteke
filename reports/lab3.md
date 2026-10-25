# 功能实现

1. 修改了获取`MemorySet`的方式，其他的和之前的syscall相同。

2. 参考`TaskControlBlock::new`和`TaskControlBlock::fork`，大概就是把他们两个综合了一下。

3. 在TCB里面添加了一个新字段，用于维护优先级等，然后获取进程时，在`TaskManager::fetch`里面改用查找stride的进程，然后stride+=pass。

# 简答

1. 实际还是p2，p2.stride因为溢出变成4了。

2. 因为P.pass = BigStride / P.priority，所以prio>=2时pass <= BigStride / 2,而且初始stride都是0,而运行过程中挑选stride最小的执行，所以max和min之间最大的差距小于pass，所以STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

3. ```rust
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       let half = u64::MAX / 2;
       self.0.wrapping_sub(other.0).partial_cmp(half)
   }
   ```

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > ChatGPT

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > https://learningos.cn/rCore-Tutorial-Guide
   >
   > https://learningos.cn/rCore-Tutorial-Book-v3

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
