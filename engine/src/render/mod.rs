use generational_arena::Arena;

pub mod builder;
pub mod camera;
pub mod color;
pub mod framework;
pub mod material;
pub mod mesh;
pub mod time;
pub mod vertex;

pub const ARENA_CAPACITY: usize = 32;

pub struct BufferArena {
    pub arena: Arena<wgpu::Buffer>,
}

impl Default for BufferArena {
    fn default() -> Self {
        Self {
            arena: Arena::with_capacity(ARENA_CAPACITY),
        }
    }
}
