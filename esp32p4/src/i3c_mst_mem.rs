#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    command_buf_port: COMMAND_BUF_PORT,
    response_buf_port: RESPONSE_BUF_PORT,
    rx_data_port: RX_DATA_PORT,
    tx_data_port: TX_DATA_PORT,
    ibi_status_buf: IBI_STATUS_BUF,
    _reserved5: [u8; 0x24],
    ibi_data_buf: IBI_DATA_BUF,
    _reserved6: [u8; 0x7c],
    dev_addr_table1_loc: DEV_ADDR_TABLE1_LOC,
    dev_addr_table2_loc: DEV_ADDR_TABLE2_LOC,
    dev_addr_table3_loc: DEV_ADDR_TABLE3_LOC,
    dev_addr_table4_loc: DEV_ADDR_TABLE4_LOC,
    dev_addr_table5_loc: DEV_ADDR_TABLE5_LOC,
    dev_addr_table6_loc: DEV_ADDR_TABLE6_LOC,
    dev_addr_table7_loc: DEV_ADDR_TABLE7_LOC,
    dev_addr_table8_loc: DEV_ADDR_TABLE8_LOC,
    dev_addr_table9_loc: DEV_ADDR_TABLE9_LOC,
    dev_addr_table10_loc: DEV_ADDR_TABLE10_LOC,
    dev_addr_table11_loc: DEV_ADDR_TABLE11_LOC,
    dev_addr_table12_loc: DEV_ADDR_TABLE12_LOC,
    _reserved18: [u8; 0x10],
    dev_char_table1_loc1: DEV_CHAR_TABLE1_LOC1,
    dev_char_table1_loc2: DEV_CHAR_TABLE1_LOC2,
    dev_char_table1_loc3: DEV_CHAR_TABLE1_LOC3,
    dev_char_table1_loc4: DEV_CHAR_TABLE1_LOC4,
    dev_char_table2_loc1: DEV_CHAR_TABLE2_LOC1,
    dev_char_table2_loc2: DEV_CHAR_TABLE2_LOC2,
    dev_char_table2_loc3: DEV_CHAR_TABLE2_LOC3,
    dev_char_table2_loc4: DEV_CHAR_TABLE2_LOC4,
    dev_char_table3_loc1: DEV_CHAR_TABLE3_LOC1,
    dev_char_table3_loc2: DEV_CHAR_TABLE3_LOC2,
    dev_char_table3_loc3: DEV_CHAR_TABLE3_LOC3,
    dev_char_table3_loc4: DEV_CHAR_TABLE3_LOC4,
    dev_char_table4_loc1: DEV_CHAR_TABLE4_LOC1,
    dev_char_table4_loc2: DEV_CHAR_TABLE4_LOC2,
    dev_char_table4_loc3: DEV_CHAR_TABLE4_LOC3,
    dev_char_table4_loc4: DEV_CHAR_TABLE4_LOC4,
    dev_char_table5_loc1: DEV_CHAR_TABLE5_LOC1,
    dev_char_table5_loc2: DEV_CHAR_TABLE5_LOC2,
    dev_char_table5_loc3: DEV_CHAR_TABLE5_LOC3,
    dev_char_table5_loc4: DEV_CHAR_TABLE5_LOC4,
    dev_char_table6_loc1: DEV_CHAR_TABLE6_LOC1,
    dev_char_table6_loc2: DEV_CHAR_TABLE6_LOC2,
    dev_char_table6_loc3: DEV_CHAR_TABLE6_LOC3,
    dev_char_table6_loc4: DEV_CHAR_TABLE6_LOC4,
    dev_char_table7_loc1: DEV_CHAR_TABLE7_LOC1,
    dev_char_table7_loc2: DEV_CHAR_TABLE7_LOC2,
    dev_char_table7_loc3: DEV_CHAR_TABLE7_LOC3,
    dev_char_table7_loc4: DEV_CHAR_TABLE7_LOC4,
    dev_char_table8_loc1: DEV_CHAR_TABLE8_LOC1,
    dev_char_table8_loc2: DEV_CHAR_TABLE8_LOC2,
    dev_char_table8_loc3: DEV_CHAR_TABLE8_LOC3,
    dev_char_table8_loc4: DEV_CHAR_TABLE8_LOC4,
    dev_char_table9_loc1: DEV_CHAR_TABLE9_LOC1,
    dev_char_table9_loc2: DEV_CHAR_TABLE9_LOC2,
    dev_char_table9_loc3: DEV_CHAR_TABLE9_LOC3,
    dev_char_table9_loc4: DEV_CHAR_TABLE9_LOC4,
    dev_char_table10_loc1: DEV_CHAR_TABLE10_LOC1,
    dev_char_table10_loc2: DEV_CHAR_TABLE10_LOC2,
    dev_char_table10_loc3: DEV_CHAR_TABLE10_LOC3,
    dev_char_table10_loc4: DEV_CHAR_TABLE10_LOC4,
    dev_char_table11_loc1: DEV_CHAR_TABLE11_LOC1,
    dev_char_table11_loc2: DEV_CHAR_TABLE11_LOC2,
    dev_char_table11_loc3: DEV_CHAR_TABLE11_LOC3,
    dev_char_table11_loc4: DEV_CHAR_TABLE11_LOC4,
    dev_char_table12_loc1: DEV_CHAR_TABLE12_LOC1,
    dev_char_table12_loc2: DEV_CHAR_TABLE12_LOC2,
    dev_char_table12_loc3: DEV_CHAR_TABLE12_LOC3,
    dev_char_table12_loc4: DEV_CHAR_TABLE12_LOC4,
}
impl RegisterBlock {
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn command_buf_port(&self) -> &COMMAND_BUF_PORT {
        &self.command_buf_port
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn response_buf_port(&self) -> &RESPONSE_BUF_PORT {
        &self.response_buf_port
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn rx_data_port(&self) -> &RX_DATA_PORT {
        &self.rx_data_port
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn tx_data_port(&self) -> &TX_DATA_PORT {
        &self.tx_data_port
    }
    #[doc = "0x18 - In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)"]
    #[inline(always)]
    pub const fn ibi_status_buf(&self) -> &IBI_STATUS_BUF {
        &self.ibi_status_buf
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn ibi_data_buf(&self) -> &IBI_DATA_BUF {
        &self.ibi_data_buf
    }
    #[doc = "0xc0 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table1_loc(&self) -> &DEV_ADDR_TABLE1_LOC {
        &self.dev_addr_table1_loc
    }
    #[doc = "0xc4 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table2_loc(&self) -> &DEV_ADDR_TABLE2_LOC {
        &self.dev_addr_table2_loc
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table3_loc(&self) -> &DEV_ADDR_TABLE3_LOC {
        &self.dev_addr_table3_loc
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn dev_addr_table4_loc(&self) -> &DEV_ADDR_TABLE4_LOC {
        &self.dev_addr_table4_loc
    }
    #[doc = "0xd0 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table5_loc(&self) -> &DEV_ADDR_TABLE5_LOC {
        &self.dev_addr_table5_loc
    }
    #[doc = "0xd4 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table6_loc(&self) -> &DEV_ADDR_TABLE6_LOC {
        &self.dev_addr_table6_loc
    }
    #[doc = "0xd8 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table7_loc(&self) -> &DEV_ADDR_TABLE7_LOC {
        &self.dev_addr_table7_loc
    }
    #[doc = "0xdc - NA"]
    #[inline(always)]
    pub const fn dev_addr_table8_loc(&self) -> &DEV_ADDR_TABLE8_LOC {
        &self.dev_addr_table8_loc
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table9_loc(&self) -> &DEV_ADDR_TABLE9_LOC {
        &self.dev_addr_table9_loc
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table10_loc(&self) -> &DEV_ADDR_TABLE10_LOC {
        &self.dev_addr_table10_loc
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table11_loc(&self) -> &DEV_ADDR_TABLE11_LOC {
        &self.dev_addr_table11_loc
    }
    #[doc = "0xec - NA"]
    #[inline(always)]
    pub const fn dev_addr_table12_loc(&self) -> &DEV_ADDR_TABLE12_LOC {
        &self.dev_addr_table12_loc
    }
    #[doc = "0x100 - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc1(&self) -> &DEV_CHAR_TABLE1_LOC1 {
        &self.dev_char_table1_loc1
    }
    #[doc = "0x104 - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc2(&self) -> &DEV_CHAR_TABLE1_LOC2 {
        &self.dev_char_table1_loc2
    }
    #[doc = "0x108 - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc3(&self) -> &DEV_CHAR_TABLE1_LOC3 {
        &self.dev_char_table1_loc3
    }
    #[doc = "0x10c - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc4(&self) -> &DEV_CHAR_TABLE1_LOC4 {
        &self.dev_char_table1_loc4
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc1(&self) -> &DEV_CHAR_TABLE2_LOC1 {
        &self.dev_char_table2_loc1
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc2(&self) -> &DEV_CHAR_TABLE2_LOC2 {
        &self.dev_char_table2_loc2
    }
    #[doc = "0x118 - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc3(&self) -> &DEV_CHAR_TABLE2_LOC3 {
        &self.dev_char_table2_loc3
    }
    #[doc = "0x11c - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc4(&self) -> &DEV_CHAR_TABLE2_LOC4 {
        &self.dev_char_table2_loc4
    }
    #[doc = "0x120 - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc1(&self) -> &DEV_CHAR_TABLE3_LOC1 {
        &self.dev_char_table3_loc1
    }
    #[doc = "0x124 - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc2(&self) -> &DEV_CHAR_TABLE3_LOC2 {
        &self.dev_char_table3_loc2
    }
    #[doc = "0x128 - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc3(&self) -> &DEV_CHAR_TABLE3_LOC3 {
        &self.dev_char_table3_loc3
    }
    #[doc = "0x12c - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc4(&self) -> &DEV_CHAR_TABLE3_LOC4 {
        &self.dev_char_table3_loc4
    }
    #[doc = "0x130 - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc1(&self) -> &DEV_CHAR_TABLE4_LOC1 {
        &self.dev_char_table4_loc1
    }
    #[doc = "0x134 - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc2(&self) -> &DEV_CHAR_TABLE4_LOC2 {
        &self.dev_char_table4_loc2
    }
    #[doc = "0x138 - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc3(&self) -> &DEV_CHAR_TABLE4_LOC3 {
        &self.dev_char_table4_loc3
    }
    #[doc = "0x13c - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc4(&self) -> &DEV_CHAR_TABLE4_LOC4 {
        &self.dev_char_table4_loc4
    }
    #[doc = "0x140 - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc1(&self) -> &DEV_CHAR_TABLE5_LOC1 {
        &self.dev_char_table5_loc1
    }
    #[doc = "0x144 - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc2(&self) -> &DEV_CHAR_TABLE5_LOC2 {
        &self.dev_char_table5_loc2
    }
    #[doc = "0x148 - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc3(&self) -> &DEV_CHAR_TABLE5_LOC3 {
        &self.dev_char_table5_loc3
    }
    #[doc = "0x14c - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc4(&self) -> &DEV_CHAR_TABLE5_LOC4 {
        &self.dev_char_table5_loc4
    }
    #[doc = "0x150 - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc1(&self) -> &DEV_CHAR_TABLE6_LOC1 {
        &self.dev_char_table6_loc1
    }
    #[doc = "0x154 - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc2(&self) -> &DEV_CHAR_TABLE6_LOC2 {
        &self.dev_char_table6_loc2
    }
    #[doc = "0x158 - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc3(&self) -> &DEV_CHAR_TABLE6_LOC3 {
        &self.dev_char_table6_loc3
    }
    #[doc = "0x15c - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc4(&self) -> &DEV_CHAR_TABLE6_LOC4 {
        &self.dev_char_table6_loc4
    }
    #[doc = "0x160 - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc1(&self) -> &DEV_CHAR_TABLE7_LOC1 {
        &self.dev_char_table7_loc1
    }
    #[doc = "0x164 - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc2(&self) -> &DEV_CHAR_TABLE7_LOC2 {
        &self.dev_char_table7_loc2
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc3(&self) -> &DEV_CHAR_TABLE7_LOC3 {
        &self.dev_char_table7_loc3
    }
    #[doc = "0x16c - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc4(&self) -> &DEV_CHAR_TABLE7_LOC4 {
        &self.dev_char_table7_loc4
    }
    #[doc = "0x170 - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc1(&self) -> &DEV_CHAR_TABLE8_LOC1 {
        &self.dev_char_table8_loc1
    }
    #[doc = "0x174 - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc2(&self) -> &DEV_CHAR_TABLE8_LOC2 {
        &self.dev_char_table8_loc2
    }
    #[doc = "0x178 - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc3(&self) -> &DEV_CHAR_TABLE8_LOC3 {
        &self.dev_char_table8_loc3
    }
    #[doc = "0x17c - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc4(&self) -> &DEV_CHAR_TABLE8_LOC4 {
        &self.dev_char_table8_loc4
    }
    #[doc = "0x180 - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc1(&self) -> &DEV_CHAR_TABLE9_LOC1 {
        &self.dev_char_table9_loc1
    }
    #[doc = "0x184 - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc2(&self) -> &DEV_CHAR_TABLE9_LOC2 {
        &self.dev_char_table9_loc2
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc3(&self) -> &DEV_CHAR_TABLE9_LOC3 {
        &self.dev_char_table9_loc3
    }
    #[doc = "0x18c - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc4(&self) -> &DEV_CHAR_TABLE9_LOC4 {
        &self.dev_char_table9_loc4
    }
    #[doc = "0x190 - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc1(&self) -> &DEV_CHAR_TABLE10_LOC1 {
        &self.dev_char_table10_loc1
    }
    #[doc = "0x194 - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc2(&self) -> &DEV_CHAR_TABLE10_LOC2 {
        &self.dev_char_table10_loc2
    }
    #[doc = "0x198 - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc3(&self) -> &DEV_CHAR_TABLE10_LOC3 {
        &self.dev_char_table10_loc3
    }
    #[doc = "0x19c - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc4(&self) -> &DEV_CHAR_TABLE10_LOC4 {
        &self.dev_char_table10_loc4
    }
    #[doc = "0x1a0 - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc1(&self) -> &DEV_CHAR_TABLE11_LOC1 {
        &self.dev_char_table11_loc1
    }
    #[doc = "0x1a4 - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc2(&self) -> &DEV_CHAR_TABLE11_LOC2 {
        &self.dev_char_table11_loc2
    }
    #[doc = "0x1a8 - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc3(&self) -> &DEV_CHAR_TABLE11_LOC3 {
        &self.dev_char_table11_loc3
    }
    #[doc = "0x1ac - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc4(&self) -> &DEV_CHAR_TABLE11_LOC4 {
        &self.dev_char_table11_loc4
    }
    #[doc = "0x1b0 - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc1(&self) -> &DEV_CHAR_TABLE12_LOC1 {
        &self.dev_char_table12_loc1
    }
    #[doc = "0x1b4 - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc2(&self) -> &DEV_CHAR_TABLE12_LOC2 {
        &self.dev_char_table12_loc2
    }
    #[doc = "0x1b8 - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc3(&self) -> &DEV_CHAR_TABLE12_LOC3 {
        &self.dev_char_table12_loc3
    }
    #[doc = "0x1bc - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc4(&self) -> &DEV_CHAR_TABLE12_LOC4 {
        &self.dev_char_table12_loc4
    }
}
#[doc = "COMMAND_BUF_PORT (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command_buf_port::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command_buf_port::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command_buf_port`] module"]
pub type COMMAND_BUF_PORT = crate::Reg<command_buf_port::COMMAND_BUF_PORT_SPEC>;
#[doc = "NA"]
pub mod command_buf_port;
#[doc = "RESPONSE_BUF_PORT (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response_buf_port::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response_buf_port`] module"]
pub type RESPONSE_BUF_PORT = crate::Reg<response_buf_port::RESPONSE_BUF_PORT_SPEC>;
#[doc = "NA"]
pub mod response_buf_port;
#[doc = "RX_DATA_PORT (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_port::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_port`] module"]
pub type RX_DATA_PORT = crate::Reg<rx_data_port::RX_DATA_PORT_SPEC>;
#[doc = "NA"]
pub mod rx_data_port;
#[doc = "TX_DATA_PORT (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data_port::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_port::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_port`] module"]
pub type TX_DATA_PORT = crate::Reg<tx_data_port::TX_DATA_PORT_SPEC>;
#[doc = "NA"]
pub mod tx_data_port;
#[doc = "IBI_STATUS_BUF (r) register accessor: In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_status_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_status_buf`] module"]
pub type IBI_STATUS_BUF = crate::Reg<ibi_status_buf::IBI_STATUS_BUF_SPEC>;
#[doc = "In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)"]
pub mod ibi_status_buf;
#[doc = "IBI_DATA_BUF (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_data_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_data_buf`] module"]
pub type IBI_DATA_BUF = crate::Reg<ibi_data_buf::IBI_DATA_BUF_SPEC>;
#[doc = "NA"]
pub mod ibi_data_buf;
#[doc = "DEV_ADDR_TABLE1_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table1_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table1_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table1_loc`] module"]
pub type DEV_ADDR_TABLE1_LOC = crate::Reg<dev_addr_table1_loc::DEV_ADDR_TABLE1_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table1_loc;
#[doc = "DEV_ADDR_TABLE2_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table2_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table2_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table2_loc`] module"]
pub type DEV_ADDR_TABLE2_LOC = crate::Reg<dev_addr_table2_loc::DEV_ADDR_TABLE2_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table2_loc;
#[doc = "DEV_ADDR_TABLE3_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table3_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table3_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table3_loc`] module"]
pub type DEV_ADDR_TABLE3_LOC = crate::Reg<dev_addr_table3_loc::DEV_ADDR_TABLE3_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table3_loc;
#[doc = "DEV_ADDR_TABLE4_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table4_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table4_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table4_loc`] module"]
pub type DEV_ADDR_TABLE4_LOC = crate::Reg<dev_addr_table4_loc::DEV_ADDR_TABLE4_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table4_loc;
#[doc = "DEV_ADDR_TABLE5_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table5_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table5_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table5_loc`] module"]
pub type DEV_ADDR_TABLE5_LOC = crate::Reg<dev_addr_table5_loc::DEV_ADDR_TABLE5_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table5_loc;
#[doc = "DEV_ADDR_TABLE6_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table6_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table6_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table6_loc`] module"]
pub type DEV_ADDR_TABLE6_LOC = crate::Reg<dev_addr_table6_loc::DEV_ADDR_TABLE6_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table6_loc;
#[doc = "DEV_ADDR_TABLE7_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table7_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table7_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table7_loc`] module"]
pub type DEV_ADDR_TABLE7_LOC = crate::Reg<dev_addr_table7_loc::DEV_ADDR_TABLE7_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table7_loc;
#[doc = "DEV_ADDR_TABLE8_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table8_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table8_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table8_loc`] module"]
pub type DEV_ADDR_TABLE8_LOC = crate::Reg<dev_addr_table8_loc::DEV_ADDR_TABLE8_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table8_loc;
#[doc = "DEV_ADDR_TABLE9_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table9_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table9_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table9_loc`] module"]
pub type DEV_ADDR_TABLE9_LOC = crate::Reg<dev_addr_table9_loc::DEV_ADDR_TABLE9_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table9_loc;
#[doc = "DEV_ADDR_TABLE10_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table10_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table10_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table10_loc`] module"]
pub type DEV_ADDR_TABLE10_LOC = crate::Reg<dev_addr_table10_loc::DEV_ADDR_TABLE10_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table10_loc;
#[doc = "DEV_ADDR_TABLE11_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table11_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table11_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table11_loc`] module"]
pub type DEV_ADDR_TABLE11_LOC = crate::Reg<dev_addr_table11_loc::DEV_ADDR_TABLE11_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table11_loc;
#[doc = "DEV_ADDR_TABLE12_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table12_loc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table12_loc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table12_loc`] module"]
pub type DEV_ADDR_TABLE12_LOC = crate::Reg<dev_addr_table12_loc::DEV_ADDR_TABLE12_LOC_SPEC>;
#[doc = "NA"]
pub mod dev_addr_table12_loc;
#[doc = "DEV_CHAR_TABLE1_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table1_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc1`] module"]
pub type DEV_CHAR_TABLE1_LOC1 = crate::Reg<dev_char_table1_loc1::DEV_CHAR_TABLE1_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table1_loc1;
#[doc = "DEV_CHAR_TABLE1_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table1_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc2`] module"]
pub type DEV_CHAR_TABLE1_LOC2 = crate::Reg<dev_char_table1_loc2::DEV_CHAR_TABLE1_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table1_loc2;
#[doc = "DEV_CHAR_TABLE1_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table1_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc3`] module"]
pub type DEV_CHAR_TABLE1_LOC3 = crate::Reg<dev_char_table1_loc3::DEV_CHAR_TABLE1_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table1_loc3;
#[doc = "DEV_CHAR_TABLE1_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table1_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc4`] module"]
pub type DEV_CHAR_TABLE1_LOC4 = crate::Reg<dev_char_table1_loc4::DEV_CHAR_TABLE1_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table1_loc4;
#[doc = "DEV_CHAR_TABLE2_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table2_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc1`] module"]
pub type DEV_CHAR_TABLE2_LOC1 = crate::Reg<dev_char_table2_loc1::DEV_CHAR_TABLE2_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table2_loc1;
#[doc = "DEV_CHAR_TABLE2_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table2_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc2`] module"]
pub type DEV_CHAR_TABLE2_LOC2 = crate::Reg<dev_char_table2_loc2::DEV_CHAR_TABLE2_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table2_loc2;
#[doc = "DEV_CHAR_TABLE2_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table2_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc3`] module"]
pub type DEV_CHAR_TABLE2_LOC3 = crate::Reg<dev_char_table2_loc3::DEV_CHAR_TABLE2_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table2_loc3;
#[doc = "DEV_CHAR_TABLE2_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table2_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc4`] module"]
pub type DEV_CHAR_TABLE2_LOC4 = crate::Reg<dev_char_table2_loc4::DEV_CHAR_TABLE2_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table2_loc4;
#[doc = "DEV_CHAR_TABLE3_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table3_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc1`] module"]
pub type DEV_CHAR_TABLE3_LOC1 = crate::Reg<dev_char_table3_loc1::DEV_CHAR_TABLE3_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table3_loc1;
#[doc = "DEV_CHAR_TABLE3_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table3_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc2`] module"]
pub type DEV_CHAR_TABLE3_LOC2 = crate::Reg<dev_char_table3_loc2::DEV_CHAR_TABLE3_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table3_loc2;
#[doc = "DEV_CHAR_TABLE3_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table3_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc3`] module"]
pub type DEV_CHAR_TABLE3_LOC3 = crate::Reg<dev_char_table3_loc3::DEV_CHAR_TABLE3_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table3_loc3;
#[doc = "DEV_CHAR_TABLE3_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table3_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc4`] module"]
pub type DEV_CHAR_TABLE3_LOC4 = crate::Reg<dev_char_table3_loc4::DEV_CHAR_TABLE3_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table3_loc4;
#[doc = "DEV_CHAR_TABLE4_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table4_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc1`] module"]
pub type DEV_CHAR_TABLE4_LOC1 = crate::Reg<dev_char_table4_loc1::DEV_CHAR_TABLE4_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table4_loc1;
#[doc = "DEV_CHAR_TABLE4_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table4_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc2`] module"]
pub type DEV_CHAR_TABLE4_LOC2 = crate::Reg<dev_char_table4_loc2::DEV_CHAR_TABLE4_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table4_loc2;
#[doc = "DEV_CHAR_TABLE4_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table4_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc3`] module"]
pub type DEV_CHAR_TABLE4_LOC3 = crate::Reg<dev_char_table4_loc3::DEV_CHAR_TABLE4_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table4_loc3;
#[doc = "DEV_CHAR_TABLE4_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table4_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc4`] module"]
pub type DEV_CHAR_TABLE4_LOC4 = crate::Reg<dev_char_table4_loc4::DEV_CHAR_TABLE4_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table4_loc4;
#[doc = "DEV_CHAR_TABLE5_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table5_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc1`] module"]
pub type DEV_CHAR_TABLE5_LOC1 = crate::Reg<dev_char_table5_loc1::DEV_CHAR_TABLE5_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table5_loc1;
#[doc = "DEV_CHAR_TABLE5_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table5_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc2`] module"]
pub type DEV_CHAR_TABLE5_LOC2 = crate::Reg<dev_char_table5_loc2::DEV_CHAR_TABLE5_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table5_loc2;
#[doc = "DEV_CHAR_TABLE5_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table5_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc3`] module"]
pub type DEV_CHAR_TABLE5_LOC3 = crate::Reg<dev_char_table5_loc3::DEV_CHAR_TABLE5_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table5_loc3;
#[doc = "DEV_CHAR_TABLE5_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table5_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc4`] module"]
pub type DEV_CHAR_TABLE5_LOC4 = crate::Reg<dev_char_table5_loc4::DEV_CHAR_TABLE5_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table5_loc4;
#[doc = "DEV_CHAR_TABLE6_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table6_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc1`] module"]
pub type DEV_CHAR_TABLE6_LOC1 = crate::Reg<dev_char_table6_loc1::DEV_CHAR_TABLE6_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table6_loc1;
#[doc = "DEV_CHAR_TABLE6_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table6_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc2`] module"]
pub type DEV_CHAR_TABLE6_LOC2 = crate::Reg<dev_char_table6_loc2::DEV_CHAR_TABLE6_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table6_loc2;
#[doc = "DEV_CHAR_TABLE6_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table6_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc3`] module"]
pub type DEV_CHAR_TABLE6_LOC3 = crate::Reg<dev_char_table6_loc3::DEV_CHAR_TABLE6_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table6_loc3;
#[doc = "DEV_CHAR_TABLE6_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table6_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc4`] module"]
pub type DEV_CHAR_TABLE6_LOC4 = crate::Reg<dev_char_table6_loc4::DEV_CHAR_TABLE6_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table6_loc4;
#[doc = "DEV_CHAR_TABLE7_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table7_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc1`] module"]
pub type DEV_CHAR_TABLE7_LOC1 = crate::Reg<dev_char_table7_loc1::DEV_CHAR_TABLE7_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table7_loc1;
#[doc = "DEV_CHAR_TABLE7_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table7_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc2`] module"]
pub type DEV_CHAR_TABLE7_LOC2 = crate::Reg<dev_char_table7_loc2::DEV_CHAR_TABLE7_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table7_loc2;
#[doc = "DEV_CHAR_TABLE7_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table7_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc3`] module"]
pub type DEV_CHAR_TABLE7_LOC3 = crate::Reg<dev_char_table7_loc3::DEV_CHAR_TABLE7_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table7_loc3;
#[doc = "DEV_CHAR_TABLE7_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table7_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc4`] module"]
pub type DEV_CHAR_TABLE7_LOC4 = crate::Reg<dev_char_table7_loc4::DEV_CHAR_TABLE7_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table7_loc4;
#[doc = "DEV_CHAR_TABLE8_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table8_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc1`] module"]
pub type DEV_CHAR_TABLE8_LOC1 = crate::Reg<dev_char_table8_loc1::DEV_CHAR_TABLE8_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table8_loc1;
#[doc = "DEV_CHAR_TABLE8_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table8_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc2`] module"]
pub type DEV_CHAR_TABLE8_LOC2 = crate::Reg<dev_char_table8_loc2::DEV_CHAR_TABLE8_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table8_loc2;
#[doc = "DEV_CHAR_TABLE8_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table8_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc3`] module"]
pub type DEV_CHAR_TABLE8_LOC3 = crate::Reg<dev_char_table8_loc3::DEV_CHAR_TABLE8_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table8_loc3;
#[doc = "DEV_CHAR_TABLE8_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table8_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc4`] module"]
pub type DEV_CHAR_TABLE8_LOC4 = crate::Reg<dev_char_table8_loc4::DEV_CHAR_TABLE8_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table8_loc4;
#[doc = "DEV_CHAR_TABLE9_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table9_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc1`] module"]
pub type DEV_CHAR_TABLE9_LOC1 = crate::Reg<dev_char_table9_loc1::DEV_CHAR_TABLE9_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table9_loc1;
#[doc = "DEV_CHAR_TABLE9_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table9_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc2`] module"]
pub type DEV_CHAR_TABLE9_LOC2 = crate::Reg<dev_char_table9_loc2::DEV_CHAR_TABLE9_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table9_loc2;
#[doc = "DEV_CHAR_TABLE9_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table9_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc3`] module"]
pub type DEV_CHAR_TABLE9_LOC3 = crate::Reg<dev_char_table9_loc3::DEV_CHAR_TABLE9_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table9_loc3;
#[doc = "DEV_CHAR_TABLE9_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table9_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc4`] module"]
pub type DEV_CHAR_TABLE9_LOC4 = crate::Reg<dev_char_table9_loc4::DEV_CHAR_TABLE9_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table9_loc4;
#[doc = "DEV_CHAR_TABLE10_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table10_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc1`] module"]
pub type DEV_CHAR_TABLE10_LOC1 = crate::Reg<dev_char_table10_loc1::DEV_CHAR_TABLE10_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table10_loc1;
#[doc = "DEV_CHAR_TABLE10_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table10_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc2`] module"]
pub type DEV_CHAR_TABLE10_LOC2 = crate::Reg<dev_char_table10_loc2::DEV_CHAR_TABLE10_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table10_loc2;
#[doc = "DEV_CHAR_TABLE10_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table10_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc3`] module"]
pub type DEV_CHAR_TABLE10_LOC3 = crate::Reg<dev_char_table10_loc3::DEV_CHAR_TABLE10_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table10_loc3;
#[doc = "DEV_CHAR_TABLE10_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table10_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc4`] module"]
pub type DEV_CHAR_TABLE10_LOC4 = crate::Reg<dev_char_table10_loc4::DEV_CHAR_TABLE10_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table10_loc4;
#[doc = "DEV_CHAR_TABLE11_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table11_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc1`] module"]
pub type DEV_CHAR_TABLE11_LOC1 = crate::Reg<dev_char_table11_loc1::DEV_CHAR_TABLE11_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table11_loc1;
#[doc = "DEV_CHAR_TABLE11_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table11_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc2`] module"]
pub type DEV_CHAR_TABLE11_LOC2 = crate::Reg<dev_char_table11_loc2::DEV_CHAR_TABLE11_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table11_loc2;
#[doc = "DEV_CHAR_TABLE11_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table11_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc3`] module"]
pub type DEV_CHAR_TABLE11_LOC3 = crate::Reg<dev_char_table11_loc3::DEV_CHAR_TABLE11_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table11_loc3;
#[doc = "DEV_CHAR_TABLE11_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table11_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc4`] module"]
pub type DEV_CHAR_TABLE11_LOC4 = crate::Reg<dev_char_table11_loc4::DEV_CHAR_TABLE11_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table11_loc4;
#[doc = "DEV_CHAR_TABLE12_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table12_loc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc1`] module"]
pub type DEV_CHAR_TABLE12_LOC1 = crate::Reg<dev_char_table12_loc1::DEV_CHAR_TABLE12_LOC1_SPEC>;
#[doc = "NA"]
pub mod dev_char_table12_loc1;
#[doc = "DEV_CHAR_TABLE12_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table12_loc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc2`] module"]
pub type DEV_CHAR_TABLE12_LOC2 = crate::Reg<dev_char_table12_loc2::DEV_CHAR_TABLE12_LOC2_SPEC>;
#[doc = "NA"]
pub mod dev_char_table12_loc2;
#[doc = "DEV_CHAR_TABLE12_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table12_loc3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc3`] module"]
pub type DEV_CHAR_TABLE12_LOC3 = crate::Reg<dev_char_table12_loc3::DEV_CHAR_TABLE12_LOC3_SPEC>;
#[doc = "NA"]
pub mod dev_char_table12_loc3;
#[doc = "DEV_CHAR_TABLE12_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table12_loc4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc4`] module"]
pub type DEV_CHAR_TABLE12_LOC4 = crate::Reg<dev_char_table12_loc4::DEV_CHAR_TABLE12_LOC4_SPEC>;
#[doc = "NA"]
pub mod dev_char_table12_loc4;
