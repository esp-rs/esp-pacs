#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster RD_KEY%s, containing RD_KEY?_DATA%s"]
pub struct RD_KEY {
    rd_key_data: [RD_KEY_DATA; 8],
}
impl RD_KEY {
    #[doc = "0x00..0x20 - Register %s of BLOCK4 (KEY0)."]
    #[inline(always)]
    pub const fn rd_key_data(&self, n: usize) -> &RD_KEY_DATA {
        &self.rd_key_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Register %s of BLOCK4 (KEY0)."]
    #[inline(always)]
    pub fn rd_key_data_iter(&self) -> impl Iterator<Item = &RD_KEY_DATA> {
        self.rd_key_data.iter()
    }
}
#[doc = "RD_KEY_DATA (r) register accessor: Register %s of BLOCK4 (KEY0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key_data`] module"]
pub type RD_KEY_DATA = crate::Reg<rd_key_data::RD_KEY_DATA_SPEC>;
#[doc = "Register %s of BLOCK4 (KEY0)."]
pub mod rd_key_data;
