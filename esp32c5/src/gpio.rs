#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    strap: STRAP,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    out1: OUT1,
    out1_w1ts: OUT1_W1TS,
    out1_w1tc: OUT1_W1TC,
    _reserved7: [u8; 0x18],
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    enable1: ENABLE1,
    enable1_w1ts: ENABLE1_W1TS,
    enable1_w1tc: ENABLE1_W1TC,
    _reserved13: [u8; 0x18],
    in_: IN,
    in1: IN1,
    _reserved15: [u8; 0x08],
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    status1: STATUS1,
    status1_w1ts: STATUS1_W1TS,
    status1_w1tc: STATUS1_W1TC,
    _reserved21: [u8; 0x18],
    procpu_int: PROCPU_INT,
    procpu_int1: PROCPU_INT1,
    _reserved23: [u8; 0x08],
    status_next: STATUS_NEXT,
    status_next1: STATUS_NEXT1,
    _reserved25: [u8; 0x08],
    pin: [PIN; 29],
    pin29: PIN29,
    pin30: PIN30,
    pin31: PIN31,
    pin32: PIN32,
    _reserved30: [u8; 0x017c],
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 128],
    _reserved31: [u8; 0x0600],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 29],
    func29_out_sel_cfg: FUNC29_OUT_SEL_CFG,
    func30_out_sel_cfg: FUNC30_OUT_SEL_CFG,
    func31_out_sel_cfg: FUNC31_OUT_SEL_CFG,
    func32_out_sel_cfg: FUNC32_OUT_SEL_CFG,
    _reserved36: [u8; 0x02b0],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Strapping pin register"]
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    #[doc = "0x04 - GPIO output register"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x08 - GPIO output set register"]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x0c - GPIO output clear register"]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x10 - GPIO output register for GPIO32-32"]
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    #[doc = "0x14 - GPIO output set register for GPIO32-32"]
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    #[doc = "0x18 - GPIO output clear register for GPIO32-32"]
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    #[doc = "0x34 - GPIO output enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x38 - GPIO output enable set register"]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x3c - GPIO output enable clear register"]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x40 - GPIO output enable register for GPIO32-32"]
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    #[doc = "0x44 - GPIO output enable set register for GPIO32-32"]
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    #[doc = "0x48 - GPIO output enable clear register for GPIO32-32"]
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    #[doc = "0x64 - GPIO input register"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x68 - GPIO input register for GPIO32-32"]
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    #[doc = "0x74 - GPIO interrupt status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x78 - GPIO interrupt status set register"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x7c - GPIO interrupt status clear register"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x80 - GPIO interrupt status register for GPIO32-32"]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x84 - GPIO interrupt status set register for GPIO32-32"]
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    #[doc = "0x88 - GPIO interrupt status clear register for GPIO32-32"]
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    #[doc = "0xa4 - GPIO_PROCPU_INT interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn procpu_int(&self) -> &PROCPU_INT {
        &self.procpu_int
    }
    #[doc = "0xa8 - GPIO_PROCPU_INT interrupt status register for GPIO32-32"]
    #[inline(always)]
    pub const fn procpu_int1(&self) -> &PROCPU_INT1 {
        &self.procpu_int1
    }
    #[doc = "0xb4 - GPIO interrupt source register"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0xb8 - GPIO interrupt source register for GPIO32-32"]
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    #[doc = "0xc4..0x138 - GPIO$N configuration register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc4..0x138 - GPIO$N configuration register"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x138 - GPIO29 configuration register"]
    #[inline(always)]
    pub const fn pin29(&self) -> &PIN29 {
        &self.pin29
    }
    #[doc = "0x13c - GPIO30 configuration register"]
    #[inline(always)]
    pub const fn pin30(&self) -> &PIN30 {
        &self.pin30
    }
    #[doc = "0x140 - GPIO31 configuration register"]
    #[inline(always)]
    pub const fn pin31(&self) -> &PIN31 {
        &self.pin31
    }
    #[doc = "0x144 - GPIO32 configuration register"]
    #[inline(always)]
    pub const fn pin32(&self) -> &PIN32 {
        &self.pin32
    }
    #[doc = "0x2c4..0x4c4 - "]
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c4..0x4c4 - "]
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    #[doc = "0x2c4 - FUNC0_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    #[doc = "0x2c8 - FUNC1_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    #[doc = "0x2cc - FUNC2_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    #[doc = "0x2d0 - FUNC3_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    #[doc = "0x2d4 - FUNC4_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    #[doc = "0x2d8 - FUNC5_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    #[doc = "0x2dc - FUNC6_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    #[doc = "0x2e0 - FUNC7_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    #[doc = "0x2e4 - FUNC8_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    #[doc = "0x2e8 - FUNC9_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    #[doc = "0x2ec - FUNC10_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    #[doc = "0x2f0 - FUNC11_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    #[doc = "0x2f4 - FUNC12_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    #[doc = "0x2f8 - FUNC13_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    #[doc = "0x2fc - FUNC14_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    #[doc = "0x300 - FUNC15_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    #[doc = "0x304 - FUNC16_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    #[doc = "0x308 - FUNC17_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    #[doc = "0x30c - FUNC18_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    #[doc = "0x310 - FUNC19_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    #[doc = "0x314 - FUNC20_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    #[doc = "0x318 - FUNC21_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    #[doc = "0x31c - FUNC22_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    #[doc = "0x320 - FUNC23_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    #[doc = "0x324 - FUNC24_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    #[doc = "0x328 - FUNC25_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    #[doc = "0x32c - FUNC26_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    #[doc = "0x330 - FUNC27_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    #[doc = "0x334 - FUNC28_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    #[doc = "0x338 - FUNC29_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    #[doc = "0x33c - FUNC30_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    #[doc = "0x340 - FUNC31_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    #[doc = "0x344 - FUNC32_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    #[doc = "0x348 - FUNC33_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    #[doc = "0x34c - FUNC34_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    #[doc = "0x350 - FUNC35_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    #[doc = "0x354 - FUNC36_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    #[doc = "0x358 - FUNC37_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    #[doc = "0x35c - FUNC38_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    #[doc = "0x360 - FUNC39_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    #[doc = "0x364 - FUNC40_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    #[doc = "0x368 - FUNC41_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    #[doc = "0x36c - FUNC42_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    #[doc = "0x370 - FUNC43_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    #[doc = "0x374 - FUNC44_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    #[doc = "0x378 - FUNC45_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    #[doc = "0x37c - FUNC46_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    #[doc = "0x380 - FUNC47_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    #[doc = "0x384 - FUNC48_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(48)
    }
    #[doc = "0x388 - FUNC49_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(49)
    }
    #[doc = "0x38c - FUNC50_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(50)
    }
    #[doc = "0x390 - FUNC51_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(51)
    }
    #[doc = "0x394 - FUNC52_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(52)
    }
    #[doc = "0x398 - FUNC53_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(53)
    }
    #[doc = "0x39c - FUNC54_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(54)
    }
    #[doc = "0x3a0 - FUNC55_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(55)
    }
    #[doc = "0x3a4 - FUNC56_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(56)
    }
    #[doc = "0x3a8 - FUNC57_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(57)
    }
    #[doc = "0x3ac - FUNC58_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(58)
    }
    #[doc = "0x3b0 - FUNC59_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(59)
    }
    #[doc = "0x3b4 - FUNC60_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(60)
    }
    #[doc = "0x3b8 - FUNC61_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(61)
    }
    #[doc = "0x3bc - FUNC62_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(62)
    }
    #[doc = "0x3c0 - FUNC63_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(63)
    }
    #[doc = "0x3c4 - FUNC64_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(64)
    }
    #[doc = "0x3c8 - FUNC65_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(65)
    }
    #[doc = "0x3cc - FUNC66_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(66)
    }
    #[doc = "0x3d0 - FUNC67_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(67)
    }
    #[doc = "0x3d4 - FUNC68_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(68)
    }
    #[doc = "0x3d8 - FUNC69_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(69)
    }
    #[doc = "0x3dc - FUNC70_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(70)
    }
    #[doc = "0x3e0 - FUNC71_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(71)
    }
    #[doc = "0x3e4 - FUNC72_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(72)
    }
    #[doc = "0x3e8 - FUNC73_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(73)
    }
    #[doc = "0x3ec - FUNC74_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(74)
    }
    #[doc = "0x3f0 - FUNC75_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(75)
    }
    #[doc = "0x3f4 - FUNC76_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(76)
    }
    #[doc = "0x3f8 - FUNC77_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(77)
    }
    #[doc = "0x3fc - FUNC78_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(78)
    }
    #[doc = "0x400 - FUNC79_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(79)
    }
    #[doc = "0x404 - FUNC80_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(80)
    }
    #[doc = "0x408 - FUNC81_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(81)
    }
    #[doc = "0x40c - FUNC82_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(82)
    }
    #[doc = "0x410 - FUNC83_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(83)
    }
    #[doc = "0x414 - FUNC84_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func84_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(84)
    }
    #[doc = "0x418 - FUNC85_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func85_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(85)
    }
    #[doc = "0x41c - FUNC86_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(86)
    }
    #[doc = "0x420 - FUNC87_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(87)
    }
    #[doc = "0x424 - FUNC88_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(88)
    }
    #[doc = "0x428 - FUNC89_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(89)
    }
    #[doc = "0x42c - FUNC90_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(90)
    }
    #[doc = "0x430 - FUNC91_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(91)
    }
    #[doc = "0x434 - FUNC92_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(92)
    }
    #[doc = "0x438 - FUNC93_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(93)
    }
    #[doc = "0x43c - FUNC94_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(94)
    }
    #[doc = "0x440 - FUNC95_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(95)
    }
    #[doc = "0x444 - FUNC96_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(96)
    }
    #[doc = "0x448 - FUNC97_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(97)
    }
    #[doc = "0x44c - FUNC98_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(98)
    }
    #[doc = "0x450 - FUNC99_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(99)
    }
    #[doc = "0x454 - FUNC100_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(100)
    }
    #[doc = "0x458 - FUNC101_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(101)
    }
    #[doc = "0x45c - FUNC102_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(102)
    }
    #[doc = "0x460 - FUNC103_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(103)
    }
    #[doc = "0x464 - FUNC104_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(104)
    }
    #[doc = "0x468 - FUNC105_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(105)
    }
    #[doc = "0x46c - FUNC106_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(106)
    }
    #[doc = "0x470 - FUNC107_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(107)
    }
    #[doc = "0x474 - FUNC108_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(108)
    }
    #[doc = "0x478 - FUNC109_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(109)
    }
    #[doc = "0x47c - FUNC110_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(110)
    }
    #[doc = "0x480 - FUNC111_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(111)
    }
    #[doc = "0x484 - FUNC112_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(112)
    }
    #[doc = "0x488 - FUNC113_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(113)
    }
    #[doc = "0x48c - FUNC114_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(114)
    }
    #[doc = "0x490 - FUNC115_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(115)
    }
    #[doc = "0x494 - FUNC116_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(116)
    }
    #[doc = "0x498 - FUNC117_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(117)
    }
    #[doc = "0x49c - FUNC118_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(118)
    }
    #[doc = "0x4a0 - FUNC119_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(119)
    }
    #[doc = "0x4a4 - FUNC120_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(120)
    }
    #[doc = "0x4a8 - FUNC121_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(121)
    }
    #[doc = "0x4ac - FUNC122_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(122)
    }
    #[doc = "0x4b0 - FUNC123_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(123)
    }
    #[doc = "0x4b4 - FUNC124_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(124)
    }
    #[doc = "0x4b8 - FUNC125_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func125_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(125)
    }
    #[doc = "0x4bc - FUNC126_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(126)
    }
    #[doc = "0x4c0 - FUNC127_IN_SEL_CFG"]
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(127)
    }
    #[doc = "0xac4..0xb38 - Configuration register for GPIO%s output"]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xac4..0xb38 - Configuration register for GPIO%s output"]
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    #[doc = "0xac4 - Configuration register for GPIO0 output"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    #[doc = "0xac8 - Configuration register for GPIO1 output"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    #[doc = "0xacc - Configuration register for GPIO2 output"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    #[doc = "0xad0 - Configuration register for GPIO3 output"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    #[doc = "0xad4 - Configuration register for GPIO4 output"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    #[doc = "0xad8 - Configuration register for GPIO5 output"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    #[doc = "0xadc - Configuration register for GPIO6 output"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    #[doc = "0xae0 - Configuration register for GPIO7 output"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    #[doc = "0xae4 - Configuration register for GPIO8 output"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    #[doc = "0xae8 - Configuration register for GPIO9 output"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    #[doc = "0xaec - Configuration register for GPIO10 output"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    #[doc = "0xaf0 - Configuration register for GPIO11 output"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    #[doc = "0xaf4 - Configuration register for GPIO12 output"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    #[doc = "0xaf8 - Configuration register for GPIO13 output"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    #[doc = "0xafc - Configuration register for GPIO14 output"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    #[doc = "0xb00 - Configuration register for GPIO15 output"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    #[doc = "0xb04 - Configuration register for GPIO16 output"]
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    #[doc = "0xb08 - Configuration register for GPIO17 output"]
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    #[doc = "0xb0c - Configuration register for GPIO18 output"]
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    #[doc = "0xb10 - Configuration register for GPIO19 output"]
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    #[doc = "0xb14 - Configuration register for GPIO20 output"]
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    #[doc = "0xb18 - Configuration register for GPIO21 output"]
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    #[doc = "0xb1c - Configuration register for GPIO22 output"]
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    #[doc = "0xb20 - Configuration register for GPIO23 output"]
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    #[doc = "0xb24 - Configuration register for GPIO24 output"]
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    #[doc = "0xb28 - Configuration register for GPIO25 output"]
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    #[doc = "0xb2c - Configuration register for GPIO26 output"]
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    #[doc = "0xb30 - Configuration register for GPIO27 output"]
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    #[doc = "0xb34 - Configuration register for GPIO28 output"]
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    #[doc = "0xb38 - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC29_OUT_SEL_CFG {
        &self.func29_out_sel_cfg
    }
    #[doc = "0xb3c - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC30_OUT_SEL_CFG {
        &self.func30_out_sel_cfg
    }
    #[doc = "0xb40 - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC31_OUT_SEL_CFG {
        &self.func31_out_sel_cfg
    }
    #[doc = "0xb44 - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC32_OUT_SEL_CFG {
        &self.func32_out_sel_cfg
    }
    #[doc = "0xdf8 - GPIO clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xdfc - GPIO version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "STRAP (r) register accessor: Strapping pin register\n\nYou can [`read`](crate::Reg::read) this register and get [`strap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "Strapping pin register"]
pub mod strap;
#[doc = "OUT (rw) register accessor: GPIO output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: GPIO output set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: GPIO output clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register"]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: GPIO output register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`out1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register for GPIO32-32"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: GPIO output set register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO32-32"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: GPIO output clear register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO32-32"]
pub mod out1_w1tc;
#[doc = "ENABLE (rw) register accessor: GPIO output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: GPIO output enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: GPIO output enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register"]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: GPIO output enable register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`enable1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register for GPIO32-32"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: GPIO output enable set register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO32-32"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: GPIO output enable clear register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO32-32"]
pub mod enable1_w1tc;
#[doc = "IN (r) register accessor: GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register"]
pub mod in_;
#[doc = "IN1 (r) register accessor: GPIO input register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`in1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register for GPIO32-32"]
pub mod in1;
#[doc = "STATUS (rw) register accessor: GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: GPIO interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: GPIO interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register"]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: GPIO interrupt status register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register for GPIO32-32"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: GPIO interrupt status set register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO32-32"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO32-32"]
pub mod status1_w1tc;
#[doc = "PROCPU_INT (r) register accessor: GPIO_PROCPU_INT interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procpu_int`] module"]
pub type PROCPU_INT = crate::Reg<procpu_int::PROCPU_INT_SPEC>;
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO0-31"]
pub mod procpu_int;
#[doc = "PROCPU_INT1 (r) register accessor: GPIO_PROCPU_INT interrupt status register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procpu_int1`] module"]
pub type PROCPU_INT1 = crate::Reg<procpu_int1::PROCPU_INT1_SPEC>;
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO32-32"]
pub mod procpu_int1;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: GPIO interrupt source register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next1`] module"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register for GPIO32-32"]
pub mod status_next1;
#[doc = "PIN (rw) register accessor: GPIO$N configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO$N configuration register"]
pub mod pin;
pub use pin as pin29;
pub use pin as pin30;
pub use pin as pin31;
pub use pin as pin32;
pub use PIN as PIN29;
pub use PIN as PIN30;
pub use PIN as PIN31;
pub use PIN as PIN32;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO%s output\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO%s output"]
pub mod func_out_sel_cfg;
#[doc = "FUNC29_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func29_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func29_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func29_out_sel_cfg`] module"]
pub type FUNC29_OUT_SEL_CFG = crate::Reg<func29_out_sel_cfg::FUNC29_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func29_out_sel_cfg;
#[doc = "FUNC30_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func30_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func30_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func30_out_sel_cfg`] module"]
pub type FUNC30_OUT_SEL_CFG = crate::Reg<func30_out_sel_cfg::FUNC30_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func30_out_sel_cfg;
#[doc = "FUNC31_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func31_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func31_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func31_out_sel_cfg`] module"]
pub type FUNC31_OUT_SEL_CFG = crate::Reg<func31_out_sel_cfg::FUNC31_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func31_out_sel_cfg;
#[doc = "FUNC32_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func32_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func32_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func32_out_sel_cfg`] module"]
pub type FUNC32_OUT_SEL_CFG = crate::Reg<func32_out_sel_cfg::FUNC32_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func32_out_sel_cfg;
#[doc = "CLOCK_GATE (rw) register accessor: GPIO clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: GPIO version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod date;
