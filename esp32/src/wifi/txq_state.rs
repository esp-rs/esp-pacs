#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "State of transmission queues"]
pub struct TXQ_STATE {
    tx_error_clear: TX_ERROR_CLEAR,
    tx_error_status: TX_ERROR_STATUS,
    tx_complete_clear: TX_COMPLETE_CLEAR,
    tx_complete_status: TX_COMPLETE_STATUS,
}
impl TXQ_STATE {
    #[doc = "0x00 - Clear the error status of a slot"]
    #[inline(always)]
    pub const fn tx_error_clear(&self) -> &TX_ERROR_CLEAR {
        &self.tx_error_clear
    }
    #[doc = "0x04 - Error status of a slot"]
    #[inline(always)]
    pub const fn tx_error_status(&self) -> &TX_ERROR_STATUS {
        &self.tx_error_status
    }
    #[doc = "0x08 - Clear the completion status of a slot"]
    #[inline(always)]
    pub const fn tx_complete_clear(&self) -> &TX_COMPLETE_CLEAR {
        &self.tx_complete_clear
    }
    #[doc = "0x0c - Completion status of a slot"]
    #[inline(always)]
    pub const fn tx_complete_status(&self) -> &TX_COMPLETE_STATUS {
        &self.tx_complete_status
    }
}
#[doc = "TX_ERROR_CLEAR (rw) register accessor: Clear the error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_error_clear`] module"]
pub type TX_ERROR_CLEAR = crate::Reg<tx_error_clear::TX_ERROR_CLEAR_SPEC>;
#[doc = "Clear the error status of a slot"]
pub mod tx_error_clear;
#[doc = "TX_ERROR_STATUS (rw) register accessor: Error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_error_status`] module"]
pub type TX_ERROR_STATUS = crate::Reg<tx_error_status::TX_ERROR_STATUS_SPEC>;
#[doc = "Error status of a slot"]
pub mod tx_error_status;
#[doc = "TX_COMPLETE_CLEAR (rw) register accessor: Clear the completion status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_complete_clear`] module"]
pub type TX_COMPLETE_CLEAR = crate::Reg<tx_complete_clear::TX_COMPLETE_CLEAR_SPEC>;
#[doc = "Clear the completion status of a slot"]
pub mod tx_complete_clear;
#[doc = "TX_COMPLETE_STATUS (rw) register accessor: Completion status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_complete_status`] module"]
pub type TX_COMPLETE_STATUS = crate::Reg<tx_complete_status::TX_COMPLETE_STATUS_SPEC>;
#[doc = "Completion status of a slot"]
pub mod tx_complete_status;
