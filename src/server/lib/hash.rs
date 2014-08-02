

extern crate rustc;
use self::rustc::util::sha2::{Digest};

mod hash {
    
    struct HasEngine {
        engine: (extern "C" fn(*mut u8, c_uint, *mut u8) -> *mut u8),
        digest_size: uint,
        block_size: uint
    }

    enum HashMethod {
        MD5,
        SHA1,
        SHA224,
        SHA256,
        SHA384,
        SHA512
    }

    impl HashEngine {
        fn new(engine: HashMethod) -> &HashEngine {
            match engine {
                MD5 => &HashEngine {
                    engine: crypto::MD5,
                    digest_size: 16,
                    block_size: 64
                },
                SHA1 => &HashEngine {
                    engine: crypto::MD5,
                    digest_size: 20,
                    block_size: 64
                },
                SHA224 => &HashEngine {
                    engine: crypto::MD5,
                    digest_size: 28,
                    block_size: 64
                },
                SHA256 => &HashEngine {
                    engine: crypto::MD5,
                    digest_size: 32,
                    block_size: 64
                },
                SHA384 => &HashEngine {
                    engine: crypto::MD5,
                    digest_size: 48,
                    block_size: 64
                },
                SHA512 => &HashEngine {
                    engine: crypto::MD5,
                    digest_size: 64,
                    block_size: 128
                }
            }
        }

        fn hash(&self, data: &[u8]) -> &Digest {
            let hash_func = self.engine;
            Digest::new(unsafe {
                vec::from_buf(hash_func(
                    vec::raw::to_ptr(data),
                    data.len() as c_uint,
                    ptr::null()), self.digest_size)
            })
        }
    }
}
