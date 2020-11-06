#![no_std]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "untrusted")] {
        #[macro_use]
        extern crate serde;
        extern crate std;
        use std::prelude::v1::*;
    } else if #[cfg(feature = "trusted")] {
        #[macro_use]
        extern crate serde_sgx;
        extern crate sgx_tstd as std;
        use std::prelude::v1::*;
    } else {
        compile_error!{"must be either trusted or untrusted"}
    }
}

#[cfg_attr(feature = "trusted", serde(crate = "serde_sgx"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct MySharedStruct {
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
