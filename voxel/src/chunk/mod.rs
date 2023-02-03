use std::fmt::Debug;

use lazy_static::lazy_static;

use self::voxel::Voxel;

pub mod voxel;

pub const CHUNK_SIZE: usize = 32;
lazy_static! {
    // when SIZE 16, BIT_SIZE is 4
    // by shifting 16 << 4 we get 1
    // we with this get indexes from the collapsed array
    pub static ref BIT_SIZE: i32 = (CHUNK_SIZE as f32).log2() as i32;
}

pub struct Chunk {
    pub voxels: [Voxel; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn linearize([x, y, z]: &[i32; 3]) -> usize {
        (z | (y << *BIT_SIZE) | (x << (*BIT_SIZE * 2))) as usize
    }

    pub fn delinearize<T, E>(idx: T) -> [i32; 3]
    where
        T: TryInto<i32, Error = E>,
        E: Debug,
    {
        let val: i32 = idx.try_into().expect("");

        [
            (val as f32 / (CHUNK_SIZE * CHUNK_SIZE) as f32) as i32,
            ((val as f32 / CHUNK_SIZE as f32) % CHUNK_SIZE as f32) as i32,
            (val as f32 % CHUNK_SIZE as f32) as i32,
        ]
    }
}

pub fn test() {
    let [_x, _y, _z] = Chunk::delinearize(3i8);
    let [_x, _y, _z] = Chunk::delinearize(3usize);
    let [_x, _y, _z] = Chunk::delinearize(3i64);
}
