#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: MODE,
    t_string: T_STRING,
    t_length: T_LENGTH,
    dma_block_num: DMA_BLOCK_NUM,
    start: START,
    continue_: CONTINUE,
    busy: BUSY,
    dma_start: DMA_START,
    dma_continue: DMA_CONTINUE,
    clear_irq: CLEAR_IRQ,
    irq_ena: IRQ_ENA,
    date: DATE,
    dma_rx_reset: DMA_RX_RESET,
    dma_tx_reset: DMA_TX_RESET,
    free: FREE,
    _3_shake_length: _3_SHAKE_LENGTH,
    _2_sm_3_h_mem: [_2_SM_3_H_MEM; 64],
    _2_sm_3_m_mem: [_2_SM_3_M_MEM; 128],
    _3_h_mem: [_3_H_MEM; 200],
    _reserved19: [u8; 0x38],
    _3_m_mem: [_3_M_MEM; 200],
}
impl RegisterBlock {
    #[doc = "0x00 - Configures SHA algorithm"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x04 - SHA 512/t configuration register 0."]
    #[inline(always)]
    pub const fn t_string(&self) -> &T_STRING {
        &self.t_string
    }
    #[doc = "0x08 - SHA 512/t configuration register 1."]
    #[inline(always)]
    pub const fn t_length(&self) -> &T_LENGTH {
        &self.t_length
    }
    #[doc = "0x0c - Block number register (only effective for DMA-SHA)"]
    #[inline(always)]
    pub const fn dma_block_num(&self) -> &DMA_BLOCK_NUM {
        &self.dma_block_num
    }
    #[doc = "0x10 - Starts the SHA accelerator for Typical SHA operation"]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x14 - Continues SHA operation (only effective in Typical SHA mode)"]
    #[inline(always)]
    pub const fn continue_(&self) -> &CONTINUE {
        &self.continue_
    }
    #[doc = "0x18 - Represents if SHA Accelerator is busy or not"]
    #[inline(always)]
    pub const fn busy(&self) -> &BUSY {
        &self.busy
    }
    #[doc = "0x1c - Starts the SHA accelerator for DMA-SHA operation"]
    #[inline(always)]
    pub const fn dma_start(&self) -> &DMA_START {
        &self.dma_start
    }
    #[doc = "0x20 - Continues SHA operation (only effective in DMA-SHA mode)"]
    #[inline(always)]
    pub const fn dma_continue(&self) -> &DMA_CONTINUE {
        &self.dma_continue
    }
    #[doc = "0x24 - DMA-SHA interrupt clear register"]
    #[inline(always)]
    pub const fn clear_irq(&self) -> &CLEAR_IRQ {
        &self.clear_irq
    }
    #[doc = "0x28 - DMA-SHA interrupt enable register"]
    #[inline(always)]
    pub const fn irq_ena(&self) -> &IRQ_ENA {
        &self.irq_ena
    }
    #[doc = "0x2c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x30 - DMA RX FIFO Reset Signal"]
    #[inline(always)]
    pub const fn dma_rx_reset(&self) -> &DMA_RX_RESET {
        &self.dma_rx_reset
    }
    #[doc = "0x34 - DMA TX FIFO Reset Signal"]
    #[inline(always)]
    pub const fn dma_tx_reset(&self) -> &DMA_TX_RESET {
        &self.dma_tx_reset
    }
    #[doc = "0x38 - SHA free state"]
    #[inline(always)]
    pub const fn free(&self) -> &FREE {
        &self.free
    }
    #[doc = "0x3c - DMA configuration register 3."]
    #[inline(always)]
    pub const fn _3_shake_length(&self) -> &_3_SHAKE_LENGTH {
        &self._3_shake_length
    }
    #[doc = "0x40..0x80 - SHA1, SHA2-256, SM3 H memory which contains intermediate hash or final hash. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x20 (R/W) \\\\ SHA2-512 : 0x00~0x40 (R/W) \\\\"]
    #[inline(always)]
    pub const fn _2_sm_3_h_mem(&self, n: usize) -> &_2_SM_3_H_MEM {
        &self._2_sm_3_h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - SHA1, SHA2-256, SM3 H memory which contains intermediate hash or final hash. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x20 (R/W) \\\\ SHA2-512 : 0x00~0x40 (R/W) \\\\"]
    #[inline(always)]
    pub fn _2_sm_3_h_mem_iter(&self) -> impl Iterator<Item = &_2_SM_3_H_MEM> {
        self._2_sm_3_h_mem.iter()
    }
    #[doc = "0x80..0x100 - SHA1, SHA2-256, SM3 M memory which contains message. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x40 \\\\ SHA2-512 : 0x00~0x80 \\\\"]
    #[inline(always)]
    pub const fn _2_sm_3_m_mem(&self, n: usize) -> &_2_SM_3_M_MEM {
        &self._2_sm_3_m_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - SHA1, SHA2-256, SM3 M memory which contains message. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x40 \\\\ SHA2-512 : 0x00~0x80 \\\\"]
    #[inline(always)]
    pub fn _2_sm_3_m_mem_iter(&self) -> impl Iterator<Item = &_2_SM_3_M_MEM> {
        self._2_sm_3_m_mem.iter()
    }
    #[doc = "0x100..0x1c8 - SHA3, SHAKE H memory which contains intermediate hash or final hash."]
    #[inline(always)]
    pub const fn _3_h_mem(&self, n: usize) -> &_3_H_MEM {
        &self._3_h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x1c8 - SHA3, SHAKE H memory which contains intermediate hash or final hash."]
    #[inline(always)]
    pub fn _3_h_mem_iter(&self) -> impl Iterator<Item = &_3_H_MEM> {
        self._3_h_mem.iter()
    }
    #[doc = "0x200..0x2c8 - SHA3, SHAKE M memory which contains message."]
    #[inline(always)]
    pub const fn _3_m_mem(&self, n: usize) -> &_3_M_MEM {
        &self._3_m_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x2c8 - SHA3, SHAKE M memory which contains message."]
    #[inline(always)]
    pub fn _3_m_mem_iter(&self) -> impl Iterator<Item = &_3_M_MEM> {
        self._3_m_mem.iter()
    }
}
#[doc = "MODE (rw) register accessor: Configures SHA algorithm\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Configures SHA algorithm"]
pub mod mode;
#[doc = "T_STRING (rw) register accessor: SHA 512/t configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_string::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_string::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_string`] module"]
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
#[doc = "SHA 512/t configuration register 0."]
pub mod t_string;
#[doc = "T_LENGTH (rw) register accessor: SHA 512/t configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_length`] module"]
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
#[doc = "SHA 512/t configuration register 1."]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM (rw) register accessor: Block number register (only effective for DMA-SHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_block_num`] module"]
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
#[doc = "Block number register (only effective for DMA-SHA)"]
pub mod dma_block_num;
#[doc = "START (w) register accessor: Starts the SHA accelerator for Typical SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Starts the SHA accelerator for Typical SHA operation"]
pub mod start;
#[doc = "CONTINUE (w) register accessor: Continues SHA operation (only effective in Typical SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in Typical SHA mode)"]
pub mod continue_;
#[doc = "BUSY (r) register accessor: Represents if SHA Accelerator is busy or not\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy`] module"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Represents if SHA Accelerator is busy or not"]
pub mod busy;
#[doc = "DMA_START (w) register accessor: Starts the SHA accelerator for DMA-SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_start`] module"]
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
#[doc = "Starts the SHA accelerator for DMA-SHA operation"]
pub mod dma_start;
#[doc = "DMA_CONTINUE (w) register accessor: Continues SHA operation (only effective in DMA-SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_continue`] module"]
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in DMA-SHA mode)"]
pub mod dma_continue;
#[doc = "CLEAR_IRQ (w) register accessor: DMA-SHA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_irq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_irq`] module"]
pub type CLEAR_IRQ = crate::Reg<clear_irq::CLEAR_IRQ_SPEC>;
#[doc = "DMA-SHA interrupt clear register"]
pub mod clear_irq;
#[doc = "IRQ_ENA (rw) register accessor: DMA-SHA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_ena`] module"]
pub type IRQ_ENA = crate::Reg<irq_ena::IRQ_ENA_SPEC>;
#[doc = "DMA-SHA interrupt enable register"]
pub mod irq_ena;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "DMA_RX_RESET (w) register accessor: DMA RX FIFO Reset Signal\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_reset`] module"]
pub type DMA_RX_RESET = crate::Reg<dma_rx_reset::DMA_RX_RESET_SPEC>;
#[doc = "DMA RX FIFO Reset Signal"]
pub mod dma_rx_reset;
#[doc = "DMA_TX_RESET (w) register accessor: DMA TX FIFO Reset Signal\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tx_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_reset`] module"]
pub type DMA_TX_RESET = crate::Reg<dma_tx_reset::DMA_TX_RESET_SPEC>;
#[doc = "DMA TX FIFO Reset Signal"]
pub mod dma_tx_reset;
#[doc = "FREE (rw) register accessor: SHA free state\n\nYou can [`read`](crate::Reg::read) this register and get [`free::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`free::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@free`] module"]
pub type FREE = crate::Reg<free::FREE_SPEC>;
#[doc = "SHA free state"]
pub mod free;
#[doc = "_3_SHAKE_LENGTH (rw) register accessor: DMA configuration register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_shake_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_shake_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_shake_length`] module"]
pub type _3_SHAKE_LENGTH = crate::Reg<_3_shake_length::_3_SHAKE_LENGTH_SPEC>;
#[doc = "DMA configuration register 3."]
pub mod _3_shake_length;
#[doc = "_2_SM_3_H_MEM (rw) register accessor: SHA1, SHA2-256, SM3 H memory which contains intermediate hash or final hash. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x20 (R/W) \\\\ SHA2-512 : 0x00~0x40 (R/W) \\\\\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_sm_3_h_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_sm_3_h_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_sm_3_h_mem`] module"]
pub type _2_SM_3_H_MEM = crate::Reg<_2_sm_3_h_mem::_2_SM_3_H_MEM_SPEC>;
#[doc = "SHA1, SHA2-256, SM3 H memory which contains intermediate hash or final hash. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x20 (R/W) \\\\ SHA2-512 : 0x00~0x40 (R/W) \\\\"]
pub mod _2_sm_3_h_mem;
#[doc = "_2_SM_3_M_MEM (rw) register accessor: SHA1, SHA2-256, SM3 M memory which contains message. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x40 \\\\ SHA2-512 : 0x00~0x80 \\\\\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_sm_3_m_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_sm_3_m_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_sm_3_m_mem`] module"]
pub type _2_SM_3_M_MEM = crate::Reg<_2_sm_3_m_mem::_2_SM_3_M_MEM_SPEC>;
#[doc = "SHA1, SHA2-256, SM3 M memory which contains message. \\\\ SHA1, SHA2-256, SM3 : 0x00~0x40 \\\\ SHA2-512 : 0x00~0x80 \\\\"]
pub mod _2_sm_3_m_mem;
#[doc = "_3_H_MEM (rw) register accessor: SHA3, SHAKE H memory which contains intermediate hash or final hash.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_h_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_h_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_h_mem`] module"]
pub type _3_H_MEM = crate::Reg<_3_h_mem::_3_H_MEM_SPEC>;
#[doc = "SHA3, SHAKE H memory which contains intermediate hash or final hash."]
pub mod _3_h_mem;
#[doc = "_3_M_MEM (rw) register accessor: SHA3, SHAKE M memory which contains message.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_m_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_m_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_m_mem`] module"]
pub type _3_M_MEM = crate::Reg<_3_m_mem::_3_M_MEM_SPEC>;
#[doc = "SHA3, SHAKE M memory which contains message."]
pub mod _3_m_mem;
