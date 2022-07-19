#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Initial configuration register."]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x04 - SHA 512/t configuration register 0."]
    pub t_string: crate::Reg<t_string::T_STRING_SPEC>,
    #[doc = "0x08 - SHA 512/t configuration register 1."]
    pub t_length: crate::Reg<t_length::T_LENGTH_SPEC>,
    #[doc = "0x0c - DMA configuration register 0."]
    pub dma_block_num: crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>,
    #[doc = "0x10 - Typical SHA configuration register 0."]
    pub start: crate::Reg<start::START_SPEC>,
    #[doc = "0x14 - Typical SHA configuration register 1."]
    pub continue_: crate::Reg<continue_::CONTINUE_SPEC>,
    #[doc = "0x18 - Busy register."]
    pub busy: crate::Reg<busy::BUSY_SPEC>,
    #[doc = "0x1c - DMA configuration register 1."]
    pub dma_start: crate::Reg<dma_start::DMA_START_SPEC>,
    #[doc = "0x20 - DMA configuration register 2."]
    pub dma_continue: crate::Reg<dma_continue::DMA_CONTINUE_SPEC>,
    #[doc = "0x24 - Interrupt clear register."]
    pub clear_irq: crate::Reg<clear_irq::CLEAR_IRQ_SPEC>,
    #[doc = "0x28 - Interrupt enable register."]
    pub irq_ena: crate::Reg<irq_ena::IRQ_ENA_SPEC>,
    #[doc = "0x2c - Date register."]
    pub date: crate::Reg<date::DATE_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40..0x80 - Sha H memory which contains intermediate hash or finial hash."]
    pub h_mem: [crate::Reg<h_mem::H_MEM_SPEC>; 64],
    #[doc = "0x80..0xc0 - Sha M memory which contains message."]
    pub m_mem: [crate::Reg<m_mem::M_MEM_SPEC>; 64],
}
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Initial configuration register."]
pub mod mode;
#[doc = "T_STRING register accessor: an alias for `Reg<T_STRING_SPEC>`"]
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
#[doc = "SHA 512/t configuration register 0."]
pub mod t_string;
#[doc = "T_LENGTH register accessor: an alias for `Reg<T_LENGTH_SPEC>`"]
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
#[doc = "SHA 512/t configuration register 1."]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM register accessor: an alias for `Reg<DMA_BLOCK_NUM_SPEC>`"]
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
#[doc = "DMA configuration register 0."]
pub mod dma_block_num;
#[doc = "START register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Typical SHA configuration register 0."]
pub mod start;
#[doc = "CONTINUE register accessor: an alias for `Reg<CONTINUE_SPEC>`"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "Typical SHA configuration register 1."]
pub mod continue_;
#[doc = "BUSY register accessor: an alias for `Reg<BUSY_SPEC>`"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Busy register."]
pub mod busy;
#[doc = "DMA_START register accessor: an alias for `Reg<DMA_START_SPEC>`"]
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
#[doc = "DMA configuration register 1."]
pub mod dma_start;
#[doc = "DMA_CONTINUE register accessor: an alias for `Reg<DMA_CONTINUE_SPEC>`"]
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
#[doc = "DMA configuration register 2."]
pub mod dma_continue;
#[doc = "CLEAR_IRQ register accessor: an alias for `Reg<CLEAR_IRQ_SPEC>`"]
pub type CLEAR_IRQ = crate::Reg<clear_irq::CLEAR_IRQ_SPEC>;
#[doc = "Interrupt clear register."]
pub mod clear_irq;
#[doc = "IRQ_ENA register accessor: an alias for `Reg<IRQ_ENA_SPEC>`"]
pub type IRQ_ENA = crate::Reg<irq_ena::IRQ_ENA_SPEC>;
#[doc = "Interrupt enable register."]
pub mod irq_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
#[doc = "H_MEM register accessor: an alias for `Reg<H_MEM_SPEC>`"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "Sha H memory which contains intermediate hash or finial hash."]
pub mod h_mem;
#[doc = "M_MEM register accessor: an alias for `Reg<M_MEM_SPEC>`"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Sha M memory which contains message."]
pub mod m_mem;
