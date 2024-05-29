#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster RD_KEY%s, containing RD_KEY?_DATA0, RD_KEY?_DATA1, RD_KEY?_DATA2, RD_KEY?_DATA3, RD_KEY?_DATA4, RD_KEY?_DATA5, RD_KEY?_DATA6, RD_KEY?_DATA7"]
pub struct RD_KEY {
    data: [DATA; 8],
}
impl RD_KEY {
    #[doc = "0x00..0x20 - Register %s of BLOCK4 (KEY0)."]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &DATA {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Register %s of BLOCK4 (KEY0)."]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &DATA> {
        self.data.iter()
    }
}
#[doc = "DATA (r) register accessor: Register %s of BLOCK4 (KEY0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Register %s of BLOCK4 (KEY0)."]
pub mod data;
