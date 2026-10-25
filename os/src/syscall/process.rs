//! Process management syscalls
use crate::{
    config::PAGE_SIZE,
    mm::{translated_byte_buffer, MapPermission, PageTable, PageTableEntry, VPNRange, VirtAddr},
    task::{
        change_program_brk, current_mem_set, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next, syscall_times,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let tv = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let len = core::mem::size_of::<TimeVal>();

    let source = unsafe { core::slice::from_raw_parts(&tv as *const TimeVal as *const u8, len) };

    let buffers = translated_byte_buffer(current_user_token(), ts as *const u8, len);

    let mut start = 0;
    for buffer in buffers {
        let end = start + buffer.len();
        buffer.copy_from_slice(&source[start..end]);
        start = end;
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");

    let find_pte = |f: fn(PageTableEntry) -> bool| {
        let va = VirtAddr::from(id);
        let offset = va.page_offset();

        let vpn = va.floor();

        let token = current_user_token();
        let page_table = PageTable::from_token(token);
        let pte = page_table.translate(vpn);

        pte.and_then(|pte| {
            (pte.is_valid() && pte.user_accessable() && f(pte)).then_some((offset, pte))
        })
    };

    match trace_request {
        0 => {
            let value = find_pte(|pte| pte.readable());
            value
                .map(|(offset, pte)| pte.ppn().get_bytes_array()[offset] as isize)
                .unwrap_or(-1)
        }
        1 => {
            let value = find_pte(|pte| pte.writable());
            let Some((offset, pte)) = value else {
                return -1;
            };

            pte.ppn().get_bytes_array()[offset] = data as u8;

            0
        }
        2 => syscall_times(id) as isize,
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");

    if start % PAGE_SIZE != 0 || port & !(0b111) != 0 || port & 0b111 == 0 {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    let len = len + (PAGE_SIZE - len % PAGE_SIZE) % PAGE_SIZE;

    let mut mem_set = current_mem_set();
    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(start + len);

    for vpn in VPNRange::new(start_va.into(), end_va.into()) {
        if let Some(pte) = mem_set.translate(vpn) {
            if pte.is_valid() {
                return -1;
            }
        }
    }

    let mut permission = MapPermission::U;
    permission |= MapPermission::from_bits_truncate((port << 1) as u8);

    if mem_set
        .insert_framed_area(start_va, end_va, permission)
        .is_err()
    {
        return -1;
    };

    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");

    if start % PAGE_SIZE != 0 {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    let len = len + (PAGE_SIZE - len % PAGE_SIZE) % PAGE_SIZE;

    let mut mem_set = current_mem_set();
    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(start + len);

    for vpn in VPNRange::new(start_va.into(), end_va.into()) {
        if let Some(pte) = mem_set.translate(vpn) {
            if !pte.is_valid() {
                return -1;
            }
        }
    }

    if mem_set.unmap(start_va, end_va).is_err() {
        return -1;
    }

    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
