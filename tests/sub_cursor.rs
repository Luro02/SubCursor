use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use pretty_assertions::assert_eq;
use sub_cursor::SubCursor;

#[test]
fn test_new() { SubCursor::new().start(20).end(100).preserve(true); }

#[test]
fn test_seek_start() {
    let buffer = (0..255).into_iter().map(|x| x as u8).collect::<Vec<_>>();
    let mut sub_cursor = SubCursor::from(Cursor::new(buffer))
        .start(20)
        .end(100)
        .preserve(false);

    // normal seek:
    assert_eq!(sub_cursor.seek(SeekFrom::Start(20)).unwrap(), 20);
    // seek past the end:
    assert_eq!(sub_cursor.seek(SeekFrom::Start(200)).unwrap(), 40);
    // seek 0
    assert_eq!(sub_cursor.seek(SeekFrom::Start(0)).unwrap(), 0);
}

#[test]
fn test_seek_current() {
    let buffer: Vec<u8> = (0..255).into_iter().map(|x| x as u8).collect();
    let mut sub_cursor = SubCursor::from(Cursor::new(buffer))
        .start(20)
        .end(100)
        .preserve(false);

    // normal seek
    assert_eq!(sub_cursor.seek(SeekFrom::Current(20)).unwrap(), 20);
    assert_eq!(sub_cursor.seek(SeekFrom::Current(20)).unwrap(), 40);

    // seek past the end:
    assert_eq!(sub_cursor.seek(SeekFrom::Current(200)).unwrap(), 0);
    assert_eq!(sub_cursor.seek(SeekFrom::Current(200)).unwrap(), 40);
    assert_eq!(sub_cursor.seek(SeekFrom::Current(220)).unwrap(), 20);
    assert_eq!(sub_cursor.seek(SeekFrom::Current(200)).unwrap(), 60);

    // seek 0
    assert_eq!(sub_cursor.seek(SeekFrom::Current(0)).unwrap(), 60);

    // seek back:
    assert_eq!(sub_cursor.seek(SeekFrom::Current(-10)).unwrap(), 50);
    assert_eq!(sub_cursor.seek(SeekFrom::Current(-50)).unwrap(), 0);
}

#[test]
fn test_seek_end() {
    let buffer: Vec<u8> = (0..255).into_iter().map(|x| x as u8).collect();
    let mut sub_cursor = SubCursor::from(Cursor::new(buffer))
        .start(20)
        .end(100)
        .preserve(false);

    // seek back
    assert_eq!(sub_cursor.seek(SeekFrom::End(-20)).unwrap(), 60);
    assert_eq!(sub_cursor.seek(SeekFrom::End(-40)).unwrap(), 40);
    assert_eq!(sub_cursor.seek(SeekFrom::End(-80)).unwrap(), 0);
    // seek past the end:
    assert_eq!(sub_cursor.seek(SeekFrom::End(1)).unwrap(), 1);
    assert_eq!(sub_cursor.seek(SeekFrom::End(30)).unwrap(), 30);
    // seek 0
    assert_eq!(sub_cursor.seek(SeekFrom::End(0)).unwrap(), 80)
}

#[test]
fn test_read() {
    let buffer: Vec<u8> = (0..255).into_iter().map(|x| x as u8).collect();
    let mut sub_cursor = SubCursor::from(Cursor::new(buffer))
        .start(20)
        .end(100)
        .preserve(false);

    let mut result_buffer = [0; 4];
    // read the first 4 bytes:
    assert_eq!(4, sub_cursor.read(&mut result_buffer).unwrap());
    assert_eq!(&result_buffer, &[20, 21, 22, 23]);

    // read the entire rest of the buffer:
    let mut result_buffer = [0; 76];
    assert_eq!(76, sub_cursor.read(&mut result_buffer).unwrap());
}

#[test]
fn test_preserve() {
    let buffer: Vec<u8> = (0..255).into_iter().map(|x| x as u8).collect();

    let cursor = Arc::new(Mutex::new(Cursor::new(buffer)));

    let mut sub_cursor = SubCursor::from(cursor.clone())
        .start(20)
        .end(100)
        .preserve(true);

    // seek the underlying cursor to 20
    {
        cursor.lock().unwrap().set_position(20);
    }

    // read some data from the stream
    let mut result_buffer = [0; 4];
    assert_eq!(4, sub_cursor.read(&mut result_buffer).unwrap());
    assert_eq!(&result_buffer, &[20, 21, 22, 23]);

    // the position of the cursor should still be at 20
    {
        assert_eq!(cursor.lock().unwrap().position(), 20);
    }

    // the cursor, should act like it's at old_position + number_of_read_bytes,
    // even though the underlying cursor is still at old_position.
    //assert_eq!(sub_cursor.absolute_position(), 24);
}

#[test]
fn test_write() {
    let cursor = Arc::new(Mutex::new(Cursor::new(vec![])));

    let mut sub_cursor = SubCursor::from(cursor.clone())
        .start(20)
        .end(100)
        .preserve(false);

    assert_eq!(sub_cursor.write(&[0, 1, 2, 3, 4]).unwrap(), 5);
    assert_eq!(
        cursor.lock().unwrap().get_ref(),
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4]
    );
    assert_eq!(80, sub_cursor.seek(SeekFrom::End(0)).unwrap());
    assert_eq!(sub_cursor.write(&[0, 1, 2, 3, 4]).unwrap(), 0);
    assert_eq!(77, sub_cursor.seek(SeekFrom::End(-3)).unwrap());
    assert_eq!(sub_cursor.write(&[0, 1, 2, 3, 4]).unwrap(), 3);
}

#[test]
fn test_len() {
    let sub_cursor = SubCursor::new().start(20).end(100).preserve(false);

    assert_eq!(sub_cursor.len(), 80);

    let sub_cursor = SubCursor::new().start(20).end(20).preserve(false);

    assert_eq!(sub_cursor.len(), 0);
}

#[test]
fn test_position() {
    let mut sub_cursor = SubCursor::new().start(20).end(100).preserve(false);

    assert_eq!(sub_cursor.position(), 0);
    assert_eq!(20, sub_cursor.seek(SeekFrom::Start(20)).unwrap());
    assert_eq!(sub_cursor.position(), 20);
}
