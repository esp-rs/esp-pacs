#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Initial configuration register."]
    pub mode: MODE,
    #[doc = "0x04 - SHA 512/t configuration register 0."]
    pub t_string: T_STRING,
    #[doc = "0x08 - SHA 512/t configuration register 1."]
    pub t_length: T_LENGTH,
    #[doc = "0x0c - DMA configuration register 0."]
    pub dma_block_num: DMA_BLOCK_NUM,
    #[doc = "0x10 - Typical SHA configuration register 0."]
    pub start: START,
    #[doc = "0x14 - Typical SHA configuration register 1."]
    pub continue_: CONTINUE,
    #[doc = "0x18 - Busy register."]
    pub busy: BUSY,
    #[doc = "0x1c - DMA configuration register 1."]
    pub dma_start: DMA_START,
    #[doc = "0x20 - DMA configuration register 2."]
    pub dma_continue: DMA_CONTINUE,
    #[doc = "0x24 - Interrupt clear register."]
    pub clear_irq: CLEAR_IRQ,
    #[doc = "0x28 - Interrupt enable register."]
    pub irq_ena: IRQ_ENA,
    #[doc = "0x2c - Date register."]
    pub date: DATE,
    _reserved12: [u8; 0x10],
    #[doc = "0x40..0x80 - Sha H memory which contains intermediate hash or finial hash."]
    pub h_mem: [H_MEM; 64],
    #[doc = "0x80..0xc0 - Sha M memory which contains message."]
    pub m_mem: [M_MEM; 64],
}
#[doc = "MODE (rw) register accessor: Initial configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Initial configuration register."]
pub mod mode;
#[doc = "T_STRING (rw) register accessor: SHA 512/t configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_string::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_string::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t_string`] module"]
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
#[doc = "SHA 512/t configuration register 0."]
pub mod t_string;
#[doc = "T_LENGTH (rw) register accessor: SHA 512/t configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t_length`] module"]
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
#[doc = "SHA 512/t configuration register 1."]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM (rw) register accessor: DMA configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_block_num`] module"]
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
#[doc = "DMA configuration register 0."]
pub mod dma_block_num;
#[doc = "START (w) register accessor: Typical SHA configuration register 0.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Typical SHA configuration register 0."]
pub mod start;
#[doc = "CONTINUE (w) register accessor: Typical SHA configuration register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "Typical SHA configuration register 1."]
pub mod continue_;
#[doc = "BUSY (r) register accessor: Busy register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busy`] module"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Busy register."]
pub mod busy;
#[doc = "DMA_START (w) register accessor: DMA configuration register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_start`] module"]
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
#[doc = "DMA configuration register 1."]
pub mod dma_start;
#[doc = "DMA_CONTINUE (w) register accessor: DMA configuration register 2.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_continue`] module"]
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
#[doc = "DMA configuration register 2."]
pub mod dma_continue;
#[doc = "CLEAR_IRQ (w) register accessor: Interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_irq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clear_irq`] module"]
pub type CLEAR_IRQ = crate::Reg<clear_irq::CLEAR_IRQ_SPEC>;
#[doc = "Interrupt clear register."]
pub mod clear_irq;
#[doc = "IRQ_ENA (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`irq_ena`] module"]
pub type IRQ_ENA = crate::Reg<irq_ena::IRQ_ENA_SPEC>;
#[doc = "Interrupt enable register."]
pub mod irq_ena;
#[doc = "DATE (rw) register accessor: Date register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
#[doc = "H_MEM (rw) register accessor: Sha H memory which contains intermediate hash or finial hash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`h_mem`] module"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "Sha H memory which contains intermediate hash or finial hash."]
pub mod h_mem;
#[doc = "M_MEM (rw) register accessor: Sha M memory which contains message.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Sha M memory which contains message."]
pub mod m_mem;
