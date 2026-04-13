#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_LINK_ADDR_CH%s, containing IN_LINK_ADDR_CH?"]
pub struct IN_LINK_ADDR_CH {
    in_link_addr: IN_LINK_ADDR,
}
impl IN_LINK_ADDR_CH {
    #[doc = "0x00 - Link list descriptor address configuration of RX channel 0"]
    #[inline(always)]
    pub const fn in_link_addr(&self) -> &IN_LINK_ADDR {
        &self.in_link_addr
    }
}
#[doc = "IN_LINK_ADDR (rw) register accessor: Link list descriptor address configuration of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr`] module"]
pub type IN_LINK_ADDR = crate::Reg<in_link_addr::IN_LINK_ADDR_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel 0"]
pub mod in_link_addr;
