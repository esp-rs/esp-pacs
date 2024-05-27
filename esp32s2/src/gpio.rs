#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    ///0x00 - GPIO bit select register
    #[inline(always)]
    pub const fn bt_select(&self) -> &BT_SELECT {
        &self.bt_select
    }
    ///0x04 - GPIO0 ~ 31 output register
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    ///0x08 - GPIO0 ~ 31 output bit set register
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    ///0x0c - GPIO0 ~ 31 output bit clear register
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    ///0x10 - GPIO32 ~ 53 output register
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    ///0x14 - GPIO32 ~ 53 output bit set register
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    ///0x18 - GPIO32 ~ 53 output bit clear register
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    ///0x1c - GPIO SDIO selection register
    #[inline(always)]
    pub const fn sdio_select(&self) -> &SDIO_SELECT {
        &self.sdio_select
    }
    ///0x20 - GPIO0 ~ 31 output enable register
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    ///0x24 - GPIO0 ~ 31 output enable bit set register
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    ///0x28 - GPIO0 ~ 31 output enable bit clear register
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    ///0x2c - GPIO32 ~ 53 output enable register
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    ///0x30 - GPIO32 ~ 53 output enable bit set register
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    ///0x34 - GPIO32 ~ 53 output enable bit clear register
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    ///0x38 - Bootstrap pin value register
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    ///0x3c - GPIO0 ~ 31 input register
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    ///0x40 - GPIO32 ~ 53 input register
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    ///0x44 - GPIO0 ~ 31 interrupt status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x48 - GPIO0 ~ 31 interrupt status bit set register
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    ///0x4c - GPIO0 ~ 31 interrupt status bit clear register
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    ///0x50 - GPIO32 ~ 53 interrupt status register
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    ///0x54 - GPIO32 ~ 53 interrupt status bit set register
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    ///0x58 - GPIO32 ~ 53 interrupt status bit clear register
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    ///0x5c - GPIO0 ~ 31 PRO_CPU interrupt status register
    #[inline(always)]
    pub const fn pcpu_int(&self) -> &PCPU_INT {
        &self.pcpu_int
    }
    ///0x60 - GPIO0 ~ 31 PRO_CPU non-maskable interrupt status register
    #[inline(always)]
    pub const fn pcpu_nmi_int(&self) -> &PCPU_NMI_INT {
        &self.pcpu_nmi_int
    }
    ///0x64 - GPIO0 ~ 31 CPU SDIO interrupt status register
    #[inline(always)]
    pub const fn cpusdio_int(&self) -> &CPUSDIO_INT {
        &self.cpusdio_int
    }
    ///0x68 - GPIO32 ~ 53 PRO_CPU interrupt status register
    #[inline(always)]
    pub const fn pcpu_int1(&self) -> &PCPU_INT1 {
        &self.pcpu_int1
    }
    ///0x6c - GPIO32 ~ 53 PRO_CPU non-maskable interrupt status register
    #[inline(always)]
    pub const fn pcpu_nmi_int1(&self) -> &PCPU_NMI_INT1 {
        &self.pcpu_nmi_int1
    }
    ///0x70 - GPIO32 ~ 53 CPU SDIO interrupt status register
    #[inline(always)]
    pub const fn cpusdio_int1(&self) -> &CPUSDIO_INT1 {
        &self.cpusdio_int1
    }
    ///0x74..0x14c - Configuration for GPIO pin %s
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    ///Iterator for array of:
    ///0x74..0x14c - Configuration for GPIO pin %s
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    ///0x14c - GPIO0 ~ 31 interrupt source register
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    ///0x150 - GPIO32 ~ 53 interrupt source register
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    ///0x154..0x554 - Peripheral function %s input selection register
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    ///Iterator for array of:
    ///0x154..0x554 - Peripheral function %s input selection register
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    ///0x154 - Peripheral function 0 input selection register
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    ///0x158 - Peripheral function 1 input selection register
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    ///0x15c - Peripheral function 2 input selection register
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    ///0x160 - Peripheral function 3 input selection register
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    ///0x164 - Peripheral function 4 input selection register
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    ///0x168 - Peripheral function 5 input selection register
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    ///0x16c - Peripheral function 6 input selection register
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    ///0x170 - Peripheral function 7 input selection register
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    ///0x174 - Peripheral function 8 input selection register
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    ///0x178 - Peripheral function 9 input selection register
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    ///0x17c - Peripheral function 10 input selection register
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    ///0x180 - Peripheral function 11 input selection register
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    ///0x184 - Peripheral function 12 input selection register
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    ///0x188 - Peripheral function 13 input selection register
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    ///0x18c - Peripheral function 14 input selection register
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    ///0x190 - Peripheral function 15 input selection register
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    ///0x194 - Peripheral function 16 input selection register
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    ///0x198 - Peripheral function 17 input selection register
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    ///0x19c - Peripheral function 18 input selection register
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    ///0x1a0 - Peripheral function 19 input selection register
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    ///0x1a4 - Peripheral function 20 input selection register
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    ///0x1a8 - Peripheral function 21 input selection register
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    ///0x1ac - Peripheral function 22 input selection register
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    ///0x1b0 - Peripheral function 23 input selection register
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    ///0x1b4 - Peripheral function 24 input selection register
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    ///0x1b8 - Peripheral function 25 input selection register
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    ///0x1bc - Peripheral function 26 input selection register
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    ///0x1c0 - Peripheral function 27 input selection register
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    ///0x1c4 - Peripheral function 28 input selection register
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    ///0x1c8 - Peripheral function 29 input selection register
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    ///0x1cc - Peripheral function 30 input selection register
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    ///0x1d0 - Peripheral function 31 input selection register
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    ///0x1d4 - Peripheral function 32 input selection register
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    ///0x1d8 - Peripheral function 33 input selection register
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    ///0x1dc - Peripheral function 34 input selection register
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    ///0x1e0 - Peripheral function 35 input selection register
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    ///0x1e4 - Peripheral function 36 input selection register
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    ///0x1e8 - Peripheral function 37 input selection register
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    ///0x1ec - Peripheral function 38 input selection register
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    ///0x1f0 - Peripheral function 39 input selection register
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    ///0x1f4 - Peripheral function 40 input selection register
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    ///0x1f8 - Peripheral function 41 input selection register
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    ///0x1fc - Peripheral function 42 input selection register
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    ///0x200 - Peripheral function 43 input selection register
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    ///0x204 - Peripheral function 44 input selection register
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    ///0x208 - Peripheral function 45 input selection register
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    ///0x20c - Peripheral function 46 input selection register
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    ///0x210 - Peripheral function 47 input selection register
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    ///0x214 - Peripheral function 48 input selection register
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(48)
    }
    ///0x218 - Peripheral function 49 input selection register
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(49)
    }
    ///0x21c - Peripheral function 50 input selection register
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(50)
    }
    ///0x220 - Peripheral function 51 input selection register
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(51)
    }
    ///0x224 - Peripheral function 52 input selection register
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(52)
    }
    ///0x228 - Peripheral function 53 input selection register
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(53)
    }
    ///0x22c - Peripheral function 54 input selection register
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(54)
    }
    ///0x230 - Peripheral function 55 input selection register
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(55)
    }
    ///0x234 - Peripheral function 56 input selection register
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(56)
    }
    ///0x238 - Peripheral function 57 input selection register
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(57)
    }
    ///0x23c - Peripheral function 58 input selection register
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(58)
    }
    ///0x240 - Peripheral function 59 input selection register
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(59)
    }
    ///0x244 - Peripheral function 60 input selection register
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(60)
    }
    ///0x248 - Peripheral function 61 input selection register
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(61)
    }
    ///0x24c - Peripheral function 62 input selection register
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(62)
    }
    ///0x250 - Peripheral function 63 input selection register
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(63)
    }
    ///0x254 - Peripheral function 64 input selection register
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(64)
    }
    ///0x258 - Peripheral function 65 input selection register
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(65)
    }
    ///0x25c - Peripheral function 66 input selection register
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(66)
    }
    ///0x260 - Peripheral function 67 input selection register
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(67)
    }
    ///0x264 - Peripheral function 68 input selection register
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(68)
    }
    ///0x268 - Peripheral function 69 input selection register
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(69)
    }
    ///0x26c - Peripheral function 70 input selection register
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(70)
    }
    ///0x270 - Peripheral function 71 input selection register
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(71)
    }
    ///0x274 - Peripheral function 72 input selection register
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(72)
    }
    ///0x278 - Peripheral function 73 input selection register
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(73)
    }
    ///0x27c - Peripheral function 74 input selection register
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(74)
    }
    ///0x280 - Peripheral function 75 input selection register
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(75)
    }
    ///0x284 - Peripheral function 76 input selection register
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(76)
    }
    ///0x288 - Peripheral function 77 input selection register
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(77)
    }
    ///0x28c - Peripheral function 78 input selection register
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(78)
    }
    ///0x290 - Peripheral function 79 input selection register
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(79)
    }
    ///0x294 - Peripheral function 80 input selection register
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(80)
    }
    ///0x298 - Peripheral function 81 input selection register
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(81)
    }
    ///0x29c - Peripheral function 82 input selection register
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(82)
    }
    ///0x2a0 - Peripheral function 83 input selection register
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(83)
    }
    ///0x2a4 - Peripheral function 84 input selection register
    #[inline(always)]
    pub const fn func84_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(84)
    }
    ///0x2a8 - Peripheral function 85 input selection register
    #[inline(always)]
    pub const fn func85_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(85)
    }
    ///0x2ac - Peripheral function 86 input selection register
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(86)
    }
    ///0x2b0 - Peripheral function 87 input selection register
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(87)
    }
    ///0x2b4 - Peripheral function 88 input selection register
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(88)
    }
    ///0x2b8 - Peripheral function 89 input selection register
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(89)
    }
    ///0x2bc - Peripheral function 90 input selection register
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(90)
    }
    ///0x2c0 - Peripheral function 91 input selection register
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(91)
    }
    ///0x2c4 - Peripheral function 92 input selection register
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(92)
    }
    ///0x2c8 - Peripheral function 93 input selection register
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(93)
    }
    ///0x2cc - Peripheral function 94 input selection register
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(94)
    }
    ///0x2d0 - Peripheral function 95 input selection register
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(95)
    }
    ///0x2d4 - Peripheral function 96 input selection register
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(96)
    }
    ///0x2d8 - Peripheral function 97 input selection register
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(97)
    }
    ///0x2dc - Peripheral function 98 input selection register
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(98)
    }
    ///0x2e0 - Peripheral function 99 input selection register
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(99)
    }
    ///0x2e4 - Peripheral function 100 input selection register
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(100)
    }
    ///0x2e8 - Peripheral function 101 input selection register
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(101)
    }
    ///0x2ec - Peripheral function 102 input selection register
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(102)
    }
    ///0x2f0 - Peripheral function 103 input selection register
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(103)
    }
    ///0x2f4 - Peripheral function 104 input selection register
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(104)
    }
    ///0x2f8 - Peripheral function 105 input selection register
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(105)
    }
    ///0x2fc - Peripheral function 106 input selection register
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(106)
    }
    ///0x300 - Peripheral function 107 input selection register
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(107)
    }
    ///0x304 - Peripheral function 108 input selection register
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(108)
    }
    ///0x308 - Peripheral function 109 input selection register
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(109)
    }
    ///0x30c - Peripheral function 110 input selection register
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(110)
    }
    ///0x310 - Peripheral function 111 input selection register
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(111)
    }
    ///0x314 - Peripheral function 112 input selection register
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(112)
    }
    ///0x318 - Peripheral function 113 input selection register
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(113)
    }
    ///0x31c - Peripheral function 114 input selection register
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(114)
    }
    ///0x320 - Peripheral function 115 input selection register
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(115)
    }
    ///0x324 - Peripheral function 116 input selection register
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(116)
    }
    ///0x328 - Peripheral function 117 input selection register
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(117)
    }
    ///0x32c - Peripheral function 118 input selection register
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(118)
    }
    ///0x330 - Peripheral function 119 input selection register
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(119)
    }
    ///0x334 - Peripheral function 120 input selection register
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(120)
    }
    ///0x338 - Peripheral function 121 input selection register
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(121)
    }
    ///0x33c - Peripheral function 122 input selection register
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(122)
    }
    ///0x340 - Peripheral function 123 input selection register
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(123)
    }
    ///0x344 - Peripheral function 124 input selection register
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(124)
    }
    ///0x348 - Peripheral function 125 input selection register
    #[inline(always)]
    pub const fn func125_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(125)
    }
    ///0x34c - Peripheral function 126 input selection register
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(126)
    }
    ///0x350 - Peripheral function 127 input selection register
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(127)
    }
    ///0x354 - Peripheral function 128 input selection register
    #[inline(always)]
    pub const fn func128_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(128)
    }
    ///0x358 - Peripheral function 129 input selection register
    #[inline(always)]
    pub const fn func129_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(129)
    }
    ///0x35c - Peripheral function 130 input selection register
    #[inline(always)]
    pub const fn func130_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(130)
    }
    ///0x360 - Peripheral function 131 input selection register
    #[inline(always)]
    pub const fn func131_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(131)
    }
    ///0x364 - Peripheral function 132 input selection register
    #[inline(always)]
    pub const fn func132_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(132)
    }
    ///0x368 - Peripheral function 133 input selection register
    #[inline(always)]
    pub const fn func133_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(133)
    }
    ///0x36c - Peripheral function 134 input selection register
    #[inline(always)]
    pub const fn func134_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(134)
    }
    ///0x370 - Peripheral function 135 input selection register
    #[inline(always)]
    pub const fn func135_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(135)
    }
    ///0x374 - Peripheral function 136 input selection register
    #[inline(always)]
    pub const fn func136_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(136)
    }
    ///0x378 - Peripheral function 137 input selection register
    #[inline(always)]
    pub const fn func137_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(137)
    }
    ///0x37c - Peripheral function 138 input selection register
    #[inline(always)]
    pub const fn func138_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(138)
    }
    ///0x380 - Peripheral function 139 input selection register
    #[inline(always)]
    pub const fn func139_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(139)
    }
    ///0x384 - Peripheral function 140 input selection register
    #[inline(always)]
    pub const fn func140_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(140)
    }
    ///0x388 - Peripheral function 141 input selection register
    #[inline(always)]
    pub const fn func141_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(141)
    }
    ///0x38c - Peripheral function 142 input selection register
    #[inline(always)]
    pub const fn func142_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(142)
    }
    ///0x390 - Peripheral function 143 input selection register
    #[inline(always)]
    pub const fn func143_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(143)
    }
    ///0x394 - Peripheral function 144 input selection register
    #[inline(always)]
    pub const fn func144_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(144)
    }
    ///0x398 - Peripheral function 145 input selection register
    #[inline(always)]
    pub const fn func145_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(145)
    }
    ///0x39c - Peripheral function 146 input selection register
    #[inline(always)]
    pub const fn func146_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(146)
    }
    ///0x3a0 - Peripheral function 147 input selection register
    #[inline(always)]
    pub const fn func147_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(147)
    }
    ///0x3a4 - Peripheral function 148 input selection register
    #[inline(always)]
    pub const fn func148_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(148)
    }
    ///0x3a8 - Peripheral function 149 input selection register
    #[inline(always)]
    pub const fn func149_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(149)
    }
    ///0x3ac - Peripheral function 150 input selection register
    #[inline(always)]
    pub const fn func150_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(150)
    }
    ///0x3b0 - Peripheral function 151 input selection register
    #[inline(always)]
    pub const fn func151_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(151)
    }
    ///0x3b4 - Peripheral function 152 input selection register
    #[inline(always)]
    pub const fn func152_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(152)
    }
    ///0x3b8 - Peripheral function 153 input selection register
    #[inline(always)]
    pub const fn func153_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(153)
    }
    ///0x3bc - Peripheral function 154 input selection register
    #[inline(always)]
    pub const fn func154_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(154)
    }
    ///0x3c0 - Peripheral function 155 input selection register
    #[inline(always)]
    pub const fn func155_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(155)
    }
    ///0x3c4 - Peripheral function 156 input selection register
    #[inline(always)]
    pub const fn func156_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(156)
    }
    ///0x3c8 - Peripheral function 157 input selection register
    #[inline(always)]
    pub const fn func157_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(157)
    }
    ///0x3cc - Peripheral function 158 input selection register
    #[inline(always)]
    pub const fn func158_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(158)
    }
    ///0x3d0 - Peripheral function 159 input selection register
    #[inline(always)]
    pub const fn func159_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(159)
    }
    ///0x3d4 - Peripheral function 160 input selection register
    #[inline(always)]
    pub const fn func160_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(160)
    }
    ///0x3d8 - Peripheral function 161 input selection register
    #[inline(always)]
    pub const fn func161_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(161)
    }
    ///0x3dc - Peripheral function 162 input selection register
    #[inline(always)]
    pub const fn func162_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(162)
    }
    ///0x3e0 - Peripheral function 163 input selection register
    #[inline(always)]
    pub const fn func163_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(163)
    }
    ///0x3e4 - Peripheral function 164 input selection register
    #[inline(always)]
    pub const fn func164_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(164)
    }
    ///0x3e8 - Peripheral function 165 input selection register
    #[inline(always)]
    pub const fn func165_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(165)
    }
    ///0x3ec - Peripheral function 166 input selection register
    #[inline(always)]
    pub const fn func166_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(166)
    }
    ///0x3f0 - Peripheral function 167 input selection register
    #[inline(always)]
    pub const fn func167_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(167)
    }
    ///0x3f4 - Peripheral function 168 input selection register
    #[inline(always)]
    pub const fn func168_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(168)
    }
    ///0x3f8 - Peripheral function 169 input selection register
    #[inline(always)]
    pub const fn func169_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(169)
    }
    ///0x3fc - Peripheral function 170 input selection register
    #[inline(always)]
    pub const fn func170_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(170)
    }
    ///0x400 - Peripheral function 171 input selection register
    #[inline(always)]
    pub const fn func171_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(171)
    }
    ///0x404 - Peripheral function 172 input selection register
    #[inline(always)]
    pub const fn func172_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(172)
    }
    ///0x408 - Peripheral function 173 input selection register
    #[inline(always)]
    pub const fn func173_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(173)
    }
    ///0x40c - Peripheral function 174 input selection register
    #[inline(always)]
    pub const fn func174_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(174)
    }
    ///0x410 - Peripheral function 175 input selection register
    #[inline(always)]
    pub const fn func175_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(175)
    }
    ///0x414 - Peripheral function 176 input selection register
    #[inline(always)]
    pub const fn func176_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(176)
    }
    ///0x418 - Peripheral function 177 input selection register
    #[inline(always)]
    pub const fn func177_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(177)
    }
    ///0x41c - Peripheral function 178 input selection register
    #[inline(always)]
    pub const fn func178_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(178)
    }
    ///0x420 - Peripheral function 179 input selection register
    #[inline(always)]
    pub const fn func179_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(179)
    }
    ///0x424 - Peripheral function 180 input selection register
    #[inline(always)]
    pub const fn func180_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(180)
    }
    ///0x428 - Peripheral function 181 input selection register
    #[inline(always)]
    pub const fn func181_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(181)
    }
    ///0x42c - Peripheral function 182 input selection register
    #[inline(always)]
    pub const fn func182_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(182)
    }
    ///0x430 - Peripheral function 183 input selection register
    #[inline(always)]
    pub const fn func183_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(183)
    }
    ///0x434 - Peripheral function 184 input selection register
    #[inline(always)]
    pub const fn func184_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(184)
    }
    ///0x438 - Peripheral function 185 input selection register
    #[inline(always)]
    pub const fn func185_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(185)
    }
    ///0x43c - Peripheral function 186 input selection register
    #[inline(always)]
    pub const fn func186_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(186)
    }
    ///0x440 - Peripheral function 187 input selection register
    #[inline(always)]
    pub const fn func187_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(187)
    }
    ///0x444 - Peripheral function 188 input selection register
    #[inline(always)]
    pub const fn func188_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(188)
    }
    ///0x448 - Peripheral function 189 input selection register
    #[inline(always)]
    pub const fn func189_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(189)
    }
    ///0x44c - Peripheral function 190 input selection register
    #[inline(always)]
    pub const fn func190_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(190)
    }
    ///0x450 - Peripheral function 191 input selection register
    #[inline(always)]
    pub const fn func191_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(191)
    }
    ///0x454 - Peripheral function 192 input selection register
    #[inline(always)]
    pub const fn func192_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(192)
    }
    ///0x458 - Peripheral function 193 input selection register
    #[inline(always)]
    pub const fn func193_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(193)
    }
    ///0x45c - Peripheral function 194 input selection register
    #[inline(always)]
    pub const fn func194_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(194)
    }
    ///0x460 - Peripheral function 195 input selection register
    #[inline(always)]
    pub const fn func195_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(195)
    }
    ///0x464 - Peripheral function 196 input selection register
    #[inline(always)]
    pub const fn func196_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(196)
    }
    ///0x468 - Peripheral function 197 input selection register
    #[inline(always)]
    pub const fn func197_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(197)
    }
    ///0x46c - Peripheral function 198 input selection register
    #[inline(always)]
    pub const fn func198_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(198)
    }
    ///0x470 - Peripheral function 199 input selection register
    #[inline(always)]
    pub const fn func199_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(199)
    }
    ///0x474 - Peripheral function 200 input selection register
    #[inline(always)]
    pub const fn func200_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(200)
    }
    ///0x478 - Peripheral function 201 input selection register
    #[inline(always)]
    pub const fn func201_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(201)
    }
    ///0x47c - Peripheral function 202 input selection register
    #[inline(always)]
    pub const fn func202_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(202)
    }
    ///0x480 - Peripheral function 203 input selection register
    #[inline(always)]
    pub const fn func203_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(203)
    }
    ///0x484 - Peripheral function 204 input selection register
    #[inline(always)]
    pub const fn func204_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(204)
    }
    ///0x488 - Peripheral function 205 input selection register
    #[inline(always)]
    pub const fn func205_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(205)
    }
    ///0x48c - Peripheral function 206 input selection register
    #[inline(always)]
    pub const fn func206_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(206)
    }
    ///0x490 - Peripheral function 207 input selection register
    #[inline(always)]
    pub const fn func207_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(207)
    }
    ///0x494 - Peripheral function 208 input selection register
    #[inline(always)]
    pub const fn func208_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(208)
    }
    ///0x498 - Peripheral function 209 input selection register
    #[inline(always)]
    pub const fn func209_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(209)
    }
    ///0x49c - Peripheral function 210 input selection register
    #[inline(always)]
    pub const fn func210_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(210)
    }
    ///0x4a0 - Peripheral function 211 input selection register
    #[inline(always)]
    pub const fn func211_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(211)
    }
    ///0x4a4 - Peripheral function 212 input selection register
    #[inline(always)]
    pub const fn func212_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(212)
    }
    ///0x4a8 - Peripheral function 213 input selection register
    #[inline(always)]
    pub const fn func213_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(213)
    }
    ///0x4ac - Peripheral function 214 input selection register
    #[inline(always)]
    pub const fn func214_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(214)
    }
    ///0x4b0 - Peripheral function 215 input selection register
    #[inline(always)]
    pub const fn func215_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(215)
    }
    ///0x4b4 - Peripheral function 216 input selection register
    #[inline(always)]
    pub const fn func216_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(216)
    }
    ///0x4b8 - Peripheral function 217 input selection register
    #[inline(always)]
    pub const fn func217_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(217)
    }
    ///0x4bc - Peripheral function 218 input selection register
    #[inline(always)]
    pub const fn func218_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(218)
    }
    ///0x4c0 - Peripheral function 219 input selection register
    #[inline(always)]
    pub const fn func219_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(219)
    }
    ///0x4c4 - Peripheral function 220 input selection register
    #[inline(always)]
    pub const fn func220_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(220)
    }
    ///0x4c8 - Peripheral function 221 input selection register
    #[inline(always)]
    pub const fn func221_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(221)
    }
    ///0x4cc - Peripheral function 222 input selection register
    #[inline(always)]
    pub const fn func222_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(222)
    }
    ///0x4d0 - Peripheral function 223 input selection register
    #[inline(always)]
    pub const fn func223_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(223)
    }
    ///0x4d4 - Peripheral function 224 input selection register
    #[inline(always)]
    pub const fn func224_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(224)
    }
    ///0x4d8 - Peripheral function 225 input selection register
    #[inline(always)]
    pub const fn func225_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(225)
    }
    ///0x4dc - Peripheral function 226 input selection register
    #[inline(always)]
    pub const fn func226_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(226)
    }
    ///0x4e0 - Peripheral function 227 input selection register
    #[inline(always)]
    pub const fn func227_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(227)
    }
    ///0x4e4 - Peripheral function 228 input selection register
    #[inline(always)]
    pub const fn func228_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(228)
    }
    ///0x4e8 - Peripheral function 229 input selection register
    #[inline(always)]
    pub const fn func229_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(229)
    }
    ///0x4ec - Peripheral function 230 input selection register
    #[inline(always)]
    pub const fn func230_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(230)
    }
    ///0x4f0 - Peripheral function 231 input selection register
    #[inline(always)]
    pub const fn func231_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(231)
    }
    ///0x4f4 - Peripheral function 232 input selection register
    #[inline(always)]
    pub const fn func232_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(232)
    }
    ///0x4f8 - Peripheral function 233 input selection register
    #[inline(always)]
    pub const fn func233_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(233)
    }
    ///0x4fc - Peripheral function 234 input selection register
    #[inline(always)]
    pub const fn func234_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(234)
    }
    ///0x500 - Peripheral function 235 input selection register
    #[inline(always)]
    pub const fn func235_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(235)
    }
    ///0x504 - Peripheral function 236 input selection register
    #[inline(always)]
    pub const fn func236_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(236)
    }
    ///0x508 - Peripheral function 237 input selection register
    #[inline(always)]
    pub const fn func237_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(237)
    }
    ///0x50c - Peripheral function 238 input selection register
    #[inline(always)]
    pub const fn func238_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(238)
    }
    ///0x510 - Peripheral function 239 input selection register
    #[inline(always)]
    pub const fn func239_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(239)
    }
    ///0x514 - Peripheral function 240 input selection register
    #[inline(always)]
    pub const fn func240_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(240)
    }
    ///0x518 - Peripheral function 241 input selection register
    #[inline(always)]
    pub const fn func241_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(241)
    }
    ///0x51c - Peripheral function 242 input selection register
    #[inline(always)]
    pub const fn func242_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(242)
    }
    ///0x520 - Peripheral function 243 input selection register
    #[inline(always)]
    pub const fn func243_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(243)
    }
    ///0x524 - Peripheral function 244 input selection register
    #[inline(always)]
    pub const fn func244_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(244)
    }
    ///0x528 - Peripheral function 245 input selection register
    #[inline(always)]
    pub const fn func245_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(245)
    }
    ///0x52c - Peripheral function 246 input selection register
    #[inline(always)]
    pub const fn func246_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(246)
    }
    ///0x530 - Peripheral function 247 input selection register
    #[inline(always)]
    pub const fn func247_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(247)
    }
    ///0x534 - Peripheral function 248 input selection register
    #[inline(always)]
    pub const fn func248_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(248)
    }
    ///0x538 - Peripheral function 249 input selection register
    #[inline(always)]
    pub const fn func249_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(249)
    }
    ///0x53c - Peripheral function 250 input selection register
    #[inline(always)]
    pub const fn func250_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(250)
    }
    ///0x540 - Peripheral function 251 input selection register
    #[inline(always)]
    pub const fn func251_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(251)
    }
    ///0x544 - Peripheral function 252 input selection register
    #[inline(always)]
    pub const fn func252_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(252)
    }
    ///0x548 - Peripheral function 253 input selection register
    #[inline(always)]
    pub const fn func253_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(253)
    }
    ///0x54c - Peripheral function 254 input selection register
    #[inline(always)]
    pub const fn func254_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(254)
    }
    ///0x550 - Peripheral function 255 input selection register
    #[inline(always)]
    pub const fn func255_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(255)
    }
    ///0x554..0x62c - Peripheral output selection for GPIO %s
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    ///Iterator for array of:
    ///0x554..0x62c - Peripheral output selection for GPIO %s
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    ///0x554 - Peripheral output selection for GPIO 0
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    ///0x558 - Peripheral output selection for GPIO 1
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    ///0x55c - Peripheral output selection for GPIO 2
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    ///0x560 - Peripheral output selection for GPIO 3
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    ///0x564 - Peripheral output selection for GPIO 4
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    ///0x568 - Peripheral output selection for GPIO 5
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    ///0x56c - Peripheral output selection for GPIO 6
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    ///0x570 - Peripheral output selection for GPIO 7
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    ///0x574 - Peripheral output selection for GPIO 8
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    ///0x578 - Peripheral output selection for GPIO 9
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    ///0x57c - Peripheral output selection for GPIO 10
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    ///0x580 - Peripheral output selection for GPIO 11
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    ///0x584 - Peripheral output selection for GPIO 12
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    ///0x588 - Peripheral output selection for GPIO 13
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    ///0x58c - Peripheral output selection for GPIO 14
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    ///0x590 - Peripheral output selection for GPIO 15
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    ///0x594 - Peripheral output selection for GPIO 16
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    ///0x598 - Peripheral output selection for GPIO 17
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    ///0x59c - Peripheral output selection for GPIO 18
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    ///0x5a0 - Peripheral output selection for GPIO 19
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    ///0x5a4 - Peripheral output selection for GPIO 20
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    ///0x5a8 - Peripheral output selection for GPIO 21
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    ///0x5ac - Peripheral output selection for GPIO 22
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    ///0x5b0 - Peripheral output selection for GPIO 23
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    ///0x5b4 - Peripheral output selection for GPIO 24
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    ///0x5b8 - Peripheral output selection for GPIO 25
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    ///0x5bc - Peripheral output selection for GPIO 26
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    ///0x5c0 - Peripheral output selection for GPIO 27
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    ///0x5c4 - Peripheral output selection for GPIO 28
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    ///0x5c8 - Peripheral output selection for GPIO 29
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    ///0x5cc - Peripheral output selection for GPIO 30
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    ///0x5d0 - Peripheral output selection for GPIO 31
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    ///0x5d4 - Peripheral output selection for GPIO 32
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    ///0x5d8 - Peripheral output selection for GPIO 33
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
    }
    ///0x5dc - Peripheral output selection for GPIO 34
    #[inline(always)]
    pub const fn func34_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(34)
    }
    ///0x5e0 - Peripheral output selection for GPIO 35
    #[inline(always)]
    pub const fn func35_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(35)
    }
    ///0x5e4 - Peripheral output selection for GPIO 36
    #[inline(always)]
    pub const fn func36_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(36)
    }
    ///0x5e8 - Peripheral output selection for GPIO 37
    #[inline(always)]
    pub const fn func37_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(37)
    }
    ///0x5ec - Peripheral output selection for GPIO 38
    #[inline(always)]
    pub const fn func38_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(38)
    }
    ///0x5f0 - Peripheral output selection for GPIO 39
    #[inline(always)]
    pub const fn func39_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(39)
    }
    ///0x5f4 - Peripheral output selection for GPIO 40
    #[inline(always)]
    pub const fn func40_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(40)
    }
    ///0x5f8 - Peripheral output selection for GPIO 41
    #[inline(always)]
    pub const fn func41_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(41)
    }
    ///0x5fc - Peripheral output selection for GPIO 42
    #[inline(always)]
    pub const fn func42_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(42)
    }
    ///0x600 - Peripheral output selection for GPIO 43
    #[inline(always)]
    pub const fn func43_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(43)
    }
    ///0x604 - Peripheral output selection for GPIO 44
    #[inline(always)]
    pub const fn func44_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(44)
    }
    ///0x608 - Peripheral output selection for GPIO 45
    #[inline(always)]
    pub const fn func45_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(45)
    }
    ///0x60c - Peripheral output selection for GPIO 46
    #[inline(always)]
    pub const fn func46_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(46)
    }
    ///0x610 - Peripheral output selection for GPIO 47
    #[inline(always)]
    pub const fn func47_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(47)
    }
    ///0x614 - Peripheral output selection for GPIO 48
    #[inline(always)]
    pub const fn func48_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(48)
    }
    ///0x618 - Peripheral output selection for GPIO 49
    #[inline(always)]
    pub const fn func49_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(49)
    }
    ///0x61c - Peripheral output selection for GPIO 50
    #[inline(always)]
    pub const fn func50_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(50)
    }
    ///0x620 - Peripheral output selection for GPIO 51
    #[inline(always)]
    pub const fn func51_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(51)
    }
    ///0x624 - Peripheral output selection for GPIO 52
    #[inline(always)]
    pub const fn func52_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(52)
    }
    ///0x628 - Peripheral output selection for GPIO 53
    #[inline(always)]
    pub const fn func53_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(53)
    }
    ///0x62c - GPIO clock gating register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x6fc - Version control register
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
/**BT_SELECT (rw) register accessor: GPIO bit select register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_select`] module*/
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
///GPIO bit select register
pub mod bt_select;
/**OUT (rw) register accessor: GPIO0 ~ 31 output register

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out`] module*/
pub type OUT = crate::Reg<out::OUT_SPEC>;
///GPIO0 ~ 31 output register
pub mod out;
/**OUT_W1TS (w) register accessor: GPIO0 ~ 31 output bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1ts`] module*/
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
///GPIO0 ~ 31 output bit set register
pub mod out_w1ts;
/**OUT_W1TC (w) register accessor: GPIO0 ~ 31 output bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1tc`] module*/
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
///GPIO0 ~ 31 output bit clear register
pub mod out_w1tc;
/**OUT1 (rw) register accessor: GPIO32 ~ 53 output register

You can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out1`] module*/
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
///GPIO32 ~ 53 output register
pub mod out1;
/**OUT1_W1TS (w) register accessor: GPIO32 ~ 53 output bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out1_w1ts`] module*/
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
///GPIO32 ~ 53 output bit set register
pub mod out1_w1ts;
/**OUT1_W1TC (w) register accessor: GPIO32 ~ 53 output bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out1_w1tc`] module*/
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
///GPIO32 ~ 53 output bit clear register
pub mod out1_w1tc;
/**SDIO_SELECT (rw) register accessor: GPIO SDIO selection register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_select`] module*/
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
///GPIO SDIO selection register
pub mod sdio_select;
/**ENABLE (rw) register accessor: GPIO0 ~ 31 output enable register

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable`] module*/
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
///GPIO0 ~ 31 output enable register
pub mod enable;
/**ENABLE_W1TS (w) register accessor: GPIO0 ~ 31 output enable bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1ts`] module*/
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
///GPIO0 ~ 31 output enable bit set register
pub mod enable_w1ts;
/**ENABLE_W1TC (w) register accessor: GPIO0 ~ 31 output enable bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1tc`] module*/
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
///GPIO0 ~ 31 output enable bit clear register
pub mod enable_w1tc;
/**ENABLE1 (rw) register accessor: GPIO32 ~ 53 output enable register

You can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable1`] module*/
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
///GPIO32 ~ 53 output enable register
pub mod enable1;
/**ENABLE1_W1TS (w) register accessor: GPIO32 ~ 53 output enable bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable1_w1ts`] module*/
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
///GPIO32 ~ 53 output enable bit set register
pub mod enable1_w1ts;
/**ENABLE1_W1TC (w) register accessor: GPIO32 ~ 53 output enable bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable1_w1tc`] module*/
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
///GPIO32 ~ 53 output enable bit clear register
pub mod enable1_w1tc;
/**STRAP (r) register accessor: Bootstrap pin value register

You can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@strap`] module*/
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
///Bootstrap pin value register
pub mod strap;
/**IN (rw) register accessor: GPIO0 ~ 31 input register

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_`] module*/
pub type IN = crate::Reg<in_::IN_SPEC>;
///GPIO0 ~ 31 input register
pub mod in_;
/**IN1 (r) register accessor: GPIO32 ~ 53 input register

You can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in1`] module*/
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
///GPIO32 ~ 53 input register
pub mod in1;
/**STATUS (rw) register accessor: GPIO0 ~ 31 interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///GPIO0 ~ 31 interrupt status register
pub mod status;
/**STATUS_W1TS (w) register accessor: GPIO0 ~ 31 interrupt status bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1ts`] module*/
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
///GPIO0 ~ 31 interrupt status bit set register
pub mod status_w1ts;
/**STATUS_W1TC (w) register accessor: GPIO0 ~ 31 interrupt status bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1tc`] module*/
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
///GPIO0 ~ 31 interrupt status bit clear register
pub mod status_w1tc;
/**STATUS1 (rw) register accessor: GPIO32 ~ 53 interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status1`] module*/
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
///GPIO32 ~ 53 interrupt status register
pub mod status1;
/**STATUS1_W1TS (w) register accessor: GPIO32 ~ 53 interrupt status bit set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status1_w1ts`] module*/
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
///GPIO32 ~ 53 interrupt status bit set register
pub mod status1_w1ts;
/**STATUS1_W1TC (w) register accessor: GPIO32 ~ 53 interrupt status bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status1_w1tc`] module*/
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
///GPIO32 ~ 53 interrupt status bit clear register
pub mod status1_w1tc;
/**PCPU_INT (r) register accessor: GPIO0 ~ 31 PRO_CPU interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcpu_int`] module*/
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
///GPIO0 ~ 31 PRO_CPU interrupt status register
pub mod pcpu_int;
/**PCPU_NMI_INT (r) register accessor: GPIO0 ~ 31 PRO_CPU non-maskable interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcpu_nmi_int`] module*/
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
///GPIO0 ~ 31 PRO_CPU non-maskable interrupt status register
pub mod pcpu_nmi_int;
/**CPUSDIO_INT (r) register accessor: GPIO0 ~ 31 CPU SDIO interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpusdio_int`] module*/
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
///GPIO0 ~ 31 CPU SDIO interrupt status register
pub mod cpusdio_int;
/**PCPU_INT1 (r) register accessor: GPIO32 ~ 53 PRO_CPU interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcpu_int1`] module*/
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
///GPIO32 ~ 53 PRO_CPU interrupt status register
pub mod pcpu_int1;
/**PCPU_NMI_INT1 (r) register accessor: GPIO32 ~ 53 PRO_CPU non-maskable interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcpu_nmi_int1`] module*/
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
///GPIO32 ~ 53 PRO_CPU non-maskable interrupt status register
pub mod pcpu_nmi_int1;
/**CPUSDIO_INT1 (r) register accessor: GPIO32 ~ 53 CPU SDIO interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpusdio_int1`] module*/
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
///GPIO32 ~ 53 CPU SDIO interrupt status register
pub mod cpusdio_int1;
/**PIN (rw) register accessor: Configuration for GPIO pin %s

You can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin`] module*/
pub type PIN = crate::Reg<pin::PIN_SPEC>;
///Configuration for GPIO pin %s
pub mod pin;
/**STATUS_NEXT (r) register accessor: GPIO0 ~ 31 interrupt source register

You can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_next`] module*/
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
///GPIO0 ~ 31 interrupt source register
pub mod status_next;
/**STATUS_NEXT1 (r) register accessor: GPIO32 ~ 53 interrupt source register

You can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_next1`] module*/
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
///GPIO32 ~ 53 interrupt source register
pub mod status_next1;
/**FUNC_IN_SEL_CFG (rw) register accessor: Peripheral function %s input selection register

You can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_in_sel_cfg`] module*/
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
///Peripheral function %s input selection register
pub mod func_in_sel_cfg;
/**FUNC_OUT_SEL_CFG (rw) register accessor: Peripheral output selection for GPIO %s

You can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_out_sel_cfg`] module*/
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
///Peripheral output selection for GPIO %s
pub mod func_out_sel_cfg;
/**CLOCK_GATE (rw) register accessor: GPIO clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///GPIO clock gating register
pub mod clock_gate;
/**REG_DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_date`] module*/
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
///Version control register
pub mod reg_date;
