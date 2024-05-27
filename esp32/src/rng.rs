#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0144],
    data: DATA,
}
impl RegisterBlock {
    ///0x144 - Random number data
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
}
/**DATA (r) register accessor: Random number data

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
pub type DATA = crate::Reg<data::DATA_SPEC>;
///Random number data
pub mod data;
