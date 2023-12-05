use super::*;

#[test]
fn it_will_create_a_byte_arena() {
    let mut ba = ByteArena::builder().limit(5).size(1024).build();

    let block = Bytes::from("Hello ");
    ba.append(block).expect("append worked");
    let block = Bytes::from("Gary");
    ba.append(block).expect("append worked");
    let final_bytes = Bytes::from(ba);
    assert_eq!(
        "Hello Gary",
        std::str::from_utf8(&final_bytes).expect("it's a valid string")
    );
}

#[test]
fn it_enforces_block_limits() {
    let mut ba = ByteArena::builder().limit(2).size(1024).build();

    let b = ba.alloc();
    let _ = ba.append(b);
    let b = ba.alloc();
    let _ = ba.append(b);
    let b = ba.alloc();

    assert_eq!(ba.append(b), Err(ByteArenaError::TooManyBlocks));
}

#[test]
fn it_enforces_block_sizes() {
    let mut ba = ByteArena::builder().limit(2).size(4).build();

    let block = Bytes::from_static(b"hello");

    assert_eq!(ba.append(block), Err(ByteArenaError::BlockTooLarge));
}
