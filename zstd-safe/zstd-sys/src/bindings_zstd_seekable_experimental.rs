/*
This file is auto-generated from the public API of the zstd library.
It is released under the same BSD license.

BSD License

For Zstandard software

Copyright (c) Meta Platforms, Inc. and affiliates. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

 * Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

 * Neither the name Facebook, nor Meta, nor the names of its contributors may
   be used to endorse or promote products derived from this software without
   specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* automatically generated by rust-bindgen 0.64.0 */

pub const ZSTD_seekTableFooterSize: u32 = 9;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut libc::c_void,
}
pub type FILE = _iobuf;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekable_CStream_s {
    _unused: [u8; 0],
}
pub type ZSTD_seekable_CStream = ZSTD_seekable_CStream_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekable_s {
    _unused: [u8; 0],
}
pub type ZSTD_seekable = ZSTD_seekable_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekTable_s {
    _unused: [u8; 0],
}
pub type ZSTD_seekTable = ZSTD_seekTable_s;
extern "C" {
    pub fn ZSTD_seekable_createCStream() -> *mut ZSTD_seekable_CStream;
}
extern "C" {
    pub fn ZSTD_seekable_freeCStream(zcs: *mut ZSTD_seekable_CStream)
        -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_initCStream(
        zcs: *mut ZSTD_seekable_CStream,
        compressionLevel: libc::c_int,
        checksumFlag: libc::c_int,
        maxFrameSize: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_compressStream(
        zcs: *mut ZSTD_seekable_CStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_endFrame(
        zcs: *mut ZSTD_seekable_CStream,
        output: *mut ZSTD_outBuffer,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_endStream(
        zcs: *mut ZSTD_seekable_CStream,
        output: *mut ZSTD_outBuffer,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_frameLog_s {
    _unused: [u8; 0],
}
pub type ZSTD_frameLog = ZSTD_frameLog_s;
extern "C" {
    pub fn ZSTD_seekable_createFrameLog(
        checksumFlag: libc::c_int,
    ) -> *mut ZSTD_frameLog;
}
extern "C" {
    pub fn ZSTD_seekable_freeFrameLog(fl: *mut ZSTD_frameLog) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_logFrame(
        fl: *mut ZSTD_frameLog,
        compressedSize: libc::c_uint,
        decompressedSize: libc::c_uint,
        checksum: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_writeSeekTable(
        fl: *mut ZSTD_frameLog,
        output: *mut ZSTD_outBuffer,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_create() -> *mut ZSTD_seekable;
}
extern "C" {
    pub fn ZSTD_seekable_free(zs: *mut ZSTD_seekable) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_initBuff(
        zs: *mut ZSTD_seekable,
        src: *const libc::c_void,
        srcSize: usize,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_initFile(
        zs: *mut ZSTD_seekable,
        src: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_decompress(
        zs: *mut ZSTD_seekable,
        dst: *mut libc::c_void,
        dstSize: usize,
        offset: libc::c_ulonglong,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_decompressFrame(
        zs: *mut ZSTD_seekable,
        dst: *mut libc::c_void,
        dstSize: usize,
        frameIndex: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_getNumFrames(
        zs: *const ZSTD_seekable,
    ) -> libc::c_uint;
}
extern "C" {
    pub fn ZSTD_seekable_getFrameCompressedOffset(
        zs: *const ZSTD_seekable,
        frameIndex: libc::c_uint,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_seekable_getFrameDecompressedOffset(
        zs: *const ZSTD_seekable,
        frameIndex: libc::c_uint,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_seekable_getFrameCompressedSize(
        zs: *const ZSTD_seekable,
        frameIndex: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_getFrameDecompressedSize(
        zs: *const ZSTD_seekable,
        frameIndex: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekable_offsetToFrameIndex(
        zs: *const ZSTD_seekable,
        offset: libc::c_ulonglong,
    ) -> libc::c_uint;
}
extern "C" {
    pub fn ZSTD_seekTable_create_fromSeekable(
        zs: *const ZSTD_seekable,
    ) -> *mut ZSTD_seekTable;
}
extern "C" {
    pub fn ZSTD_seekTable_free(st: *mut ZSTD_seekTable) -> usize;
}
extern "C" {
    pub fn ZSTD_seekTable_getNumFrames(
        st: *const ZSTD_seekTable,
    ) -> libc::c_uint;
}
extern "C" {
    pub fn ZSTD_seekTable_getFrameCompressedOffset(
        st: *const ZSTD_seekTable,
        frameIndex: libc::c_uint,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_seekTable_getFrameDecompressedOffset(
        st: *const ZSTD_seekTable,
        frameIndex: libc::c_uint,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_seekTable_getFrameCompressedSize(
        st: *const ZSTD_seekTable,
        frameIndex: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekTable_getFrameDecompressedSize(
        st: *const ZSTD_seekTable,
        frameIndex: libc::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_seekTable_offsetToFrameIndex(
        st: *const ZSTD_seekTable,
        offset: libc::c_ulonglong,
    ) -> libc::c_uint;
}
pub type ZSTD_seekable_read = ::core::option::Option<
    unsafe extern "C" fn(
        opaque: *mut libc::c_void,
        buffer: *mut libc::c_void,
        n: usize,
    ) -> libc::c_int,
>;
pub type ZSTD_seekable_seek = ::core::option::Option<
    unsafe extern "C" fn(
        opaque: *mut libc::c_void,
        offset: libc::c_longlong,
        origin: libc::c_int,
    ) -> libc::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekable_customFile {
    pub opaque: *mut libc::c_void,
    pub read: ZSTD_seekable_read,
    pub seek: ZSTD_seekable_seek,
}
extern "C" {
    pub fn ZSTD_seekable_initAdvanced(
        zs: *mut ZSTD_seekable,
        src: ZSTD_seekable_customFile,
    ) -> usize;
}
