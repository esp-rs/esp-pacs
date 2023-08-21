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
    #[doc = "0x40 - AES working mode configuration register"]
    pub mode: MODE,
    #[doc = "0x44 - Endian configuration register"]
    pub endian: ENDIAN,
    #[doc = "0x48 - Operation start controlling register"]
    pub trigger: TRIGGER,
    #[doc = "0x4c - Operation status register"]
    pub state: STATE,
    #[doc = "0x50..0x60 - initialization vector"]
    pub iv_: [IV_; 4],
    #[doc = "0x60..0x70 - GCM hash subkey"]
    pub h_: [H_; 4],
    #[doc = "0x70..0x80 - J0"]
    pub j0_: [J0_; 4],
    #[doc = "0x80..0x90 - T0"]
    pub t0_: [T0_; 4],
    #[doc = "0x90 - DMA enable register"]
    pub dma_enable: DMA_ENABLE,
    #[doc = "0x94 - Block operation type register"]
    pub block_mode: BLOCK_MODE,
    #[doc = "0x98 - Block number configuration register"]
    pub block_num: BLOCK_NUM,
    #[doc = "0x9c - Standard incrementing function register"]
    pub inc_sel: INC_SEL,
    #[doc = "0xa0 - AAD block number configuration register"]
    pub aad_block_num: AAD_BLOCK_NUM,
    #[doc = "0xa4 - Remainder bit number of plaintext/ciphertext"]
    pub remainder_bit_num: REMAINDER_BIT_NUM,
    #[doc = "0xa8 - Operation continue controlling register"]
    pub continue_op: CONTINUE_OP,
    #[doc = "0xac - DMA-AES interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0xb0 - DMA-AES interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0xb4 - Version control register"]
    pub date: DATE,
    #[doc = "0xb8 - Operation exit controlling register"]
    pub dma_exit: DMA_EXIT,
}
#[doc = "KEY_ (rw) register accessor: AES key register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key_`] module"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = "AES key register %s"]
pub mod key_;
#[doc = "TEXT_IN_ (rw) register accessor: Source data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`text_in_`] module"]
pub type TEXT_IN_ = crate::Reg<text_in_::TEXT_IN__SPEC>;
#[doc = "Source data register %s"]
pub mod text_in_;
#[doc = "TEXT_OUT_ (rw) register accessor: Result data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`text_out_`] module"]
pub type TEXT_OUT_ = crate::Reg<text_out_::TEXT_OUT__SPEC>;
#[doc = "Result data register %s"]
pub mod text_out_;
#[doc = "MODE (rw) register accessor: AES working mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES working mode configuration register"]
pub mod mode;
#[doc = "ENDIAN (rw) register accessor: Endian configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "Endian configuration register"]
pub mod endian;
#[doc = "TRIGGER (w) register accessor: Operation start controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Operation start controlling register"]
pub mod trigger;
#[doc = "STATE (r) register accessor: Operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Operation status register"]
pub mod state;
#[doc = "IV_ (rw) register accessor: initialization vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iv_`] module"]
pub type IV_ = crate::Reg<iv_::IV__SPEC>;
#[doc = "initialization vector"]
pub mod iv_;
#[doc = "H_ (r) register accessor: GCM hash subkey\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`h_`] module"]
pub type H_ = crate::Reg<h_::H__SPEC>;
#[doc = "GCM hash subkey"]
pub mod h_;
#[doc = "J0_ (rw) register accessor: J0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`j0_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`j0_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`j0_`] module"]
pub type J0_ = crate::Reg<j0_::J0__SPEC>;
#[doc = "J0"]
pub mod j0_;
#[doc = "T0_ (r) register accessor: T0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`t0_`] module"]
pub type T0_ = crate::Reg<t0_::T0__SPEC>;
#[doc = "T0"]
pub mod t0_;
#[doc = "DMA_ENABLE (rw) register accessor: DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_enable`] module"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "DMA enable register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE (rw) register accessor: Block operation type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`block_mode`] module"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "Block operation type register"]
pub mod block_mode;
#[doc = "BLOCK_NUM (rw) register accessor: Block number configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`block_num`] module"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "Block number configuration register"]
pub mod block_num;
#[doc = "INC_SEL (rw) register accessor: Standard incrementing function register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inc_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inc_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inc_sel`] module"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM (rw) register accessor: AAD block number configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aad_block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aad_block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aad_block_num`] module"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "AAD block number configuration register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM (rw) register accessor: Remainder bit number of plaintext/ciphertext\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remainder_bit_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remainder_bit_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`remainder_bit_num`] module"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "Remainder bit number of plaintext/ciphertext"]
pub mod remainder_bit_num;
#[doc = "CONTINUE_OP (w) register accessor: Operation continue controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_op::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`continue_op`] module"]
pub type CONTINUE_OP = crate::Reg<continue_op::CONTINUE_OP_SPEC>;
#[doc = "Operation continue controlling register"]
pub mod continue_op;
#[doc = "INT_CLR (w) register accessor: DMA-AES interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "DMA-AES interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: DMA-AES interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "DMA_EXIT (w) register accessor: Operation exit controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_exit::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_exit`] module"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "Operation exit controlling register"]
pub mod dma_exit;
