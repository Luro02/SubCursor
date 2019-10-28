// Other library, that does almost the same
// https://github.com/hinaria/slice/
use std::fmt;
use std::io::{self, Cursor};
use std::io::{Read, Seek, SeekFrom, Write};

use std::sync::{Arc, Mutex};

/// A [`SubCursor`] allows to only have access to parts of the underlying
/// [`Read`]er or [`Write`]r.
///
/// # Example
///
/// ```
/// # use sub_cursor::SubCursor;
/// # use std::io;
/// use std::io::{Cursor, Read};
///
/// # fn main() -> io::Result<()> {
/// let cursor = Cursor::new(b"Hello World from a SubCursor!".to_vec());
/// let mut sub_cursor = SubCursor::from(cursor).start(19).end(28);
///
/// // read all the data in the reader
/// let mut result = [0; 9];
/// # assert_eq!(9,
/// sub_cursor.read(&mut result)?
/// # );
/// assert_eq!(&result, b"SubCursor");
/// # Ok(())
/// # }
/// ```
///
/// # Note
///
/// There are only [`Default`], [`Debug`] and [`Clone`] implemented for
/// [`SubCursor`], because a [`Mutex`] is used internally, which doesn't
/// implement [`PartialEq`], [`PartialOrd`], [`Eq`], [`Ord`] and [`Hash`].
///
/// There is a proposal for this here: <https://github.com/rust-lang/rfcs/issues/2055>
///
/// [`Debug`]: std::fmt::Debug
/// [`Hash`]: std::hash::Hash
#[derive(Default, Debug, Clone)]
pub struct SubCursor<T> {
    cursor: Arc<Mutex<T>>,
    start: usize,
    end: usize,
    position: u64,
    preserve: bool,
}

#[allow(dead_code)]
impl SubCursor<Cursor<Vec<u8>>> {
    /// Creates a new [`SubCursor`], with an underlying vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// let sub_cursor = SubCursor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cursor: Arc::new(Mutex::new(Cursor::new(vec![]))),
            start: 0,
            end: 0,
            position: 0,
            preserve: false,
        }
    }
}

#[allow(dead_code)]
impl<T> SubCursor<T> {
    /// Sets the start of the [`SubCursor`].
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// use std::io::Read;
    ///
    /// // create a SubCursor, that starts at 3
    /// let mut sub_cursor = SubCursor::from(vec![1, 2, 3, 4, 5, 6]).start(3);
    /// let mut buffer = vec![0; 3];
    ///
    /// // read all bytes of the SubCursor
    /// # let value =
    /// sub_cursor.read(&mut buffer)?;
    /// # assert_eq!(value, 3);
    /// assert_eq!(buffer, vec![4, 5, 6]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// It will reset the position to the start, to prevent the construction of
    /// an invalid [`SubCursor`].
    // TODO: what about the case, where one could try to construct a [`SubCursor`],
    // with a start, that's bigger than the end? like this
    // SubCursor::new().start(7).end(3)? this should cause a panic!
    pub fn start(&self, value: usize) -> Self {
        Self {
            // very cheap to clone:
            cursor: self.cursor.clone(),
            start: value,
            position: value as u64,
            end: self.end,
            preserve: self.preserve,
        }
    }

    /// Sets the end of the [`SubCursor`].
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// use std::io::Read;
    ///
    /// // create a SubCursor, that starts at 3 and ends at 5
    /// let mut sub_cursor = SubCursor::from(vec![1, 2, 3, 4, 5, 6]).start(3).end(5);
    /// let mut buffer = vec![0; 2];
    ///
    /// // read all bytes of the SubCursor
    /// # let value =
    /// sub_cursor.read(&mut buffer)?;
    /// # assert_eq!(value, 2);
    /// assert_eq!(buffer, vec![4, 5]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// It will reset the position to the start, to prevent the construction of
    /// an invalid [`SubCursor`]. This function won't check for validity of the
    /// end value.
    pub fn end(&self, value: usize) -> Self {
        Self {
            // very cheap to clone:
            cursor: self.cursor.clone(),
            start: self.start,
            position: self.start as u64,
            end: value,
            preserve: self.preserve,
        }
    }

    /// The [`SubCursor`] won't change the position of the underlying cursor.
    /// Normally after some data is read, the underlying cursor will also move,
    /// but this flag `preserves` the position of the underlying cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// # use std::io;
    /// use std::io::{Cursor, Read};
    /// use std::sync::{Arc, Mutex};
    ///
    /// # fn main() -> io::Result<()> {
    /// let cursor = Arc::new(Mutex::new(Cursor::new(vec![b'#'; 200])));
    ///
    /// let mut sub_cursor = SubCursor::from(cursor.clone())
    ///     .start(20)
    ///     .end(100)
    ///     .preserve(true);
    ///
    /// // read some data from the SubCursor
    /// let mut buffer = vec![0; 20];
    /// // this will seek the SubCursor to position 20 and the
    /// // internal cursor would be at 40, because 20 bytes were read.
    /// sub_cursor.read(&mut buffer)?;
    ///
    /// assert_eq!(sub_cursor.position(), 20);
    /// // but this is not the case and the internal cursor is still at it's original position,
    /// // because the preserve option has been enabled.
    /// assert_eq!(cursor.lock().unwrap().position(), 0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// The preserve option is enabled by default and should be disabled, if
    /// seek operations of the underlying cursor take a long time.
    pub fn preserve(&self, value: bool) -> Self {
        Self {
            // very cheap to clone:
            cursor: self.cursor.clone(),
            start: self.start,
            position: self.position,
            end: self.end,
            preserve: value,
        }
    }

    /// Returns the length of this cursor.
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// let sub_cursor = SubCursor::new().start(4).end(10);
    ///
    /// assert_eq!(sub_cursor.len(), 6);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// This call does not check the size of the underlying stream and instead
    /// simply subtracts the provided [`end`] from the [`start`].
    ///
    /// [`start`]: #method.start
    /// [`end`]: #method.end
    #[inline]
    pub const fn len(&self) -> usize { (self.end - self.start) as usize }

    /// Returns `true`, if the cursor has a length of `0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// let sub_cursor = SubCursor::new();
    /// assert!(sub_cursor.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns the current position of this [`SubCursor`].
    ///
    /// # Example
    ///
    /// ```
    /// #![feature(seek_convenience)]
    /// # use sub_cursor::SubCursor;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// use std::io::{Seek, SeekFrom};
    ///
    /// let mut sub_cursor = SubCursor::new().end(20);
    /// sub_cursor.seek(SeekFrom::Start(10))?;
    ///
    /// assert_eq!(sub_cursor.position(), 10);
    /// assert_eq!(sub_cursor.stream_position()?, 10);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub const fn position(&self) -> u64 { (self.position - self.start as u64) as u64 }

    /// Sets the position of this cursor.
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// use std::io::{Seek, SeekFrom};
    ///
    /// let mut sub_cursor = SubCursor::new().end(20);
    /// sub_cursor.seek(SeekFrom::Start(10))?;
    ///
    /// assert_eq!(sub_cursor.position(), 10);
    /// sub_cursor.set_position(5);
    ///
    /// assert_eq!(sub_cursor.position(), 5);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn set_position(&mut self, pos: u64) {
        self.position = pos.checked_rem(self.len() as u64).unwrap_or(pos) + self.start as u64
    }

    /// Create a new [`SubCursor`] from this [`SubCursor`].
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// let sub_cursor = SubCursor::new().start(5).end(10);
    /// let another_sub_cursor = sub_cursor.sub_cursor();
    /// ```
    ///
    /// # Note
    ///
    /// The [`preserve`] option will be inherited and the position will be set
    /// to [`start`].
    ///
    /// [`start`]: #method.start
    /// [`preserve`]: #method.preserve
    pub fn sub_cursor(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            start: self.start,
            end: self.end,
            position: self.start as u64,
            preserve: self.preserve,
        }
    }

    /// Consumes this cursor, returning the underlying value.
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::io::Cursor;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let sub_cursor = SubCursor::new();
    /// let internal_value: Arc<Mutex<Cursor<Vec<u8>>>> = sub_cursor.into_inner();
    ///
    /// // converting the internal value to a Vec<u8>
    /// let buffer = Arc::try_unwrap(internal_value)
    ///     // the unwrap will fail, if there is more, than one reference to the underlying data
    ///     // active
    ///     .expect("failed to unwrap Arc<T>, because another Arc is active!")
    ///     .into_inner()?
    ///     .into_inner();
    ///
    /// assert_eq!(buffer, vec![]);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn into_inner(self) -> Arc<Mutex<T>> { self.cursor }

    /// Returns the [`start`] of this [`SubCursor`].
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// let sub_cursor = SubCursor::new().start(4);
    ///
    /// assert_eq!(sub_cursor.get_start(), 4);
    /// ```
    ///
    /// [`start`]: #method.start
    #[inline]
    pub const fn get_start(&self) -> usize { self.start }

    /// Returns the [`end`] of this [`SubCursor`].
    ///
    /// # Example
    ///
    /// ```
    /// # use sub_cursor::SubCursor;
    /// let sub_cursor = SubCursor::new().end(4);
    ///
    /// assert_eq!(sub_cursor.get_end(), 4);
    /// ```
    ///
    /// [`end`]: #method.end
    #[inline]
    pub const fn get_end(&self) -> usize { self.end }
}

impl<T> Seek for SubCursor<T>
where
    T: Seek,
{
    /// Seek to the provided position.
    ///
    /// # Error
    ///
    /// This function will error, if you attempt to seek before 0.
    ///
    /// # Notes
    ///
    /// This function is "lazy" and does not mutate the internal cursor,
    /// until read or write is called.
    ///
    /// Seeking beyond the end will cause the cursor to "overflow":
    /// ```rust
    /// # use std::io;
    /// # use sub_cursor::SubCursor;
    /// # fn main() -> io::Result<()> {
    /// use std::io::{Seek, SeekFrom};
    ///
    /// let mut sub_cursor = SubCursor::new().end(20);
    /// sub_cursor.seek(SeekFrom::Current(2))?;
    ///
    /// let position = sub_cursor.seek(SeekFrom::Current(usize::max_value() as i64))?;
    /// assert_eq!(position, 1);
    /// # Ok(())
    /// # }
    /// ```
    fn seek(&mut self, style: SeekFrom) -> io::Result<u64> {
        let mut relative_position = self.position();

        // early return, because if the length is 0, there is nothing to seek...
        if self.len() == 0 {
            return Ok(0);
        }

        let (base_pos, offset) = match style {
            SeekFrom::Start(offset) => {
                relative_position = offset.checked_rem(self.len() as u64).unwrap_or(offset);
                self.position = relative_position + self.start as u64;

                return Ok(relative_position);
            }
            SeekFrom::End(offset) => (self.len() as u64, offset),
            SeekFrom::Current(offset) => (relative_position, offset),
        };

        let new_pos = {
            if offset >= 0 {
                Some(base_pos.wrapping_add(offset.checked_abs().unwrap_or(0) as u64))
            } else {
                base_pos.checked_sub(offset.wrapping_neg().checked_abs().unwrap_or(0) as u64)
            }
        };

        match new_pos {
            Some(n) => {
                if n > self.len() as u64 {
                    relative_position = n.checked_rem(self.len() as u64).unwrap_or(n);
                } else {
                    relative_position = n;
                }
                self.position = relative_position + self.start as u64;
                Ok(relative_position)
            }
            None => {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid seek to a negative offset",
                ))
            }
        }
    }

    fn stream_len(&mut self) -> io::Result<u64> { Ok(self.len() as u64) }

    fn stream_position(&mut self) -> io::Result<u64> { Ok(self.position()) }
}

// calculates the number of available bytes.
fn calculate_available_bytes(buffer_length: u64, end: u64, position: u64) -> u64 {
    // if the wanted bytes are more, than there is available:
    if buffer_length > end - position {
        // reduce it to the maximum number of bytes:
        end - position
    } else {
        buffer_length
    }
}

impl<T> Read for SubCursor<T>
where
    T: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.position >= self.end as u64 {
            Ok(0)
        } else {
            // check how many bytes are available:
            let available_bytes =
                calculate_available_bytes(buf.len() as u64, self.end as u64, self.position);

            let position = {
                let mut cursor = self.cursor.lock().unwrap();
                cursor.stream_position()?
            };

            // seek to the current position
            {
                let mut cursor = self.cursor.lock().unwrap();
                cursor.seek(SeekFrom::Start(self.position as u64))?;
            }

            // result is the number of bytes, that have been read
            let result = {
                let mut cursor = self.cursor.lock().unwrap();
                cursor.by_ref().take(available_bytes).read(buf)?
            };

            // seek back to the old position, if preserve is enabled
            if self.preserve {
                // seek to the old position
                {
                    let mut cursor = self.cursor.lock().unwrap();
                    cursor.seek(SeekFrom::Start(position))?;
                }
            }

            // update the new absolute position
            self.position += result as u64;

            Ok(result)
        }
    }
}

impl<T> Write for SubCursor<T>
where
    T: Write + Seek,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // skip if the cursor is at the EOF
        if self.position >= self.end as u64 {
            return Ok(0);
        }

        // check how many bytes are available:
        let available_bytes =
            calculate_available_bytes(buf.len() as u64, self.end as u64, self.position);

        let position = {
            if self.preserve {
                // remember old position:
                {
                    Some(self.cursor.lock().unwrap().stream_position()?)
                }
            } else {
                None
            }
        };

        // seek to the current position
        {
            let mut cursor = self.cursor.lock().unwrap();
            cursor.seek(SeekFrom::Start(self.position as u64))?;
        }

        // write as many bytes as possible in the buffer
        let result = {
            self.cursor
                .lock()
                .unwrap()
                .write(&buf[0..available_bytes as usize])?
        };

        if let Some(position) = position {
            // seek to the old position
            {
                let mut cursor = self.cursor.lock().unwrap();
                cursor.seek(SeekFrom::Start(position))?;
            }
        }

        Ok(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        // flush the underlying writer
        {
            self.cursor.lock().unwrap().flush()?;
        }
        Ok(())
    }
}

/// Creates a [`SubCursor`] from any type, that implements [`Seek`].
///
/// # Example
///
/// ```
/// # use sub_cursor::SubCursor;
/// use std::io::Cursor;
///
/// let cursor = Cursor::new(vec![1, 2, 3, 4, 5, 6]);
/// let sub_cursor = SubCursor::from(cursor);
///
/// assert_eq!(sub_cursor.get_start(), 0);
/// assert_eq!(sub_cursor.get_end(), 6);
/// ```
///
/// # Note
///
/// The [`SubCursor`] will start at `0` and end at the `end of the stream` or
/// `0`, if it fails to get the end via [`Seek::stream_len`].
///
/// By default the [`preserve`] option is enabled.
///
/// [`preserve`]: #method.preserve
impl<T: Seek> From<T> for SubCursor<T> {
    fn from(mut value: T) -> Self {
        Self {
            start: 0,
            end: value.stream_len().unwrap_or(0) as usize,
            cursor: Arc::new(Mutex::new(value)),
            position: 0,
            preserve: true,
        }
    }
}

impl From<Vec<u8>> for SubCursor<Cursor<Vec<u8>>> {
    fn from(value: Vec<u8>) -> Self {
        Self {
            start: 0,
            end: value.len(),
            cursor: Arc::new(Mutex::new(Cursor::new(value))),
            position: 0,
            preserve: true,
        }
    }
}

// TODO: missing end?!
impl<T> From<Mutex<T>> for SubCursor<T> {
    fn from(value: Mutex<T>) -> Self {
        Self {
            start: 0,
            end: 0,
            cursor: Arc::new(value),
            position: 0,
            preserve: true,
        }
    }
}

// TODO: missing end
impl<T> From<Arc<Mutex<T>>> for SubCursor<T> {
    fn from(value: Arc<Mutex<T>>) -> Self {
        Self {
            cursor: value,
            start: 0,
            end: 0,
            position: 0,
            preserve: true,
        }
    }
}

/// Display implementation for a [`SubCursor`]. The first number is the length
/// of the stream and the second is the current position in the stream.
///
/// # Example
///
/// ```
/// # use sub_cursor::SubCursor;
/// let sub_cursor = SubCursor::new().start(10).end(12345).preserve(false);
///
/// assert_eq!(
///     sub_cursor.to_string(),
///     "SubCursor<12335@0, preserve=false>".to_string()
/// );
/// ```
impl<T> fmt::Display for SubCursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SubCursor<{}@{}, preserve={}>",
            self.len(),
            self.position(),
            self.preserve
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            SubCursor::new().to_string(),
            "SubCursor<0@0, preserve=false>".to_string()
        );

        assert_eq!(
            SubCursor::new().preserve(true).start(5).end(10).to_string(),
            "SubCursor<5@0, preserve=true>".to_string()
        );
    }

    #[test]
    fn test_seek_maximum() {
        let mut sub_cursor = SubCursor::new().start(0).end(usize::max_value());

        sub_cursor.seek(SeekFrom::Current(1)).unwrap();

        let position = sub_cursor
            .seek(SeekFrom::Current(u64::max_value() as i64))
            .unwrap();

        assert_eq!(position, 0);

        sub_cursor.seek(SeekFrom::Current(2)).unwrap();

        let position = sub_cursor
            .seek(SeekFrom::Current(u64::max_value() as i64))
            .unwrap();

        assert_eq!(position, 1);

        let position = sub_cursor.seek(SeekFrom::Start(u64::max_value())).unwrap();
        assert_eq!(position, 0);
    }
}
