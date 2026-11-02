# 功能实现

1. linkat: 先获取旧文件的inode_id，然后修改根Inode，创建一个使用新name和旧id的DirEntry，然后写入。

2. unlinkat: 先通过name找到Inode，然后通过name获取在root中的位置，之后互换末尾的DirEntry与找到的位置的DirEntry，之后释放它在DiskInode中记录的数据，如果发现没有硬链接了，就dealloc inode_id。

3. fstat: 先给File trait添加fn stat(&self) -> Stat;然后给OSInode实现stat，获取id就是get_disk_inode_pos反过来，使用block_id和block_offset计算inode_id。

# 简答

1. ch6
   root inode用于定位其他Inode，如果它的内容损坏，会导致其他的Inode无法访问

2. ch7
   1. 把一个程序的输出定向到另一个程序的输入，比如`cat /path/to/file | wl-copy`
   2. 做一个消息中心，消息中心和每个需要通信的进程建立一个管道，然后进程间只需要向消息中心发送消息，然后消息中心再把输入消息转发到对应的管道。

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > ChatGPT

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > https://learningos.cn/rCore-Tutorial-Guide
   >
   > https://learningos.cn/rCore-Tutorial-Book-v3

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
