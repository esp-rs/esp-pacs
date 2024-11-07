#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_CRC_CH%s, containing IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?, RX_CH_ARB_WEIGH_CH?, RX_ARB_WEIGH_OPT_DIR_CH?"]
pub struct IN_CRC_CH {
    in_crc_init_data: IN_CRC_INIT_DATA,
    rx_crc_width: RX_CRC_WIDTH,
    in_crc_clear: IN_CRC_CLEAR,
    in_crc_final_result: IN_CRC_FINAL_RESULT,
    rx_crc_en_wr_data: RX_CRC_EN_WR_DATA,
    rx_crc_en_addr: RX_CRC_EN_ADDR,
    rx_crc_data_en_wr_data: RX_CRC_DATA_EN_WR_DATA,
    rx_crc_data_en_addr: RX_CRC_DATA_EN_ADDR,
    rx_ch_arb_weigh: RX_CH_ARB_WEIGH,
    rx_arb_weigh_opt_dir: RX_ARB_WEIGH_OPT_DIR,
}
impl IN_CRC_CH {
    #[doc = "0x00 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn in_crc_init_data(&self) -> &IN_CRC_INIT_DATA {
        &self.in_crc_init_data
    }
    #[doc = "0x04 - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
    #[inline(always)]
    pub const fn rx_crc_width(&self) -> &RX_CRC_WIDTH {
        &self.rx_crc_width
    }
    #[doc = "0x08 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_clear(&self) -> &IN_CRC_CLEAR {
        &self.in_crc_clear
    }
    #[doc = "0x0c - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_final_result(&self) -> &IN_CRC_FINAL_RESULT {
        &self.in_crc_final_result
    }
    #[doc = "0x10 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn rx_crc_en_wr_data(&self) -> &RX_CRC_EN_WR_DATA {
        &self.rx_crc_en_wr_data
    }
    #[doc = "0x14 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn rx_crc_en_addr(&self) -> &RX_CRC_EN_ADDR {
        &self.rx_crc_en_addr
    }
    #[doc = "0x18 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data(&self) -> &RX_CRC_DATA_EN_WR_DATA {
        &self.rx_crc_data_en_wr_data
    }
    #[doc = "0x1c - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_addr(&self) -> &RX_CRC_DATA_EN_ADDR {
        &self.rx_crc_data_en_addr
    }
    #[doc = "0x20 - This register is used to config ch0 arbiter weigh"]
    #[inline(always)]
    pub const fn rx_ch_arb_weigh(&self) -> &RX_CH_ARB_WEIGH {
        &self.rx_ch_arb_weigh
    }
    #[doc = "0x24 - This register is used to config off or on weigh optimization"]
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir(&self) -> &RX_ARB_WEIGH_OPT_DIR {
        &self.rx_arb_weigh_opt_dir
    }
}
#[doc = "IN_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_init_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_init_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_init_data`] module"]
pub type IN_CRC_INIT_DATA = crate::Reg<in_crc_init_data::IN_CRC_INIT_DATA_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod in_crc_init_data;
#[doc = "RX_CRC_WIDTH (rw) register accessor: This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_width`] module"]
pub type RX_CRC_WIDTH = crate::Reg<rx_crc_width::RX_CRC_WIDTH_SPEC>;
#[doc = "This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
pub mod rx_crc_width;
#[doc = "IN_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_clear`] module"]
pub type IN_CRC_CLEAR = crate::Reg<in_crc_clear::IN_CRC_CLEAR_SPEC>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod in_crc_clear;
#[doc = "IN_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_final_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_final_result`] module"]
pub type IN_CRC_FINAL_RESULT = crate::Reg<in_crc_final_result::IN_CRC_FINAL_RESULT_SPEC>;
#[doc = "This register is used to store ch0 crc result"]
pub mod in_crc_final_result;
#[doc = "RX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_wr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_wr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_wr_data`] module"]
pub type RX_CRC_EN_WR_DATA = crate::Reg<rx_crc_en_wr_data::RX_CRC_EN_WR_DATA_SPEC>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod rx_crc_en_wr_data;
#[doc = "RX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_addr`] module"]
pub type RX_CRC_EN_ADDR = crate::Reg<rx_crc_en_addr::RX_CRC_EN_ADDR_SPEC>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod rx_crc_en_addr;
#[doc = "RX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_wr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_wr_data`] module"]
pub type RX_CRC_DATA_EN_WR_DATA = crate::Reg<rx_crc_data_en_wr_data::RX_CRC_DATA_EN_WR_DATA_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod rx_crc_data_en_wr_data;
#[doc = "RX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_addr`] module"]
pub type RX_CRC_DATA_EN_ADDR = crate::Reg<rx_crc_data_en_addr::RX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod rx_crc_data_en_addr;
#[doc = "RX_CH_ARB_WEIGH (rw) register accessor: This register is used to config ch0 arbiter weigh\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weigh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weigh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weigh`] module"]
pub type RX_CH_ARB_WEIGH = crate::Reg<rx_ch_arb_weigh::RX_CH_ARB_WEIGH_SPEC>;
#[doc = "This register is used to config ch0 arbiter weigh"]
pub mod rx_ch_arb_weigh;
#[doc = "RX_ARB_WEIGH_OPT_DIR (rw) register accessor: This register is used to config off or on weigh optimization\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weigh_opt_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weigh_opt_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weigh_opt_dir`] module"]
pub type RX_ARB_WEIGH_OPT_DIR = crate::Reg<rx_arb_weigh_opt_dir::RX_ARB_WEIGH_OPT_DIR_SPEC>;
#[doc = "This register is used to config off or on weigh optimization"]
pub mod rx_arb_weigh_opt_dir;
