#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
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
    pcpu_int: PCPU_INT,
    pcpu_nmi_int: PCPU_NMI_INT,
    cpusdio_int: CPUSDIO_INT,
    pcpu_int1: PCPU_INT1,
    pcpu_nmi_int1: PCPU_NMI_INT1,
    cpusdio_int1: CPUSDIO_INT1,
    pin: [PIN; 54],
    status_next: STATUS_NEXT,
    status_next1: STATUS_NEXT1,
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 256],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 54],
    clock_gate: CLOCK_GATE,
    _reserved35: [u8; 0xcc],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO bit select register"]
    #[inline(always)]
    pub const fn bt_select(&self) -> &BT_SELECT {
        &self.bt_select
    }
    #[doc = "0x04 - GPIO output register for GPIO0-31"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x08 - GPIO output set register for GPIO0-31"]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x0c - GPIO output clear register for GPIO0-31"]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x10 - GPIO output register for GPIO32-53"]
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    #[doc = "0x14 - GPIO output set register for GPIO32-53"]
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    #[doc = "0x18 - GPIO output clear register for GPIO32-53"]
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    #[doc = "0x1c - GPIO sdio select register"]
    #[inline(always)]
    pub const fn sdio_select(&self) -> &SDIO_SELECT {
        &self.sdio_select
    }
    #[doc = "0x20 - GPIO output enable register for GPIO0-31"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x24 - GPIO output enable set register for GPIO0-31"]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x28 - GPIO output enable clear register for GPIO0-31"]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x2c - GPIO output enable register for GPIO32-53"]
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    #[doc = "0x30 - GPIO output enable set register for GPIO32-53"]
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    #[doc = "0x34 - GPIO output enable clear register for GPIO32-53"]
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    #[doc = "0x38 - pad strapping register"]
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    #[doc = "0x3c - GPIO input register for GPIO0-31"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x40 - GPIO input register for GPIO32-53"]
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    #[doc = "0x44 - GPIO interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x48 - GPIO interrupt status set register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x4c - GPIO interrupt status clear register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x50 - GPIO interrupt status register for GPIO32-53"]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x54 - GPIO interrupt status set register for GPIO32-53"]
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    #[doc = "0x58 - GPIO interrupt status clear register for GPIO32-53"]
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    #[doc = "0x5c - GPIO PRO_CPU interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn pcpu_int(&self) -> &PCPU_INT {
        &self.pcpu_int
    }
    #[doc = "0x60 - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn pcpu_nmi_int(&self) -> &PCPU_NMI_INT {
        &self.pcpu_nmi_int
    }
    #[doc = "0x64 - GPIO CPUSDIO interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn cpusdio_int(&self) -> &CPUSDIO_INT {
        &self.cpusdio_int
    }
    #[doc = "0x68 - GPIO PRO_CPU interrupt status register for GPIO32-53"]
    #[inline(always)]
    pub const fn pcpu_int1(&self) -> &PCPU_INT1 {
        &self.pcpu_int1
    }
    #[doc = "0x6c - GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-53"]
    #[inline(always)]
    pub const fn pcpu_nmi_int1(&self) -> &PCPU_NMI_INT1 {
        &self.pcpu_nmi_int1
    }
    #[doc = "0x70 - GPIO CPUSDIO interrupt status register for GPIO32-53"]
    #[inline(always)]
    pub const fn cpusdio_int1(&self) -> &CPUSDIO_INT1 {
        &self.cpusdio_int1
    }
    #[doc = "0x74..0x14c - GPIO pin configuration register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x14c - GPIO pin configuration register"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x14c - GPIO interrupt source register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0x150 - GPIO interrupt source register for GPIO32-53"]
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    #[doc = "0x154..0x554 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x154..0x554 - GPIO input function configuration register"]
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    #[doc = "0x154 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    #[doc = "0x158 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    #[doc = "0x15c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    #[doc = "0x160 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    #[doc = "0x164 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    #[doc = "0x168 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    #[doc = "0x16c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    #[doc = "0x170 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    #[doc = "0x174 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    #[doc = "0x178 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    #[doc = "0x17c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    #[doc = "0x180 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    #[doc = "0x184 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    #[doc = "0x188 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    #[doc = "0x18c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    #[doc = "0x190 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    #[doc = "0x194 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    #[doc = "0x198 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    #[doc = "0x19c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    #[doc = "0x1a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    #[doc = "0x1a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    #[doc = "0x1a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    #[doc = "0x1ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    #[doc = "0x1b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    #[doc = "0x1b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    #[doc = "0x1b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    #[doc = "0x1bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    #[doc = "0x1c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    #[doc = "0x1c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    #[doc = "0x1c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    #[doc = "0x1cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    #[doc = "0x1d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    #[doc = "0x1d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    #[doc = "0x1d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    #[doc = "0x1dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    #[doc = "0x1e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    #[doc = "0x1e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    #[doc = "0x1e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    #[doc = "0x1ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    #[doc = "0x1f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    #[doc = "0x1f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    #[doc = "0x1f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    #[doc = "0x1fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    #[doc = "0x200 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    #[doc = "0x204 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    #[doc = "0x208 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    #[doc = "0x20c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    #[doc = "0x210 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    #[doc = "0x214 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(48)
    }
    #[doc = "0x218 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(49)
    }
    #[doc = "0x21c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(50)
    }
    #[doc = "0x220 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(51)
    }
    #[doc = "0x224 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(52)
    }
    #[doc = "0x228 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(53)
    }
    #[doc = "0x22c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(54)
    }
    #[doc = "0x230 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(55)
    }
    #[doc = "0x234 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(56)
    }
    #[doc = "0x238 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(57)
    }
    #[doc = "0x23c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(58)
    }
    #[doc = "0x240 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(59)
    }
    #[doc = "0x244 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(60)
    }
    #[doc = "0x248 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(61)
    }
    #[doc = "0x24c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(62)
    }
    #[doc = "0x250 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(63)
    }
    #[doc = "0x254 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(64)
    }
    #[doc = "0x258 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(65)
    }
    #[doc = "0x25c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(66)
    }
    #[doc = "0x260 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(67)
    }
    #[doc = "0x264 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(68)
    }
    #[doc = "0x268 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(69)
    }
    #[doc = "0x26c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(70)
    }
    #[doc = "0x270 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(71)
    }
    #[doc = "0x274 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(72)
    }
    #[doc = "0x278 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(73)
    }
    #[doc = "0x27c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(74)
    }
    #[doc = "0x280 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(75)
    }
    #[doc = "0x284 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(76)
    }
    #[doc = "0x288 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(77)
    }
    #[doc = "0x28c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(78)
    }
    #[doc = "0x290 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(79)
    }
    #[doc = "0x294 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(80)
    }
    #[doc = "0x298 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(81)
    }
    #[doc = "0x29c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(82)
    }
    #[doc = "0x2a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(83)
    }
    #[doc = "0x2a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func84_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(84)
    }
    #[doc = "0x2a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func85_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(85)
    }
    #[doc = "0x2ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(86)
    }
    #[doc = "0x2b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(87)
    }
    #[doc = "0x2b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(88)
    }
    #[doc = "0x2b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(89)
    }
    #[doc = "0x2bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(90)
    }
    #[doc = "0x2c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(91)
    }
    #[doc = "0x2c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(92)
    }
    #[doc = "0x2c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(93)
    }
    #[doc = "0x2cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(94)
    }
    #[doc = "0x2d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(95)
    }
    #[doc = "0x2d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(96)
    }
    #[doc = "0x2d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(97)
    }
    #[doc = "0x2dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(98)
    }
    #[doc = "0x2e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(99)
    }
    #[doc = "0x2e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(100)
    }
    #[doc = "0x2e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(101)
    }
    #[doc = "0x2ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(102)
    }
    #[doc = "0x2f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(103)
    }
    #[doc = "0x2f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(104)
    }
    #[doc = "0x2f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(105)
    }
    #[doc = "0x2fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(106)
    }
    #[doc = "0x300 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(107)
    }
    #[doc = "0x304 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(108)
    }
    #[doc = "0x308 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(109)
    }
    #[doc = "0x30c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(110)
    }
    #[doc = "0x310 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(111)
    }
    #[doc = "0x314 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(112)
    }
    #[doc = "0x318 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(113)
    }
    #[doc = "0x31c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(114)
    }
    #[doc = "0x320 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(115)
    }
    #[doc = "0x324 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(116)
    }
    #[doc = "0x328 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(117)
    }
    #[doc = "0x32c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(118)
    }
    #[doc = "0x330 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(119)
    }
    #[doc = "0x334 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(120)
    }
    #[doc = "0x338 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(121)
    }
    #[doc = "0x33c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(122)
    }
    #[doc = "0x340 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(123)
    }
    #[doc = "0x344 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(124)
    }
    #[doc = "0x348 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func125_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(125)
    }
    #[doc = "0x34c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(126)
    }
    #[doc = "0x350 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(127)
    }
    #[doc = "0x354 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func128_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(128)
    }
    #[doc = "0x358 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func129_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(129)
    }
    #[doc = "0x35c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func130_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(130)
    }
    #[doc = "0x360 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func131_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(131)
    }
    #[doc = "0x364 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func132_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(132)
    }
    #[doc = "0x368 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func133_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(133)
    }
    #[doc = "0x36c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func134_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(134)
    }
    #[doc = "0x370 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func135_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(135)
    }
    #[doc = "0x374 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func136_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(136)
    }
    #[doc = "0x378 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func137_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(137)
    }
    #[doc = "0x37c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func138_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(138)
    }
    #[doc = "0x380 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func139_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(139)
    }
    #[doc = "0x384 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func140_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(140)
    }
    #[doc = "0x388 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func141_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(141)
    }
    #[doc = "0x38c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func142_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(142)
    }
    #[doc = "0x390 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func143_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(143)
    }
    #[doc = "0x394 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func144_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(144)
    }
    #[doc = "0x398 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func145_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(145)
    }
    #[doc = "0x39c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func146_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(146)
    }
    #[doc = "0x3a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func147_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(147)
    }
    #[doc = "0x3a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func148_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(148)
    }
    #[doc = "0x3a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func149_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(149)
    }
    #[doc = "0x3ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func150_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(150)
    }
    #[doc = "0x3b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func151_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(151)
    }
    #[doc = "0x3b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func152_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(152)
    }
    #[doc = "0x3b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func153_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(153)
    }
    #[doc = "0x3bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func154_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(154)
    }
    #[doc = "0x3c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func155_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(155)
    }
    #[doc = "0x3c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func156_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(156)
    }
    #[doc = "0x3c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func157_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(157)
    }
    #[doc = "0x3cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func158_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(158)
    }
    #[doc = "0x3d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func159_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(159)
    }
    #[doc = "0x3d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func160_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(160)
    }
    #[doc = "0x3d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func161_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(161)
    }
    #[doc = "0x3dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func162_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(162)
    }
    #[doc = "0x3e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func163_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(163)
    }
    #[doc = "0x3e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func164_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(164)
    }
    #[doc = "0x3e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func165_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(165)
    }
    #[doc = "0x3ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func166_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(166)
    }
    #[doc = "0x3f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func167_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(167)
    }
    #[doc = "0x3f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func168_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(168)
    }
    #[doc = "0x3f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func169_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(169)
    }
    #[doc = "0x3fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func170_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(170)
    }
    #[doc = "0x400 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func171_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(171)
    }
    #[doc = "0x404 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func172_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(172)
    }
    #[doc = "0x408 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func173_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(173)
    }
    #[doc = "0x40c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func174_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(174)
    }
    #[doc = "0x410 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func175_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(175)
    }
    #[doc = "0x414 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func176_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(176)
    }
    #[doc = "0x418 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func177_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(177)
    }
    #[doc = "0x41c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func178_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(178)
    }
    #[doc = "0x420 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func179_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(179)
    }
    #[doc = "0x424 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func180_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(180)
    }
    #[doc = "0x428 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func181_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(181)
    }
    #[doc = "0x42c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func182_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(182)
    }
    #[doc = "0x430 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func183_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(183)
    }
    #[doc = "0x434 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func184_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(184)
    }
    #[doc = "0x438 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func185_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(185)
    }
    #[doc = "0x43c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func186_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(186)
    }
    #[doc = "0x440 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func187_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(187)
    }
    #[doc = "0x444 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func188_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(188)
    }
    #[doc = "0x448 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func189_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(189)
    }
    #[doc = "0x44c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func190_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(190)
    }
    #[doc = "0x450 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func191_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(191)
    }
    #[doc = "0x454 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func192_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(192)
    }
    #[doc = "0x458 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func193_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(193)
    }
    #[doc = "0x45c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func194_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(194)
    }
    #[doc = "0x460 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func195_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(195)
    }
    #[doc = "0x464 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func196_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(196)
    }
    #[doc = "0x468 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func197_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(197)
    }
    #[doc = "0x46c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func198_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(198)
    }
    #[doc = "0x470 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func199_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(199)
    }
    #[doc = "0x474 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func200_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(200)
    }
    #[doc = "0x478 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func201_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(201)
    }
    #[doc = "0x47c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func202_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(202)
    }
    #[doc = "0x480 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func203_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(203)
    }
    #[doc = "0x484 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func204_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(204)
    }
    #[doc = "0x488 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func205_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(205)
    }
    #[doc = "0x48c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func206_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(206)
    }
    #[doc = "0x490 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func207_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(207)
    }
    #[doc = "0x494 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func208_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(208)
    }
    #[doc = "0x498 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func209_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(209)
    }
    #[doc = "0x49c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func210_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(210)
    }
    #[doc = "0x4a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func211_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(211)
    }
    #[doc = "0x4a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func212_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(212)
    }
    #[doc = "0x4a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func213_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(213)
    }
    #[doc = "0x4ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func214_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(214)
    }
    #[doc = "0x4b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func215_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(215)
    }
    #[doc = "0x4b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func216_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(216)
    }
    #[doc = "0x4b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func217_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(217)
    }
    #[doc = "0x4bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func218_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(218)
    }
    #[doc = "0x4c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func219_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(219)
    }
    #[doc = "0x4c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func220_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(220)
    }
    #[doc = "0x4c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func221_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(221)
    }
    #[doc = "0x4cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func222_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(222)
    }
    #[doc = "0x4d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func223_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(223)
    }
    #[doc = "0x4d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func224_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(224)
    }
    #[doc = "0x4d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func225_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(225)
    }
    #[doc = "0x4dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func226_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(226)
    }
    #[doc = "0x4e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func227_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(227)
    }
    #[doc = "0x4e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func228_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(228)
    }
    #[doc = "0x4e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func229_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(229)
    }
    #[doc = "0x4ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func230_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(230)
    }
    #[doc = "0x4f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func231_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(231)
    }
    #[doc = "0x4f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func232_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(232)
    }
    #[doc = "0x4f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func233_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(233)
    }
    #[doc = "0x4fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func234_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(234)
    }
    #[doc = "0x500 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func235_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(235)
    }
    #[doc = "0x504 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func236_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(236)
    }
    #[doc = "0x508 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func237_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(237)
    }
    #[doc = "0x50c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func238_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(238)
    }
    #[doc = "0x510 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func239_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(239)
    }
    #[doc = "0x514 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func240_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(240)
    }
    #[doc = "0x518 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func241_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(241)
    }
    #[doc = "0x51c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func242_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(242)
    }
    #[doc = "0x520 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func243_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(243)
    }
    #[doc = "0x524 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func244_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(244)
    }
    #[doc = "0x528 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func245_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(245)
    }
    #[doc = "0x52c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func246_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(246)
    }
    #[doc = "0x530 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func247_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(247)
    }
    #[doc = "0x534 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func248_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(248)
    }
    #[doc = "0x538 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func249_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(249)
    }
    #[doc = "0x53c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func250_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(250)
    }
    #[doc = "0x540 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func251_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(251)
    }
    #[doc = "0x544 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func252_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(252)
    }
    #[doc = "0x548 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func253_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(253)
    }
    #[doc = "0x54c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func254_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(254)
    }
    #[doc = "0x550 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func255_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(255)
    }
    #[doc = "0x554..0x62c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x554..0x62c - GPIO output function select register"]
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    #[doc = "0x554 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    #[doc = "0x558 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    #[doc = "0x55c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    #[doc = "0x560 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    #[doc = "0x564 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    #[doc = "0x568 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    #[doc = "0x56c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    #[doc = "0x570 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    #[doc = "0x574 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    #[doc = "0x578 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    #[doc = "0x57c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    #[doc = "0x580 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    #[doc = "0x584 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    #[doc = "0x588 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    #[doc = "0x58c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    #[doc = "0x590 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    #[doc = "0x594 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    #[doc = "0x598 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    #[doc = "0x59c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    #[doc = "0x5a0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    #[doc = "0x5a4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    #[doc = "0x5a8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    #[doc = "0x5ac - GPIO output function select register"]
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    #[doc = "0x5b0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    #[doc = "0x5b4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    #[doc = "0x5b8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    #[doc = "0x5bc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    #[doc = "0x5c0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    #[doc = "0x5c4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    #[doc = "0x5c8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    #[doc = "0x5cc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    #[doc = "0x5d0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    #[doc = "0x5d4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    #[doc = "0x5d8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
    }
    #[doc = "0x5dc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func34_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(34)
    }
    #[doc = "0x5e0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func35_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(35)
    }
    #[doc = "0x5e4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func36_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(36)
    }
    #[doc = "0x5e8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func37_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(37)
    }
    #[doc = "0x5ec - GPIO output function select register"]
    #[inline(always)]
    pub const fn func38_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(38)
    }
    #[doc = "0x5f0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func39_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(39)
    }
    #[doc = "0x5f4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func40_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(40)
    }
    #[doc = "0x5f8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func41_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(41)
    }
    #[doc = "0x5fc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func42_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(42)
    }
    #[doc = "0x600 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func43_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(43)
    }
    #[doc = "0x604 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func44_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(44)
    }
    #[doc = "0x608 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func45_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(45)
    }
    #[doc = "0x60c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func46_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(46)
    }
    #[doc = "0x610 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func47_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(47)
    }
    #[doc = "0x614 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func48_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(48)
    }
    #[doc = "0x618 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func49_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(49)
    }
    #[doc = "0x61c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func50_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(50)
    }
    #[doc = "0x620 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func51_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(51)
    }
    #[doc = "0x624 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func52_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(52)
    }
    #[doc = "0x628 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func53_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(53)
    }
    #[doc = "0x62c - GPIO clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x6fc - GPIO version register"]
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
#[doc = "BT_SELECT (rw) register accessor: GPIO bit select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_select`] module"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = "GPIO bit select register"]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: GPIO output register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register for GPIO0-31"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: GPIO output set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO0-31"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: GPIO output clear register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO0-31"]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: GPIO output register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register for GPIO32-53"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: GPIO output set register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO32-53"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: GPIO output clear register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO32-53"]
pub mod out1_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: GPIO sdio select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_select`] module"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = "GPIO sdio select register"]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: GPIO output enable register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register for GPIO0-31"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: GPIO output enable set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO0-31"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: GPIO output enable clear register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO0-31"]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: GPIO output enable register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register for GPIO32-53"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: GPIO output enable set register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO32-53"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: GPIO output enable clear register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO32-53"]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: pad strapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "IN (rw) register accessor: GPIO input register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register for GPIO0-31"]
pub mod in_;
#[doc = "IN1 (rw) register accessor: GPIO input register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register for GPIO32-53"]
pub mod in1;
#[doc = "STATUS (rw) register accessor: GPIO interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register for GPIO0-31"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: GPIO interrupt status set register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO0-31"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO0-31\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO0-31"]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: GPIO interrupt status register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register for GPIO32-53"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: GPIO interrupt status set register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO32-53"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO32-53"]
pub mod status1_w1tc;
#[doc = "PCPU_INT (r) register accessor: GPIO PRO_CPU interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_int`] module"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register for GPIO0-31"]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_nmi_int`] module"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: GPIO CPUSDIO interrupt status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpusdio_int`] module"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register for GPIO0-31"]
pub mod cpusdio_int;
#[doc = "PCPU_INT1 (r) register accessor: GPIO PRO_CPU interrupt status register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_int1`] module"]
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register for GPIO32-53"]
pub mod pcpu_int1;
#[doc = "PCPU_NMI_INT1 (r) register accessor: GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcpu_nmi_int1`] module"]
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-53"]
pub mod pcpu_nmi_int1;
#[doc = "CPUSDIO_INT1 (r) register accessor: GPIO CPUSDIO interrupt status register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpusdio_int1`] module"]
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register for GPIO32-53"]
pub mod cpusdio_int1;
#[doc = "PIN (rw) register accessor: GPIO pin configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register for GPIO0-31"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: GPIO interrupt source register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next1`] module"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register for GPIO32-53"]
pub mod status_next1;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: GPIO output function select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func_out_sel_cfg;
#[doc = "CLOCK_GATE (rw) register accessor: GPIO clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "REG_DATE (rw) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_date`] module"]
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod reg_date;
