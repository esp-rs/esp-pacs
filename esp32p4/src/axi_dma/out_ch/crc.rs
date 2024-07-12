#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR"]
pub struct CRC {
    out_crc_init_data: OUT_CRC_INIT_DATA,
    tx_crc_width: TX_CRC_WIDTH,
    out_crc_clear: OUT_CRC_CLEAR,
    out_crc_final_result: OUT_CRC_FINAL_RESULT,
    tx_crc_en_wr_data: TX_CRC_EN_WR_DATA,
    tx_crc_en_addr: TX_CRC_EN_ADDR,
    tx_crc_data_en_wr_data: TX_CRC_DATA_EN_WR_DATA,
    tx_crc_data_en_addr: TX_CRC_DATA_EN_ADDR,
}
impl CRC {
    #[doc = "0x00 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn out_crc_init_data(&self) -> &OUT_CRC_INIT_DATA {
        &self.out_crc_init_data
    }
    #[doc = "0x04 - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn tx_crc_width(&self) -> &TX_CRC_WIDTH {
        &self.tx_crc_width
    }
    #[doc = "0x08 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_clear(&self) -> &OUT_CRC_CLEAR {
        &self.out_crc_clear
    }
    #[doc = "0x0c - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_final_result(&self) -> &OUT_CRC_FINAL_RESULT {
        &self.out_crc_final_result
    }
    #[doc = "0x10 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn tx_crc_en_wr_data(&self) -> &TX_CRC_EN_WR_DATA {
        &self.tx_crc_en_wr_data
    }
    #[doc = "0x14 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn tx_crc_en_addr(&self) -> &TX_CRC_EN_ADDR {
        &self.tx_crc_en_addr
    }
    #[doc = "0x18 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data(&self) -> &TX_CRC_DATA_EN_WR_DATA {
        &self.tx_crc_data_en_wr_data
    }
    #[doc = "0x1c - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_addr(&self) -> &TX_CRC_DATA_EN_ADDR {
        &self.tx_crc_data_en_addr
    }
}
#[doc = "OUT_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_init_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_init_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_init_data`] module"]
pub type OUT_CRC_INIT_DATA = crate::Reg<out_crc_init_data::OUT_CRC_INIT_DATA_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod out_crc_init_data;
#[doc = "TX_CRC_WIDTH (rw) register accessor: This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_width`] module"]
pub type TX_CRC_WIDTH = crate::Reg<tx_crc_width::TX_CRC_WIDTH_SPEC>;
#[doc = "This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
pub mod tx_crc_width;
#[doc = "OUT_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_clear`] module"]
pub type OUT_CRC_CLEAR = crate::Reg<out_crc_clear::OUT_CRC_CLEAR_SPEC>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod out_crc_clear;
#[doc = "OUT_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_final_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_final_result`] module"]
pub type OUT_CRC_FINAL_RESULT = crate::Reg<out_crc_final_result::OUT_CRC_FINAL_RESULT_SPEC>;
#[doc = "This register is used to store ch0 crc result"]
pub mod out_crc_final_result;
#[doc = "TX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_wr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_wr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_wr_data`] module"]
pub type TX_CRC_EN_WR_DATA = crate::Reg<tx_crc_en_wr_data::TX_CRC_EN_WR_DATA_SPEC>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod tx_crc_en_wr_data;
#[doc = "TX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_addr`] module"]
pub type TX_CRC_EN_ADDR = crate::Reg<tx_crc_en_addr::TX_CRC_EN_ADDR_SPEC>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod tx_crc_en_addr;
#[doc = "TX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_wr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_wr_data`] module"]
pub type TX_CRC_DATA_EN_WR_DATA = crate::Reg<tx_crc_data_en_wr_data::TX_CRC_DATA_EN_WR_DATA_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod tx_crc_data_en_wr_data;
#[doc = "TX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_addr`] module"]
pub type TX_CRC_DATA_EN_ADDR = crate::Reg<tx_crc_data_en_addr::TX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod tx_crc_data_en_addr;
