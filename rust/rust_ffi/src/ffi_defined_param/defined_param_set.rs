extern "C" {
    fn get__win32() -> libc::c_int;
    fn get___cygwin__() -> libc::c_int;
    fn get__debug() -> libc::c_int;
    fn get_have_lzma_h() -> libc::c_int;
    fn get_have_bzlib_h() -> libc::c_int;
    fn get_bz_config_error() -> libc::c_int;
    fn get__lzma_prob32() -> libc::c_int;
    fn get_have_zlib_h() -> libc::c_int;
    fn get_have_timegm() -> libc::c_int;
    fn get_have__mkgmtime64() -> libc::c_int;
    fn get_debug() -> libc::c_int;
    fn get_have_libxml_xmlreader_h() -> libc::c_int;
    fn get_have_bsdxml_h() -> libc::c_int;
    fn get_have_expat_h() -> libc::c_int;
    fn get_lzma_version_major() -> libc::c_int;
    fn get_archive_has_md5() -> libc::c_int;
    fn get_debug_print_toc() -> libc::c_int;
    fn get_archive_has_sha1() -> libc::c_int;
    fn get_have_liblzma() -> libc::c_int;
    fn get_have_copyfile_h() -> libc::c_int;
    fn get_have_strnlen() -> libc::c_int;
    fn get_time_t_max() -> libc::c_int;
    fn get_time_t_min() -> libc::c_int;
    fn get_s_iflnk() -> libc::c_int;
    fn get_s_ifsock() -> libc::c_int;
    fn get_s_ifchr() -> libc::c_int;
    fn get_s_ifblk() -> libc::c_int;
    fn get_s_ififo() -> libc::c_int;
    fn get_have_struct_stat_st_mtimespec_tv_nsec() -> libc::c_int;
    fn get_have_struct_stat_st_mtim_tv_nsec() -> libc::c_int;
    fn get_have_struct_stat_st_mtime_n() -> libc::c_int;
    fn get_have_struct_stat_st_umtime() -> libc::c_int;
    fn get_have_struct_stat_st_mtime_usec() -> libc::c_int;
    fn get_have_iconv() -> libc::c_int;
    fn get_have_localtime_r() -> libc::c_int;
    fn get_have__localtime64_s() -> libc::c_int;
    fn get_check_crc_on_solid_skip() -> libc::c_int;
    fn get_dont_fail_on_crc_error() -> libc::c_int;
}

pub fn defined_param_set() {
    have_zlib_h_add_cfg();
    have_timegm_add_cfg();
    have__mkgmtime64_add_cfg();
    c_debug_add_cfg();
    have_libxml_xmlreader_h_add_cfg();
    have_expat_h_add_cfg();
    have_lzma_version_major_add_cfg();
    archive_has_md5_add_cfg();
    archive_has_sha1_add_cfg();
    debug_print_toc_add_cfg();
    have_bsdxml_h_add_cfg();
    _win32_add_cfg();
    __cygwin___add_cfg();
    _debug_add_cfg();
    have_lzma_h_add_cfg();
    have_bzlib_h_add_cfg();
    bz_config_error_add_cfg();
    _lzma_prob32_add_cfg();
    have_liblzma_add_cfg();
    have_copyfile_h_add_cfg();
    have_strnlen();
    have_time_t_min();
    have_time_t_max();
    have_s_iflnk();
    have_s_ifsock();
    have_s_ifchr();
    have_s_ifblk();
    have_s_ififo();
    have_struct_stat_st_mtimespec_tv_nsec();
    have_struct_stat_st_mtim_tv_nsec();
    have_struct_stat_st_mtime_n();
    have_struct_stat_st_umtime();
    have_struct_stat_st_mtime_usec();
    have_iconv_add_cfg();
    have_localtime_r_add_cfg();
    have__localtime64_s_add_cfg();
    have_check_crc_on_solid_skip_add_cfg();
    have_dont_fail_on_crc_error_add_cfg();
}

fn have_zlib_h_add_cfg() {
    if unsafe { get_have_zlib_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_ZLIB_H");
    }
}

fn have_timegm_add_cfg() {
    if unsafe { get_have_timegm() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_TIMEGM");
    }
}

fn have__mkgmtime64_add_cfg() {
    if unsafe { get_have__mkgmtime64() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE__MKGMTIME64");
    }
}

fn c_debug_add_cfg() {
    if unsafe { get_debug() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=C_DEBUG");
    }
}

fn have_libxml_xmlreader_h_add_cfg() {
    if unsafe { get_have_libxml_xmlreader_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_LIBXML_XMLREADER_H");
    }
}
fn have_bsdxml_h_add_cfg() {
    if unsafe { get_have_bsdxml_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_BSDXML_H");
    }
}
fn have_expat_h_add_cfg() {
    if unsafe { get_have_expat_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_EXPAT_H");
    }
}

fn have_lzma_version_major_add_cfg() {
    if unsafe { get_lzma_version_major() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=LZMA_VERSION_MAJOR");
    }
}

fn archive_has_md5_add_cfg() {
    if unsafe { get_archive_has_md5() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=ARCHIVE_HAS_MD5");
    }
}
fn archive_has_sha1_add_cfg() {
    if unsafe { get_archive_has_sha1() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=ARCHIVE_HAS_SHA1");
    }
}

fn debug_print_toc_add_cfg() {
    if unsafe { get_debug_print_toc() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=DEBUG_PRINT_TOC");
    }
}

fn _win32_add_cfg() {
    if unsafe { get__win32() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=_WIN32");
    }
}

fn __cygwin___add_cfg() {
    if unsafe { get___cygwin__() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=__CYGWIN__");
    }
}

fn _debug_add_cfg() {
    if unsafe { get__debug() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=_DEBUG");
    }
}

fn have_lzma_h_add_cfg() {
    if unsafe { get_have_lzma_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_LZMA_H");
    }
}

fn have_bzlib_h_add_cfg() {
    if unsafe { get_have_bzlib_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_BZLIB_H");
    }
}

fn bz_config_error_add_cfg() {
    if unsafe { get_bz_config_error() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=BZ_CONFIG_ERROR");
    }
}

fn _lzma_prob32_add_cfg() {
    if unsafe { get__lzma_prob32() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=_LZMA_PROB32");
    }
}

fn have_liblzma_add_cfg() {
    if unsafe { get_have_liblzma() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_LIBLZMA");
    }
}

fn have_copyfile_h_add_cfg() {
    if unsafe { get_have_copyfile_h() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_COPYFILE_H");
    }
}

fn have_time_t_min() {
    if unsafe { get_time_t_min() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_TIME_T_MIN");
    }
}

fn have_time_t_max() {
    if unsafe { get_time_t_max() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_TIME_T_MAX");
    }
}

fn have_strnlen() {
    if unsafe { get_have_strnlen() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_STRLEN");
    }
}

fn have_struct_stat_st_mtimespec_tv_nsec() {
    if unsafe { get_have_struct_stat_st_mtimespec_tv_nsec() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC");
    }
}

fn have_struct_stat_st_mtim_tv_nsec() {
    if unsafe { get_have_struct_stat_st_mtim_tv_nsec() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_STRUCT_STAT_ST_MTIM_TV_NSEC");
    }
}

fn have_struct_stat_st_mtime_n() {
    if unsafe { get_have_struct_stat_st_mtime_n() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_STRUCT_STAT_ST_MTIME_N");
    }
}

fn have_struct_stat_st_umtime() {
    if unsafe { get_have_struct_stat_st_umtime() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_STRUCT_STAT_ST_UMTIME");
    }
}

fn have_struct_stat_st_mtime_usec() {
    if unsafe { get_have_struct_stat_st_mtime_usec() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_STRUCT_STAT_ST_MTIME_USEC");
    }
}

fn have_s_iflnk() {
    if unsafe { get_s_iflnk() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=S_IFLNK");
    }
}

fn have_s_ifsock() {
    if unsafe { get_s_ifsock() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=S_IFSOCK");
    }
}

fn have_s_ifchr() {
    if unsafe { get_s_ifchr() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=S_IFCHR");
    }
}

fn have_s_ifblk() {
    if unsafe { get_s_ifblk() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=S_IFBLK");
    }
}

fn have_s_ififo() {
    if unsafe { get_s_ififo() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=S_IFIFO");
    }
}

fn have_iconv_add_cfg() {
    if unsafe { get_have_iconv() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_ICONV");
    }
}

fn have_localtime_r_add_cfg() {
    if unsafe { get_have_localtime_r() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE_LOCALTIME_R");
    }
}

fn have__localtime64_s_add_cfg() {
    if unsafe { get_have__localtime64_s() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=HAVE__LOCALTIME64_S");
    }
}

fn have_check_crc_on_solid_skip_add_cfg() {
    if unsafe { get_check_crc_on_solid_skip() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=CHECK_CRC_ON_SOLID_SKIP");
    }
}

fn have_dont_fail_on_crc_error_add_cfg() {
    if unsafe { get_dont_fail_on_crc_error() } == 1 as libc::c_int {
        println!("cargo:rustc-cfg=DONT_FAIL_ON_CRC_ERROR");
    }
}
