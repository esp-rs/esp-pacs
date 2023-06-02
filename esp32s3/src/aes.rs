#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - AES key register %s"]
    pub key_: [KEY_; 8],
    #[doc = "0x20..0x30 - Source data register %s"]
    pub text_in_: [TEXT_IN_; 4],
    #[doc = "0x30..0x40 - Result data register %s"]
    pub text_out_: [TEXT_OUT_; 4],
    #[doc = "0x40 - AES Mode register"]
    pub mode: MODE,
    _reserved4: [u8; 0x04],
    #[doc = "0x48 - AES trigger register"]
    pub trigger: TRIGGER,
    #[doc = "0x4c - AES state register"]
    pub state: STATE,
    #[doc = "0x50..0x60 - The memory that stores initialization vector"]
    pub iv_mem: [IV_MEM; 16],
    #[doc = "0x60..0x70 - The memory that stores GCM hash subkey"]
    pub h_mem: [H_MEM; 16],
    #[doc = "0x70..0x80 - The memory that stores J0"]
    pub j0_mem: [J0_MEM; 16],
    #[doc = "0x80..0x90 - The memory that stores T0"]
    pub t0_mem: [T0_MEM; 16],
    #[doc = "0x90 - AES accelerator working mode register"]
    pub dma_enable: DMA_ENABLE,
    #[doc = "0x94 - AES cipher block mode register"]
    pub block_mode: BLOCK_MODE,
    #[doc = "0x98 - AES block number register"]
    pub block_num: BLOCK_NUM,
    #[doc = "0x9c - Standard incrementing function configure register"]
    pub inc_sel: INC_SEL,
    #[doc = "0xa0 - Additional Authential Data block number register"]
    pub aad_block_num: AAD_BLOCK_NUM,
    #[doc = "0xa4 - AES remainder bit number register"]
    pub remainder_bit_num: REMAINDER_BIT_NUM,
    #[doc = "0xa8 - AES continue register"]
    pub continue_: CONTINUE,
    #[doc = "0xac - AES Interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0xb0 - DMA-AES Interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0xb4 - AES version control register"]
    pub date: DATE,
    #[doc = "0xb8 - AES-DMA exit config"]
    pub dma_exit: DMA_EXIT,
}
#[doc = "KEY_ (rw) register accessor: an alias for `Reg<KEY__SPEC>`"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = "AES key register %s"]
pub mod key_;
#[doc = "TEXT_IN_ (rw) register accessor: an alias for `Reg<TEXT_IN__SPEC>`"]
pub type TEXT_IN_ = crate::Reg<text_in_::TEXT_IN__SPEC>;
#[doc = "Source data register %s"]
pub mod text_in_;
#[doc = "TEXT_OUT_ (rw) register accessor: an alias for `Reg<TEXT_OUT__SPEC>`"]
pub type TEXT_OUT_ = crate::Reg<text_out_::TEXT_OUT__SPEC>;
#[doc = "Result data register %s"]
pub mod text_out_;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES Mode register"]
pub mod mode;
#[doc = "TRIGGER (w) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "AES trigger register"]
pub mod trigger;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "AES state register"]
pub mod state;
#[doc = "IV_MEM (rw) register accessor: an alias for `Reg<IV_MEM_SPEC>`"]
pub type IV_MEM = crate::Reg<iv_mem::IV_MEM_SPEC>;
#[doc = "The memory that stores initialization vector"]
pub mod iv_mem;
#[doc = "H_MEM (rw) register accessor: an alias for `Reg<H_MEM_SPEC>`"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "The memory that stores GCM hash subkey"]
pub mod h_mem;
#[doc = "J0_MEM (rw) register accessor: an alias for `Reg<J0_MEM_SPEC>`"]
pub type J0_MEM = crate::Reg<j0_mem::J0_MEM_SPEC>;
#[doc = "The memory that stores J0"]
pub mod j0_mem;
#[doc = "T0_MEM (rw) register accessor: an alias for `Reg<T0_MEM_SPEC>`"]
pub type T0_MEM = crate::Reg<t0_mem::T0_MEM_SPEC>;
#[doc = "The memory that stores T0"]
pub mod t0_mem;
#[doc = "DMA_ENABLE (rw) register accessor: an alias for `Reg<DMA_ENABLE_SPEC>`"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "AES accelerator working mode register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE (rw) register accessor: an alias for `Reg<BLOCK_MODE_SPEC>`"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "AES cipher block mode register"]
pub mod block_mode;
#[doc = "BLOCK_NUM (rw) register accessor: an alias for `Reg<BLOCK_NUM_SPEC>`"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "AES block number register"]
pub mod block_num;
#[doc = "INC_SEL (rw) register accessor: an alias for `Reg<INC_SEL_SPEC>`"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function configure register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM (rw) register accessor: an alias for `Reg<AAD_BLOCK_NUM_SPEC>`"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "Additional Authential Data block number register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM (rw) register accessor: an alias for `Reg<REMAINDER_BIT_NUM_SPEC>`"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "AES remainder bit number register"]
pub mod remainder_bit_num;
#[doc = "CONTINUE (w) register accessor: an alias for `Reg<CONTINUE_SPEC>`"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "AES continue register"]
pub mod continue_;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "AES Interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES Interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "AES version control register"]
pub mod date;
#[doc = "DMA_EXIT (w) register accessor: an alias for `Reg<DMA_EXIT_SPEC>`"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "AES-DMA exit config"]
pub mod dma_exit;
