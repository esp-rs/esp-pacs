#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster OUT_CRC_CH%s, containing OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?"]
pub struct OUT_CRC_CH {
    out_crc_init_data: OUT_CRC_INIT_DATA,
    tx_crc_width: TX_CRC_WIDTH,
    out_crc_clear: OUT_CRC_CLEAR,
    out_crc_final_result: OUT_CRC_FINAL_RESULT,
    tx_crc_en_wr_data: TX_CRC_EN_WR_DATA,
    tx_crc_en_addr: TX_CRC_EN_ADDR,
    tx_crc_data_en_wr_data: TX_CRC_DATA_EN_WR_DATA,
    tx_crc_data_en_addr: TX_CRC_DATA_EN_ADDR,
    tx_ch_arb_weigh: TX_CH_ARB_WEIGH,
    tx_arb_weigh_opt_dir: TX_ARB_WEIGH_OPT_DIR,
}
impl OUT_CRC_CH {
    #[doc = "0x00 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn out_crc_init_data(&self) -> &OUT_CRC_INIT_DATA {
        &self.out_crc_init_data
    }
    #[doc = "0x04 - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
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
    #[doc = "0x20 - This register is used to config ch0 arbiter weigh"]
    #[inline(always)]
    pub const fn tx_ch_arb_weigh(&self) -> &TX_CH_ARB_WEIGH {
        &self.tx_ch_arb_weigh
    }
    #[doc = "0x24 - This register is used to config off or on weigh optimization"]
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir(&self) -> &TX_ARB_WEIGH_OPT_DIR {
        &self.tx_arb_weigh_opt_dir
    }
}
#[doc = "OUT_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_init_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_init_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_init_data`] module"]
pub type OUT_CRC_INIT_DATA = crate::Reg<out_crc_init_data::OUT_CRC_INIT_DATA_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod out_crc_init_data;
#[doc = "TX_CRC_WIDTH (rw) register accessor: This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_width`] module"]
pub type TX_CRC_WIDTH = crate::Reg<tx_crc_width::TX_CRC_WIDTH_SPEC>;
#[doc = "This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
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
#[doc = "TX_CH_ARB_WEIGH (rw) register accessor: This register is used to config ch0 arbiter weigh\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weigh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weigh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weigh`] module"]
pub type TX_CH_ARB_WEIGH = crate::Reg<tx_ch_arb_weigh::TX_CH_ARB_WEIGH_SPEC>;
#[doc = "This register is used to config ch0 arbiter weigh"]
pub mod tx_ch_arb_weigh;
#[doc = "TX_ARB_WEIGH_OPT_DIR (rw) register accessor: This register is used to config off or on weigh optimization\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weigh_opt_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weigh_opt_dir`] module"]
pub type TX_ARB_WEIGH_OPT_DIR = crate::Reg<tx_arb_weigh_opt_dir::TX_ARB_WEIGH_OPT_DIR_SPEC>;
#[doc = "This register is used to config off or on weigh optimization"]
pub mod tx_arb_weigh_opt_dir;
