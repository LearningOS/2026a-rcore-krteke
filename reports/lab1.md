# 功能实现

1. `trace_request`值为0或1时非常简单，就是地址读写。

2. `trace_request`值为2时，我在`TaskControlBlock`里增加了`syscall_times: [usize; MAX_SYSCALL_NUM]`，并且增加了`record_syscall`和`syscall_times`以及`TaskManager`里面对应的内部方法。

   记录时在`syscall`函数的入口调用`record_syscall`来记录id。

   查询时和记录时都使用`current_task`得到当前任务，然后使用`id`来从`syscall_times: [usize; MAX_SYSCALL_NUM]`里面获取或更新`id`的调用次数。

# 简答

1.  > RustSBI version 0.4.1

    观测到

    > [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
    >
    > [kernel] IllegalInstruction in application, kernel killed it.
    >
    > [kernel] IllegalInstruction in application, kernel killed it.

    `ch2b_bad_address.rs`因为访问了非法的内存地址而被kill掉。
    `ch2b_bad_instructions.rs`在U态尝试执行S特权指令。
    `ch2b_bad_register.rs`在U态访问S态寄存器。

2.  `__alltraps` & `__restore`
    1. `sp` 代表指向内核栈的栈指针。用于恢复用户程序和第一次启动一个用户程序。
    2. 特殊处理了几个CSR，`sstatus`告诉cpu需要回到的特权级，`sepc`用于告诉cpu返回到哪个指令地址，`sscratch`恢复用户栈指针，后续和`sp`交换。
    3. 因为`x2`目前储存的是内核栈的栈指针，需要在后面`csrrw sp, sscratch, sp`和用户栈交换，`x4`不需要使用。
    4. 现在`sp`是用户栈指针，`sscratch`是内核栈指针。
    5. `sret`，根据 RISC-V 特权级规范，sret 会根据 sstatus.SPP 决定返回的特权级，并从 sepc 恢复程序计数器 PC。
    6. `sp`: 内核栈指针。`sscratch`: 用户栈指针。
    7. `ecall`

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > ChatGPT，交流了risc-v汇编相关内容。

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > https://rcore-os.cn/rCore-Tutorial-Book-v3
   >
   > https://learningos.cn/rCore-Tutorial-Guide

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
