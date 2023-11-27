#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    ch_ena_ad0: CH_ENA_AD0,
    ch_ena_ad0_set: CH_ENA_AD0_SET,
    ch_ena_ad0_clr: CH_ENA_AD0_CLR,
    ch_ena_ad1: CH_ENA_AD1,
    ch_ena_ad1_set: CH_ENA_AD1_SET,
    ch_ena_ad1_clr: CH_ENA_AD1_CLR,
    ch0_evt_id: CH0_EVT_ID,
    ch0_task_id: CH0_TASK_ID,
    ch1_evt_id: CH1_EVT_ID,
    ch1_task_id: CH1_TASK_ID,
    ch2_evt_id: CH2_EVT_ID,
    ch2_task_id: CH2_TASK_ID,
    ch3_evt_id: CH3_EVT_ID,
    ch3_task_id: CH3_TASK_ID,
    ch4_evt_id: CH4_EVT_ID,
    ch4_task_id: CH4_TASK_ID,
    ch5_evt_id: CH5_EVT_ID,
    ch5_task_id: CH5_TASK_ID,
    ch6_evt_id: CH6_EVT_ID,
    ch6_task_id: CH6_TASK_ID,
    ch7_evt_id: CH7_EVT_ID,
    ch7_task_id: CH7_TASK_ID,
    ch8_evt_id: CH8_EVT_ID,
    ch8_task_id: CH8_TASK_ID,
    ch9_evt_id: CH9_EVT_ID,
    ch9_task_id: CH9_TASK_ID,
    ch10_evt_id: CH10_EVT_ID,
    ch10_task_id: CH10_TASK_ID,
    ch11_evt_id: CH11_EVT_ID,
    ch11_task_id: CH11_TASK_ID,
    ch12_evt_id: CH12_EVT_ID,
    ch12_task_id: CH12_TASK_ID,
    ch13_evt_id: CH13_EVT_ID,
    ch13_task_id: CH13_TASK_ID,
    ch14_evt_id: CH14_EVT_ID,
    ch14_task_id: CH14_TASK_ID,
    ch15_evt_id: CH15_EVT_ID,
    ch15_task_id: CH15_TASK_ID,
    ch16_evt_id: CH16_EVT_ID,
    ch16_task_id: CH16_TASK_ID,
    ch17_evt_id: CH17_EVT_ID,
    ch17_task_id: CH17_TASK_ID,
    ch18_evt_id: CH18_EVT_ID,
    ch18_task_id: CH18_TASK_ID,
    ch19_evt_id: CH19_EVT_ID,
    ch19_task_id: CH19_TASK_ID,
    ch20_evt_id: CH20_EVT_ID,
    ch20_task_id: CH20_TASK_ID,
    ch21_evt_id: CH21_EVT_ID,
    ch21_task_id: CH21_TASK_ID,
    ch22_evt_id: CH22_EVT_ID,
    ch22_task_id: CH22_TASK_ID,
    ch23_evt_id: CH23_EVT_ID,
    ch23_task_id: CH23_TASK_ID,
    ch24_evt_id: CH24_EVT_ID,
    ch24_task_id: CH24_TASK_ID,
    ch25_evt_id: CH25_EVT_ID,
    ch25_task_id: CH25_TASK_ID,
    ch26_evt_id: CH26_EVT_ID,
    ch26_task_id: CH26_TASK_ID,
    ch27_evt_id: CH27_EVT_ID,
    ch27_task_id: CH27_TASK_ID,
    ch28_evt_id: CH28_EVT_ID,
    ch28_task_id: CH28_TASK_ID,
    ch29_evt_id: CH29_EVT_ID,
    ch29_task_id: CH29_TASK_ID,
    ch30_evt_id: CH30_EVT_ID,
    ch30_task_id: CH30_TASK_ID,
    ch31_evt_id: CH31_EVT_ID,
    ch31_task_id: CH31_TASK_ID,
    ch32_evt_id: CH32_EVT_ID,
    ch32_task_id: CH32_TASK_ID,
    ch33_evt_id: CH33_EVT_ID,
    ch33_task_id: CH33_TASK_ID,
    ch34_evt_id: CH34_EVT_ID,
    ch34_task_id: CH34_TASK_ID,
    ch35_evt_id: CH35_EVT_ID,
    ch35_task_id: CH35_TASK_ID,
    ch36_evt_id: CH36_EVT_ID,
    ch36_task_id: CH36_TASK_ID,
    ch37_evt_id: CH37_EVT_ID,
    ch37_task_id: CH37_TASK_ID,
    ch38_evt_id: CH38_EVT_ID,
    ch38_task_id: CH38_TASK_ID,
    ch39_evt_id: CH39_EVT_ID,
    ch39_task_id: CH39_TASK_ID,
    ch40_evt_id: CH40_EVT_ID,
    ch40_task_id: CH40_TASK_ID,
    ch41_evt_id: CH41_EVT_ID,
    ch41_task_id: CH41_TASK_ID,
    ch42_evt_id: CH42_EVT_ID,
    ch42_task_id: CH42_TASK_ID,
    ch43_evt_id: CH43_EVT_ID,
    ch43_task_id: CH43_TASK_ID,
    ch44_evt_id: CH44_EVT_ID,
    ch44_task_id: CH44_TASK_ID,
    ch45_evt_id: CH45_EVT_ID,
    ch45_task_id: CH45_TASK_ID,
    ch46_evt_id: CH46_EVT_ID,
    ch46_task_id: CH46_TASK_ID,
    ch47_evt_id: CH47_EVT_ID,
    ch47_task_id: CH47_TASK_ID,
    ch48_evt_id: CH48_EVT_ID,
    ch48_task_id: CH48_TASK_ID,
    ch49_evt_id: CH49_EVT_ID,
    ch49_task_id: CH49_TASK_ID,
    evt_st0: EVT_ST0,
    evt_st0_clr: EVT_ST0_CLR,
    evt_st1: EVT_ST1,
    evt_st1_clr: EVT_ST1_CLR,
    evt_st2: EVT_ST2,
    evt_st2_clr: EVT_ST2_CLR,
    evt_st3: EVT_ST3,
    evt_st3_clr: EVT_ST3_CLR,
    evt_st4: EVT_ST4,
    evt_st4_clr: EVT_ST4_CLR,
    evt_st5: EVT_ST5,
    evt_st5_clr: EVT_ST5_CLR,
    evt_st6: EVT_ST6,
    evt_st6_clr: EVT_ST6_CLR,
    evt_st7: EVT_ST7,
    evt_st7_clr: EVT_ST7_CLR,
    task_st0: TASK_ST0,
    task_st0_clr: TASK_ST0_CLR,
    task_st1: TASK_ST1,
    task_st1_clr: TASK_ST1_CLR,
    task_st2: TASK_ST2,
    task_st2_clr: TASK_ST2_CLR,
    task_st3: TASK_ST3,
    task_st3_clr: TASK_ST3_CLR,
    task_st4: TASK_ST4,
    task_st4_clr: TASK_ST4_CLR,
    task_st5: TASK_ST5,
    task_st5_clr: TASK_ST5_CLR,
    task_st6: TASK_ST6,
    task_st6_clr: TASK_ST6_CLR,
    clk_en: CLK_EN,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel enable status register"]
    #[inline(always)]
    pub const fn ch_ena_ad0(&self) -> &CH_ENA_AD0 {
        &self.ch_ena_ad0
    }
    #[doc = "0x04 - Channel enable set register"]
    #[inline(always)]
    pub const fn ch_ena_ad0_set(&self) -> &CH_ENA_AD0_SET {
        &self.ch_ena_ad0_set
    }
    #[doc = "0x08 - Channel enable clear register"]
    #[inline(always)]
    pub const fn ch_ena_ad0_clr(&self) -> &CH_ENA_AD0_CLR {
        &self.ch_ena_ad0_clr
    }
    #[doc = "0x0c - Channel enable status register"]
    #[inline(always)]
    pub const fn ch_ena_ad1(&self) -> &CH_ENA_AD1 {
        &self.ch_ena_ad1
    }
    #[doc = "0x10 - Channel enable set register"]
    #[inline(always)]
    pub const fn ch_ena_ad1_set(&self) -> &CH_ENA_AD1_SET {
        &self.ch_ena_ad1_set
    }
    #[doc = "0x14 - Channel enable clear register"]
    #[inline(always)]
    pub const fn ch_ena_ad1_clr(&self) -> &CH_ENA_AD1_CLR {
        &self.ch_ena_ad1_clr
    }
    #[doc = "0x18 - Channel0 event id register"]
    #[inline(always)]
    pub const fn ch0_evt_id(&self) -> &CH0_EVT_ID {
        &self.ch0_evt_id
    }
    #[doc = "0x1c - Channel0 task id register"]
    #[inline(always)]
    pub const fn ch0_task_id(&self) -> &CH0_TASK_ID {
        &self.ch0_task_id
    }
    #[doc = "0x20 - Channel1 event id register"]
    #[inline(always)]
    pub const fn ch1_evt_id(&self) -> &CH1_EVT_ID {
        &self.ch1_evt_id
    }
    #[doc = "0x24 - Channel1 task id register"]
    #[inline(always)]
    pub const fn ch1_task_id(&self) -> &CH1_TASK_ID {
        &self.ch1_task_id
    }
    #[doc = "0x28 - Channel2 event id register"]
    #[inline(always)]
    pub const fn ch2_evt_id(&self) -> &CH2_EVT_ID {
        &self.ch2_evt_id
    }
    #[doc = "0x2c - Channel2 task id register"]
    #[inline(always)]
    pub const fn ch2_task_id(&self) -> &CH2_TASK_ID {
        &self.ch2_task_id
    }
    #[doc = "0x30 - Channel3 event id register"]
    #[inline(always)]
    pub const fn ch3_evt_id(&self) -> &CH3_EVT_ID {
        &self.ch3_evt_id
    }
    #[doc = "0x34 - Channel3 task id register"]
    #[inline(always)]
    pub const fn ch3_task_id(&self) -> &CH3_TASK_ID {
        &self.ch3_task_id
    }
    #[doc = "0x38 - Channel4 event id register"]
    #[inline(always)]
    pub const fn ch4_evt_id(&self) -> &CH4_EVT_ID {
        &self.ch4_evt_id
    }
    #[doc = "0x3c - Channel4 task id register"]
    #[inline(always)]
    pub const fn ch4_task_id(&self) -> &CH4_TASK_ID {
        &self.ch4_task_id
    }
    #[doc = "0x40 - Channel5 event id register"]
    #[inline(always)]
    pub const fn ch5_evt_id(&self) -> &CH5_EVT_ID {
        &self.ch5_evt_id
    }
    #[doc = "0x44 - Channel5 task id register"]
    #[inline(always)]
    pub const fn ch5_task_id(&self) -> &CH5_TASK_ID {
        &self.ch5_task_id
    }
    #[doc = "0x48 - Channel6 event id register"]
    #[inline(always)]
    pub const fn ch6_evt_id(&self) -> &CH6_EVT_ID {
        &self.ch6_evt_id
    }
    #[doc = "0x4c - Channel6 task id register"]
    #[inline(always)]
    pub const fn ch6_task_id(&self) -> &CH6_TASK_ID {
        &self.ch6_task_id
    }
    #[doc = "0x50 - Channel7 event id register"]
    #[inline(always)]
    pub const fn ch7_evt_id(&self) -> &CH7_EVT_ID {
        &self.ch7_evt_id
    }
    #[doc = "0x54 - Channel7 task id register"]
    #[inline(always)]
    pub const fn ch7_task_id(&self) -> &CH7_TASK_ID {
        &self.ch7_task_id
    }
    #[doc = "0x58 - Channel8 event id register"]
    #[inline(always)]
    pub const fn ch8_evt_id(&self) -> &CH8_EVT_ID {
        &self.ch8_evt_id
    }
    #[doc = "0x5c - Channel8 task id register"]
    #[inline(always)]
    pub const fn ch8_task_id(&self) -> &CH8_TASK_ID {
        &self.ch8_task_id
    }
    #[doc = "0x60 - Channel9 event id register"]
    #[inline(always)]
    pub const fn ch9_evt_id(&self) -> &CH9_EVT_ID {
        &self.ch9_evt_id
    }
    #[doc = "0x64 - Channel9 task id register"]
    #[inline(always)]
    pub const fn ch9_task_id(&self) -> &CH9_TASK_ID {
        &self.ch9_task_id
    }
    #[doc = "0x68 - Channel10 event id register"]
    #[inline(always)]
    pub const fn ch10_evt_id(&self) -> &CH10_EVT_ID {
        &self.ch10_evt_id
    }
    #[doc = "0x6c - Channel10 task id register"]
    #[inline(always)]
    pub const fn ch10_task_id(&self) -> &CH10_TASK_ID {
        &self.ch10_task_id
    }
    #[doc = "0x70 - Channel11 event id register"]
    #[inline(always)]
    pub const fn ch11_evt_id(&self) -> &CH11_EVT_ID {
        &self.ch11_evt_id
    }
    #[doc = "0x74 - Channel11 task id register"]
    #[inline(always)]
    pub const fn ch11_task_id(&self) -> &CH11_TASK_ID {
        &self.ch11_task_id
    }
    #[doc = "0x78 - Channel12 event id register"]
    #[inline(always)]
    pub const fn ch12_evt_id(&self) -> &CH12_EVT_ID {
        &self.ch12_evt_id
    }
    #[doc = "0x7c - Channel12 task id register"]
    #[inline(always)]
    pub const fn ch12_task_id(&self) -> &CH12_TASK_ID {
        &self.ch12_task_id
    }
    #[doc = "0x80 - Channel13 event id register"]
    #[inline(always)]
    pub const fn ch13_evt_id(&self) -> &CH13_EVT_ID {
        &self.ch13_evt_id
    }
    #[doc = "0x84 - Channel13 task id register"]
    #[inline(always)]
    pub const fn ch13_task_id(&self) -> &CH13_TASK_ID {
        &self.ch13_task_id
    }
    #[doc = "0x88 - Channel14 event id register"]
    #[inline(always)]
    pub const fn ch14_evt_id(&self) -> &CH14_EVT_ID {
        &self.ch14_evt_id
    }
    #[doc = "0x8c - Channel14 task id register"]
    #[inline(always)]
    pub const fn ch14_task_id(&self) -> &CH14_TASK_ID {
        &self.ch14_task_id
    }
    #[doc = "0x90 - Channel15 event id register"]
    #[inline(always)]
    pub const fn ch15_evt_id(&self) -> &CH15_EVT_ID {
        &self.ch15_evt_id
    }
    #[doc = "0x94 - Channel15 task id register"]
    #[inline(always)]
    pub const fn ch15_task_id(&self) -> &CH15_TASK_ID {
        &self.ch15_task_id
    }
    #[doc = "0x98 - Channel16 event id register"]
    #[inline(always)]
    pub const fn ch16_evt_id(&self) -> &CH16_EVT_ID {
        &self.ch16_evt_id
    }
    #[doc = "0x9c - Channel16 task id register"]
    #[inline(always)]
    pub const fn ch16_task_id(&self) -> &CH16_TASK_ID {
        &self.ch16_task_id
    }
    #[doc = "0xa0 - Channel17 event id register"]
    #[inline(always)]
    pub const fn ch17_evt_id(&self) -> &CH17_EVT_ID {
        &self.ch17_evt_id
    }
    #[doc = "0xa4 - Channel17 task id register"]
    #[inline(always)]
    pub const fn ch17_task_id(&self) -> &CH17_TASK_ID {
        &self.ch17_task_id
    }
    #[doc = "0xa8 - Channel18 event id register"]
    #[inline(always)]
    pub const fn ch18_evt_id(&self) -> &CH18_EVT_ID {
        &self.ch18_evt_id
    }
    #[doc = "0xac - Channel18 task id register"]
    #[inline(always)]
    pub const fn ch18_task_id(&self) -> &CH18_TASK_ID {
        &self.ch18_task_id
    }
    #[doc = "0xb0 - Channel19 event id register"]
    #[inline(always)]
    pub const fn ch19_evt_id(&self) -> &CH19_EVT_ID {
        &self.ch19_evt_id
    }
    #[doc = "0xb4 - Channel19 task id register"]
    #[inline(always)]
    pub const fn ch19_task_id(&self) -> &CH19_TASK_ID {
        &self.ch19_task_id
    }
    #[doc = "0xb8 - Channel20 event id register"]
    #[inline(always)]
    pub const fn ch20_evt_id(&self) -> &CH20_EVT_ID {
        &self.ch20_evt_id
    }
    #[doc = "0xbc - Channel20 task id register"]
    #[inline(always)]
    pub const fn ch20_task_id(&self) -> &CH20_TASK_ID {
        &self.ch20_task_id
    }
    #[doc = "0xc0 - Channel21 event id register"]
    #[inline(always)]
    pub const fn ch21_evt_id(&self) -> &CH21_EVT_ID {
        &self.ch21_evt_id
    }
    #[doc = "0xc4 - Channel21 task id register"]
    #[inline(always)]
    pub const fn ch21_task_id(&self) -> &CH21_TASK_ID {
        &self.ch21_task_id
    }
    #[doc = "0xc8 - Channel22 event id register"]
    #[inline(always)]
    pub const fn ch22_evt_id(&self) -> &CH22_EVT_ID {
        &self.ch22_evt_id
    }
    #[doc = "0xcc - Channel22 task id register"]
    #[inline(always)]
    pub const fn ch22_task_id(&self) -> &CH22_TASK_ID {
        &self.ch22_task_id
    }
    #[doc = "0xd0 - Channel23 event id register"]
    #[inline(always)]
    pub const fn ch23_evt_id(&self) -> &CH23_EVT_ID {
        &self.ch23_evt_id
    }
    #[doc = "0xd4 - Channel23 task id register"]
    #[inline(always)]
    pub const fn ch23_task_id(&self) -> &CH23_TASK_ID {
        &self.ch23_task_id
    }
    #[doc = "0xd8 - Channel24 event id register"]
    #[inline(always)]
    pub const fn ch24_evt_id(&self) -> &CH24_EVT_ID {
        &self.ch24_evt_id
    }
    #[doc = "0xdc - Channel24 task id register"]
    #[inline(always)]
    pub const fn ch24_task_id(&self) -> &CH24_TASK_ID {
        &self.ch24_task_id
    }
    #[doc = "0xe0 - Channel25 event id register"]
    #[inline(always)]
    pub const fn ch25_evt_id(&self) -> &CH25_EVT_ID {
        &self.ch25_evt_id
    }
    #[doc = "0xe4 - Channel25 task id register"]
    #[inline(always)]
    pub const fn ch25_task_id(&self) -> &CH25_TASK_ID {
        &self.ch25_task_id
    }
    #[doc = "0xe8 - Channel26 event id register"]
    #[inline(always)]
    pub const fn ch26_evt_id(&self) -> &CH26_EVT_ID {
        &self.ch26_evt_id
    }
    #[doc = "0xec - Channel26 task id register"]
    #[inline(always)]
    pub const fn ch26_task_id(&self) -> &CH26_TASK_ID {
        &self.ch26_task_id
    }
    #[doc = "0xf0 - Channel27 event id register"]
    #[inline(always)]
    pub const fn ch27_evt_id(&self) -> &CH27_EVT_ID {
        &self.ch27_evt_id
    }
    #[doc = "0xf4 - Channel27 task id register"]
    #[inline(always)]
    pub const fn ch27_task_id(&self) -> &CH27_TASK_ID {
        &self.ch27_task_id
    }
    #[doc = "0xf8 - Channel28 event id register"]
    #[inline(always)]
    pub const fn ch28_evt_id(&self) -> &CH28_EVT_ID {
        &self.ch28_evt_id
    }
    #[doc = "0xfc - Channel28 task id register"]
    #[inline(always)]
    pub const fn ch28_task_id(&self) -> &CH28_TASK_ID {
        &self.ch28_task_id
    }
    #[doc = "0x100 - Channel29 event id register"]
    #[inline(always)]
    pub const fn ch29_evt_id(&self) -> &CH29_EVT_ID {
        &self.ch29_evt_id
    }
    #[doc = "0x104 - Channel29 task id register"]
    #[inline(always)]
    pub const fn ch29_task_id(&self) -> &CH29_TASK_ID {
        &self.ch29_task_id
    }
    #[doc = "0x108 - Channel30 event id register"]
    #[inline(always)]
    pub const fn ch30_evt_id(&self) -> &CH30_EVT_ID {
        &self.ch30_evt_id
    }
    #[doc = "0x10c - Channel30 task id register"]
    #[inline(always)]
    pub const fn ch30_task_id(&self) -> &CH30_TASK_ID {
        &self.ch30_task_id
    }
    #[doc = "0x110 - Channel31 event id register"]
    #[inline(always)]
    pub const fn ch31_evt_id(&self) -> &CH31_EVT_ID {
        &self.ch31_evt_id
    }
    #[doc = "0x114 - Channel31 task id register"]
    #[inline(always)]
    pub const fn ch31_task_id(&self) -> &CH31_TASK_ID {
        &self.ch31_task_id
    }
    #[doc = "0x118 - Channel32 event id register"]
    #[inline(always)]
    pub const fn ch32_evt_id(&self) -> &CH32_EVT_ID {
        &self.ch32_evt_id
    }
    #[doc = "0x11c - Channel32 task id register"]
    #[inline(always)]
    pub const fn ch32_task_id(&self) -> &CH32_TASK_ID {
        &self.ch32_task_id
    }
    #[doc = "0x120 - Channel33 event id register"]
    #[inline(always)]
    pub const fn ch33_evt_id(&self) -> &CH33_EVT_ID {
        &self.ch33_evt_id
    }
    #[doc = "0x124 - Channel33 task id register"]
    #[inline(always)]
    pub const fn ch33_task_id(&self) -> &CH33_TASK_ID {
        &self.ch33_task_id
    }
    #[doc = "0x128 - Channel34 event id register"]
    #[inline(always)]
    pub const fn ch34_evt_id(&self) -> &CH34_EVT_ID {
        &self.ch34_evt_id
    }
    #[doc = "0x12c - Channel34 task id register"]
    #[inline(always)]
    pub const fn ch34_task_id(&self) -> &CH34_TASK_ID {
        &self.ch34_task_id
    }
    #[doc = "0x130 - Channel35 event id register"]
    #[inline(always)]
    pub const fn ch35_evt_id(&self) -> &CH35_EVT_ID {
        &self.ch35_evt_id
    }
    #[doc = "0x134 - Channel35 task id register"]
    #[inline(always)]
    pub const fn ch35_task_id(&self) -> &CH35_TASK_ID {
        &self.ch35_task_id
    }
    #[doc = "0x138 - Channel36 event id register"]
    #[inline(always)]
    pub const fn ch36_evt_id(&self) -> &CH36_EVT_ID {
        &self.ch36_evt_id
    }
    #[doc = "0x13c - Channel36 task id register"]
    #[inline(always)]
    pub const fn ch36_task_id(&self) -> &CH36_TASK_ID {
        &self.ch36_task_id
    }
    #[doc = "0x140 - Channel37 event id register"]
    #[inline(always)]
    pub const fn ch37_evt_id(&self) -> &CH37_EVT_ID {
        &self.ch37_evt_id
    }
    #[doc = "0x144 - Channel37 task id register"]
    #[inline(always)]
    pub const fn ch37_task_id(&self) -> &CH37_TASK_ID {
        &self.ch37_task_id
    }
    #[doc = "0x148 - Channel38 event id register"]
    #[inline(always)]
    pub const fn ch38_evt_id(&self) -> &CH38_EVT_ID {
        &self.ch38_evt_id
    }
    #[doc = "0x14c - Channel38 task id register"]
    #[inline(always)]
    pub const fn ch38_task_id(&self) -> &CH38_TASK_ID {
        &self.ch38_task_id
    }
    #[doc = "0x150 - Channel39 event id register"]
    #[inline(always)]
    pub const fn ch39_evt_id(&self) -> &CH39_EVT_ID {
        &self.ch39_evt_id
    }
    #[doc = "0x154 - Channel39 task id register"]
    #[inline(always)]
    pub const fn ch39_task_id(&self) -> &CH39_TASK_ID {
        &self.ch39_task_id
    }
    #[doc = "0x158 - Channel40 event id register"]
    #[inline(always)]
    pub const fn ch40_evt_id(&self) -> &CH40_EVT_ID {
        &self.ch40_evt_id
    }
    #[doc = "0x15c - Channel40 task id register"]
    #[inline(always)]
    pub const fn ch40_task_id(&self) -> &CH40_TASK_ID {
        &self.ch40_task_id
    }
    #[doc = "0x160 - Channel41 event id register"]
    #[inline(always)]
    pub const fn ch41_evt_id(&self) -> &CH41_EVT_ID {
        &self.ch41_evt_id
    }
    #[doc = "0x164 - Channel41 task id register"]
    #[inline(always)]
    pub const fn ch41_task_id(&self) -> &CH41_TASK_ID {
        &self.ch41_task_id
    }
    #[doc = "0x168 - Channel42 event id register"]
    #[inline(always)]
    pub const fn ch42_evt_id(&self) -> &CH42_EVT_ID {
        &self.ch42_evt_id
    }
    #[doc = "0x16c - Channel42 task id register"]
    #[inline(always)]
    pub const fn ch42_task_id(&self) -> &CH42_TASK_ID {
        &self.ch42_task_id
    }
    #[doc = "0x170 - Channel43 event id register"]
    #[inline(always)]
    pub const fn ch43_evt_id(&self) -> &CH43_EVT_ID {
        &self.ch43_evt_id
    }
    #[doc = "0x174 - Channel43 task id register"]
    #[inline(always)]
    pub const fn ch43_task_id(&self) -> &CH43_TASK_ID {
        &self.ch43_task_id
    }
    #[doc = "0x178 - Channel44 event id register"]
    #[inline(always)]
    pub const fn ch44_evt_id(&self) -> &CH44_EVT_ID {
        &self.ch44_evt_id
    }
    #[doc = "0x17c - Channel44 task id register"]
    #[inline(always)]
    pub const fn ch44_task_id(&self) -> &CH44_TASK_ID {
        &self.ch44_task_id
    }
    #[doc = "0x180 - Channel45 event id register"]
    #[inline(always)]
    pub const fn ch45_evt_id(&self) -> &CH45_EVT_ID {
        &self.ch45_evt_id
    }
    #[doc = "0x184 - Channel45 task id register"]
    #[inline(always)]
    pub const fn ch45_task_id(&self) -> &CH45_TASK_ID {
        &self.ch45_task_id
    }
    #[doc = "0x188 - Channel46 event id register"]
    #[inline(always)]
    pub const fn ch46_evt_id(&self) -> &CH46_EVT_ID {
        &self.ch46_evt_id
    }
    #[doc = "0x18c - Channel46 task id register"]
    #[inline(always)]
    pub const fn ch46_task_id(&self) -> &CH46_TASK_ID {
        &self.ch46_task_id
    }
    #[doc = "0x190 - Channel47 event id register"]
    #[inline(always)]
    pub const fn ch47_evt_id(&self) -> &CH47_EVT_ID {
        &self.ch47_evt_id
    }
    #[doc = "0x194 - Channel47 task id register"]
    #[inline(always)]
    pub const fn ch47_task_id(&self) -> &CH47_TASK_ID {
        &self.ch47_task_id
    }
    #[doc = "0x198 - Channel48 event id register"]
    #[inline(always)]
    pub const fn ch48_evt_id(&self) -> &CH48_EVT_ID {
        &self.ch48_evt_id
    }
    #[doc = "0x19c - Channel48 task id register"]
    #[inline(always)]
    pub const fn ch48_task_id(&self) -> &CH48_TASK_ID {
        &self.ch48_task_id
    }
    #[doc = "0x1a0 - Channel49 event id register"]
    #[inline(always)]
    pub const fn ch49_evt_id(&self) -> &CH49_EVT_ID {
        &self.ch49_evt_id
    }
    #[doc = "0x1a4 - Channel49 task id register"]
    #[inline(always)]
    pub const fn ch49_task_id(&self) -> &CH49_TASK_ID {
        &self.ch49_task_id
    }
    #[doc = "0x1a8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st0(&self) -> &EVT_ST0 {
        &self.evt_st0
    }
    #[doc = "0x1ac - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st0_clr(&self) -> &EVT_ST0_CLR {
        &self.evt_st0_clr
    }
    #[doc = "0x1b0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st1(&self) -> &EVT_ST1 {
        &self.evt_st1
    }
    #[doc = "0x1b4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st1_clr(&self) -> &EVT_ST1_CLR {
        &self.evt_st1_clr
    }
    #[doc = "0x1b8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st2(&self) -> &EVT_ST2 {
        &self.evt_st2
    }
    #[doc = "0x1bc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st2_clr(&self) -> &EVT_ST2_CLR {
        &self.evt_st2_clr
    }
    #[doc = "0x1c0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st3(&self) -> &EVT_ST3 {
        &self.evt_st3
    }
    #[doc = "0x1c4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st3_clr(&self) -> &EVT_ST3_CLR {
        &self.evt_st3_clr
    }
    #[doc = "0x1c8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st4(&self) -> &EVT_ST4 {
        &self.evt_st4
    }
    #[doc = "0x1cc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st4_clr(&self) -> &EVT_ST4_CLR {
        &self.evt_st4_clr
    }
    #[doc = "0x1d0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st5(&self) -> &EVT_ST5 {
        &self.evt_st5
    }
    #[doc = "0x1d4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st5_clr(&self) -> &EVT_ST5_CLR {
        &self.evt_st5_clr
    }
    #[doc = "0x1d8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st6(&self) -> &EVT_ST6 {
        &self.evt_st6
    }
    #[doc = "0x1dc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st6_clr(&self) -> &EVT_ST6_CLR {
        &self.evt_st6_clr
    }
    #[doc = "0x1e0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st7(&self) -> &EVT_ST7 {
        &self.evt_st7
    }
    #[doc = "0x1e4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st7_clr(&self) -> &EVT_ST7_CLR {
        &self.evt_st7_clr
    }
    #[doc = "0x1e8 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st0(&self) -> &TASK_ST0 {
        &self.task_st0
    }
    #[doc = "0x1ec - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st0_clr(&self) -> &TASK_ST0_CLR {
        &self.task_st0_clr
    }
    #[doc = "0x1f0 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st1(&self) -> &TASK_ST1 {
        &self.task_st1
    }
    #[doc = "0x1f4 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st1_clr(&self) -> &TASK_ST1_CLR {
        &self.task_st1_clr
    }
    #[doc = "0x1f8 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st2(&self) -> &TASK_ST2 {
        &self.task_st2
    }
    #[doc = "0x1fc - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st2_clr(&self) -> &TASK_ST2_CLR {
        &self.task_st2_clr
    }
    #[doc = "0x200 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st3(&self) -> &TASK_ST3 {
        &self.task_st3
    }
    #[doc = "0x204 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st3_clr(&self) -> &TASK_ST3_CLR {
        &self.task_st3_clr
    }
    #[doc = "0x208 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st4(&self) -> &TASK_ST4 {
        &self.task_st4
    }
    #[doc = "0x20c - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st4_clr(&self) -> &TASK_ST4_CLR {
        &self.task_st4_clr
    }
    #[doc = "0x210 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st5(&self) -> &TASK_ST5 {
        &self.task_st5
    }
    #[doc = "0x214 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st5_clr(&self) -> &TASK_ST5_CLR {
        &self.task_st5_clr
    }
    #[doc = "0x218 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st6(&self) -> &TASK_ST6 {
        &self.task_st6
    }
    #[doc = "0x21c - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st6_clr(&self) -> &TASK_ST6_CLR {
        &self.task_st6_clr
    }
    #[doc = "0x220 - ETM clock enable register"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x224 - ETM date register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CH_ENA_AD0 (rw) register accessor: Channel enable status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0`] module"]
pub type CH_ENA_AD0 = crate::Reg<ch_ena_ad0::CH_ENA_AD0_SPEC>;
#[doc = "Channel enable status register"]
pub mod ch_ena_ad0;
#[doc = "CH_ENA_AD0_SET (w) register accessor: Channel enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0_set`] module"]
pub type CH_ENA_AD0_SET = crate::Reg<ch_ena_ad0_set::CH_ENA_AD0_SET_SPEC>;
#[doc = "Channel enable set register"]
pub mod ch_ena_ad0_set;
#[doc = "CH_ENA_AD0_CLR (w) register accessor: Channel enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0_clr`] module"]
pub type CH_ENA_AD0_CLR = crate::Reg<ch_ena_ad0_clr::CH_ENA_AD0_CLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod ch_ena_ad0_clr;
#[doc = "CH_ENA_AD1 (rw) register accessor: Channel enable status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1`] module"]
pub type CH_ENA_AD1 = crate::Reg<ch_ena_ad1::CH_ENA_AD1_SPEC>;
#[doc = "Channel enable status register"]
pub mod ch_ena_ad1;
#[doc = "CH_ENA_AD1_SET (w) register accessor: Channel enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1_set`] module"]
pub type CH_ENA_AD1_SET = crate::Reg<ch_ena_ad1_set::CH_ENA_AD1_SET_SPEC>;
#[doc = "Channel enable set register"]
pub mod ch_ena_ad1_set;
#[doc = "CH_ENA_AD1_CLR (w) register accessor: Channel enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1_clr`] module"]
pub type CH_ENA_AD1_CLR = crate::Reg<ch_ena_ad1_clr::CH_ENA_AD1_CLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod ch_ena_ad1_clr;
#[doc = "CH0_EVT_ID (rw) register accessor: Channel0 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_evt_id`] module"]
pub type CH0_EVT_ID = crate::Reg<ch0_evt_id::CH0_EVT_ID_SPEC>;
#[doc = "Channel0 event id register"]
pub mod ch0_evt_id;
#[doc = "CH0_TASK_ID (rw) register accessor: Channel0 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_task_id`] module"]
pub type CH0_TASK_ID = crate::Reg<ch0_task_id::CH0_TASK_ID_SPEC>;
#[doc = "Channel0 task id register"]
pub mod ch0_task_id;
#[doc = "CH1_EVT_ID (rw) register accessor: Channel1 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_evt_id`] module"]
pub type CH1_EVT_ID = crate::Reg<ch1_evt_id::CH1_EVT_ID_SPEC>;
#[doc = "Channel1 event id register"]
pub mod ch1_evt_id;
#[doc = "CH1_TASK_ID (rw) register accessor: Channel1 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_task_id`] module"]
pub type CH1_TASK_ID = crate::Reg<ch1_task_id::CH1_TASK_ID_SPEC>;
#[doc = "Channel1 task id register"]
pub mod ch1_task_id;
#[doc = "CH2_EVT_ID (rw) register accessor: Channel2 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_evt_id`] module"]
pub type CH2_EVT_ID = crate::Reg<ch2_evt_id::CH2_EVT_ID_SPEC>;
#[doc = "Channel2 event id register"]
pub mod ch2_evt_id;
#[doc = "CH2_TASK_ID (rw) register accessor: Channel2 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_task_id`] module"]
pub type CH2_TASK_ID = crate::Reg<ch2_task_id::CH2_TASK_ID_SPEC>;
#[doc = "Channel2 task id register"]
pub mod ch2_task_id;
#[doc = "CH3_EVT_ID (rw) register accessor: Channel3 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_evt_id`] module"]
pub type CH3_EVT_ID = crate::Reg<ch3_evt_id::CH3_EVT_ID_SPEC>;
#[doc = "Channel3 event id register"]
pub mod ch3_evt_id;
#[doc = "CH3_TASK_ID (rw) register accessor: Channel3 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_task_id`] module"]
pub type CH3_TASK_ID = crate::Reg<ch3_task_id::CH3_TASK_ID_SPEC>;
#[doc = "Channel3 task id register"]
pub mod ch3_task_id;
#[doc = "CH4_EVT_ID (rw) register accessor: Channel4 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_evt_id`] module"]
pub type CH4_EVT_ID = crate::Reg<ch4_evt_id::CH4_EVT_ID_SPEC>;
#[doc = "Channel4 event id register"]
pub mod ch4_evt_id;
#[doc = "CH4_TASK_ID (rw) register accessor: Channel4 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_task_id`] module"]
pub type CH4_TASK_ID = crate::Reg<ch4_task_id::CH4_TASK_ID_SPEC>;
#[doc = "Channel4 task id register"]
pub mod ch4_task_id;
#[doc = "CH5_EVT_ID (rw) register accessor: Channel5 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_evt_id`] module"]
pub type CH5_EVT_ID = crate::Reg<ch5_evt_id::CH5_EVT_ID_SPEC>;
#[doc = "Channel5 event id register"]
pub mod ch5_evt_id;
#[doc = "CH5_TASK_ID (rw) register accessor: Channel5 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_task_id`] module"]
pub type CH5_TASK_ID = crate::Reg<ch5_task_id::CH5_TASK_ID_SPEC>;
#[doc = "Channel5 task id register"]
pub mod ch5_task_id;
#[doc = "CH6_EVT_ID (rw) register accessor: Channel6 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_evt_id`] module"]
pub type CH6_EVT_ID = crate::Reg<ch6_evt_id::CH6_EVT_ID_SPEC>;
#[doc = "Channel6 event id register"]
pub mod ch6_evt_id;
#[doc = "CH6_TASK_ID (rw) register accessor: Channel6 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_task_id`] module"]
pub type CH6_TASK_ID = crate::Reg<ch6_task_id::CH6_TASK_ID_SPEC>;
#[doc = "Channel6 task id register"]
pub mod ch6_task_id;
#[doc = "CH7_EVT_ID (rw) register accessor: Channel7 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_evt_id`] module"]
pub type CH7_EVT_ID = crate::Reg<ch7_evt_id::CH7_EVT_ID_SPEC>;
#[doc = "Channel7 event id register"]
pub mod ch7_evt_id;
#[doc = "CH7_TASK_ID (rw) register accessor: Channel7 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_task_id`] module"]
pub type CH7_TASK_ID = crate::Reg<ch7_task_id::CH7_TASK_ID_SPEC>;
#[doc = "Channel7 task id register"]
pub mod ch7_task_id;
#[doc = "CH8_EVT_ID (rw) register accessor: Channel8 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_evt_id`] module"]
pub type CH8_EVT_ID = crate::Reg<ch8_evt_id::CH8_EVT_ID_SPEC>;
#[doc = "Channel8 event id register"]
pub mod ch8_evt_id;
#[doc = "CH8_TASK_ID (rw) register accessor: Channel8 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_task_id`] module"]
pub type CH8_TASK_ID = crate::Reg<ch8_task_id::CH8_TASK_ID_SPEC>;
#[doc = "Channel8 task id register"]
pub mod ch8_task_id;
#[doc = "CH9_EVT_ID (rw) register accessor: Channel9 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_evt_id`] module"]
pub type CH9_EVT_ID = crate::Reg<ch9_evt_id::CH9_EVT_ID_SPEC>;
#[doc = "Channel9 event id register"]
pub mod ch9_evt_id;
#[doc = "CH9_TASK_ID (rw) register accessor: Channel9 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_task_id`] module"]
pub type CH9_TASK_ID = crate::Reg<ch9_task_id::CH9_TASK_ID_SPEC>;
#[doc = "Channel9 task id register"]
pub mod ch9_task_id;
#[doc = "CH10_EVT_ID (rw) register accessor: Channel10 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_evt_id`] module"]
pub type CH10_EVT_ID = crate::Reg<ch10_evt_id::CH10_EVT_ID_SPEC>;
#[doc = "Channel10 event id register"]
pub mod ch10_evt_id;
#[doc = "CH10_TASK_ID (rw) register accessor: Channel10 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_task_id`] module"]
pub type CH10_TASK_ID = crate::Reg<ch10_task_id::CH10_TASK_ID_SPEC>;
#[doc = "Channel10 task id register"]
pub mod ch10_task_id;
#[doc = "CH11_EVT_ID (rw) register accessor: Channel11 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_evt_id`] module"]
pub type CH11_EVT_ID = crate::Reg<ch11_evt_id::CH11_EVT_ID_SPEC>;
#[doc = "Channel11 event id register"]
pub mod ch11_evt_id;
#[doc = "CH11_TASK_ID (rw) register accessor: Channel11 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_task_id`] module"]
pub type CH11_TASK_ID = crate::Reg<ch11_task_id::CH11_TASK_ID_SPEC>;
#[doc = "Channel11 task id register"]
pub mod ch11_task_id;
#[doc = "CH12_EVT_ID (rw) register accessor: Channel12 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_evt_id`] module"]
pub type CH12_EVT_ID = crate::Reg<ch12_evt_id::CH12_EVT_ID_SPEC>;
#[doc = "Channel12 event id register"]
pub mod ch12_evt_id;
#[doc = "CH12_TASK_ID (rw) register accessor: Channel12 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_task_id`] module"]
pub type CH12_TASK_ID = crate::Reg<ch12_task_id::CH12_TASK_ID_SPEC>;
#[doc = "Channel12 task id register"]
pub mod ch12_task_id;
#[doc = "CH13_EVT_ID (rw) register accessor: Channel13 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_evt_id`] module"]
pub type CH13_EVT_ID = crate::Reg<ch13_evt_id::CH13_EVT_ID_SPEC>;
#[doc = "Channel13 event id register"]
pub mod ch13_evt_id;
#[doc = "CH13_TASK_ID (rw) register accessor: Channel13 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_task_id`] module"]
pub type CH13_TASK_ID = crate::Reg<ch13_task_id::CH13_TASK_ID_SPEC>;
#[doc = "Channel13 task id register"]
pub mod ch13_task_id;
#[doc = "CH14_EVT_ID (rw) register accessor: Channel14 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_evt_id`] module"]
pub type CH14_EVT_ID = crate::Reg<ch14_evt_id::CH14_EVT_ID_SPEC>;
#[doc = "Channel14 event id register"]
pub mod ch14_evt_id;
#[doc = "CH14_TASK_ID (rw) register accessor: Channel14 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_task_id`] module"]
pub type CH14_TASK_ID = crate::Reg<ch14_task_id::CH14_TASK_ID_SPEC>;
#[doc = "Channel14 task id register"]
pub mod ch14_task_id;
#[doc = "CH15_EVT_ID (rw) register accessor: Channel15 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_evt_id`] module"]
pub type CH15_EVT_ID = crate::Reg<ch15_evt_id::CH15_EVT_ID_SPEC>;
#[doc = "Channel15 event id register"]
pub mod ch15_evt_id;
#[doc = "CH15_TASK_ID (rw) register accessor: Channel15 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_task_id`] module"]
pub type CH15_TASK_ID = crate::Reg<ch15_task_id::CH15_TASK_ID_SPEC>;
#[doc = "Channel15 task id register"]
pub mod ch15_task_id;
#[doc = "CH16_EVT_ID (rw) register accessor: Channel16 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_evt_id`] module"]
pub type CH16_EVT_ID = crate::Reg<ch16_evt_id::CH16_EVT_ID_SPEC>;
#[doc = "Channel16 event id register"]
pub mod ch16_evt_id;
#[doc = "CH16_TASK_ID (rw) register accessor: Channel16 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_task_id`] module"]
pub type CH16_TASK_ID = crate::Reg<ch16_task_id::CH16_TASK_ID_SPEC>;
#[doc = "Channel16 task id register"]
pub mod ch16_task_id;
#[doc = "CH17_EVT_ID (rw) register accessor: Channel17 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_evt_id`] module"]
pub type CH17_EVT_ID = crate::Reg<ch17_evt_id::CH17_EVT_ID_SPEC>;
#[doc = "Channel17 event id register"]
pub mod ch17_evt_id;
#[doc = "CH17_TASK_ID (rw) register accessor: Channel17 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_task_id`] module"]
pub type CH17_TASK_ID = crate::Reg<ch17_task_id::CH17_TASK_ID_SPEC>;
#[doc = "Channel17 task id register"]
pub mod ch17_task_id;
#[doc = "CH18_EVT_ID (rw) register accessor: Channel18 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_evt_id`] module"]
pub type CH18_EVT_ID = crate::Reg<ch18_evt_id::CH18_EVT_ID_SPEC>;
#[doc = "Channel18 event id register"]
pub mod ch18_evt_id;
#[doc = "CH18_TASK_ID (rw) register accessor: Channel18 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_task_id`] module"]
pub type CH18_TASK_ID = crate::Reg<ch18_task_id::CH18_TASK_ID_SPEC>;
#[doc = "Channel18 task id register"]
pub mod ch18_task_id;
#[doc = "CH19_EVT_ID (rw) register accessor: Channel19 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_evt_id`] module"]
pub type CH19_EVT_ID = crate::Reg<ch19_evt_id::CH19_EVT_ID_SPEC>;
#[doc = "Channel19 event id register"]
pub mod ch19_evt_id;
#[doc = "CH19_TASK_ID (rw) register accessor: Channel19 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_task_id`] module"]
pub type CH19_TASK_ID = crate::Reg<ch19_task_id::CH19_TASK_ID_SPEC>;
#[doc = "Channel19 task id register"]
pub mod ch19_task_id;
#[doc = "CH20_EVT_ID (rw) register accessor: Channel20 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_evt_id`] module"]
pub type CH20_EVT_ID = crate::Reg<ch20_evt_id::CH20_EVT_ID_SPEC>;
#[doc = "Channel20 event id register"]
pub mod ch20_evt_id;
#[doc = "CH20_TASK_ID (rw) register accessor: Channel20 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_task_id`] module"]
pub type CH20_TASK_ID = crate::Reg<ch20_task_id::CH20_TASK_ID_SPEC>;
#[doc = "Channel20 task id register"]
pub mod ch20_task_id;
#[doc = "CH21_EVT_ID (rw) register accessor: Channel21 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_evt_id`] module"]
pub type CH21_EVT_ID = crate::Reg<ch21_evt_id::CH21_EVT_ID_SPEC>;
#[doc = "Channel21 event id register"]
pub mod ch21_evt_id;
#[doc = "CH21_TASK_ID (rw) register accessor: Channel21 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_task_id`] module"]
pub type CH21_TASK_ID = crate::Reg<ch21_task_id::CH21_TASK_ID_SPEC>;
#[doc = "Channel21 task id register"]
pub mod ch21_task_id;
#[doc = "CH22_EVT_ID (rw) register accessor: Channel22 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_evt_id`] module"]
pub type CH22_EVT_ID = crate::Reg<ch22_evt_id::CH22_EVT_ID_SPEC>;
#[doc = "Channel22 event id register"]
pub mod ch22_evt_id;
#[doc = "CH22_TASK_ID (rw) register accessor: Channel22 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_task_id`] module"]
pub type CH22_TASK_ID = crate::Reg<ch22_task_id::CH22_TASK_ID_SPEC>;
#[doc = "Channel22 task id register"]
pub mod ch22_task_id;
#[doc = "CH23_EVT_ID (rw) register accessor: Channel23 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_evt_id`] module"]
pub type CH23_EVT_ID = crate::Reg<ch23_evt_id::CH23_EVT_ID_SPEC>;
#[doc = "Channel23 event id register"]
pub mod ch23_evt_id;
#[doc = "CH23_TASK_ID (rw) register accessor: Channel23 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_task_id`] module"]
pub type CH23_TASK_ID = crate::Reg<ch23_task_id::CH23_TASK_ID_SPEC>;
#[doc = "Channel23 task id register"]
pub mod ch23_task_id;
#[doc = "CH24_EVT_ID (rw) register accessor: Channel24 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch24_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch24_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch24_evt_id`] module"]
pub type CH24_EVT_ID = crate::Reg<ch24_evt_id::CH24_EVT_ID_SPEC>;
#[doc = "Channel24 event id register"]
pub mod ch24_evt_id;
#[doc = "CH24_TASK_ID (rw) register accessor: Channel24 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch24_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch24_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch24_task_id`] module"]
pub type CH24_TASK_ID = crate::Reg<ch24_task_id::CH24_TASK_ID_SPEC>;
#[doc = "Channel24 task id register"]
pub mod ch24_task_id;
#[doc = "CH25_EVT_ID (rw) register accessor: Channel25 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch25_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch25_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch25_evt_id`] module"]
pub type CH25_EVT_ID = crate::Reg<ch25_evt_id::CH25_EVT_ID_SPEC>;
#[doc = "Channel25 event id register"]
pub mod ch25_evt_id;
#[doc = "CH25_TASK_ID (rw) register accessor: Channel25 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch25_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch25_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch25_task_id`] module"]
pub type CH25_TASK_ID = crate::Reg<ch25_task_id::CH25_TASK_ID_SPEC>;
#[doc = "Channel25 task id register"]
pub mod ch25_task_id;
#[doc = "CH26_EVT_ID (rw) register accessor: Channel26 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch26_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch26_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch26_evt_id`] module"]
pub type CH26_EVT_ID = crate::Reg<ch26_evt_id::CH26_EVT_ID_SPEC>;
#[doc = "Channel26 event id register"]
pub mod ch26_evt_id;
#[doc = "CH26_TASK_ID (rw) register accessor: Channel26 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch26_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch26_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch26_task_id`] module"]
pub type CH26_TASK_ID = crate::Reg<ch26_task_id::CH26_TASK_ID_SPEC>;
#[doc = "Channel26 task id register"]
pub mod ch26_task_id;
#[doc = "CH27_EVT_ID (rw) register accessor: Channel27 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch27_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch27_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch27_evt_id`] module"]
pub type CH27_EVT_ID = crate::Reg<ch27_evt_id::CH27_EVT_ID_SPEC>;
#[doc = "Channel27 event id register"]
pub mod ch27_evt_id;
#[doc = "CH27_TASK_ID (rw) register accessor: Channel27 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch27_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch27_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch27_task_id`] module"]
pub type CH27_TASK_ID = crate::Reg<ch27_task_id::CH27_TASK_ID_SPEC>;
#[doc = "Channel27 task id register"]
pub mod ch27_task_id;
#[doc = "CH28_EVT_ID (rw) register accessor: Channel28 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch28_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch28_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch28_evt_id`] module"]
pub type CH28_EVT_ID = crate::Reg<ch28_evt_id::CH28_EVT_ID_SPEC>;
#[doc = "Channel28 event id register"]
pub mod ch28_evt_id;
#[doc = "CH28_TASK_ID (rw) register accessor: Channel28 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch28_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch28_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch28_task_id`] module"]
pub type CH28_TASK_ID = crate::Reg<ch28_task_id::CH28_TASK_ID_SPEC>;
#[doc = "Channel28 task id register"]
pub mod ch28_task_id;
#[doc = "CH29_EVT_ID (rw) register accessor: Channel29 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch29_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch29_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch29_evt_id`] module"]
pub type CH29_EVT_ID = crate::Reg<ch29_evt_id::CH29_EVT_ID_SPEC>;
#[doc = "Channel29 event id register"]
pub mod ch29_evt_id;
#[doc = "CH29_TASK_ID (rw) register accessor: Channel29 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch29_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch29_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch29_task_id`] module"]
pub type CH29_TASK_ID = crate::Reg<ch29_task_id::CH29_TASK_ID_SPEC>;
#[doc = "Channel29 task id register"]
pub mod ch29_task_id;
#[doc = "CH30_EVT_ID (rw) register accessor: Channel30 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch30_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch30_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch30_evt_id`] module"]
pub type CH30_EVT_ID = crate::Reg<ch30_evt_id::CH30_EVT_ID_SPEC>;
#[doc = "Channel30 event id register"]
pub mod ch30_evt_id;
#[doc = "CH30_TASK_ID (rw) register accessor: Channel30 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch30_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch30_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch30_task_id`] module"]
pub type CH30_TASK_ID = crate::Reg<ch30_task_id::CH30_TASK_ID_SPEC>;
#[doc = "Channel30 task id register"]
pub mod ch30_task_id;
#[doc = "CH31_EVT_ID (rw) register accessor: Channel31 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch31_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch31_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch31_evt_id`] module"]
pub type CH31_EVT_ID = crate::Reg<ch31_evt_id::CH31_EVT_ID_SPEC>;
#[doc = "Channel31 event id register"]
pub mod ch31_evt_id;
#[doc = "CH31_TASK_ID (rw) register accessor: Channel31 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch31_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch31_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch31_task_id`] module"]
pub type CH31_TASK_ID = crate::Reg<ch31_task_id::CH31_TASK_ID_SPEC>;
#[doc = "Channel31 task id register"]
pub mod ch31_task_id;
#[doc = "CH32_EVT_ID (rw) register accessor: Channel32 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch32_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch32_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch32_evt_id`] module"]
pub type CH32_EVT_ID = crate::Reg<ch32_evt_id::CH32_EVT_ID_SPEC>;
#[doc = "Channel32 event id register"]
pub mod ch32_evt_id;
#[doc = "CH32_TASK_ID (rw) register accessor: Channel32 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch32_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch32_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch32_task_id`] module"]
pub type CH32_TASK_ID = crate::Reg<ch32_task_id::CH32_TASK_ID_SPEC>;
#[doc = "Channel32 task id register"]
pub mod ch32_task_id;
#[doc = "CH33_EVT_ID (rw) register accessor: Channel33 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch33_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch33_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch33_evt_id`] module"]
pub type CH33_EVT_ID = crate::Reg<ch33_evt_id::CH33_EVT_ID_SPEC>;
#[doc = "Channel33 event id register"]
pub mod ch33_evt_id;
#[doc = "CH33_TASK_ID (rw) register accessor: Channel33 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch33_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch33_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch33_task_id`] module"]
pub type CH33_TASK_ID = crate::Reg<ch33_task_id::CH33_TASK_ID_SPEC>;
#[doc = "Channel33 task id register"]
pub mod ch33_task_id;
#[doc = "CH34_EVT_ID (rw) register accessor: Channel34 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch34_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch34_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch34_evt_id`] module"]
pub type CH34_EVT_ID = crate::Reg<ch34_evt_id::CH34_EVT_ID_SPEC>;
#[doc = "Channel34 event id register"]
pub mod ch34_evt_id;
#[doc = "CH34_TASK_ID (rw) register accessor: Channel34 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch34_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch34_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch34_task_id`] module"]
pub type CH34_TASK_ID = crate::Reg<ch34_task_id::CH34_TASK_ID_SPEC>;
#[doc = "Channel34 task id register"]
pub mod ch34_task_id;
#[doc = "CH35_EVT_ID (rw) register accessor: Channel35 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch35_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch35_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch35_evt_id`] module"]
pub type CH35_EVT_ID = crate::Reg<ch35_evt_id::CH35_EVT_ID_SPEC>;
#[doc = "Channel35 event id register"]
pub mod ch35_evt_id;
#[doc = "CH35_TASK_ID (rw) register accessor: Channel35 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch35_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch35_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch35_task_id`] module"]
pub type CH35_TASK_ID = crate::Reg<ch35_task_id::CH35_TASK_ID_SPEC>;
#[doc = "Channel35 task id register"]
pub mod ch35_task_id;
#[doc = "CH36_EVT_ID (rw) register accessor: Channel36 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch36_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch36_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch36_evt_id`] module"]
pub type CH36_EVT_ID = crate::Reg<ch36_evt_id::CH36_EVT_ID_SPEC>;
#[doc = "Channel36 event id register"]
pub mod ch36_evt_id;
#[doc = "CH36_TASK_ID (rw) register accessor: Channel36 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch36_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch36_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch36_task_id`] module"]
pub type CH36_TASK_ID = crate::Reg<ch36_task_id::CH36_TASK_ID_SPEC>;
#[doc = "Channel36 task id register"]
pub mod ch36_task_id;
#[doc = "CH37_EVT_ID (rw) register accessor: Channel37 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch37_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch37_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch37_evt_id`] module"]
pub type CH37_EVT_ID = crate::Reg<ch37_evt_id::CH37_EVT_ID_SPEC>;
#[doc = "Channel37 event id register"]
pub mod ch37_evt_id;
#[doc = "CH37_TASK_ID (rw) register accessor: Channel37 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch37_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch37_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch37_task_id`] module"]
pub type CH37_TASK_ID = crate::Reg<ch37_task_id::CH37_TASK_ID_SPEC>;
#[doc = "Channel37 task id register"]
pub mod ch37_task_id;
#[doc = "CH38_EVT_ID (rw) register accessor: Channel38 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch38_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch38_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch38_evt_id`] module"]
pub type CH38_EVT_ID = crate::Reg<ch38_evt_id::CH38_EVT_ID_SPEC>;
#[doc = "Channel38 event id register"]
pub mod ch38_evt_id;
#[doc = "CH38_TASK_ID (rw) register accessor: Channel38 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch38_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch38_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch38_task_id`] module"]
pub type CH38_TASK_ID = crate::Reg<ch38_task_id::CH38_TASK_ID_SPEC>;
#[doc = "Channel38 task id register"]
pub mod ch38_task_id;
#[doc = "CH39_EVT_ID (rw) register accessor: Channel39 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch39_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch39_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch39_evt_id`] module"]
pub type CH39_EVT_ID = crate::Reg<ch39_evt_id::CH39_EVT_ID_SPEC>;
#[doc = "Channel39 event id register"]
pub mod ch39_evt_id;
#[doc = "CH39_TASK_ID (rw) register accessor: Channel39 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch39_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch39_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch39_task_id`] module"]
pub type CH39_TASK_ID = crate::Reg<ch39_task_id::CH39_TASK_ID_SPEC>;
#[doc = "Channel39 task id register"]
pub mod ch39_task_id;
#[doc = "CH40_EVT_ID (rw) register accessor: Channel40 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch40_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch40_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch40_evt_id`] module"]
pub type CH40_EVT_ID = crate::Reg<ch40_evt_id::CH40_EVT_ID_SPEC>;
#[doc = "Channel40 event id register"]
pub mod ch40_evt_id;
#[doc = "CH40_TASK_ID (rw) register accessor: Channel40 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch40_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch40_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch40_task_id`] module"]
pub type CH40_TASK_ID = crate::Reg<ch40_task_id::CH40_TASK_ID_SPEC>;
#[doc = "Channel40 task id register"]
pub mod ch40_task_id;
#[doc = "CH41_EVT_ID (rw) register accessor: Channel41 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch41_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch41_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch41_evt_id`] module"]
pub type CH41_EVT_ID = crate::Reg<ch41_evt_id::CH41_EVT_ID_SPEC>;
#[doc = "Channel41 event id register"]
pub mod ch41_evt_id;
#[doc = "CH41_TASK_ID (rw) register accessor: Channel41 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch41_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch41_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch41_task_id`] module"]
pub type CH41_TASK_ID = crate::Reg<ch41_task_id::CH41_TASK_ID_SPEC>;
#[doc = "Channel41 task id register"]
pub mod ch41_task_id;
#[doc = "CH42_EVT_ID (rw) register accessor: Channel42 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch42_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch42_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch42_evt_id`] module"]
pub type CH42_EVT_ID = crate::Reg<ch42_evt_id::CH42_EVT_ID_SPEC>;
#[doc = "Channel42 event id register"]
pub mod ch42_evt_id;
#[doc = "CH42_TASK_ID (rw) register accessor: Channel42 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch42_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch42_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch42_task_id`] module"]
pub type CH42_TASK_ID = crate::Reg<ch42_task_id::CH42_TASK_ID_SPEC>;
#[doc = "Channel42 task id register"]
pub mod ch42_task_id;
#[doc = "CH43_EVT_ID (rw) register accessor: Channel43 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch43_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch43_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch43_evt_id`] module"]
pub type CH43_EVT_ID = crate::Reg<ch43_evt_id::CH43_EVT_ID_SPEC>;
#[doc = "Channel43 event id register"]
pub mod ch43_evt_id;
#[doc = "CH43_TASK_ID (rw) register accessor: Channel43 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch43_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch43_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch43_task_id`] module"]
pub type CH43_TASK_ID = crate::Reg<ch43_task_id::CH43_TASK_ID_SPEC>;
#[doc = "Channel43 task id register"]
pub mod ch43_task_id;
#[doc = "CH44_EVT_ID (rw) register accessor: Channel44 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch44_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch44_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch44_evt_id`] module"]
pub type CH44_EVT_ID = crate::Reg<ch44_evt_id::CH44_EVT_ID_SPEC>;
#[doc = "Channel44 event id register"]
pub mod ch44_evt_id;
#[doc = "CH44_TASK_ID (rw) register accessor: Channel44 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch44_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch44_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch44_task_id`] module"]
pub type CH44_TASK_ID = crate::Reg<ch44_task_id::CH44_TASK_ID_SPEC>;
#[doc = "Channel44 task id register"]
pub mod ch44_task_id;
#[doc = "CH45_EVT_ID (rw) register accessor: Channel45 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch45_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch45_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch45_evt_id`] module"]
pub type CH45_EVT_ID = crate::Reg<ch45_evt_id::CH45_EVT_ID_SPEC>;
#[doc = "Channel45 event id register"]
pub mod ch45_evt_id;
#[doc = "CH45_TASK_ID (rw) register accessor: Channel45 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch45_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch45_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch45_task_id`] module"]
pub type CH45_TASK_ID = crate::Reg<ch45_task_id::CH45_TASK_ID_SPEC>;
#[doc = "Channel45 task id register"]
pub mod ch45_task_id;
#[doc = "CH46_EVT_ID (rw) register accessor: Channel46 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch46_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch46_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch46_evt_id`] module"]
pub type CH46_EVT_ID = crate::Reg<ch46_evt_id::CH46_EVT_ID_SPEC>;
#[doc = "Channel46 event id register"]
pub mod ch46_evt_id;
#[doc = "CH46_TASK_ID (rw) register accessor: Channel46 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch46_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch46_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch46_task_id`] module"]
pub type CH46_TASK_ID = crate::Reg<ch46_task_id::CH46_TASK_ID_SPEC>;
#[doc = "Channel46 task id register"]
pub mod ch46_task_id;
#[doc = "CH47_EVT_ID (rw) register accessor: Channel47 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch47_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch47_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch47_evt_id`] module"]
pub type CH47_EVT_ID = crate::Reg<ch47_evt_id::CH47_EVT_ID_SPEC>;
#[doc = "Channel47 event id register"]
pub mod ch47_evt_id;
#[doc = "CH47_TASK_ID (rw) register accessor: Channel47 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch47_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch47_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch47_task_id`] module"]
pub type CH47_TASK_ID = crate::Reg<ch47_task_id::CH47_TASK_ID_SPEC>;
#[doc = "Channel47 task id register"]
pub mod ch47_task_id;
#[doc = "CH48_EVT_ID (rw) register accessor: Channel48 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch48_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch48_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch48_evt_id`] module"]
pub type CH48_EVT_ID = crate::Reg<ch48_evt_id::CH48_EVT_ID_SPEC>;
#[doc = "Channel48 event id register"]
pub mod ch48_evt_id;
#[doc = "CH48_TASK_ID (rw) register accessor: Channel48 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch48_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch48_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch48_task_id`] module"]
pub type CH48_TASK_ID = crate::Reg<ch48_task_id::CH48_TASK_ID_SPEC>;
#[doc = "Channel48 task id register"]
pub mod ch48_task_id;
#[doc = "CH49_EVT_ID (rw) register accessor: Channel49 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch49_evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch49_evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch49_evt_id`] module"]
pub type CH49_EVT_ID = crate::Reg<ch49_evt_id::CH49_EVT_ID_SPEC>;
#[doc = "Channel49 event id register"]
pub mod ch49_evt_id;
#[doc = "CH49_TASK_ID (rw) register accessor: Channel49 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch49_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch49_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch49_task_id`] module"]
pub type CH49_TASK_ID = crate::Reg<ch49_task_id::CH49_TASK_ID_SPEC>;
#[doc = "Channel49 task id register"]
pub mod ch49_task_id;
#[doc = "EVT_ST0 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st0`] module"]
pub type EVT_ST0 = crate::Reg<evt_st0::EVT_ST0_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st0;
#[doc = "EVT_ST0_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st0_clr`] module"]
pub type EVT_ST0_CLR = crate::Reg<evt_st0_clr::EVT_ST0_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st0_clr;
#[doc = "EVT_ST1 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st1`] module"]
pub type EVT_ST1 = crate::Reg<evt_st1::EVT_ST1_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st1;
#[doc = "EVT_ST1_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st1_clr`] module"]
pub type EVT_ST1_CLR = crate::Reg<evt_st1_clr::EVT_ST1_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st1_clr;
#[doc = "EVT_ST2 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st2`] module"]
pub type EVT_ST2 = crate::Reg<evt_st2::EVT_ST2_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st2;
#[doc = "EVT_ST2_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st2_clr`] module"]
pub type EVT_ST2_CLR = crate::Reg<evt_st2_clr::EVT_ST2_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st2_clr;
#[doc = "EVT_ST3 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st3`] module"]
pub type EVT_ST3 = crate::Reg<evt_st3::EVT_ST3_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st3;
#[doc = "EVT_ST3_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st3_clr`] module"]
pub type EVT_ST3_CLR = crate::Reg<evt_st3_clr::EVT_ST3_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st3_clr;
#[doc = "EVT_ST4 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st4`] module"]
pub type EVT_ST4 = crate::Reg<evt_st4::EVT_ST4_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st4;
#[doc = "EVT_ST4_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st4_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st4_clr`] module"]
pub type EVT_ST4_CLR = crate::Reg<evt_st4_clr::EVT_ST4_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st4_clr;
#[doc = "EVT_ST5 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st5`] module"]
pub type EVT_ST5 = crate::Reg<evt_st5::EVT_ST5_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st5;
#[doc = "EVT_ST5_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st5_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st5_clr`] module"]
pub type EVT_ST5_CLR = crate::Reg<evt_st5_clr::EVT_ST5_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st5_clr;
#[doc = "EVT_ST6 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st6`] module"]
pub type EVT_ST6 = crate::Reg<evt_st6::EVT_ST6_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st6;
#[doc = "EVT_ST6_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st6_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st6_clr`] module"]
pub type EVT_ST6_CLR = crate::Reg<evt_st6_clr::EVT_ST6_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st6_clr;
#[doc = "EVT_ST7 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st7`] module"]
pub type EVT_ST7 = crate::Reg<evt_st7::EVT_ST7_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st7;
#[doc = "EVT_ST7_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st7_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st7_clr`] module"]
pub type EVT_ST7_CLR = crate::Reg<evt_st7_clr::EVT_ST7_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st7_clr;
#[doc = "TASK_ST0 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st0`] module"]
pub type TASK_ST0 = crate::Reg<task_st0::TASK_ST0_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st0;
#[doc = "TASK_ST0_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st0_clr`] module"]
pub type TASK_ST0_CLR = crate::Reg<task_st0_clr::TASK_ST0_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st0_clr;
#[doc = "TASK_ST1 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st1`] module"]
pub type TASK_ST1 = crate::Reg<task_st1::TASK_ST1_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st1;
#[doc = "TASK_ST1_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st1_clr`] module"]
pub type TASK_ST1_CLR = crate::Reg<task_st1_clr::TASK_ST1_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st1_clr;
#[doc = "TASK_ST2 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st2`] module"]
pub type TASK_ST2 = crate::Reg<task_st2::TASK_ST2_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st2;
#[doc = "TASK_ST2_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st2_clr`] module"]
pub type TASK_ST2_CLR = crate::Reg<task_st2_clr::TASK_ST2_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st2_clr;
#[doc = "TASK_ST3 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st3`] module"]
pub type TASK_ST3 = crate::Reg<task_st3::TASK_ST3_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st3;
#[doc = "TASK_ST3_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st3_clr`] module"]
pub type TASK_ST3_CLR = crate::Reg<task_st3_clr::TASK_ST3_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st3_clr;
#[doc = "TASK_ST4 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st4`] module"]
pub type TASK_ST4 = crate::Reg<task_st4::TASK_ST4_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st4;
#[doc = "TASK_ST4_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st4_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st4_clr`] module"]
pub type TASK_ST4_CLR = crate::Reg<task_st4_clr::TASK_ST4_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st4_clr;
#[doc = "TASK_ST5 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st5`] module"]
pub type TASK_ST5 = crate::Reg<task_st5::TASK_ST5_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st5;
#[doc = "TASK_ST5_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st5_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st5_clr`] module"]
pub type TASK_ST5_CLR = crate::Reg<task_st5_clr::TASK_ST5_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st5_clr;
#[doc = "TASK_ST6 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st6`] module"]
pub type TASK_ST6 = crate::Reg<task_st6::TASK_ST6_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st6;
#[doc = "TASK_ST6_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st6_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st6_clr`] module"]
pub type TASK_ST6_CLR = crate::Reg<task_st6_clr::TASK_ST6_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st6_clr;
#[doc = "CLK_EN (rw) register accessor: ETM clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "ETM clock enable register"]
pub mod clk_en;
#[doc = "DATE (rw) register accessor: ETM date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "ETM date register"]
pub mod date;
