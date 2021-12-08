//
// Sysinfo
//
// Copyright (c) 2021 Guillaume Gomez
//

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

pub(crate) fn get_all_data_from_file(file: &mut File, size: usize) -> io::Result<String> {
    let mut buf = String::with_capacity(size);
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub(crate) fn get_all_data<P: AsRef<Path>>(file_path: P, size: usize) -> io::Result<String> {
    let mut file = File::open(file_path.as_ref())?;
    get_all_data_from_file(&mut file, size)
}

#[allow(clippy::useless_conversion)]
pub fn realpath(original: &Path) -> std::path::PathBuf {
    use libc::{c_char, lstat, stat, S_IFLNK, S_IFMT};
    use std::fs;
    use std::mem::MaybeUninit;
    use std::path::PathBuf;

    fn and(x: u32, y: u32) -> u32 {
        x & y
    }

    // let ori = Path::new(original.to_str().unwrap());
    // Right now lstat on windows doesn't work quite well
    // if cfg!(windows) {
    //     return PathBuf::from(ori);
    // }
    let result = PathBuf::from(original);
    let mut result_s = result.to_str().unwrap_or("").as_bytes().to_vec();
    result_s.push(0);
    let mut buf = MaybeUninit::<stat>::uninit();
    let res = unsafe { lstat(result_s.as_ptr() as *const c_char, buf.as_mut_ptr()) };
    let buf = unsafe { buf.assume_init() };
    if res < 0 || and(buf.st_mode.into(), S_IFMT.into()) != S_IFLNK.into() {
        PathBuf::new()
    } else {
        match fs::read_link(&result) {
            Ok(f) => f,
            Err(_) => PathBuf::new(),
        }
    }
}
