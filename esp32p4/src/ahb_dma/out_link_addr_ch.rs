#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster OUT_LINK_ADDR_CH%s, containing OUT_LINK_ADDR_CH?"]
pub struct OUT_LINK_ADDR_CH {
    out_link_addr: OUT_LINK_ADDR,
}
impl OUT_LINK_ADDR_CH {
    #[doc = "0x00 - Link list descriptor address configuration of TX channel 0"]
    #[inline(always)]
    pub const fn out_link_addr(&self) -> &OUT_LINK_ADDR {
        &self.out_link_addr
    }
}
#[doc = "OUT_LINK_ADDR (rw) register accessor: Link list descriptor address configuration of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr`] module"]
pub type OUT_LINK_ADDR = crate::Reg<out_link_addr::OUT_LINK_ADDR_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel 0"]
pub mod out_link_addr;
