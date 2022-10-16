use super::*;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>; //

    /*
    //hash is implemented from crates.io crypto-hash, configured into cargo.toml row 17
    fn hash (&self) -> Vec<u8> { 
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
        //we need to import hashable to lib.rs row 87 & 88
        //now we have a hashable trait!
    }
     */

    fn hash (&self) -> Hash { //
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
