//!  # Seekable Format
//!
//!  The seekable format splits the compressed data into a series of "frames",
//!  each compressed individually so that decompression of a section in the
//!  middle of an archive only requires zstd to decompress at most a frame's
//!  worth of extra data, instead of the entire archive.

use core::num::NonZeroU32;
use core::ptr::NonNull;
use zstd_sys::{
    ZSTD_seekable_CStream, ZSTD_seekable_compressStream,
    ZSTD_seekable_createCStream, ZSTD_seekable_endFrame,
    ZSTD_seekable_endStream, ZSTD_seekable_freeCStream,
    ZSTD_seekable_initCStream,
};

use crate::{
    parse_code, ptr_mut, CompressionLevel, InBuffer, OutBuffer, SafeResult,
    WriteBuf,
};

pub struct CStream(NonNull<ZSTD_seekable_CStream>);
impl CStream {
    /// Tries to create a `CStream`
    ///
    /// Returns `None` if zstd returns a NULL pointer - may happen if allocation fails
    pub fn try_create() -> Option<Self> {
        let cstream = unsafe { ZSTD_seekable_createCStream() };
        NonNull::new(cstream).map(Self)
    }
    /// Wrap `ZSTD_seekable_CStream`
    ///
    /// # Panics
    ///
    /// If zstd returns a NULL pointer.
    pub fn create() -> Self {
        Self::try_create()
            .expect("zstd returned null pointer when creating new CStream")
    }
    /// Initializes the CStream with the given compression level, checksum, and max frame size
    ///
    /// ## Parameters
    ///
    /// - `compression_level`: Stream compression level
    /// - `checksum`: Indicates whether or not the seek table should include frame checksums on the uncomrpessed data for verification
    /// - `max_frame_size`: Indicates the size at which to automatically start a new seekable frame
    ///
    /// ## Return
    ///
    /// Returns a size hint for input to provide for compression, or an error
    /// code if there was an error.
    ///
    /// ### Notes
    ///
    /// Passing `None` to `max_frame_size` implies the default maximum size.
    ///
    /// Smaller frame sizes allow faster decompression of small segments, since
    /// retrieving a single byte requires decompression of the full frame where
    /// the byte belongs.
    ///
    /// In general, size the frames to roughly correspond to the access granularity
    /// (when it's known).
    ///
    /// But small sizes also reduce compression ratio.
    ///
    /// Avoid really tiny frame sizes (< 1 kb), that would hurt compression ratio
    /// considerably.
    pub fn init(
        &mut self,
        compression_level: CompressionLevel,
        checksum: bool,
        max_frame_size: Option<NonZeroU32>,
    ) -> SafeResult {
        let checksum = if checksum { 1 } else { 0 };
        let max_frame_size =
            max_frame_size.map(|v| v.into()).unwrap_or_default();
        let code = unsafe {
            ZSTD_seekable_initCStream(
                self.0.as_ptr(),
                compression_level,
                checksum,
                max_frame_size,
            )
        };
        parse_code(code)
    }
    /// Consumes an input stream.
    ///
    /// ## Parameters
    ///
    /// - `output`: Destination buffer to write compressed data
    /// - `input`: Source buffer containing uncompressed data
    ///
    /// ## Return
    ///
    /// Returns a size hint, preferred number of bytes to use as input for next
    /// function call, or an error code.
    ///
    /// ### Notes
    ///
    /// This function will automatically update both `pos` fields.
    ///
    /// Note that it may not consume the entire input, in which case `pos < size`,
    /// and it's up to the caller to present again remaining data.
    ///
    /// Returned value is just a hint, to help latency a little, any other value will
    /// work fine.
    pub fn compress<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
        input: &mut InBuffer<'_>,
    ) -> SafeResult {
        let mut output = output.wrap();
        let mut input = input.wrap();
        let code = unsafe {
            ZSTD_seekable_compressStream(
                self.0.as_ptr(),
                ptr_mut(&mut output),
                ptr_mut(&mut input),
            )
        };
        parse_code(code)
    }
    /// Ends the current frame and starts a new one.
    ///
    /// ## Parameters
    ///
    /// - `output`: Output buffer to receive the rest of the frame
    ///
    /// ## Return
    ///
    /// Returns nothing on success, error code on error
    pub fn end_frame<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
    ) -> Result<(), usize> {
        let mut output = output.wrap();
        let code = unsafe {
            ZSTD_seekable_endFrame(self.0.as_ptr(), ptr_mut(&mut output))
        };
        parse_code(code).map(|_| ())
    }
    /// Ends the current frame, and then writes the seek table so that decompressors
    /// can efficiently find compressed frames.
    ///
    /// ## Parameters
    ///
    /// - `output`: Output buffer to receive the rest of the frame and seek table
    ///
    /// ## Returns
    ///
    /// `0` on success, a number > 0 if the buffer was too small to receive
    /// full data, and an error code on error.
    ///
    /// ### Notes
    ///
    /// If a number > 0 is returned, this function could be called again until
    /// all remaining data is flushed out and 0 is returned.
    pub fn end_stream<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
    ) -> SafeResult {
        let mut output = output.wrap();
        let code = unsafe {
            ZSTD_seekable_endStream(self.0.as_ptr(), ptr_mut(&mut output))
        };
        parse_code(code)
    }
}
impl Drop for CStream {
    fn drop(&mut self) {
        unsafe { ZSTD_seekable_freeCStream(self.0.as_ptr()) };
    }
}
