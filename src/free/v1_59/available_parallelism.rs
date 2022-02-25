use core::num::NonZeroUsize;
use std::io;

#[cfg(windows)]
pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    use core::ffi::c_void;
    use std::os::raw::c_ulong;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    struct SYSTEM_INFO {
        wProcessorArchitecture: u16,
        wReserved: u16,
        dwPageSize: c_ulong,
        lpMinimumApplicationAddress: *mut c_void,
        lpMaximumApplicationAddress: *mut c_void,
        dwActiveProcessorMask: usize,
        dwNumberOfProcessors: c_ulong,
        dwProcessorType: c_ulong,
        dwAllocationGranularity: c_ulong,
        wProcessorLevel: u16,
        wProcessorRevision: u16,
    }

    let res = unsafe {
        let mut sysinfo: SYSTEM_INFO = core::mem::zeroed();
        GetSystemInfo(&mut sysinfo);
        sysinfo.dwNumberOfProcessors as usize
    };
    match res {
        0 => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The number of hardware threads is not known for the target platform",
        )),
        cpus => Ok(unsafe { NonZeroUsize::new_unchecked(cpus) }),
    }
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd"))]
pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    use core::ptr;

    let mut cpus: libc::c_uint = 0;
    let mut cpus_size = core::mem::size_of_val(&cpus);

    unsafe {
        cpus = libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as libc::c_uint;
    }

    if cpus < 1 {
        let mut mib = [libc::CTL_HW, libc::HW_NCPU, 0, 0];
        let res = unsafe {
            libc::sysctl(
                mib.as_mut_ptr(),
                2,
                &mut cpus as *mut _ as *mut _,
                &mut cpus_size as *mut _ as *mut _,
                ptr::null_mut(),
                0,
            )
        };

        if res == -1 {
            return Err(io::Error::last_os_error());
        } else if cpus == 0 {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The number of hardware threads is not known for the target platform",
            ));
        }
    }
    Ok(unsafe { NonZeroUsize::new_unchecked(cpus as usize) })
}

#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "solaris",
    target_os = "illumos",
))]
pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    use core::mem;

    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        let mut set: libc::cpu_set_t = unsafe { mem::zeroed() };
        if unsafe { libc::sched_getaffinity(0, mem::size_of::<libc::cpu_set_t>(), &mut set) } == 0 {
            let count = unsafe { libc::CPU_COUNT(&set) };
            return Ok(unsafe { NonZeroUsize::new_unchecked(count as usize) });
        }
    }
    match unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) } {
        -1 => Err(io::Error::last_os_error()),
        0 => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The number of hardware threads is not known for the target platform",
        )),
        cpus => Ok(unsafe { NonZeroUsize::new_unchecked(cpus as usize) }),
    }
}

#[cfg(target_os = "openbsd")]
pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    use crate::ptr;

    let mut cpus: libc::c_uint = 0;
    let mut cpus_size = crate::mem::size_of_val(&cpus);
    let mut mib = [libc::CTL_HW, libc::HW_NCPU, 0, 0];

    let res = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            2,
            &mut cpus as *mut _ as *mut _,
            &mut cpus_size as *mut _ as *mut _,
            ptr::null_mut(),
            0,
        )
    };

    if res == -1 {
        Err(io::Error::last_os_error())
    } else if cpus == 0 {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The number of hardware threads is not known for the target platform",
        ))
    } else {
        Ok(unsafe { NonZeroUsize::new_unchecked(cpus as usize) })
    }
}

#[cfg(target_os = "haiku")]
pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsafe {
        let mut sinfo: libc::system_info = crate::mem::zeroed();
        let res = libc::get_system_info(&mut sinfo);

        if res != libc::B_OK {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The number of hardware threads is not known for the target platform",
            ));
        }

        Ok(NonZeroUsize::new_unchecked(sinfo.cpu_count as usize))
    }
}

#[cfg(not(any(
    windows,
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "android",
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "openbsd",
    target_os = "haiku",
)))]
pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Getting the number of hardware threads is not supported on the target platform",
    ))
}
