use std::mem::MaybeUninit;
use std::hash::Hasher;

#[link(name = "polymur-hash")]
extern "C" {
    fn polymur_init_params_ext(p: *mut MaybeUninit<[u64; 4]>, k_seed: u64, s_seed: u64);
    fn polymur_init_params_from_seed_ext(p: *mut MaybeUninit<[u64; 4]>, seed: u64);
    fn polymur_hash_ext(buf: *const u8, len: usize, p: *const [u64; 4], tweak: u64) -> u64;
}

#[derive(Default)]
pub struct PolymurHasher {
    state: [u64; 4],
    memory: u64,
}

impl PolymurHasher {
    pub fn new() -> Self {
        Self::new_with_64bit_seed(std::f64::consts::PI.to_bits())
    }

    pub fn new_with_64bit_seed(seed: u64) -> Self {
        let mut state = MaybeUninit::<[u64; 4]>::uninit();

        unsafe {
            polymur_init_params_from_seed_ext(&mut state, seed);
            Self {
                state: state.assume_init(),
                memory: 0,
            }
        }
    }

    pub fn new_with_128bit_seed(seed: u128) -> Self {
        let mut state = MaybeUninit::<[u64; 4]>::uninit();

        unsafe {
            polymur_init_params_ext(
                &mut state,
                seed as u64,
                (seed >> 64) as u64,
            );

            Self {
                state: state.assume_init(),
                memory: 0,
            }
        }
    }
}

impl Hasher for PolymurHasher {
    fn write(&mut self, bytes: &[u8]) {
        unsafe {
            self.memory ^= polymur_hash_ext(bytes.as_ptr(), bytes.len(), &self.state, 0);
        }
    }

    fn finish(&self) -> u64 {
        self.memory
    }
}
