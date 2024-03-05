#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    bt_select: BT_SELECT,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    out1: OUT1,
    out1_w1ts: OUT1_W1TS,
    out1_w1tc: OUT1_W1TC,
    sdio_select: SDIO_SELECT,
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    enable1: ENABLE1,
    enable1_w1ts: ENABLE1_W1TS,
    enable1_w1tc: ENABLE1_W1TC,
    strap: STRAP,
    in_: IN,
    in1: IN1,
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    status1: STATUS1,
    status1_w1ts: STATUS1_W1TS,
    status1_w1tc: STATUS1_W1TC,
    _reserved23: [u8; 0x04],
    acpu_int: ACPU_INT,
    acpu_nmi_int: ACPU_NMI_INT,
    pcpu_int: PCPU_INT,
    pcpu_nmi_int: PCPU_NMI_INT,
    cpusdio_int: CPUSDIO_INT,
    acpu_int1: ACPU_INT1,
    acpu_nmi_int1: ACPU_NMI_INT1,
    pcpu_int1: PCPU_INT1,
    pcpu_nmi_int1: PCPU_NMI_INT1,
    cpusdio_int1: CPUSDIO_INT1,
    pin: [PIN; 40],
    cali_conf: CALI_CONF,
    cali_data: CALI_DATA,
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 256],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 40],
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn bt_select(&self) -> &BT_SELECT {
        &self.bt_select
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn sdio_select(&self) -> &SDIO_SELECT {
        &self.sdio_select
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn acpu_int(&self) -> &ACPU_INT {
        &self.acpu_int
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn acpu_nmi_int(&self) -> &ACPU_NMI_INT {
        &self.acpu_nmi_int
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn pcpu_int(&self) -> &PCPU_INT {
        &self.pcpu_int
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn pcpu_nmi_int(&self) -> &PCPU_NMI_INT {
        &self.pcpu_nmi_int
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn cpusdio_int(&self) -> &CPUSDIO_INT {
        &self.cpusdio_int
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn acpu_int1(&self) -> &ACPU_INT1 {
        &self.acpu_int1
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn acpu_nmi_int1(&self) -> &ACPU_NMI_INT1 {
        &self.acpu_nmi_int1
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn pcpu_int1(&self) -> &PCPU_INT1 {
        &self.pcpu_int1
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn pcpu_nmi_int1(&self) -> &PCPU_NMI_INT1 {
        &self.pcpu_nmi_int1
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn cpusdio_int1(&self) -> &CPUSDIO_INT1 {
        &self.cpusdio_int1
    }
    #[doc = "0x88..0x128 - "]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x128 - "]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn cali_conf(&self) -> &CALI_CONF {
        &self.cali_conf
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn cali_data(&self) -> &CALI_DATA {
        &self.cali_data
    }
    #[doc = "0x130..0x530 - "]
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x530 - "]
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    #[doc = "0x130 - FUNC0_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    #[doc = "0x134 - FUNC1_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    #[doc = "0x138 - FUNC2_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    #[doc = "0x13c - FUNC3_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    #[doc = "0x140 - FUNC4_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    #[doc = "0x144 - FUNC5_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    #[doc = "0x148 - FUNC6_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    #[doc = "0x14c - FUNC7_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    #[doc = "0x150 - FUNC8_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    #[doc = "0x154 - FUNC9_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    #[doc = "0x158 - FUNC10_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    #[doc = "0x15c - FUNC11_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    #[doc = "0x160 - FUNC12_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    #[doc = "0x164 - FUNC13_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    #[doc = "0x168 - FUNC14_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    #[doc = "0x16c - FUNC15_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    #[doc = "0x170 - FUNC16_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    #[doc = "0x174 - FUNC17_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    #[doc = "0x178 - FUNC18_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    #[doc = "0x17c - FUNC19_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    #[doc = "0x180 - FUNC20_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    #[doc = "0x184 - FUNC21_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    #[doc = "0x188 - FUNC22_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    #[doc = "0x18c - FUNC23_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    #[doc = "0x190 - FUNC24_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    #[doc = "0x194 - FUNC25_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    #[doc = "0x198 - FUNC26_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    #[doc = "0x19c - FUNC27_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    #[doc = "0x1a0 - FUNC28_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    #[doc = "0x1a4 - FUNC29_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    #[doc = "0x1a8 - FUNC30_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    #[doc = "0x1ac - FUNC31_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    #[doc = "0x1b0 - FUNC32_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    #[doc = "0x1b4 - FUNC33_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    #[doc = "0x1b8 - FUNC34_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    #[doc = "0x1bc - FUNC35_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    #[doc = "0x1c0 - FUNC36_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    #[doc = "0x1c4 - FUNC37_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    #[doc = "0x1c8 - FUNC38_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    #[doc = "0x1cc - FUNC39_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    #[doc = "0x1d0 - FUNC40_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    #[doc = "0x1d4 - FUNC41_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    #[doc = "0x1d8 - FUNC42_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    #[doc = "0x1dc - FUNC43_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    #[doc = "0x1e0 - FUNC44_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    #[doc = "0x1e4 - FUNC45_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    #[doc = "0x1e8 - FUNC46_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    #[doc = "0x1ec - FUNC47_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    #[doc = "0x1f0 - FUNC48_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(48)
    }
    #[doc = "0x1f4 - FUNC49_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(49)
    }
    #[doc = "0x1f8 - FUNC50_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(50)
    }
    #[doc = "0x1fc - FUNC51_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(51)
    }
    #[doc = "0x200 - FUNC52_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(52)
    }
    #[doc = "0x204 - FUNC53_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(53)
    }
    #[doc = "0x208 - FUNC54_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(54)
    }
    #[doc = "0x20c - FUNC55_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(55)
    }
    #[doc = "0x210 - FUNC56_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(56)
    }
    #[doc = "0x214 - FUNC57_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(57)
    }
    #[doc = "0x218 - FUNC58_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(58)
    }
    #[doc = "0x21c - FUNC59_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(59)
    }
    #[doc = "0x220 - FUNC60_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(60)
    }
    #[doc = "0x224 - FUNC61_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(61)
    }
    #[doc = "0x228 - FUNC62_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(62)
    }
    #[doc = "0x22c - FUNC63_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(63)
    }
    #[doc = "0x230 - FUNC64_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(64)
    }
    #[doc = "0x234 - FUNC65_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(65)
    }
    #[doc = "0x238 - FUNC66_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(66)
    }
    #[doc = "0x23c - FUNC67_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(67)
    }
    #[doc = "0x240 - FUNC68_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(68)
    }
    #[doc = "0x244 - FUNC69_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(69)
    }
    #[doc = "0x248 - FUNC70_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(70)
    }
    #[doc = "0x24c - FUNC71_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(71)
    }
    #[doc = "0x250 - FUNC72_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(72)
    }
    #[doc = "0x254 - FUNC73_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(73)
    }
    #[doc = "0x258 - FUNC74_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(74)
    }
    #[doc = "0x25c - FUNC75_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(75)
    }
    #[doc = "0x260 - FUNC76_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(76)
    }
    #[doc = "0x264 - FUNC77_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(77)
    }
    #[doc = "0x268 - FUNC78_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(78)
    }
    #[doc = "0x26c - FUNC79_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(79)
    }
    #[doc = "0x270 - FUNC80_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(80)
    }
    #[doc = "0x274 - FUNC81_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(81)
    }
    #[doc = "0x278 - FUNC82_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(82)
    }
    #[doc = "0x27c - FUNC83_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(83)
    }
    #[doc = "0x280 - FUNC84_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func84_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(84)
    }
    #[doc = "0x284 - FUNC85_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func85_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(85)
    }
    #[doc = "0x288 - FUNC86_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(86)
    }
    #[doc = "0x28c - FUNC87_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(87)
    }
    #[doc = "0x290 - FUNC88_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(88)
    }
    #[doc = "0x294 - FUNC89_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(89)
    }
    #[doc = "0x298 - FUNC90_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(90)
    }
    #[doc = "0x29c - FUNC91_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(91)
    }
    #[doc = "0x2a0 - FUNC92_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(92)
    }
    #[doc = "0x2a4 - FUNC93_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(93)
    }
    #[doc = "0x2a8 - FUNC94_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(94)
    }
    #[doc = "0x2ac - FUNC95_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(95)
    }
    #[doc = "0x2b0 - FUNC96_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(96)
    }
    #[doc = "0x2b4 - FUNC97_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(97)
    }
    #[doc = "0x2b8 - FUNC98_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(98)
    }
    #[doc = "0x2bc - FUNC99_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(99)
    }
    #[doc = "0x2c0 - FUNC100_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(100)
    }
    #[doc = "0x2c4 - FUNC101_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(101)
    }
    #[doc = "0x2c8 - FUNC102_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(102)
    }
    #[doc = "0x2cc - FUNC103_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(103)
    }
    #[doc = "0x2d0 - FUNC104_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(104)
    }
    #[doc = "0x2d4 - FUNC105_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(105)
    }
    #[doc = "0x2d8 - FUNC106_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(106)
    }
    #[doc = "0x2dc - FUNC107_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(107)
    }
    #[doc = "0x2e0 - FUNC108_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(108)
    }
    #[doc = "0x2e4 - FUNC109_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(109)
    }
    #[doc = "0x2e8 - FUNC110_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(110)
    }
    #[doc = "0x2ec - FUNC111_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(111)
    }
    #[doc = "0x2f0 - FUNC112_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(112)
    }
    #[doc = "0x2f4 - FUNC113_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(113)
    }
    #[doc = "0x2f8 - FUNC114_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(114)
    }
    #[doc = "0x2fc - FUNC115_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(115)
    }
    #[doc = "0x300 - FUNC116_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(116)
    }
    #[doc = "0x304 - FUNC117_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(117)
    }
    #[doc = "0x308 - FUNC118_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(118)
    }
    #[doc = "0x30c - FUNC119_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(119)
    }
    #[doc = "0x310 - FUNC120_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(120)
    }
    #[doc = "0x314 - FUNC121_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(121)
    }
    #[doc = "0x318 - FUNC122_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(122)
    }
    #[doc = "0x31c - FUNC123_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(123)
    }
    #[doc = "0x320 - FUNC124_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(124)
    }
    #[doc = "0x324 - FUNC125_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func125_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(125)
    }
    #[doc = "0x328 - FUNC126_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(126)
    }
    #[doc = "0x32c - FUNC127_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(127)
    }
    #[doc = "0x330 - FUNC128_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func128_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(128)
    }
    #[doc = "0x334 - FUNC129_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func129_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(129)
    }
    #[doc = "0x338 - FUNC130_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func130_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(130)
    }
    #[doc = "0x33c - FUNC131_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func131_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(131)
    }
    #[doc = "0x340 - FUNC132_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func132_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(132)
    }
    #[doc = "0x344 - FUNC133_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func133_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(133)
    }
    #[doc = "0x348 - FUNC134_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func134_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(134)
    }
    #[doc = "0x34c - FUNC135_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func135_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(135)
    }
    #[doc = "0x350 - FUNC136_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func136_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(136)
    }
    #[doc = "0x354 - FUNC137_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func137_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(137)
    }
    #[doc = "0x358 - FUNC138_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func138_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(138)
    }
    #[doc = "0x35c - FUNC139_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func139_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(139)
    }
    #[doc = "0x360 - FUNC140_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func140_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(140)
    }
    #[doc = "0x364 - FUNC141_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func141_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(141)
    }
    #[doc = "0x368 - FUNC142_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func142_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(142)
    }
    #[doc = "0x36c - FUNC143_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func143_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(143)
    }
    #[doc = "0x370 - FUNC144_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func144_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(144)
    }
    #[doc = "0x374 - FUNC145_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func145_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(145)
    }
    #[doc = "0x378 - FUNC146_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func146_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(146)
    }
    #[doc = "0x37c - FUNC147_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func147_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(147)
    }
    #[doc = "0x380 - FUNC148_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func148_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(148)
    }
    #[doc = "0x384 - FUNC149_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func149_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(149)
    }
    #[doc = "0x388 - FUNC150_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func150_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(150)
    }
    #[doc = "0x38c - FUNC151_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func151_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(151)
    }
    #[doc = "0x390 - FUNC152_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func152_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(152)
    }
    #[doc = "0x394 - FUNC153_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func153_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(153)
    }
    #[doc = "0x398 - FUNC154_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func154_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(154)
    }
    #[doc = "0x39c - FUNC155_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func155_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(155)
    }
    #[doc = "0x3a0 - FUNC156_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func156_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(156)
    }
    #[doc = "0x3a4 - FUNC157_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func157_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(157)
    }
    #[doc = "0x3a8 - FUNC158_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func158_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(158)
    }
    #[doc = "0x3ac - FUNC159_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func159_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(159)
    }
    #[doc = "0x3b0 - FUNC160_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func160_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(160)
    }
    #[doc = "0x3b4 - FUNC161_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func161_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(161)
    }
    #[doc = "0x3b8 - FUNC162_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func162_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(162)
    }
    #[doc = "0x3bc - FUNC163_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func163_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(163)
    }
    #[doc = "0x3c0 - FUNC164_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func164_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(164)
    }
    #[doc = "0x3c4 - FUNC165_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func165_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(165)
    }
    #[doc = "0x3c8 - FUNC166_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func166_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(166)
    }
    #[doc = "0x3cc - FUNC167_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func167_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(167)
    }
    #[doc = "0x3d0 - FUNC168_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func168_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(168)
    }
    #[doc = "0x3d4 - FUNC169_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func169_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(169)
    }
    #[doc = "0x3d8 - FUNC170_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func170_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(170)
    }
    #[doc = "0x3dc - FUNC171_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func171_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(171)
    }
    #[doc = "0x3e0 - FUNC172_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func172_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(172)
    }
    #[doc = "0x3e4 - FUNC173_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func173_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(173)
    }
    #[doc = "0x3e8 - FUNC174_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func174_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(174)
    }
    #[doc = "0x3ec - FUNC175_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func175_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(175)
    }
    #[doc = "0x3f0 - FUNC176_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func176_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(176)
    }
    #[doc = "0x3f4 - FUNC177_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func177_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(177)
    }
    #[doc = "0x3f8 - FUNC178_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func178_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(178)
    }
    #[doc = "0x3fc - FUNC179_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func179_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(179)
    }
    #[doc = "0x400 - FUNC180_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func180_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(180)
    }
    #[doc = "0x404 - FUNC181_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func181_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(181)
    }
    #[doc = "0x408 - FUNC182_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func182_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(182)
    }
    #[doc = "0x40c - FUNC183_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func183_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(183)
    }
    #[doc = "0x410 - FUNC184_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func184_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(184)
    }
    #[doc = "0x414 - FUNC185_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func185_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(185)
    }
    #[doc = "0x418 - FUNC186_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func186_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(186)
    }
    #[doc = "0x41c - FUNC187_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func187_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(187)
    }
    #[doc = "0x420 - FUNC188_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func188_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(188)
    }
    #[doc = "0x424 - FUNC189_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func189_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(189)
    }
    #[doc = "0x428 - FUNC190_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func190_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(190)
    }
    #[doc = "0x42c - FUNC191_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func191_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(191)
    }
    #[doc = "0x430 - FUNC192_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func192_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(192)
    }
    #[doc = "0x434 - FUNC193_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func193_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(193)
    }
    #[doc = "0x438 - FUNC194_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func194_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(194)
    }
    #[doc = "0x43c - FUNC195_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func195_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(195)
    }
    #[doc = "0x440 - FUNC196_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func196_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(196)
    }
    #[doc = "0x444 - FUNC197_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func197_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(197)
    }
    #[doc = "0x448 - FUNC198_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func198_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(198)
    }
    #[doc = "0x44c - FUNC199_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func199_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(199)
    }
    #[doc = "0x450 - FUNC200_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func200_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(200)
    }
    #[doc = "0x454 - FUNC201_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func201_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(201)
    }
    #[doc = "0x458 - FUNC202_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func202_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(202)
    }
    #[doc = "0x45c - FUNC203_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func203_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(203)
    }
    #[doc = "0x460 - FUNC204_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func204_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(204)
    }
    #[doc = "0x464 - FUNC205_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func205_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(205)
    }
    #[doc = "0x468 - FUNC206_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func206_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(206)
    }
    #[doc = "0x46c - FUNC207_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func207_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(207)
    }
    #[doc = "0x470 - FUNC208_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func208_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(208)
    }
    #[doc = "0x474 - FUNC209_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func209_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(209)
    }
    #[doc = "0x478 - FUNC210_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func210_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(210)
    }
    #[doc = "0x47c - FUNC211_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func211_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(211)
    }
    #[doc = "0x480 - FUNC212_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func212_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(212)
    }
    #[doc = "0x484 - FUNC213_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func213_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(213)
    }
    #[doc = "0x488 - FUNC214_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func214_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(214)
    }
    #[doc = "0x48c - FUNC215_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func215_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(215)
    }
    #[doc = "0x490 - FUNC216_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func216_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(216)
    }
    #[doc = "0x494 - FUNC217_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func217_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(217)
    }
    #[doc = "0x498 - FUNC218_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func218_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(218)
    }
    #[doc = "0x49c - FUNC219_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func219_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(219)
    }
    #[doc = "0x4a0 - FUNC220_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func220_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(220)
    }
    #[doc = "0x4a4 - FUNC221_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func221_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(221)
    }
    #[doc = "0x4a8 - FUNC222_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func222_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(222)
    }
    #[doc = "0x4ac - FUNC223_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func223_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(223)
    }
    #[doc = "0x4b0 - FUNC224_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func224_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(224)
    }
    #[doc = "0x4b4 - FUNC225_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func225_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(225)
    }
    #[doc = "0x4b8 - FUNC226_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func226_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(226)
    }
    #[doc = "0x4bc - FUNC227_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func227_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(227)
    }
    #[doc = "0x4c0 - FUNC228_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func228_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(228)
    }
    #[doc = "0x4c4 - FUNC229_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func229_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(229)
    }
    #[doc = "0x4c8 - FUNC230_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func230_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(230)
    }
    #[doc = "0x4cc - FUNC231_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func231_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(231)
    }
    #[doc = "0x4d0 - FUNC232_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func232_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(232)
    }
    #[doc = "0x4d4 - FUNC233_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func233_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(233)
    }
    #[doc = "0x4d8 - FUNC234_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func234_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(234)
    }
    #[doc = "0x4dc - FUNC235_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func235_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(235)
    }
    #[doc = "0x4e0 - FUNC236_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func236_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(236)
    }
    #[doc = "0x4e4 - FUNC237_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func237_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(237)
    }
    #[doc = "0x4e8 - FUNC238_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func238_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(238)
    }
    #[doc = "0x4ec - FUNC239_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func239_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(239)
    }
    #[doc = "0x4f0 - FUNC240_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func240_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(240)
    }
    #[doc = "0x4f4 - FUNC241_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func241_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(241)
    }
    #[doc = "0x4f8 - FUNC242_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func242_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(242)
    }
    #[doc = "0x4fc - FUNC243_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func243_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(243)
    }
    #[doc = "0x500 - FUNC244_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func244_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(244)
    }
    #[doc = "0x504 - FUNC245_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func245_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(245)
    }
    #[doc = "0x508 - FUNC246_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func246_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(246)
    }
    #[doc = "0x50c - FUNC247_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func247_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(247)
    }
    #[doc = "0x510 - FUNC248_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func248_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(248)
    }
    #[doc = "0x514 - FUNC249_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func249_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(249)
    }
    #[doc = "0x518 - FUNC250_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func250_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(250)
    }
    #[doc = "0x51c - FUNC251_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func251_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(251)
    }
    #[doc = "0x520 - FUNC252_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func252_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(252)
    }
    #[doc = "0x524 - FUNC253_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func253_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(253)
    }
    #[doc = "0x528 - FUNC254_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func254_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(254)
    }
    #[doc = "0x52c - FUNC255_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func255_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(255)
    }
    #[doc = "0x530..0x5d0 - "]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x530..0x5d0 - "]
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    #[doc = "0x530 - FUNC0_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    #[doc = "0x534 - FUNC1_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    #[doc = "0x538 - FUNC2_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    #[doc = "0x53c - FUNC3_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    #[doc = "0x540 - FUNC4_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    #[doc = "0x544 - FUNC5_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    #[doc = "0x548 - FUNC6_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    #[doc = "0x54c - FUNC7_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    #[doc = "0x550 - FUNC8_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    #[doc = "0x554 - FUNC9_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    #[doc = "0x558 - FUNC10_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    #[doc = "0x55c - FUNC11_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    #[doc = "0x560 - FUNC12_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    #[doc = "0x564 - FUNC13_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    #[doc = "0x568 - FUNC14_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    #[doc = "0x56c - FUNC15_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    #[doc = "0x570 - FUNC16_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    #[doc = "0x574 - FUNC17_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    #[doc = "0x578 - FUNC18_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    #[doc = "0x57c - FUNC19_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    #[doc = "0x580 - FUNC20_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    #[doc = "0x584 - FUNC21_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    #[doc = "0x588 - FUNC22_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    #[doc = "0x58c - FUNC23_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    #[doc = "0x590 - FUNC24_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    #[doc = "0x594 - FUNC25_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    #[doc = "0x598 - FUNC26_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    #[doc = "0x59c - FUNC27_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    #[doc = "0x5a0 - FUNC28_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    #[doc = "0x5a4 - FUNC29_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    #[doc = "0x5a8 - FUNC30_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    #[doc = "0x5ac - FUNC31_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    #[doc = "0x5b0 - FUNC32_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    #[doc = "0x5b4 - FUNC33_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
    }
    #[doc = "0x5b8 - FUNC34_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func34_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(34)
    }
    #[doc = "0x5bc - FUNC35_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func35_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(35)
    }
    #[doc = "0x5c0 - FUNC36_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func36_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(36)
    }
    #[doc = "0x5c4 - FUNC37_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func37_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(37)
    }
    #[doc = "0x5c8 - FUNC38_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func38_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(38)
    }
    #[doc = "0x5cc - FUNC39_OUT_SEL_CFG"]
    #[inline(always)]
    pub const fn func39_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(39)
    }
}
#[doc = "BT_SELECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_select`] module"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = ""]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = ""]
pub mod out;
#[doc = "OUT_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = ""]
pub mod out_w1ts;
#[doc = "OUT_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = ""]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = ""]
pub mod out1;
#[doc = "OUT1_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = ""]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = ""]
pub mod out1_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_select`] module"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = ""]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = ""]
pub mod enable;
#[doc = "ENABLE_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = ""]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = ""]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = ""]
pub mod enable1;
#[doc = "ENABLE1_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = ""]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = ""]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = ""]
pub mod strap;
#[doc = "IN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = ""]
pub mod in_;
#[doc = "IN1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = ""]
pub mod in1;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "STATUS_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = ""]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = ""]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = ""]
pub mod status1;
#[doc = "STATUS1_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = ""]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = ""]
pub mod status1_w1tc;
#[doc = "ACPU_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acpu_int`] module"]
pub type ACPU_INT = crate::Reg<acpu_int::ACPU_INT_SPEC>;
#[doc = ""]
pub mod acpu_int;
#[doc = "ACPU_NMI_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acpu_nmi_int`] module"]
pub type ACPU_NMI_INT = crate::Reg<acpu_nmi_int::ACPU_NMI_INT_SPEC>;
#[doc = ""]
pub mod acpu_nmi_int;
#[doc = "PCPU_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_int`] module"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = ""]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_nmi_int`] module"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = ""]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpusdio_int`] module"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = ""]
pub mod cpusdio_int;
#[doc = "ACPU_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acpu_int1`] module"]
pub type ACPU_INT1 = crate::Reg<acpu_int1::ACPU_INT1_SPEC>;
#[doc = ""]
pub mod acpu_int1;
#[doc = "ACPU_NMI_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acpu_nmi_int1`] module"]
pub type ACPU_NMI_INT1 = crate::Reg<acpu_nmi_int1::ACPU_NMI_INT1_SPEC>;
#[doc = ""]
pub mod acpu_nmi_int1;
#[doc = "PCPU_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_int1`] module"]
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
#[doc = ""]
pub mod pcpu_int1;
#[doc = "PCPU_NMI_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_nmi_int1`] module"]
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
#[doc = ""]
pub mod pcpu_nmi_int1;
#[doc = "CPUSDIO_INT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpusdio_int1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpusdio_int1`] module"]
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
#[doc = ""]
pub mod cpusdio_int1;
#[doc = "PIN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "cali_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cali_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali_conf`] module"]
#[doc(alias = "cali_conf")]
pub type CALI_CONF = crate::Reg<cali_conf::CALI_CONF_SPEC>;
#[doc = ""]
pub mod cali_conf;
#[doc = "cali_data (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cali_data`] module"]
#[doc(alias = "cali_data")]
pub type CALI_DATA = crate::Reg<cali_data::CALI_DATA_SPEC>;
#[doc = ""]
pub mod cali_data;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_out_sel_cfg;
