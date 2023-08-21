#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - channel enable register"]
    pub ch_ena_ad0: CH_ENA_AD0,
    #[doc = "0x04 - channel enable set register"]
    pub ch_ena_ad0_set: CH_ENA_AD0_SET,
    #[doc = "0x08 - channel enable clear register"]
    pub ch_ena_ad0_clr: CH_ENA_AD0_CLR,
    #[doc = "0x0c - channel enable register"]
    pub ch_ena_ad1: CH_ENA_AD1,
    #[doc = "0x10 - channel enable set register"]
    pub ch_ena_ad1_set: CH_ENA_AD1_SET,
    #[doc = "0x14 - channel enable clear register"]
    pub ch_ena_ad1_clr: CH_ENA_AD1_CLR,
    #[doc = "0x18 - channel0 event id register"]
    pub ch0_evt_id: CH0_EVT_ID,
    #[doc = "0x1c - channel0 task id register"]
    pub ch0_task_id: CH0_TASK_ID,
    #[doc = "0x20 - channel1 event id register"]
    pub ch1_evt_id: CH1_EVT_ID,
    #[doc = "0x24 - channel1 task id register"]
    pub ch1_task_id: CH1_TASK_ID,
    #[doc = "0x28 - channel2 event id register"]
    pub ch2_evt_id: CH2_EVT_ID,
    #[doc = "0x2c - channel2 task id register"]
    pub ch2_task_id: CH2_TASK_ID,
    #[doc = "0x30 - channel3 event id register"]
    pub ch3_evt_id: CH3_EVT_ID,
    #[doc = "0x34 - channel3 task id register"]
    pub ch3_task_id: CH3_TASK_ID,
    #[doc = "0x38 - channel4 event id register"]
    pub ch4_evt_id: CH4_EVT_ID,
    #[doc = "0x3c - channel4 task id register"]
    pub ch4_task_id: CH4_TASK_ID,
    #[doc = "0x40 - channel5 event id register"]
    pub ch5_evt_id: CH5_EVT_ID,
    #[doc = "0x44 - channel5 task id register"]
    pub ch5_task_id: CH5_TASK_ID,
    #[doc = "0x48 - channel6 event id register"]
    pub ch6_evt_id: CH6_EVT_ID,
    #[doc = "0x4c - channel6 task id register"]
    pub ch6_task_id: CH6_TASK_ID,
    #[doc = "0x50 - channel7 event id register"]
    pub ch7_evt_id: CH7_EVT_ID,
    #[doc = "0x54 - channel7 task id register"]
    pub ch7_task_id: CH7_TASK_ID,
    #[doc = "0x58 - channel8 event id register"]
    pub ch8_evt_id: CH8_EVT_ID,
    #[doc = "0x5c - channel8 task id register"]
    pub ch8_task_id: CH8_TASK_ID,
    #[doc = "0x60 - channel9 event id register"]
    pub ch9_evt_id: CH9_EVT_ID,
    #[doc = "0x64 - channel9 task id register"]
    pub ch9_task_id: CH9_TASK_ID,
    #[doc = "0x68 - channel10 event id register"]
    pub ch10_evt_id: CH10_EVT_ID,
    #[doc = "0x6c - channel10 task id register"]
    pub ch10_task_id: CH10_TASK_ID,
    #[doc = "0x70 - channel11 event id register"]
    pub ch11_evt_id: CH11_EVT_ID,
    #[doc = "0x74 - channel11 task id register"]
    pub ch11_task_id: CH11_TASK_ID,
    #[doc = "0x78 - channel12 event id register"]
    pub ch12_evt_id: CH12_EVT_ID,
    #[doc = "0x7c - channel12 task id register"]
    pub ch12_task_id: CH12_TASK_ID,
    #[doc = "0x80 - channel13 event id register"]
    pub ch13_evt_id: CH13_EVT_ID,
    #[doc = "0x84 - channel13 task id register"]
    pub ch13_task_id: CH13_TASK_ID,
    #[doc = "0x88 - channel14 event id register"]
    pub ch14_evt_id: CH14_EVT_ID,
    #[doc = "0x8c - channel14 task id register"]
    pub ch14_task_id: CH14_TASK_ID,
    #[doc = "0x90 - channel15 event id register"]
    pub ch15_evt_id: CH15_EVT_ID,
    #[doc = "0x94 - channel15 task id register"]
    pub ch15_task_id: CH15_TASK_ID,
    #[doc = "0x98 - channel16 event id register"]
    pub ch16_evt_id: CH16_EVT_ID,
    #[doc = "0x9c - channel16 task id register"]
    pub ch16_task_id: CH16_TASK_ID,
    #[doc = "0xa0 - channel17 event id register"]
    pub ch17_evt_id: CH17_EVT_ID,
    #[doc = "0xa4 - channel17 task id register"]
    pub ch17_task_id: CH17_TASK_ID,
    #[doc = "0xa8 - channel18 event id register"]
    pub ch18_evt_id: CH18_EVT_ID,
    #[doc = "0xac - channel18 task id register"]
    pub ch18_task_id: CH18_TASK_ID,
    #[doc = "0xb0 - channel19 event id register"]
    pub ch19_evt_id: CH19_EVT_ID,
    #[doc = "0xb4 - channel19 task id register"]
    pub ch19_task_id: CH19_TASK_ID,
    #[doc = "0xb8 - channel20 event id register"]
    pub ch20_evt_id: CH20_EVT_ID,
    #[doc = "0xbc - channel20 task id register"]
    pub ch20_task_id: CH20_TASK_ID,
    #[doc = "0xc0 - channel21 event id register"]
    pub ch21_evt_id: CH21_EVT_ID,
    #[doc = "0xc4 - channel21 task id register"]
    pub ch21_task_id: CH21_TASK_ID,
    #[doc = "0xc8 - channel22 event id register"]
    pub ch22_evt_id: CH22_EVT_ID,
    #[doc = "0xcc - channel22 task id register"]
    pub ch22_task_id: CH22_TASK_ID,
    #[doc = "0xd0 - channel23 event id register"]
    pub ch23_evt_id: CH23_EVT_ID,
    #[doc = "0xd4 - channel23 task id register"]
    pub ch23_task_id: CH23_TASK_ID,
    #[doc = "0xd8 - channel24 event id register"]
    pub ch24_evt_id: CH24_EVT_ID,
    #[doc = "0xdc - channel24 task id register"]
    pub ch24_task_id: CH24_TASK_ID,
    #[doc = "0xe0 - channel25 event id register"]
    pub ch25_evt_id: CH25_EVT_ID,
    #[doc = "0xe4 - channel25 task id register"]
    pub ch25_task_id: CH25_TASK_ID,
    #[doc = "0xe8 - channel26 event id register"]
    pub ch26_evt_id: CH26_EVT_ID,
    #[doc = "0xec - channel26 task id register"]
    pub ch26_task_id: CH26_TASK_ID,
    #[doc = "0xf0 - channel27 event id register"]
    pub ch27_evt_id: CH27_EVT_ID,
    #[doc = "0xf4 - channel27 task id register"]
    pub ch27_task_id: CH27_TASK_ID,
    #[doc = "0xf8 - channel28 event id register"]
    pub ch28_evt_id: CH28_EVT_ID,
    #[doc = "0xfc - channel28 task id register"]
    pub ch28_task_id: CH28_TASK_ID,
    #[doc = "0x100 - channel29 event id register"]
    pub ch29_evt_id: CH29_EVT_ID,
    #[doc = "0x104 - channel29 task id register"]
    pub ch29_task_id: CH29_TASK_ID,
    #[doc = "0x108 - channel30 event id register"]
    pub ch30_evt_id: CH30_EVT_ID,
    #[doc = "0x10c - channel30 task id register"]
    pub ch30_task_id: CH30_TASK_ID,
    #[doc = "0x110 - channel31 event id register"]
    pub ch31_evt_id: CH31_EVT_ID,
    #[doc = "0x114 - channel31 task id register"]
    pub ch31_task_id: CH31_TASK_ID,
    #[doc = "0x118 - channel32 event id register"]
    pub ch32_evt_id: CH32_EVT_ID,
    #[doc = "0x11c - channel32 task id register"]
    pub ch32_task_id: CH32_TASK_ID,
    #[doc = "0x120 - channel33 event id register"]
    pub ch33_evt_id: CH33_EVT_ID,
    #[doc = "0x124 - channel33 task id register"]
    pub ch33_task_id: CH33_TASK_ID,
    #[doc = "0x128 - channel34 event id register"]
    pub ch34_evt_id: CH34_EVT_ID,
    #[doc = "0x12c - channel34 task id register"]
    pub ch34_task_id: CH34_TASK_ID,
    #[doc = "0x130 - channel35 event id register"]
    pub ch35_evt_id: CH35_EVT_ID,
    #[doc = "0x134 - channel35 task id register"]
    pub ch35_task_id: CH35_TASK_ID,
    #[doc = "0x138 - channel36 event id register"]
    pub ch36_evt_id: CH36_EVT_ID,
    #[doc = "0x13c - channel36 task id register"]
    pub ch36_task_id: CH36_TASK_ID,
    #[doc = "0x140 - channel37 event id register"]
    pub ch37_evt_id: CH37_EVT_ID,
    #[doc = "0x144 - channel37 task id register"]
    pub ch37_task_id: CH37_TASK_ID,
    #[doc = "0x148 - channel38 event id register"]
    pub ch38_evt_id: CH38_EVT_ID,
    #[doc = "0x14c - channel38 task id register"]
    pub ch38_task_id: CH38_TASK_ID,
    #[doc = "0x150 - channel39 event id register"]
    pub ch39_evt_id: CH39_EVT_ID,
    #[doc = "0x154 - channel39 task id register"]
    pub ch39_task_id: CH39_TASK_ID,
    #[doc = "0x158 - channel40 event id register"]
    pub ch40_evt_id: CH40_EVT_ID,
    #[doc = "0x15c - channel40 task id register"]
    pub ch40_task_id: CH40_TASK_ID,
    #[doc = "0x160 - channel41 event id register"]
    pub ch41_evt_id: CH41_EVT_ID,
    #[doc = "0x164 - channel41 task id register"]
    pub ch41_task_id: CH41_TASK_ID,
    #[doc = "0x168 - channel42 event id register"]
    pub ch42_evt_id: CH42_EVT_ID,
    #[doc = "0x16c - channel42 task id register"]
    pub ch42_task_id: CH42_TASK_ID,
    #[doc = "0x170 - channel43 event id register"]
    pub ch43_evt_id: CH43_EVT_ID,
    #[doc = "0x174 - channel43 task id register"]
    pub ch43_task_id: CH43_TASK_ID,
    #[doc = "0x178 - channel44 event id register"]
    pub ch44_evt_id: CH44_EVT_ID,
    #[doc = "0x17c - channel44 task id register"]
    pub ch44_task_id: CH44_TASK_ID,
    #[doc = "0x180 - channel45 event id register"]
    pub ch45_evt_id: CH45_EVT_ID,
    #[doc = "0x184 - channel45 task id register"]
    pub ch45_task_id: CH45_TASK_ID,
    #[doc = "0x188 - channel46 event id register"]
    pub ch46_evt_id: CH46_EVT_ID,
    #[doc = "0x18c - channel46 task id register"]
    pub ch46_task_id: CH46_TASK_ID,
    #[doc = "0x190 - channel47 event id register"]
    pub ch47_evt_id: CH47_EVT_ID,
    #[doc = "0x194 - channel47 task id register"]
    pub ch47_task_id: CH47_TASK_ID,
    #[doc = "0x198 - channel48 event id register"]
    pub ch48_evt_id: CH48_EVT_ID,
    #[doc = "0x19c - channel48 task id register"]
    pub ch48_task_id: CH48_TASK_ID,
    #[doc = "0x1a0 - channel49 event id register"]
    pub ch49_evt_id: CH49_EVT_ID,
    #[doc = "0x1a4 - channel49 task id register"]
    pub ch49_task_id: CH49_TASK_ID,
    #[doc = "0x1a8 - etm clock enable register"]
    pub clk_en: CLK_EN,
    #[doc = "0x1ac - etm date register"]
    pub date: DATE,
}
#[doc = "CH_ENA_AD0 (rw) register accessor: channel enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_ena_ad0`] module"]
pub type CH_ENA_AD0 = crate::Reg<ch_ena_ad0::CH_ENA_AD0_SPEC>;
#[doc = "channel enable register"]
pub mod ch_ena_ad0;
#[doc = "CH_ENA_AD0_SET (w) register accessor: channel enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_ena_ad0_set`] module"]
pub type CH_ENA_AD0_SET = crate::Reg<ch_ena_ad0_set::CH_ENA_AD0_SET_SPEC>;
#[doc = "channel enable set register"]
pub mod ch_ena_ad0_set;
#[doc = "CH_ENA_AD0_CLR (w) register accessor: channel enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_ena_ad0_clr`] module"]
pub type CH_ENA_AD0_CLR = crate::Reg<ch_ena_ad0_clr::CH_ENA_AD0_CLR_SPEC>;
#[doc = "channel enable clear register"]
pub mod ch_ena_ad0_clr;
#[doc = "CH_ENA_AD1 (rw) register accessor: channel enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_ena_ad1`] module"]
pub type CH_ENA_AD1 = crate::Reg<ch_ena_ad1::CH_ENA_AD1_SPEC>;
#[doc = "channel enable register"]
pub mod ch_ena_ad1;
#[doc = "CH_ENA_AD1_SET (w) register accessor: channel enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_ena_ad1_set`] module"]
pub type CH_ENA_AD1_SET = crate::Reg<ch_ena_ad1_set::CH_ENA_AD1_SET_SPEC>;
#[doc = "channel enable set register"]
pub mod ch_ena_ad1_set;
#[doc = "CH_ENA_AD1_CLR (w) register accessor: channel enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch_ena_ad1_clr`] module"]
pub type CH_ENA_AD1_CLR = crate::Reg<ch_ena_ad1_clr::CH_ENA_AD1_CLR_SPEC>;
#[doc = "channel enable clear register"]
pub mod ch_ena_ad1_clr;
#[doc = "CH0_EVT_ID (rw) register accessor: channel0 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch0_evt_id`] module"]
pub type CH0_EVT_ID = crate::Reg<ch0_evt_id::CH0_EVT_ID_SPEC>;
#[doc = "channel0 event id register"]
pub mod ch0_evt_id;
#[doc = "CH0_TASK_ID (rw) register accessor: channel0 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch0_task_id`] module"]
pub type CH0_TASK_ID = crate::Reg<ch0_task_id::CH0_TASK_ID_SPEC>;
#[doc = "channel0 task id register"]
pub mod ch0_task_id;
#[doc = "CH1_EVT_ID (rw) register accessor: channel1 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch1_evt_id`] module"]
pub type CH1_EVT_ID = crate::Reg<ch1_evt_id::CH1_EVT_ID_SPEC>;
#[doc = "channel1 event id register"]
pub mod ch1_evt_id;
#[doc = "CH1_TASK_ID (rw) register accessor: channel1 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch1_task_id`] module"]
pub type CH1_TASK_ID = crate::Reg<ch1_task_id::CH1_TASK_ID_SPEC>;
#[doc = "channel1 task id register"]
pub mod ch1_task_id;
#[doc = "CH2_EVT_ID (rw) register accessor: channel2 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch2_evt_id`] module"]
pub type CH2_EVT_ID = crate::Reg<ch2_evt_id::CH2_EVT_ID_SPEC>;
#[doc = "channel2 event id register"]
pub mod ch2_evt_id;
#[doc = "CH2_TASK_ID (rw) register accessor: channel2 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch2_task_id`] module"]
pub type CH2_TASK_ID = crate::Reg<ch2_task_id::CH2_TASK_ID_SPEC>;
#[doc = "channel2 task id register"]
pub mod ch2_task_id;
#[doc = "CH3_EVT_ID (rw) register accessor: channel3 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch3_evt_id`] module"]
pub type CH3_EVT_ID = crate::Reg<ch3_evt_id::CH3_EVT_ID_SPEC>;
#[doc = "channel3 event id register"]
pub mod ch3_evt_id;
#[doc = "CH3_TASK_ID (rw) register accessor: channel3 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch3_task_id`] module"]
pub type CH3_TASK_ID = crate::Reg<ch3_task_id::CH3_TASK_ID_SPEC>;
#[doc = "channel3 task id register"]
pub mod ch3_task_id;
#[doc = "CH4_EVT_ID (rw) register accessor: channel4 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch4_evt_id`] module"]
pub type CH4_EVT_ID = crate::Reg<ch4_evt_id::CH4_EVT_ID_SPEC>;
#[doc = "channel4 event id register"]
pub mod ch4_evt_id;
#[doc = "CH4_TASK_ID (rw) register accessor: channel4 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch4_task_id`] module"]
pub type CH4_TASK_ID = crate::Reg<ch4_task_id::CH4_TASK_ID_SPEC>;
#[doc = "channel4 task id register"]
pub mod ch4_task_id;
#[doc = "CH5_EVT_ID (rw) register accessor: channel5 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch5_evt_id`] module"]
pub type CH5_EVT_ID = crate::Reg<ch5_evt_id::CH5_EVT_ID_SPEC>;
#[doc = "channel5 event id register"]
pub mod ch5_evt_id;
#[doc = "CH5_TASK_ID (rw) register accessor: channel5 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch5_task_id`] module"]
pub type CH5_TASK_ID = crate::Reg<ch5_task_id::CH5_TASK_ID_SPEC>;
#[doc = "channel5 task id register"]
pub mod ch5_task_id;
#[doc = "CH6_EVT_ID (rw) register accessor: channel6 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch6_evt_id`] module"]
pub type CH6_EVT_ID = crate::Reg<ch6_evt_id::CH6_EVT_ID_SPEC>;
#[doc = "channel6 event id register"]
pub mod ch6_evt_id;
#[doc = "CH6_TASK_ID (rw) register accessor: channel6 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch6_task_id`] module"]
pub type CH6_TASK_ID = crate::Reg<ch6_task_id::CH6_TASK_ID_SPEC>;
#[doc = "channel6 task id register"]
pub mod ch6_task_id;
#[doc = "CH7_EVT_ID (rw) register accessor: channel7 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch7_evt_id`] module"]
pub type CH7_EVT_ID = crate::Reg<ch7_evt_id::CH7_EVT_ID_SPEC>;
#[doc = "channel7 event id register"]
pub mod ch7_evt_id;
#[doc = "CH7_TASK_ID (rw) register accessor: channel7 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch7_task_id`] module"]
pub type CH7_TASK_ID = crate::Reg<ch7_task_id::CH7_TASK_ID_SPEC>;
#[doc = "channel7 task id register"]
pub mod ch7_task_id;
#[doc = "CH8_EVT_ID (rw) register accessor: channel8 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch8_evt_id`] module"]
pub type CH8_EVT_ID = crate::Reg<ch8_evt_id::CH8_EVT_ID_SPEC>;
#[doc = "channel8 event id register"]
pub mod ch8_evt_id;
#[doc = "CH8_TASK_ID (rw) register accessor: channel8 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch8_task_id`] module"]
pub type CH8_TASK_ID = crate::Reg<ch8_task_id::CH8_TASK_ID_SPEC>;
#[doc = "channel8 task id register"]
pub mod ch8_task_id;
#[doc = "CH9_EVT_ID (rw) register accessor: channel9 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch9_evt_id`] module"]
pub type CH9_EVT_ID = crate::Reg<ch9_evt_id::CH9_EVT_ID_SPEC>;
#[doc = "channel9 event id register"]
pub mod ch9_evt_id;
#[doc = "CH9_TASK_ID (rw) register accessor: channel9 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch9_task_id`] module"]
pub type CH9_TASK_ID = crate::Reg<ch9_task_id::CH9_TASK_ID_SPEC>;
#[doc = "channel9 task id register"]
pub mod ch9_task_id;
#[doc = "CH10_EVT_ID (rw) register accessor: channel10 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch10_evt_id`] module"]
pub type CH10_EVT_ID = crate::Reg<ch10_evt_id::CH10_EVT_ID_SPEC>;
#[doc = "channel10 event id register"]
pub mod ch10_evt_id;
#[doc = "CH10_TASK_ID (rw) register accessor: channel10 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch10_task_id`] module"]
pub type CH10_TASK_ID = crate::Reg<ch10_task_id::CH10_TASK_ID_SPEC>;
#[doc = "channel10 task id register"]
pub mod ch10_task_id;
#[doc = "CH11_EVT_ID (rw) register accessor: channel11 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch11_evt_id`] module"]
pub type CH11_EVT_ID = crate::Reg<ch11_evt_id::CH11_EVT_ID_SPEC>;
#[doc = "channel11 event id register"]
pub mod ch11_evt_id;
#[doc = "CH11_TASK_ID (rw) register accessor: channel11 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch11_task_id`] module"]
pub type CH11_TASK_ID = crate::Reg<ch11_task_id::CH11_TASK_ID_SPEC>;
#[doc = "channel11 task id register"]
pub mod ch11_task_id;
#[doc = "CH12_EVT_ID (rw) register accessor: channel12 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch12_evt_id`] module"]
pub type CH12_EVT_ID = crate::Reg<ch12_evt_id::CH12_EVT_ID_SPEC>;
#[doc = "channel12 event id register"]
pub mod ch12_evt_id;
#[doc = "CH12_TASK_ID (rw) register accessor: channel12 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch12_task_id`] module"]
pub type CH12_TASK_ID = crate::Reg<ch12_task_id::CH12_TASK_ID_SPEC>;
#[doc = "channel12 task id register"]
pub mod ch12_task_id;
#[doc = "CH13_EVT_ID (rw) register accessor: channel13 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch13_evt_id`] module"]
pub type CH13_EVT_ID = crate::Reg<ch13_evt_id::CH13_EVT_ID_SPEC>;
#[doc = "channel13 event id register"]
pub mod ch13_evt_id;
#[doc = "CH13_TASK_ID (rw) register accessor: channel13 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch13_task_id`] module"]
pub type CH13_TASK_ID = crate::Reg<ch13_task_id::CH13_TASK_ID_SPEC>;
#[doc = "channel13 task id register"]
pub mod ch13_task_id;
#[doc = "CH14_EVT_ID (rw) register accessor: channel14 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch14_evt_id`] module"]
pub type CH14_EVT_ID = crate::Reg<ch14_evt_id::CH14_EVT_ID_SPEC>;
#[doc = "channel14 event id register"]
pub mod ch14_evt_id;
#[doc = "CH14_TASK_ID (rw) register accessor: channel14 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch14_task_id`] module"]
pub type CH14_TASK_ID = crate::Reg<ch14_task_id::CH14_TASK_ID_SPEC>;
#[doc = "channel14 task id register"]
pub mod ch14_task_id;
#[doc = "CH15_EVT_ID (rw) register accessor: channel15 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch15_evt_id`] module"]
pub type CH15_EVT_ID = crate::Reg<ch15_evt_id::CH15_EVT_ID_SPEC>;
#[doc = "channel15 event id register"]
pub mod ch15_evt_id;
#[doc = "CH15_TASK_ID (rw) register accessor: channel15 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch15_task_id`] module"]
pub type CH15_TASK_ID = crate::Reg<ch15_task_id::CH15_TASK_ID_SPEC>;
#[doc = "channel15 task id register"]
pub mod ch15_task_id;
#[doc = "CH16_EVT_ID (rw) register accessor: channel16 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch16_evt_id`] module"]
pub type CH16_EVT_ID = crate::Reg<ch16_evt_id::CH16_EVT_ID_SPEC>;
#[doc = "channel16 event id register"]
pub mod ch16_evt_id;
#[doc = "CH16_TASK_ID (rw) register accessor: channel16 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch16_task_id`] module"]
pub type CH16_TASK_ID = crate::Reg<ch16_task_id::CH16_TASK_ID_SPEC>;
#[doc = "channel16 task id register"]
pub mod ch16_task_id;
#[doc = "CH17_EVT_ID (rw) register accessor: channel17 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch17_evt_id`] module"]
pub type CH17_EVT_ID = crate::Reg<ch17_evt_id::CH17_EVT_ID_SPEC>;
#[doc = "channel17 event id register"]
pub mod ch17_evt_id;
#[doc = "CH17_TASK_ID (rw) register accessor: channel17 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch17_task_id`] module"]
pub type CH17_TASK_ID = crate::Reg<ch17_task_id::CH17_TASK_ID_SPEC>;
#[doc = "channel17 task id register"]
pub mod ch17_task_id;
#[doc = "CH18_EVT_ID (rw) register accessor: channel18 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch18_evt_id`] module"]
pub type CH18_EVT_ID = crate::Reg<ch18_evt_id::CH18_EVT_ID_SPEC>;
#[doc = "channel18 event id register"]
pub mod ch18_evt_id;
#[doc = "CH18_TASK_ID (rw) register accessor: channel18 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch18_task_id`] module"]
pub type CH18_TASK_ID = crate::Reg<ch18_task_id::CH18_TASK_ID_SPEC>;
#[doc = "channel18 task id register"]
pub mod ch18_task_id;
#[doc = "CH19_EVT_ID (rw) register accessor: channel19 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch19_evt_id`] module"]
pub type CH19_EVT_ID = crate::Reg<ch19_evt_id::CH19_EVT_ID_SPEC>;
#[doc = "channel19 event id register"]
pub mod ch19_evt_id;
#[doc = "CH19_TASK_ID (rw) register accessor: channel19 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch19_task_id`] module"]
pub type CH19_TASK_ID = crate::Reg<ch19_task_id::CH19_TASK_ID_SPEC>;
#[doc = "channel19 task id register"]
pub mod ch19_task_id;
#[doc = "CH20_EVT_ID (rw) register accessor: channel20 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch20_evt_id`] module"]
pub type CH20_EVT_ID = crate::Reg<ch20_evt_id::CH20_EVT_ID_SPEC>;
#[doc = "channel20 event id register"]
pub mod ch20_evt_id;
#[doc = "CH20_TASK_ID (rw) register accessor: channel20 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch20_task_id`] module"]
pub type CH20_TASK_ID = crate::Reg<ch20_task_id::CH20_TASK_ID_SPEC>;
#[doc = "channel20 task id register"]
pub mod ch20_task_id;
#[doc = "CH21_EVT_ID (rw) register accessor: channel21 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch21_evt_id`] module"]
pub type CH21_EVT_ID = crate::Reg<ch21_evt_id::CH21_EVT_ID_SPEC>;
#[doc = "channel21 event id register"]
pub mod ch21_evt_id;
#[doc = "CH21_TASK_ID (rw) register accessor: channel21 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch21_task_id`] module"]
pub type CH21_TASK_ID = crate::Reg<ch21_task_id::CH21_TASK_ID_SPEC>;
#[doc = "channel21 task id register"]
pub mod ch21_task_id;
#[doc = "CH22_EVT_ID (rw) register accessor: channel22 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch22_evt_id`] module"]
pub type CH22_EVT_ID = crate::Reg<ch22_evt_id::CH22_EVT_ID_SPEC>;
#[doc = "channel22 event id register"]
pub mod ch22_evt_id;
#[doc = "CH22_TASK_ID (rw) register accessor: channel22 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch22_task_id`] module"]
pub type CH22_TASK_ID = crate::Reg<ch22_task_id::CH22_TASK_ID_SPEC>;
#[doc = "channel22 task id register"]
pub mod ch22_task_id;
#[doc = "CH23_EVT_ID (rw) register accessor: channel23 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch23_evt_id`] module"]
pub type CH23_EVT_ID = crate::Reg<ch23_evt_id::CH23_EVT_ID_SPEC>;
#[doc = "channel23 event id register"]
pub mod ch23_evt_id;
#[doc = "CH23_TASK_ID (rw) register accessor: channel23 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch23_task_id`] module"]
pub type CH23_TASK_ID = crate::Reg<ch23_task_id::CH23_TASK_ID_SPEC>;
#[doc = "channel23 task id register"]
pub mod ch23_task_id;
#[doc = "CH24_EVT_ID (rw) register accessor: channel24 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch24_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch24_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch24_evt_id`] module"]
pub type CH24_EVT_ID = crate::Reg<ch24_evt_id::CH24_EVT_ID_SPEC>;
#[doc = "channel24 event id register"]
pub mod ch24_evt_id;
#[doc = "CH24_TASK_ID (rw) register accessor: channel24 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch24_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch24_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch24_task_id`] module"]
pub type CH24_TASK_ID = crate::Reg<ch24_task_id::CH24_TASK_ID_SPEC>;
#[doc = "channel24 task id register"]
pub mod ch24_task_id;
#[doc = "CH25_EVT_ID (rw) register accessor: channel25 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch25_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch25_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch25_evt_id`] module"]
pub type CH25_EVT_ID = crate::Reg<ch25_evt_id::CH25_EVT_ID_SPEC>;
#[doc = "channel25 event id register"]
pub mod ch25_evt_id;
#[doc = "CH25_TASK_ID (rw) register accessor: channel25 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch25_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch25_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch25_task_id`] module"]
pub type CH25_TASK_ID = crate::Reg<ch25_task_id::CH25_TASK_ID_SPEC>;
#[doc = "channel25 task id register"]
pub mod ch25_task_id;
#[doc = "CH26_EVT_ID (rw) register accessor: channel26 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch26_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch26_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch26_evt_id`] module"]
pub type CH26_EVT_ID = crate::Reg<ch26_evt_id::CH26_EVT_ID_SPEC>;
#[doc = "channel26 event id register"]
pub mod ch26_evt_id;
#[doc = "CH26_TASK_ID (rw) register accessor: channel26 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch26_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch26_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch26_task_id`] module"]
pub type CH26_TASK_ID = crate::Reg<ch26_task_id::CH26_TASK_ID_SPEC>;
#[doc = "channel26 task id register"]
pub mod ch26_task_id;
#[doc = "CH27_EVT_ID (rw) register accessor: channel27 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch27_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch27_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch27_evt_id`] module"]
pub type CH27_EVT_ID = crate::Reg<ch27_evt_id::CH27_EVT_ID_SPEC>;
#[doc = "channel27 event id register"]
pub mod ch27_evt_id;
#[doc = "CH27_TASK_ID (rw) register accessor: channel27 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch27_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch27_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch27_task_id`] module"]
pub type CH27_TASK_ID = crate::Reg<ch27_task_id::CH27_TASK_ID_SPEC>;
#[doc = "channel27 task id register"]
pub mod ch27_task_id;
#[doc = "CH28_EVT_ID (rw) register accessor: channel28 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch28_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch28_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch28_evt_id`] module"]
pub type CH28_EVT_ID = crate::Reg<ch28_evt_id::CH28_EVT_ID_SPEC>;
#[doc = "channel28 event id register"]
pub mod ch28_evt_id;
#[doc = "CH28_TASK_ID (rw) register accessor: channel28 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch28_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch28_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch28_task_id`] module"]
pub type CH28_TASK_ID = crate::Reg<ch28_task_id::CH28_TASK_ID_SPEC>;
#[doc = "channel28 task id register"]
pub mod ch28_task_id;
#[doc = "CH29_EVT_ID (rw) register accessor: channel29 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch29_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch29_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch29_evt_id`] module"]
pub type CH29_EVT_ID = crate::Reg<ch29_evt_id::CH29_EVT_ID_SPEC>;
#[doc = "channel29 event id register"]
pub mod ch29_evt_id;
#[doc = "CH29_TASK_ID (rw) register accessor: channel29 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch29_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch29_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch29_task_id`] module"]
pub type CH29_TASK_ID = crate::Reg<ch29_task_id::CH29_TASK_ID_SPEC>;
#[doc = "channel29 task id register"]
pub mod ch29_task_id;
#[doc = "CH30_EVT_ID (rw) register accessor: channel30 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch30_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch30_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch30_evt_id`] module"]
pub type CH30_EVT_ID = crate::Reg<ch30_evt_id::CH30_EVT_ID_SPEC>;
#[doc = "channel30 event id register"]
pub mod ch30_evt_id;
#[doc = "CH30_TASK_ID (rw) register accessor: channel30 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch30_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch30_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch30_task_id`] module"]
pub type CH30_TASK_ID = crate::Reg<ch30_task_id::CH30_TASK_ID_SPEC>;
#[doc = "channel30 task id register"]
pub mod ch30_task_id;
#[doc = "CH31_EVT_ID (rw) register accessor: channel31 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch31_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch31_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch31_evt_id`] module"]
pub type CH31_EVT_ID = crate::Reg<ch31_evt_id::CH31_EVT_ID_SPEC>;
#[doc = "channel31 event id register"]
pub mod ch31_evt_id;
#[doc = "CH31_TASK_ID (rw) register accessor: channel31 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch31_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch31_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch31_task_id`] module"]
pub type CH31_TASK_ID = crate::Reg<ch31_task_id::CH31_TASK_ID_SPEC>;
#[doc = "channel31 task id register"]
pub mod ch31_task_id;
#[doc = "CH32_EVT_ID (rw) register accessor: channel32 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch32_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch32_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch32_evt_id`] module"]
pub type CH32_EVT_ID = crate::Reg<ch32_evt_id::CH32_EVT_ID_SPEC>;
#[doc = "channel32 event id register"]
pub mod ch32_evt_id;
#[doc = "CH32_TASK_ID (rw) register accessor: channel32 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch32_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch32_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch32_task_id`] module"]
pub type CH32_TASK_ID = crate::Reg<ch32_task_id::CH32_TASK_ID_SPEC>;
#[doc = "channel32 task id register"]
pub mod ch32_task_id;
#[doc = "CH33_EVT_ID (rw) register accessor: channel33 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch33_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch33_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch33_evt_id`] module"]
pub type CH33_EVT_ID = crate::Reg<ch33_evt_id::CH33_EVT_ID_SPEC>;
#[doc = "channel33 event id register"]
pub mod ch33_evt_id;
#[doc = "CH33_TASK_ID (rw) register accessor: channel33 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch33_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch33_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch33_task_id`] module"]
pub type CH33_TASK_ID = crate::Reg<ch33_task_id::CH33_TASK_ID_SPEC>;
#[doc = "channel33 task id register"]
pub mod ch33_task_id;
#[doc = "CH34_EVT_ID (rw) register accessor: channel34 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch34_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch34_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch34_evt_id`] module"]
pub type CH34_EVT_ID = crate::Reg<ch34_evt_id::CH34_EVT_ID_SPEC>;
#[doc = "channel34 event id register"]
pub mod ch34_evt_id;
#[doc = "CH34_TASK_ID (rw) register accessor: channel34 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch34_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch34_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch34_task_id`] module"]
pub type CH34_TASK_ID = crate::Reg<ch34_task_id::CH34_TASK_ID_SPEC>;
#[doc = "channel34 task id register"]
pub mod ch34_task_id;
#[doc = "CH35_EVT_ID (rw) register accessor: channel35 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch35_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch35_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch35_evt_id`] module"]
pub type CH35_EVT_ID = crate::Reg<ch35_evt_id::CH35_EVT_ID_SPEC>;
#[doc = "channel35 event id register"]
pub mod ch35_evt_id;
#[doc = "CH35_TASK_ID (rw) register accessor: channel35 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch35_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch35_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch35_task_id`] module"]
pub type CH35_TASK_ID = crate::Reg<ch35_task_id::CH35_TASK_ID_SPEC>;
#[doc = "channel35 task id register"]
pub mod ch35_task_id;
#[doc = "CH36_EVT_ID (rw) register accessor: channel36 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch36_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch36_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch36_evt_id`] module"]
pub type CH36_EVT_ID = crate::Reg<ch36_evt_id::CH36_EVT_ID_SPEC>;
#[doc = "channel36 event id register"]
pub mod ch36_evt_id;
#[doc = "CH36_TASK_ID (rw) register accessor: channel36 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch36_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch36_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch36_task_id`] module"]
pub type CH36_TASK_ID = crate::Reg<ch36_task_id::CH36_TASK_ID_SPEC>;
#[doc = "channel36 task id register"]
pub mod ch36_task_id;
#[doc = "CH37_EVT_ID (rw) register accessor: channel37 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch37_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch37_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch37_evt_id`] module"]
pub type CH37_EVT_ID = crate::Reg<ch37_evt_id::CH37_EVT_ID_SPEC>;
#[doc = "channel37 event id register"]
pub mod ch37_evt_id;
#[doc = "CH37_TASK_ID (rw) register accessor: channel37 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch37_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch37_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch37_task_id`] module"]
pub type CH37_TASK_ID = crate::Reg<ch37_task_id::CH37_TASK_ID_SPEC>;
#[doc = "channel37 task id register"]
pub mod ch37_task_id;
#[doc = "CH38_EVT_ID (rw) register accessor: channel38 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch38_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch38_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch38_evt_id`] module"]
pub type CH38_EVT_ID = crate::Reg<ch38_evt_id::CH38_EVT_ID_SPEC>;
#[doc = "channel38 event id register"]
pub mod ch38_evt_id;
#[doc = "CH38_TASK_ID (rw) register accessor: channel38 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch38_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch38_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch38_task_id`] module"]
pub type CH38_TASK_ID = crate::Reg<ch38_task_id::CH38_TASK_ID_SPEC>;
#[doc = "channel38 task id register"]
pub mod ch38_task_id;
#[doc = "CH39_EVT_ID (rw) register accessor: channel39 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch39_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch39_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch39_evt_id`] module"]
pub type CH39_EVT_ID = crate::Reg<ch39_evt_id::CH39_EVT_ID_SPEC>;
#[doc = "channel39 event id register"]
pub mod ch39_evt_id;
#[doc = "CH39_TASK_ID (rw) register accessor: channel39 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch39_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch39_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch39_task_id`] module"]
pub type CH39_TASK_ID = crate::Reg<ch39_task_id::CH39_TASK_ID_SPEC>;
#[doc = "channel39 task id register"]
pub mod ch39_task_id;
#[doc = "CH40_EVT_ID (rw) register accessor: channel40 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch40_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch40_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch40_evt_id`] module"]
pub type CH40_EVT_ID = crate::Reg<ch40_evt_id::CH40_EVT_ID_SPEC>;
#[doc = "channel40 event id register"]
pub mod ch40_evt_id;
#[doc = "CH40_TASK_ID (rw) register accessor: channel40 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch40_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch40_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch40_task_id`] module"]
pub type CH40_TASK_ID = crate::Reg<ch40_task_id::CH40_TASK_ID_SPEC>;
#[doc = "channel40 task id register"]
pub mod ch40_task_id;
#[doc = "CH41_EVT_ID (rw) register accessor: channel41 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch41_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch41_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch41_evt_id`] module"]
pub type CH41_EVT_ID = crate::Reg<ch41_evt_id::CH41_EVT_ID_SPEC>;
#[doc = "channel41 event id register"]
pub mod ch41_evt_id;
#[doc = "CH41_TASK_ID (rw) register accessor: channel41 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch41_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch41_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch41_task_id`] module"]
pub type CH41_TASK_ID = crate::Reg<ch41_task_id::CH41_TASK_ID_SPEC>;
#[doc = "channel41 task id register"]
pub mod ch41_task_id;
#[doc = "CH42_EVT_ID (rw) register accessor: channel42 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch42_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch42_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch42_evt_id`] module"]
pub type CH42_EVT_ID = crate::Reg<ch42_evt_id::CH42_EVT_ID_SPEC>;
#[doc = "channel42 event id register"]
pub mod ch42_evt_id;
#[doc = "CH42_TASK_ID (rw) register accessor: channel42 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch42_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch42_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch42_task_id`] module"]
pub type CH42_TASK_ID = crate::Reg<ch42_task_id::CH42_TASK_ID_SPEC>;
#[doc = "channel42 task id register"]
pub mod ch42_task_id;
#[doc = "CH43_EVT_ID (rw) register accessor: channel43 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch43_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch43_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch43_evt_id`] module"]
pub type CH43_EVT_ID = crate::Reg<ch43_evt_id::CH43_EVT_ID_SPEC>;
#[doc = "channel43 event id register"]
pub mod ch43_evt_id;
#[doc = "CH43_TASK_ID (rw) register accessor: channel43 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch43_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch43_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch43_task_id`] module"]
pub type CH43_TASK_ID = crate::Reg<ch43_task_id::CH43_TASK_ID_SPEC>;
#[doc = "channel43 task id register"]
pub mod ch43_task_id;
#[doc = "CH44_EVT_ID (rw) register accessor: channel44 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch44_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch44_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch44_evt_id`] module"]
pub type CH44_EVT_ID = crate::Reg<ch44_evt_id::CH44_EVT_ID_SPEC>;
#[doc = "channel44 event id register"]
pub mod ch44_evt_id;
#[doc = "CH44_TASK_ID (rw) register accessor: channel44 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch44_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch44_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch44_task_id`] module"]
pub type CH44_TASK_ID = crate::Reg<ch44_task_id::CH44_TASK_ID_SPEC>;
#[doc = "channel44 task id register"]
pub mod ch44_task_id;
#[doc = "CH45_EVT_ID (rw) register accessor: channel45 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch45_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch45_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch45_evt_id`] module"]
pub type CH45_EVT_ID = crate::Reg<ch45_evt_id::CH45_EVT_ID_SPEC>;
#[doc = "channel45 event id register"]
pub mod ch45_evt_id;
#[doc = "CH45_TASK_ID (rw) register accessor: channel45 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch45_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch45_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch45_task_id`] module"]
pub type CH45_TASK_ID = crate::Reg<ch45_task_id::CH45_TASK_ID_SPEC>;
#[doc = "channel45 task id register"]
pub mod ch45_task_id;
#[doc = "CH46_EVT_ID (rw) register accessor: channel46 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch46_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch46_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch46_evt_id`] module"]
pub type CH46_EVT_ID = crate::Reg<ch46_evt_id::CH46_EVT_ID_SPEC>;
#[doc = "channel46 event id register"]
pub mod ch46_evt_id;
#[doc = "CH46_TASK_ID (rw) register accessor: channel46 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch46_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch46_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch46_task_id`] module"]
pub type CH46_TASK_ID = crate::Reg<ch46_task_id::CH46_TASK_ID_SPEC>;
#[doc = "channel46 task id register"]
pub mod ch46_task_id;
#[doc = "CH47_EVT_ID (rw) register accessor: channel47 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch47_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch47_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch47_evt_id`] module"]
pub type CH47_EVT_ID = crate::Reg<ch47_evt_id::CH47_EVT_ID_SPEC>;
#[doc = "channel47 event id register"]
pub mod ch47_evt_id;
#[doc = "CH47_TASK_ID (rw) register accessor: channel47 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch47_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch47_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch47_task_id`] module"]
pub type CH47_TASK_ID = crate::Reg<ch47_task_id::CH47_TASK_ID_SPEC>;
#[doc = "channel47 task id register"]
pub mod ch47_task_id;
#[doc = "CH48_EVT_ID (rw) register accessor: channel48 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch48_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch48_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch48_evt_id`] module"]
pub type CH48_EVT_ID = crate::Reg<ch48_evt_id::CH48_EVT_ID_SPEC>;
#[doc = "channel48 event id register"]
pub mod ch48_evt_id;
#[doc = "CH48_TASK_ID (rw) register accessor: channel48 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch48_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch48_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch48_task_id`] module"]
pub type CH48_TASK_ID = crate::Reg<ch48_task_id::CH48_TASK_ID_SPEC>;
#[doc = "channel48 task id register"]
pub mod ch48_task_id;
#[doc = "CH49_EVT_ID (rw) register accessor: channel49 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch49_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch49_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch49_evt_id`] module"]
pub type CH49_EVT_ID = crate::Reg<ch49_evt_id::CH49_EVT_ID_SPEC>;
#[doc = "channel49 event id register"]
pub mod ch49_evt_id;
#[doc = "CH49_TASK_ID (rw) register accessor: channel49 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch49_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch49_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch49_task_id`] module"]
pub type CH49_TASK_ID = crate::Reg<ch49_task_id::CH49_TASK_ID_SPEC>;
#[doc = "channel49 task id register"]
pub mod ch49_task_id;
#[doc = "CLK_EN (rw) register accessor: etm clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "etm clock enable register"]
pub mod clk_en;
#[doc = "DATE (rw) register accessor: etm date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "etm date register"]
pub mod date;
