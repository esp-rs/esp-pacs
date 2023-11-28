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
    _reserved7: [u8; 0x04],
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
    intr_0: INTR_0,
    intr1_0: INTR1_0,
    intr_1: INTR_1,
    intr1_1: INTR1_1,
    status_next: STATUS_NEXT,
    status_next1: STATUS_NEXT1,
    pin: [PIN; 57],
    _reserved29: [u8; 0x04],
    func1_in_sel_cfg: FUNC1_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC2_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC3_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC4_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC5_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC6_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC7_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC8_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC9_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC10_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC11_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC12_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC13_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC14_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC15_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC16_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC17_IN_SEL_CFG,
    func18_in_sel_cfg: FUNC18_IN_SEL_CFG,
    func19_in_sel_cfg: FUNC19_IN_SEL_CFG,
    func20_in_sel_cfg: FUNC20_IN_SEL_CFG,
    func21_in_sel_cfg: FUNC21_IN_SEL_CFG,
    func22_in_sel_cfg: FUNC22_IN_SEL_CFG,
    func23_in_sel_cfg: FUNC23_IN_SEL_CFG,
    func24_in_sel_cfg: FUNC24_IN_SEL_CFG,
    func25_in_sel_cfg: FUNC25_IN_SEL_CFG,
    func26_in_sel_cfg: FUNC26_IN_SEL_CFG,
    func27_in_sel_cfg: FUNC27_IN_SEL_CFG,
    func28_in_sel_cfg: FUNC28_IN_SEL_CFG,
    func29_in_sel_cfg: FUNC29_IN_SEL_CFG,
    func30_in_sel_cfg: FUNC30_IN_SEL_CFG,
    func31_in_sel_cfg: FUNC31_IN_SEL_CFG,
    func32_in_sel_cfg: FUNC32_IN_SEL_CFG,
    func33_in_sel_cfg: FUNC33_IN_SEL_CFG,
    func34_in_sel_cfg: FUNC34_IN_SEL_CFG,
    func35_in_sel_cfg: FUNC35_IN_SEL_CFG,
    func36_in_sel_cfg: FUNC36_IN_SEL_CFG,
    func37_in_sel_cfg: FUNC37_IN_SEL_CFG,
    func38_in_sel_cfg: FUNC38_IN_SEL_CFG,
    func39_in_sel_cfg: FUNC39_IN_SEL_CFG,
    func40_in_sel_cfg: FUNC40_IN_SEL_CFG,
    func41_in_sel_cfg: FUNC41_IN_SEL_CFG,
    func42_in_sel_cfg: FUNC42_IN_SEL_CFG,
    func43_in_sel_cfg: FUNC43_IN_SEL_CFG,
    func44_in_sel_cfg: FUNC44_IN_SEL_CFG,
    func45_in_sel_cfg: FUNC45_IN_SEL_CFG,
    _reserved74: [u8; 0x04],
    func47_in_sel_cfg: FUNC47_IN_SEL_CFG,
    func48_in_sel_cfg: FUNC48_IN_SEL_CFG,
    func49_in_sel_cfg: FUNC49_IN_SEL_CFG,
    func50_in_sel_cfg: FUNC50_IN_SEL_CFG,
    func51_in_sel_cfg: FUNC51_IN_SEL_CFG,
    func52_in_sel_cfg: FUNC52_IN_SEL_CFG,
    func53_in_sel_cfg: FUNC53_IN_SEL_CFG,
    func54_in_sel_cfg: FUNC54_IN_SEL_CFG,
    func55_in_sel_cfg: FUNC55_IN_SEL_CFG,
    func56_in_sel_cfg: FUNC56_IN_SEL_CFG,
    func57_in_sel_cfg: FUNC57_IN_SEL_CFG,
    func58_in_sel_cfg: FUNC58_IN_SEL_CFG,
    func59_in_sel_cfg: FUNC59_IN_SEL_CFG,
    func60_in_sel_cfg: FUNC60_IN_SEL_CFG,
    func61_in_sel_cfg: FUNC61_IN_SEL_CFG,
    func62_in_sel_cfg: FUNC62_IN_SEL_CFG,
    func63_in_sel_cfg: FUNC63_IN_SEL_CFG,
    func64_in_sel_cfg: FUNC64_IN_SEL_CFG,
    func65_in_sel_cfg: FUNC65_IN_SEL_CFG,
    func66_in_sel_cfg: FUNC66_IN_SEL_CFG,
    _reserved94: [u8; 0x04],
    func68_in_sel_cfg: FUNC68_IN_SEL_CFG,
    func69_in_sel_cfg: FUNC69_IN_SEL_CFG,
    func70_in_sel_cfg: FUNC70_IN_SEL_CFG,
    func71_in_sel_cfg: FUNC71_IN_SEL_CFG,
    _reserved98: [u8; 0x08],
    func74_in_sel_cfg: FUNC74_IN_SEL_CFG,
    func75_in_sel_cfg: FUNC75_IN_SEL_CFG,
    func76_in_sel_cfg: FUNC76_IN_SEL_CFG,
    func77_in_sel_cfg: FUNC77_IN_SEL_CFG,
    func78_in_sel_cfg: FUNC78_IN_SEL_CFG,
    _reserved103: [u8; 0x04],
    func80_in_sel_cfg: FUNC80_IN_SEL_CFG,
    _reserved104: [u8; 0x08],
    func83_in_sel_cfg: FUNC83_IN_SEL_CFG,
    _reserved105: [u8; 0x08],
    func86_in_sel_cfg: FUNC86_IN_SEL_CFG,
    _reserved106: [u8; 0x08],
    func89_in_sel_cfg: FUNC89_IN_SEL_CFG,
    func90_in_sel_cfg: FUNC90_IN_SEL_CFG,
    func91_in_sel_cfg: FUNC91_IN_SEL_CFG,
    func92_in_sel_cfg: FUNC92_IN_SEL_CFG,
    func93_in_sel_cfg: FUNC93_IN_SEL_CFG,
    func94_in_sel_cfg: FUNC94_IN_SEL_CFG,
    func95_in_sel_cfg: FUNC95_IN_SEL_CFG,
    func96_in_sel_cfg: FUNC96_IN_SEL_CFG,
    func97_in_sel_cfg: FUNC97_IN_SEL_CFG,
    func98_in_sel_cfg: FUNC98_IN_SEL_CFG,
    func99_in_sel_cfg: FUNC99_IN_SEL_CFG,
    func100_in_sel_cfg: FUNC100_IN_SEL_CFG,
    func101_in_sel_cfg: FUNC101_IN_SEL_CFG,
    func102_in_sel_cfg: FUNC102_IN_SEL_CFG,
    func103_in_sel_cfg: FUNC103_IN_SEL_CFG,
    func104_in_sel_cfg: FUNC104_IN_SEL_CFG,
    func105_in_sel_cfg: FUNC105_IN_SEL_CFG,
    func106_in_sel_cfg: FUNC106_IN_SEL_CFG,
    func107_in_sel_cfg: FUNC107_IN_SEL_CFG,
    func108_in_sel_cfg: FUNC108_IN_SEL_CFG,
    func109_in_sel_cfg: FUNC109_IN_SEL_CFG,
    func110_in_sel_cfg: FUNC110_IN_SEL_CFG,
    func111_in_sel_cfg: FUNC111_IN_SEL_CFG,
    func112_in_sel_cfg: FUNC112_IN_SEL_CFG,
    func113_in_sel_cfg: FUNC113_IN_SEL_CFG,
    func114_in_sel_cfg: FUNC114_IN_SEL_CFG,
    _reserved132: [u8; 0x08],
    func117_in_sel_cfg: FUNC117_IN_SEL_CFG,
    func118_in_sel_cfg: FUNC118_IN_SEL_CFG,
    _reserved134: [u8; 0x1c],
    func126_in_sel_cfg: FUNC126_IN_SEL_CFG,
    func127_in_sel_cfg: FUNC127_IN_SEL_CFG,
    func128_in_sel_cfg: FUNC128_IN_SEL_CFG,
    func129_in_sel_cfg: FUNC129_IN_SEL_CFG,
    func130_in_sel_cfg: FUNC130_IN_SEL_CFG,
    func131_in_sel_cfg: FUNC131_IN_SEL_CFG,
    func132_in_sel_cfg: FUNC132_IN_SEL_CFG,
    func133_in_sel_cfg: FUNC133_IN_SEL_CFG,
    func134_in_sel_cfg: FUNC134_IN_SEL_CFG,
    func135_in_sel_cfg: FUNC135_IN_SEL_CFG,
    func136_in_sel_cfg: FUNC136_IN_SEL_CFG,
    func137_in_sel_cfg: FUNC137_IN_SEL_CFG,
    func138_in_sel_cfg: FUNC138_IN_SEL_CFG,
    func139_in_sel_cfg: FUNC139_IN_SEL_CFG,
    func140_in_sel_cfg: FUNC140_IN_SEL_CFG,
    func141_in_sel_cfg: FUNC141_IN_SEL_CFG,
    func142_in_sel_cfg: FUNC142_IN_SEL_CFG,
    func143_in_sel_cfg: FUNC143_IN_SEL_CFG,
    func144_in_sel_cfg: FUNC144_IN_SEL_CFG,
    func145_in_sel_cfg: FUNC145_IN_SEL_CFG,
    func146_in_sel_cfg: FUNC146_IN_SEL_CFG,
    func147_in_sel_cfg: FUNC147_IN_SEL_CFG,
    func148_in_sel_cfg: FUNC148_IN_SEL_CFG,
    func149_in_sel_cfg: FUNC149_IN_SEL_CFG,
    func150_in_sel_cfg: FUNC150_IN_SEL_CFG,
    func151_in_sel_cfg: FUNC151_IN_SEL_CFG,
    func152_in_sel_cfg: FUNC152_IN_SEL_CFG,
    func153_in_sel_cfg: FUNC153_IN_SEL_CFG,
    func154_in_sel_cfg: FUNC154_IN_SEL_CFG,
    func155_in_sel_cfg: FUNC155_IN_SEL_CFG,
    func156_in_sel_cfg: FUNC156_IN_SEL_CFG,
    _reserved165: [u8; 0x04],
    func158_in_sel_cfg: FUNC158_IN_SEL_CFG,
    func159_in_sel_cfg: FUNC159_IN_SEL_CFG,
    func160_in_sel_cfg: FUNC160_IN_SEL_CFG,
    func161_in_sel_cfg: FUNC161_IN_SEL_CFG,
    func162_in_sel_cfg: FUNC162_IN_SEL_CFG,
    func163_in_sel_cfg: FUNC163_IN_SEL_CFG,
    func164_in_sel_cfg: FUNC164_IN_SEL_CFG,
    func165_in_sel_cfg: FUNC165_IN_SEL_CFG,
    func166_in_sel_cfg: FUNC166_IN_SEL_CFG,
    func167_in_sel_cfg: FUNC167_IN_SEL_CFG,
    func168_in_sel_cfg: FUNC168_IN_SEL_CFG,
    func169_in_sel_cfg: FUNC169_IN_SEL_CFG,
    func170_in_sel_cfg: FUNC170_IN_SEL_CFG,
    func171_in_sel_cfg: FUNC171_IN_SEL_CFG,
    func172_in_sel_cfg: FUNC172_IN_SEL_CFG,
    func173_in_sel_cfg: FUNC173_IN_SEL_CFG,
    func174_in_sel_cfg: FUNC174_IN_SEL_CFG,
    func175_in_sel_cfg: FUNC175_IN_SEL_CFG,
    func176_in_sel_cfg: FUNC176_IN_SEL_CFG,
    func177_in_sel_cfg: FUNC177_IN_SEL_CFG,
    func178_in_sel_cfg: FUNC178_IN_SEL_CFG,
    func179_in_sel_cfg: FUNC179_IN_SEL_CFG,
    func180_in_sel_cfg: FUNC180_IN_SEL_CFG,
    func181_in_sel_cfg: FUNC181_IN_SEL_CFG,
    func182_in_sel_cfg: FUNC182_IN_SEL_CFG,
    func183_in_sel_cfg: FUNC183_IN_SEL_CFG,
    func184_in_sel_cfg: FUNC184_IN_SEL_CFG,
    func185_in_sel_cfg: FUNC185_IN_SEL_CFG,
    func186_in_sel_cfg: FUNC186_IN_SEL_CFG,
    func187_in_sel_cfg: FUNC187_IN_SEL_CFG,
    func188_in_sel_cfg: FUNC188_IN_SEL_CFG,
    func189_in_sel_cfg: FUNC189_IN_SEL_CFG,
    func190_in_sel_cfg: FUNC190_IN_SEL_CFG,
    func191_in_sel_cfg: FUNC191_IN_SEL_CFG,
    func192_in_sel_cfg: FUNC192_IN_SEL_CFG,
    func193_in_sel_cfg: FUNC193_IN_SEL_CFG,
    func194_in_sel_cfg: FUNC194_IN_SEL_CFG,
    func195_in_sel_cfg: FUNC195_IN_SEL_CFG,
    func196_in_sel_cfg: FUNC196_IN_SEL_CFG,
    func197_in_sel_cfg: FUNC197_IN_SEL_CFG,
    func198_in_sel_cfg: FUNC198_IN_SEL_CFG,
    func199_in_sel_cfg: FUNC199_IN_SEL_CFG,
    func200_in_sel_cfg: FUNC200_IN_SEL_CFG,
    func201_in_sel_cfg: FUNC201_IN_SEL_CFG,
    func202_in_sel_cfg: FUNC202_IN_SEL_CFG,
    func203_in_sel_cfg: FUNC203_IN_SEL_CFG,
    _reserved211: [u8; 0x28],
    func214_in_sel_cfg: FUNC214_IN_SEL_CFG,
    func215_in_sel_cfg: FUNC215_IN_SEL_CFG,
    func216_in_sel_cfg: FUNC216_IN_SEL_CFG,
    func217_in_sel_cfg: FUNC217_IN_SEL_CFG,
    func218_in_sel_cfg: FUNC218_IN_SEL_CFG,
    func219_in_sel_cfg: FUNC219_IN_SEL_CFG,
    func220_in_sel_cfg: FUNC220_IN_SEL_CFG,
    func221_in_sel_cfg: FUNC221_IN_SEL_CFG,
    func222_in_sel_cfg: FUNC222_IN_SEL_CFG,
    func223_in_sel_cfg: FUNC223_IN_SEL_CFG,
    func224_in_sel_cfg: FUNC224_IN_SEL_CFG,
    func225_in_sel_cfg: FUNC225_IN_SEL_CFG,
    func226_in_sel_cfg: FUNC226_IN_SEL_CFG,
    func227_in_sel_cfg: FUNC227_IN_SEL_CFG,
    func228_in_sel_cfg: FUNC228_IN_SEL_CFG,
    func229_in_sel_cfg: FUNC229_IN_SEL_CFG,
    func230_in_sel_cfg: FUNC230_IN_SEL_CFG,
    func231_in_sel_cfg: FUNC231_IN_SEL_CFG,
    func232_in_sel_cfg: FUNC232_IN_SEL_CFG,
    func233_in_sel_cfg: FUNC233_IN_SEL_CFG,
    func234_in_sel_cfg: FUNC234_IN_SEL_CFG,
    func235_in_sel_cfg: FUNC235_IN_SEL_CFG,
    func236_in_sel_cfg: FUNC236_IN_SEL_CFG,
    func237_in_sel_cfg: FUNC237_IN_SEL_CFG,
    func238_in_sel_cfg: FUNC238_IN_SEL_CFG,
    func239_in_sel_cfg: FUNC239_IN_SEL_CFG,
    func240_in_sel_cfg: FUNC240_IN_SEL_CFG,
    func241_in_sel_cfg: FUNC241_IN_SEL_CFG,
    func242_in_sel_cfg: FUNC242_IN_SEL_CFG,
    func243_in_sel_cfg: FUNC243_IN_SEL_CFG,
    func244_in_sel_cfg: FUNC244_IN_SEL_CFG,
    func245_in_sel_cfg: FUNC245_IN_SEL_CFG,
    func246_in_sel_cfg: FUNC246_IN_SEL_CFG,
    func247_in_sel_cfg: FUNC247_IN_SEL_CFG,
    func248_in_sel_cfg: FUNC248_IN_SEL_CFG,
    func249_in_sel_cfg: FUNC249_IN_SEL_CFG,
    func250_in_sel_cfg: FUNC250_IN_SEL_CFG,
    func251_in_sel_cfg: FUNC251_IN_SEL_CFG,
    func252_in_sel_cfg: FUNC252_IN_SEL_CFG,
    func253_in_sel_cfg: FUNC253_IN_SEL_CFG,
    func254_in_sel_cfg: FUNC254_IN_SEL_CFG,
    func255_in_sel_cfg: FUNC255_IN_SEL_CFG,
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 57],
    intr_2: INTR_2,
    intr1_2: INTR1_2,
    intr_3: INTR_3,
    intr1_3: INTR1_3,
    clock_gate: CLOCK_GATE,
    _reserved259: [u8; 0xb0],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    zero_det0_filter_cnt: ZERO_DET0_FILTER_CNT,
    zero_det1_filter_cnt: ZERO_DET1_FILTER_CNT,
    send_seq: SEND_SEQ,
    recive_seq: RECIVE_SEQ,
    bistin_sel: BISTIN_SEL,
    bist_ctrl: BIST_CTRL,
    _reserved269: [u8; 0xd4],
    date: DATE,
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
    #[doc = "0x10 - GPIO output register for GPIO32-56"]
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    #[doc = "0x14 - GPIO output set register for GPIO32-56"]
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    #[doc = "0x18 - GPIO output clear register for GPIO32-56"]
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
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
    #[doc = "0x2c - GPIO output enable register for GPIO32-56"]
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    #[doc = "0x30 - GPIO output enable set register for GPIO32-56"]
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    #[doc = "0x34 - GPIO output enable clear register for GPIO32-56"]
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
    #[doc = "0x40 - GPIO input register for GPIO32-56"]
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
    #[doc = "0x50 - GPIO interrupt status register for GPIO32-56"]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x54 - GPIO interrupt status set register for GPIO32-56"]
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    #[doc = "0x58 - GPIO interrupt status clear register for GPIO32-56"]
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    #[doc = "0x5c - GPIO interrupt 0 status register for GPIO0-31"]
    #[inline(always)]
    pub const fn intr_0(&self) -> &INTR_0 {
        &self.intr_0
    }
    #[doc = "0x60 - GPIO interrupt 0 status register for GPIO32-56"]
    #[inline(always)]
    pub const fn intr1_0(&self) -> &INTR1_0 {
        &self.intr1_0
    }
    #[doc = "0x64 - GPIO interrupt 1 status register for GPIO0-31"]
    #[inline(always)]
    pub const fn intr_1(&self) -> &INTR_1 {
        &self.intr_1
    }
    #[doc = "0x68 - GPIO interrupt 1 status register for GPIO32-56"]
    #[inline(always)]
    pub const fn intr1_1(&self) -> &INTR1_1 {
        &self.intr1_1
    }
    #[doc = "0x6c - GPIO interrupt source register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0x70 - GPIO interrupt source register for GPIO32-56"]
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    #[doc = "0x74..0x158 - GPIO pin configuration register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x158 - GPIO pin configuration register"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x15c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC1_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x160 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC2_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x164 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC3_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x168 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC4_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x16c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC5_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x170 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC6_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x174 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC7_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x178 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC8_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x17c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC9_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x180 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC10_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x184 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC11_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x188 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC12_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x18c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC13_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x190 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC14_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x194 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC15_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x198 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC16_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x19c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC17_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x1a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC18_IN_SEL_CFG {
        &self.func18_in_sel_cfg
    }
    #[doc = "0x1a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC19_IN_SEL_CFG {
        &self.func19_in_sel_cfg
    }
    #[doc = "0x1a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC20_IN_SEL_CFG {
        &self.func20_in_sel_cfg
    }
    #[doc = "0x1ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC21_IN_SEL_CFG {
        &self.func21_in_sel_cfg
    }
    #[doc = "0x1b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC22_IN_SEL_CFG {
        &self.func22_in_sel_cfg
    }
    #[doc = "0x1b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC23_IN_SEL_CFG {
        &self.func23_in_sel_cfg
    }
    #[doc = "0x1b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC24_IN_SEL_CFG {
        &self.func24_in_sel_cfg
    }
    #[doc = "0x1bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC25_IN_SEL_CFG {
        &self.func25_in_sel_cfg
    }
    #[doc = "0x1c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC26_IN_SEL_CFG {
        &self.func26_in_sel_cfg
    }
    #[doc = "0x1c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC27_IN_SEL_CFG {
        &self.func27_in_sel_cfg
    }
    #[doc = "0x1c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC28_IN_SEL_CFG {
        &self.func28_in_sel_cfg
    }
    #[doc = "0x1cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC29_IN_SEL_CFG {
        &self.func29_in_sel_cfg
    }
    #[doc = "0x1d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC30_IN_SEL_CFG {
        &self.func30_in_sel_cfg
    }
    #[doc = "0x1d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC31_IN_SEL_CFG {
        &self.func31_in_sel_cfg
    }
    #[doc = "0x1d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC32_IN_SEL_CFG {
        &self.func32_in_sel_cfg
    }
    #[doc = "0x1dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC33_IN_SEL_CFG {
        &self.func33_in_sel_cfg
    }
    #[doc = "0x1e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC34_IN_SEL_CFG {
        &self.func34_in_sel_cfg
    }
    #[doc = "0x1e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC35_IN_SEL_CFG {
        &self.func35_in_sel_cfg
    }
    #[doc = "0x1e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC36_IN_SEL_CFG {
        &self.func36_in_sel_cfg
    }
    #[doc = "0x1ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC37_IN_SEL_CFG {
        &self.func37_in_sel_cfg
    }
    #[doc = "0x1f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC38_IN_SEL_CFG {
        &self.func38_in_sel_cfg
    }
    #[doc = "0x1f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC39_IN_SEL_CFG {
        &self.func39_in_sel_cfg
    }
    #[doc = "0x1f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC40_IN_SEL_CFG {
        &self.func40_in_sel_cfg
    }
    #[doc = "0x1fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC41_IN_SEL_CFG {
        &self.func41_in_sel_cfg
    }
    #[doc = "0x200 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC42_IN_SEL_CFG {
        &self.func42_in_sel_cfg
    }
    #[doc = "0x204 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC43_IN_SEL_CFG {
        &self.func43_in_sel_cfg
    }
    #[doc = "0x208 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC44_IN_SEL_CFG {
        &self.func44_in_sel_cfg
    }
    #[doc = "0x20c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC45_IN_SEL_CFG {
        &self.func45_in_sel_cfg
    }
    #[doc = "0x214 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC47_IN_SEL_CFG {
        &self.func47_in_sel_cfg
    }
    #[doc = "0x218 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC48_IN_SEL_CFG {
        &self.func48_in_sel_cfg
    }
    #[doc = "0x21c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC49_IN_SEL_CFG {
        &self.func49_in_sel_cfg
    }
    #[doc = "0x220 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC50_IN_SEL_CFG {
        &self.func50_in_sel_cfg
    }
    #[doc = "0x224 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC51_IN_SEL_CFG {
        &self.func51_in_sel_cfg
    }
    #[doc = "0x228 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC52_IN_SEL_CFG {
        &self.func52_in_sel_cfg
    }
    #[doc = "0x22c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC53_IN_SEL_CFG {
        &self.func53_in_sel_cfg
    }
    #[doc = "0x230 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC54_IN_SEL_CFG {
        &self.func54_in_sel_cfg
    }
    #[doc = "0x234 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC55_IN_SEL_CFG {
        &self.func55_in_sel_cfg
    }
    #[doc = "0x238 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC56_IN_SEL_CFG {
        &self.func56_in_sel_cfg
    }
    #[doc = "0x23c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC57_IN_SEL_CFG {
        &self.func57_in_sel_cfg
    }
    #[doc = "0x240 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC58_IN_SEL_CFG {
        &self.func58_in_sel_cfg
    }
    #[doc = "0x244 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC59_IN_SEL_CFG {
        &self.func59_in_sel_cfg
    }
    #[doc = "0x248 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC60_IN_SEL_CFG {
        &self.func60_in_sel_cfg
    }
    #[doc = "0x24c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC61_IN_SEL_CFG {
        &self.func61_in_sel_cfg
    }
    #[doc = "0x250 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC62_IN_SEL_CFG {
        &self.func62_in_sel_cfg
    }
    #[doc = "0x254 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC63_IN_SEL_CFG {
        &self.func63_in_sel_cfg
    }
    #[doc = "0x258 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC64_IN_SEL_CFG {
        &self.func64_in_sel_cfg
    }
    #[doc = "0x25c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC65_IN_SEL_CFG {
        &self.func65_in_sel_cfg
    }
    #[doc = "0x260 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC66_IN_SEL_CFG {
        &self.func66_in_sel_cfg
    }
    #[doc = "0x268 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC68_IN_SEL_CFG {
        &self.func68_in_sel_cfg
    }
    #[doc = "0x26c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC69_IN_SEL_CFG {
        &self.func69_in_sel_cfg
    }
    #[doc = "0x270 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC70_IN_SEL_CFG {
        &self.func70_in_sel_cfg
    }
    #[doc = "0x274 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC71_IN_SEL_CFG {
        &self.func71_in_sel_cfg
    }
    #[doc = "0x280 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC74_IN_SEL_CFG {
        &self.func74_in_sel_cfg
    }
    #[doc = "0x284 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC75_IN_SEL_CFG {
        &self.func75_in_sel_cfg
    }
    #[doc = "0x288 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC76_IN_SEL_CFG {
        &self.func76_in_sel_cfg
    }
    #[doc = "0x28c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC77_IN_SEL_CFG {
        &self.func77_in_sel_cfg
    }
    #[doc = "0x290 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC78_IN_SEL_CFG {
        &self.func78_in_sel_cfg
    }
    #[doc = "0x298 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC80_IN_SEL_CFG {
        &self.func80_in_sel_cfg
    }
    #[doc = "0x2a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC83_IN_SEL_CFG {
        &self.func83_in_sel_cfg
    }
    #[doc = "0x2b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC86_IN_SEL_CFG {
        &self.func86_in_sel_cfg
    }
    #[doc = "0x2bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC89_IN_SEL_CFG {
        &self.func89_in_sel_cfg
    }
    #[doc = "0x2c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC90_IN_SEL_CFG {
        &self.func90_in_sel_cfg
    }
    #[doc = "0x2c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC91_IN_SEL_CFG {
        &self.func91_in_sel_cfg
    }
    #[doc = "0x2c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC92_IN_SEL_CFG {
        &self.func92_in_sel_cfg
    }
    #[doc = "0x2cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC93_IN_SEL_CFG {
        &self.func93_in_sel_cfg
    }
    #[doc = "0x2d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC94_IN_SEL_CFG {
        &self.func94_in_sel_cfg
    }
    #[doc = "0x2d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC95_IN_SEL_CFG {
        &self.func95_in_sel_cfg
    }
    #[doc = "0x2d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC96_IN_SEL_CFG {
        &self.func96_in_sel_cfg
    }
    #[doc = "0x2dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC97_IN_SEL_CFG {
        &self.func97_in_sel_cfg
    }
    #[doc = "0x2e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC98_IN_SEL_CFG {
        &self.func98_in_sel_cfg
    }
    #[doc = "0x2e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC99_IN_SEL_CFG {
        &self.func99_in_sel_cfg
    }
    #[doc = "0x2e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC100_IN_SEL_CFG {
        &self.func100_in_sel_cfg
    }
    #[doc = "0x2ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC101_IN_SEL_CFG {
        &self.func101_in_sel_cfg
    }
    #[doc = "0x2f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC102_IN_SEL_CFG {
        &self.func102_in_sel_cfg
    }
    #[doc = "0x2f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC103_IN_SEL_CFG {
        &self.func103_in_sel_cfg
    }
    #[doc = "0x2f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC104_IN_SEL_CFG {
        &self.func104_in_sel_cfg
    }
    #[doc = "0x2fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC105_IN_SEL_CFG {
        &self.func105_in_sel_cfg
    }
    #[doc = "0x300 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC106_IN_SEL_CFG {
        &self.func106_in_sel_cfg
    }
    #[doc = "0x304 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC107_IN_SEL_CFG {
        &self.func107_in_sel_cfg
    }
    #[doc = "0x308 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC108_IN_SEL_CFG {
        &self.func108_in_sel_cfg
    }
    #[doc = "0x30c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC109_IN_SEL_CFG {
        &self.func109_in_sel_cfg
    }
    #[doc = "0x310 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC110_IN_SEL_CFG {
        &self.func110_in_sel_cfg
    }
    #[doc = "0x314 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC111_IN_SEL_CFG {
        &self.func111_in_sel_cfg
    }
    #[doc = "0x318 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC112_IN_SEL_CFG {
        &self.func112_in_sel_cfg
    }
    #[doc = "0x31c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC113_IN_SEL_CFG {
        &self.func113_in_sel_cfg
    }
    #[doc = "0x320 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC114_IN_SEL_CFG {
        &self.func114_in_sel_cfg
    }
    #[doc = "0x32c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC117_IN_SEL_CFG {
        &self.func117_in_sel_cfg
    }
    #[doc = "0x330 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC118_IN_SEL_CFG {
        &self.func118_in_sel_cfg
    }
    #[doc = "0x350 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC126_IN_SEL_CFG {
        &self.func126_in_sel_cfg
    }
    #[doc = "0x354 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC127_IN_SEL_CFG {
        &self.func127_in_sel_cfg
    }
    #[doc = "0x358 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func128_in_sel_cfg(&self) -> &FUNC128_IN_SEL_CFG {
        &self.func128_in_sel_cfg
    }
    #[doc = "0x35c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func129_in_sel_cfg(&self) -> &FUNC129_IN_SEL_CFG {
        &self.func129_in_sel_cfg
    }
    #[doc = "0x360 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func130_in_sel_cfg(&self) -> &FUNC130_IN_SEL_CFG {
        &self.func130_in_sel_cfg
    }
    #[doc = "0x364 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func131_in_sel_cfg(&self) -> &FUNC131_IN_SEL_CFG {
        &self.func131_in_sel_cfg
    }
    #[doc = "0x368 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func132_in_sel_cfg(&self) -> &FUNC132_IN_SEL_CFG {
        &self.func132_in_sel_cfg
    }
    #[doc = "0x36c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func133_in_sel_cfg(&self) -> &FUNC133_IN_SEL_CFG {
        &self.func133_in_sel_cfg
    }
    #[doc = "0x370 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func134_in_sel_cfg(&self) -> &FUNC134_IN_SEL_CFG {
        &self.func134_in_sel_cfg
    }
    #[doc = "0x374 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func135_in_sel_cfg(&self) -> &FUNC135_IN_SEL_CFG {
        &self.func135_in_sel_cfg
    }
    #[doc = "0x378 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func136_in_sel_cfg(&self) -> &FUNC136_IN_SEL_CFG {
        &self.func136_in_sel_cfg
    }
    #[doc = "0x37c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func137_in_sel_cfg(&self) -> &FUNC137_IN_SEL_CFG {
        &self.func137_in_sel_cfg
    }
    #[doc = "0x380 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func138_in_sel_cfg(&self) -> &FUNC138_IN_SEL_CFG {
        &self.func138_in_sel_cfg
    }
    #[doc = "0x384 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func139_in_sel_cfg(&self) -> &FUNC139_IN_SEL_CFG {
        &self.func139_in_sel_cfg
    }
    #[doc = "0x388 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func140_in_sel_cfg(&self) -> &FUNC140_IN_SEL_CFG {
        &self.func140_in_sel_cfg
    }
    #[doc = "0x38c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func141_in_sel_cfg(&self) -> &FUNC141_IN_SEL_CFG {
        &self.func141_in_sel_cfg
    }
    #[doc = "0x390 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func142_in_sel_cfg(&self) -> &FUNC142_IN_SEL_CFG {
        &self.func142_in_sel_cfg
    }
    #[doc = "0x394 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func143_in_sel_cfg(&self) -> &FUNC143_IN_SEL_CFG {
        &self.func143_in_sel_cfg
    }
    #[doc = "0x398 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func144_in_sel_cfg(&self) -> &FUNC144_IN_SEL_CFG {
        &self.func144_in_sel_cfg
    }
    #[doc = "0x39c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func145_in_sel_cfg(&self) -> &FUNC145_IN_SEL_CFG {
        &self.func145_in_sel_cfg
    }
    #[doc = "0x3a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func146_in_sel_cfg(&self) -> &FUNC146_IN_SEL_CFG {
        &self.func146_in_sel_cfg
    }
    #[doc = "0x3a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func147_in_sel_cfg(&self) -> &FUNC147_IN_SEL_CFG {
        &self.func147_in_sel_cfg
    }
    #[doc = "0x3a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func148_in_sel_cfg(&self) -> &FUNC148_IN_SEL_CFG {
        &self.func148_in_sel_cfg
    }
    #[doc = "0x3ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func149_in_sel_cfg(&self) -> &FUNC149_IN_SEL_CFG {
        &self.func149_in_sel_cfg
    }
    #[doc = "0x3b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func150_in_sel_cfg(&self) -> &FUNC150_IN_SEL_CFG {
        &self.func150_in_sel_cfg
    }
    #[doc = "0x3b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func151_in_sel_cfg(&self) -> &FUNC151_IN_SEL_CFG {
        &self.func151_in_sel_cfg
    }
    #[doc = "0x3b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func152_in_sel_cfg(&self) -> &FUNC152_IN_SEL_CFG {
        &self.func152_in_sel_cfg
    }
    #[doc = "0x3bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func153_in_sel_cfg(&self) -> &FUNC153_IN_SEL_CFG {
        &self.func153_in_sel_cfg
    }
    #[doc = "0x3c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func154_in_sel_cfg(&self) -> &FUNC154_IN_SEL_CFG {
        &self.func154_in_sel_cfg
    }
    #[doc = "0x3c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func155_in_sel_cfg(&self) -> &FUNC155_IN_SEL_CFG {
        &self.func155_in_sel_cfg
    }
    #[doc = "0x3c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func156_in_sel_cfg(&self) -> &FUNC156_IN_SEL_CFG {
        &self.func156_in_sel_cfg
    }
    #[doc = "0x3d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func158_in_sel_cfg(&self) -> &FUNC158_IN_SEL_CFG {
        &self.func158_in_sel_cfg
    }
    #[doc = "0x3d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func159_in_sel_cfg(&self) -> &FUNC159_IN_SEL_CFG {
        &self.func159_in_sel_cfg
    }
    #[doc = "0x3d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func160_in_sel_cfg(&self) -> &FUNC160_IN_SEL_CFG {
        &self.func160_in_sel_cfg
    }
    #[doc = "0x3dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func161_in_sel_cfg(&self) -> &FUNC161_IN_SEL_CFG {
        &self.func161_in_sel_cfg
    }
    #[doc = "0x3e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func162_in_sel_cfg(&self) -> &FUNC162_IN_SEL_CFG {
        &self.func162_in_sel_cfg
    }
    #[doc = "0x3e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func163_in_sel_cfg(&self) -> &FUNC163_IN_SEL_CFG {
        &self.func163_in_sel_cfg
    }
    #[doc = "0x3e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func164_in_sel_cfg(&self) -> &FUNC164_IN_SEL_CFG {
        &self.func164_in_sel_cfg
    }
    #[doc = "0x3ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func165_in_sel_cfg(&self) -> &FUNC165_IN_SEL_CFG {
        &self.func165_in_sel_cfg
    }
    #[doc = "0x3f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func166_in_sel_cfg(&self) -> &FUNC166_IN_SEL_CFG {
        &self.func166_in_sel_cfg
    }
    #[doc = "0x3f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func167_in_sel_cfg(&self) -> &FUNC167_IN_SEL_CFG {
        &self.func167_in_sel_cfg
    }
    #[doc = "0x3f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func168_in_sel_cfg(&self) -> &FUNC168_IN_SEL_CFG {
        &self.func168_in_sel_cfg
    }
    #[doc = "0x3fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func169_in_sel_cfg(&self) -> &FUNC169_IN_SEL_CFG {
        &self.func169_in_sel_cfg
    }
    #[doc = "0x400 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func170_in_sel_cfg(&self) -> &FUNC170_IN_SEL_CFG {
        &self.func170_in_sel_cfg
    }
    #[doc = "0x404 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func171_in_sel_cfg(&self) -> &FUNC171_IN_SEL_CFG {
        &self.func171_in_sel_cfg
    }
    #[doc = "0x408 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func172_in_sel_cfg(&self) -> &FUNC172_IN_SEL_CFG {
        &self.func172_in_sel_cfg
    }
    #[doc = "0x40c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func173_in_sel_cfg(&self) -> &FUNC173_IN_SEL_CFG {
        &self.func173_in_sel_cfg
    }
    #[doc = "0x410 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func174_in_sel_cfg(&self) -> &FUNC174_IN_SEL_CFG {
        &self.func174_in_sel_cfg
    }
    #[doc = "0x414 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func175_in_sel_cfg(&self) -> &FUNC175_IN_SEL_CFG {
        &self.func175_in_sel_cfg
    }
    #[doc = "0x418 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func176_in_sel_cfg(&self) -> &FUNC176_IN_SEL_CFG {
        &self.func176_in_sel_cfg
    }
    #[doc = "0x41c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func177_in_sel_cfg(&self) -> &FUNC177_IN_SEL_CFG {
        &self.func177_in_sel_cfg
    }
    #[doc = "0x420 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func178_in_sel_cfg(&self) -> &FUNC178_IN_SEL_CFG {
        &self.func178_in_sel_cfg
    }
    #[doc = "0x424 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func179_in_sel_cfg(&self) -> &FUNC179_IN_SEL_CFG {
        &self.func179_in_sel_cfg
    }
    #[doc = "0x428 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func180_in_sel_cfg(&self) -> &FUNC180_IN_SEL_CFG {
        &self.func180_in_sel_cfg
    }
    #[doc = "0x42c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func181_in_sel_cfg(&self) -> &FUNC181_IN_SEL_CFG {
        &self.func181_in_sel_cfg
    }
    #[doc = "0x430 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func182_in_sel_cfg(&self) -> &FUNC182_IN_SEL_CFG {
        &self.func182_in_sel_cfg
    }
    #[doc = "0x434 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func183_in_sel_cfg(&self) -> &FUNC183_IN_SEL_CFG {
        &self.func183_in_sel_cfg
    }
    #[doc = "0x438 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func184_in_sel_cfg(&self) -> &FUNC184_IN_SEL_CFG {
        &self.func184_in_sel_cfg
    }
    #[doc = "0x43c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func185_in_sel_cfg(&self) -> &FUNC185_IN_SEL_CFG {
        &self.func185_in_sel_cfg
    }
    #[doc = "0x440 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func186_in_sel_cfg(&self) -> &FUNC186_IN_SEL_CFG {
        &self.func186_in_sel_cfg
    }
    #[doc = "0x444 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func187_in_sel_cfg(&self) -> &FUNC187_IN_SEL_CFG {
        &self.func187_in_sel_cfg
    }
    #[doc = "0x448 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func188_in_sel_cfg(&self) -> &FUNC188_IN_SEL_CFG {
        &self.func188_in_sel_cfg
    }
    #[doc = "0x44c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func189_in_sel_cfg(&self) -> &FUNC189_IN_SEL_CFG {
        &self.func189_in_sel_cfg
    }
    #[doc = "0x450 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func190_in_sel_cfg(&self) -> &FUNC190_IN_SEL_CFG {
        &self.func190_in_sel_cfg
    }
    #[doc = "0x454 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func191_in_sel_cfg(&self) -> &FUNC191_IN_SEL_CFG {
        &self.func191_in_sel_cfg
    }
    #[doc = "0x458 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func192_in_sel_cfg(&self) -> &FUNC192_IN_SEL_CFG {
        &self.func192_in_sel_cfg
    }
    #[doc = "0x45c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func193_in_sel_cfg(&self) -> &FUNC193_IN_SEL_CFG {
        &self.func193_in_sel_cfg
    }
    #[doc = "0x460 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func194_in_sel_cfg(&self) -> &FUNC194_IN_SEL_CFG {
        &self.func194_in_sel_cfg
    }
    #[doc = "0x464 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func195_in_sel_cfg(&self) -> &FUNC195_IN_SEL_CFG {
        &self.func195_in_sel_cfg
    }
    #[doc = "0x468 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func196_in_sel_cfg(&self) -> &FUNC196_IN_SEL_CFG {
        &self.func196_in_sel_cfg
    }
    #[doc = "0x46c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func197_in_sel_cfg(&self) -> &FUNC197_IN_SEL_CFG {
        &self.func197_in_sel_cfg
    }
    #[doc = "0x470 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func198_in_sel_cfg(&self) -> &FUNC198_IN_SEL_CFG {
        &self.func198_in_sel_cfg
    }
    #[doc = "0x474 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func199_in_sel_cfg(&self) -> &FUNC199_IN_SEL_CFG {
        &self.func199_in_sel_cfg
    }
    #[doc = "0x478 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func200_in_sel_cfg(&self) -> &FUNC200_IN_SEL_CFG {
        &self.func200_in_sel_cfg
    }
    #[doc = "0x47c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func201_in_sel_cfg(&self) -> &FUNC201_IN_SEL_CFG {
        &self.func201_in_sel_cfg
    }
    #[doc = "0x480 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func202_in_sel_cfg(&self) -> &FUNC202_IN_SEL_CFG {
        &self.func202_in_sel_cfg
    }
    #[doc = "0x484 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func203_in_sel_cfg(&self) -> &FUNC203_IN_SEL_CFG {
        &self.func203_in_sel_cfg
    }
    #[doc = "0x4b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func214_in_sel_cfg(&self) -> &FUNC214_IN_SEL_CFG {
        &self.func214_in_sel_cfg
    }
    #[doc = "0x4b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func215_in_sel_cfg(&self) -> &FUNC215_IN_SEL_CFG {
        &self.func215_in_sel_cfg
    }
    #[doc = "0x4b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func216_in_sel_cfg(&self) -> &FUNC216_IN_SEL_CFG {
        &self.func216_in_sel_cfg
    }
    #[doc = "0x4bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func217_in_sel_cfg(&self) -> &FUNC217_IN_SEL_CFG {
        &self.func217_in_sel_cfg
    }
    #[doc = "0x4c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func218_in_sel_cfg(&self) -> &FUNC218_IN_SEL_CFG {
        &self.func218_in_sel_cfg
    }
    #[doc = "0x4c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func219_in_sel_cfg(&self) -> &FUNC219_IN_SEL_CFG {
        &self.func219_in_sel_cfg
    }
    #[doc = "0x4c8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func220_in_sel_cfg(&self) -> &FUNC220_IN_SEL_CFG {
        &self.func220_in_sel_cfg
    }
    #[doc = "0x4cc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func221_in_sel_cfg(&self) -> &FUNC221_IN_SEL_CFG {
        &self.func221_in_sel_cfg
    }
    #[doc = "0x4d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func222_in_sel_cfg(&self) -> &FUNC222_IN_SEL_CFG {
        &self.func222_in_sel_cfg
    }
    #[doc = "0x4d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func223_in_sel_cfg(&self) -> &FUNC223_IN_SEL_CFG {
        &self.func223_in_sel_cfg
    }
    #[doc = "0x4d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func224_in_sel_cfg(&self) -> &FUNC224_IN_SEL_CFG {
        &self.func224_in_sel_cfg
    }
    #[doc = "0x4dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func225_in_sel_cfg(&self) -> &FUNC225_IN_SEL_CFG {
        &self.func225_in_sel_cfg
    }
    #[doc = "0x4e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func226_in_sel_cfg(&self) -> &FUNC226_IN_SEL_CFG {
        &self.func226_in_sel_cfg
    }
    #[doc = "0x4e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func227_in_sel_cfg(&self) -> &FUNC227_IN_SEL_CFG {
        &self.func227_in_sel_cfg
    }
    #[doc = "0x4e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func228_in_sel_cfg(&self) -> &FUNC228_IN_SEL_CFG {
        &self.func228_in_sel_cfg
    }
    #[doc = "0x4ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func229_in_sel_cfg(&self) -> &FUNC229_IN_SEL_CFG {
        &self.func229_in_sel_cfg
    }
    #[doc = "0x4f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func230_in_sel_cfg(&self) -> &FUNC230_IN_SEL_CFG {
        &self.func230_in_sel_cfg
    }
    #[doc = "0x4f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func231_in_sel_cfg(&self) -> &FUNC231_IN_SEL_CFG {
        &self.func231_in_sel_cfg
    }
    #[doc = "0x4f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func232_in_sel_cfg(&self) -> &FUNC232_IN_SEL_CFG {
        &self.func232_in_sel_cfg
    }
    #[doc = "0x4fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func233_in_sel_cfg(&self) -> &FUNC233_IN_SEL_CFG {
        &self.func233_in_sel_cfg
    }
    #[doc = "0x500 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func234_in_sel_cfg(&self) -> &FUNC234_IN_SEL_CFG {
        &self.func234_in_sel_cfg
    }
    #[doc = "0x504 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func235_in_sel_cfg(&self) -> &FUNC235_IN_SEL_CFG {
        &self.func235_in_sel_cfg
    }
    #[doc = "0x508 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func236_in_sel_cfg(&self) -> &FUNC236_IN_SEL_CFG {
        &self.func236_in_sel_cfg
    }
    #[doc = "0x50c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func237_in_sel_cfg(&self) -> &FUNC237_IN_SEL_CFG {
        &self.func237_in_sel_cfg
    }
    #[doc = "0x510 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func238_in_sel_cfg(&self) -> &FUNC238_IN_SEL_CFG {
        &self.func238_in_sel_cfg
    }
    #[doc = "0x514 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func239_in_sel_cfg(&self) -> &FUNC239_IN_SEL_CFG {
        &self.func239_in_sel_cfg
    }
    #[doc = "0x518 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func240_in_sel_cfg(&self) -> &FUNC240_IN_SEL_CFG {
        &self.func240_in_sel_cfg
    }
    #[doc = "0x51c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func241_in_sel_cfg(&self) -> &FUNC241_IN_SEL_CFG {
        &self.func241_in_sel_cfg
    }
    #[doc = "0x520 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func242_in_sel_cfg(&self) -> &FUNC242_IN_SEL_CFG {
        &self.func242_in_sel_cfg
    }
    #[doc = "0x524 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func243_in_sel_cfg(&self) -> &FUNC243_IN_SEL_CFG {
        &self.func243_in_sel_cfg
    }
    #[doc = "0x528 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func244_in_sel_cfg(&self) -> &FUNC244_IN_SEL_CFG {
        &self.func244_in_sel_cfg
    }
    #[doc = "0x52c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func245_in_sel_cfg(&self) -> &FUNC245_IN_SEL_CFG {
        &self.func245_in_sel_cfg
    }
    #[doc = "0x530 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func246_in_sel_cfg(&self) -> &FUNC246_IN_SEL_CFG {
        &self.func246_in_sel_cfg
    }
    #[doc = "0x534 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func247_in_sel_cfg(&self) -> &FUNC247_IN_SEL_CFG {
        &self.func247_in_sel_cfg
    }
    #[doc = "0x538 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func248_in_sel_cfg(&self) -> &FUNC248_IN_SEL_CFG {
        &self.func248_in_sel_cfg
    }
    #[doc = "0x53c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func249_in_sel_cfg(&self) -> &FUNC249_IN_SEL_CFG {
        &self.func249_in_sel_cfg
    }
    #[doc = "0x540 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func250_in_sel_cfg(&self) -> &FUNC250_IN_SEL_CFG {
        &self.func250_in_sel_cfg
    }
    #[doc = "0x544 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func251_in_sel_cfg(&self) -> &FUNC251_IN_SEL_CFG {
        &self.func251_in_sel_cfg
    }
    #[doc = "0x548 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func252_in_sel_cfg(&self) -> &FUNC252_IN_SEL_CFG {
        &self.func252_in_sel_cfg
    }
    #[doc = "0x54c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func253_in_sel_cfg(&self) -> &FUNC253_IN_SEL_CFG {
        &self.func253_in_sel_cfg
    }
    #[doc = "0x550 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func254_in_sel_cfg(&self) -> &FUNC254_IN_SEL_CFG {
        &self.func254_in_sel_cfg
    }
    #[doc = "0x554 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func255_in_sel_cfg(&self) -> &FUNC255_IN_SEL_CFG {
        &self.func255_in_sel_cfg
    }
    #[doc = "0x558..0x63c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x558..0x63c - GPIO output function select register"]
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    #[doc = "0x558 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    #[doc = "0x55c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    #[doc = "0x560 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    #[doc = "0x564 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    #[doc = "0x568 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    #[doc = "0x56c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    #[doc = "0x570 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    #[doc = "0x574 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    #[doc = "0x578 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    #[doc = "0x57c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    #[doc = "0x580 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    #[doc = "0x584 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    #[doc = "0x588 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    #[doc = "0x58c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    #[doc = "0x590 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    #[doc = "0x594 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    #[doc = "0x598 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    #[doc = "0x59c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    #[doc = "0x5a0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    #[doc = "0x5a4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    #[doc = "0x5a8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    #[doc = "0x5ac - GPIO output function select register"]
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    #[doc = "0x5b0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    #[doc = "0x5b4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    #[doc = "0x5b8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    #[doc = "0x5bc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    #[doc = "0x5c0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    #[doc = "0x5c4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    #[doc = "0x5c8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    #[doc = "0x5cc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    #[doc = "0x5d0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    #[doc = "0x5d4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    #[doc = "0x5d8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    #[doc = "0x5dc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
    }
    #[doc = "0x5e0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func34_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(34)
    }
    #[doc = "0x5e4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func35_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(35)
    }
    #[doc = "0x5e8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func36_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(36)
    }
    #[doc = "0x5ec - GPIO output function select register"]
    #[inline(always)]
    pub const fn func37_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(37)
    }
    #[doc = "0x5f0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func38_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(38)
    }
    #[doc = "0x5f4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func39_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(39)
    }
    #[doc = "0x5f8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func40_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(40)
    }
    #[doc = "0x5fc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func41_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(41)
    }
    #[doc = "0x600 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func42_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(42)
    }
    #[doc = "0x604 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func43_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(43)
    }
    #[doc = "0x608 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func44_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(44)
    }
    #[doc = "0x60c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func45_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(45)
    }
    #[doc = "0x610 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func46_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(46)
    }
    #[doc = "0x614 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func47_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(47)
    }
    #[doc = "0x618 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func48_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(48)
    }
    #[doc = "0x61c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func49_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(49)
    }
    #[doc = "0x620 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func50_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(50)
    }
    #[doc = "0x624 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func51_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(51)
    }
    #[doc = "0x628 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func52_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(52)
    }
    #[doc = "0x62c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func53_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(53)
    }
    #[doc = "0x630 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func54_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(54)
    }
    #[doc = "0x634 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func55_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(55)
    }
    #[doc = "0x638 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func56_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(56)
    }
    #[doc = "0x63c - GPIO interrupt 2 status register for GPIO0-31"]
    #[inline(always)]
    pub const fn intr_2(&self) -> &INTR_2 {
        &self.intr_2
    }
    #[doc = "0x640 - GPIO interrupt 2 status register for GPIO32-56"]
    #[inline(always)]
    pub const fn intr1_2(&self) -> &INTR1_2 {
        &self.intr1_2
    }
    #[doc = "0x644 - GPIO interrupt 3 status register for GPIO0-31"]
    #[inline(always)]
    pub const fn intr_3(&self) -> &INTR_3 {
        &self.intr_3
    }
    #[doc = "0x648 - GPIO interrupt 3 status register for GPIO32-56"]
    #[inline(always)]
    pub const fn intr1_3(&self) -> &INTR1_3 {
        &self.intr1_3
    }
    #[doc = "0x64c - GPIO clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x700 - analog comparator interrupt raw"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x704 - analog comparator interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x708 - analog comparator interrupt enable"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x70c - analog comparator interrupt clear"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x710 - GPIO analog comparator zero detect filter count"]
    #[inline(always)]
    pub const fn zero_det0_filter_cnt(&self) -> &ZERO_DET0_FILTER_CNT {
        &self.zero_det0_filter_cnt
    }
    #[doc = "0x714 - GPIO analog comparator zero detect filter count"]
    #[inline(always)]
    pub const fn zero_det1_filter_cnt(&self) -> &ZERO_DET1_FILTER_CNT {
        &self.zero_det1_filter_cnt
    }
    #[doc = "0x718 - High speed sdio pad bist send sequence"]
    #[inline(always)]
    pub const fn send_seq(&self) -> &SEND_SEQ {
        &self.send_seq
    }
    #[doc = "0x71c - High speed sdio pad bist recive sequence"]
    #[inline(always)]
    pub const fn recive_seq(&self) -> &RECIVE_SEQ {
        &self.recive_seq
    }
    #[doc = "0x720 - High speed sdio pad bist in pad sel"]
    #[inline(always)]
    pub const fn bistin_sel(&self) -> &BISTIN_SEL {
        &self.bistin_sel
    }
    #[doc = "0x724 - High speed sdio pad bist control"]
    #[inline(always)]
    pub const fn bist_ctrl(&self) -> &BIST_CTRL {
        &self.bist_ctrl
    }
    #[doc = "0x7fc - GPIO version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
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
#[doc = "OUT1 (rw) register accessor: GPIO output register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register for GPIO32-56"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: GPIO output set register for GPIO32-56\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO32-56"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: GPIO output clear register for GPIO32-56\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO32-56"]
pub mod out1_w1tc;
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
#[doc = "ENABLE1 (rw) register accessor: GPIO output enable register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register for GPIO32-56"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: GPIO output enable set register for GPIO32-56\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO32-56"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: GPIO output enable clear register for GPIO32-56\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO32-56"]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: pad strapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "IN (r) register accessor: GPIO input register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register for GPIO0-31"]
pub mod in_;
#[doc = "IN1 (r) register accessor: GPIO input register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register for GPIO32-56"]
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
#[doc = "STATUS1 (rw) register accessor: GPIO interrupt status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register for GPIO32-56"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: GPIO interrupt status set register for GPIO32-56\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO32-56"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO32-56\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO32-56"]
pub mod status1_w1tc;
#[doc = "INTR_0 (r) register accessor: GPIO interrupt 0 status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_0`] module"]
pub type INTR_0 = crate::Reg<intr_0::INTR_0_SPEC>;
#[doc = "GPIO interrupt 0 status register for GPIO0-31"]
pub mod intr_0;
#[doc = "INTR1_0 (r) register accessor: GPIO interrupt 0 status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr1_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr1_0`] module"]
pub type INTR1_0 = crate::Reg<intr1_0::INTR1_0_SPEC>;
#[doc = "GPIO interrupt 0 status register for GPIO32-56"]
pub mod intr1_0;
#[doc = "INTR_1 (r) register accessor: GPIO interrupt 1 status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_1`] module"]
pub type INTR_1 = crate::Reg<intr_1::INTR_1_SPEC>;
#[doc = "GPIO interrupt 1 status register for GPIO0-31"]
pub mod intr_1;
#[doc = "INTR1_1 (r) register accessor: GPIO interrupt 1 status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr1_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr1_1`] module"]
pub type INTR1_1 = crate::Reg<intr1_1::INTR1_1_SPEC>;
#[doc = "GPIO interrupt 1 status register for GPIO32-56"]
pub mod intr1_1;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register for GPIO0-31"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: GPIO interrupt source register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next1`] module"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register for GPIO32-56"]
pub mod status_next1;
#[doc = "PIN (rw) register accessor: GPIO pin configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin;
#[doc = "FUNC1_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func1_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func1_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func1_in_sel_cfg`] module"]
pub type FUNC1_IN_SEL_CFG = crate::Reg<func1_in_sel_cfg::FUNC1_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func1_in_sel_cfg;
#[doc = "FUNC2_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func2_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func2_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func2_in_sel_cfg`] module"]
pub type FUNC2_IN_SEL_CFG = crate::Reg<func2_in_sel_cfg::FUNC2_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func2_in_sel_cfg;
#[doc = "FUNC3_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func3_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func3_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func3_in_sel_cfg`] module"]
pub type FUNC3_IN_SEL_CFG = crate::Reg<func3_in_sel_cfg::FUNC3_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func3_in_sel_cfg;
#[doc = "FUNC4_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func4_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func4_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func4_in_sel_cfg`] module"]
pub type FUNC4_IN_SEL_CFG = crate::Reg<func4_in_sel_cfg::FUNC4_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func4_in_sel_cfg;
#[doc = "FUNC5_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func5_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func5_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func5_in_sel_cfg`] module"]
pub type FUNC5_IN_SEL_CFG = crate::Reg<func5_in_sel_cfg::FUNC5_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func5_in_sel_cfg;
#[doc = "FUNC6_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func6_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func6_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func6_in_sel_cfg`] module"]
pub type FUNC6_IN_SEL_CFG = crate::Reg<func6_in_sel_cfg::FUNC6_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func6_in_sel_cfg;
#[doc = "FUNC7_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func7_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func7_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func7_in_sel_cfg`] module"]
pub type FUNC7_IN_SEL_CFG = crate::Reg<func7_in_sel_cfg::FUNC7_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func7_in_sel_cfg;
#[doc = "FUNC8_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func8_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func8_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func8_in_sel_cfg`] module"]
pub type FUNC8_IN_SEL_CFG = crate::Reg<func8_in_sel_cfg::FUNC8_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func8_in_sel_cfg;
#[doc = "FUNC9_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func9_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func9_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func9_in_sel_cfg`] module"]
pub type FUNC9_IN_SEL_CFG = crate::Reg<func9_in_sel_cfg::FUNC9_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func9_in_sel_cfg;
#[doc = "FUNC10_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func10_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func10_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func10_in_sel_cfg`] module"]
pub type FUNC10_IN_SEL_CFG = crate::Reg<func10_in_sel_cfg::FUNC10_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func10_in_sel_cfg;
#[doc = "FUNC11_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func11_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func11_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func11_in_sel_cfg`] module"]
pub type FUNC11_IN_SEL_CFG = crate::Reg<func11_in_sel_cfg::FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func11_in_sel_cfg;
#[doc = "FUNC12_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func12_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func12_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func12_in_sel_cfg`] module"]
pub type FUNC12_IN_SEL_CFG = crate::Reg<func12_in_sel_cfg::FUNC12_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func12_in_sel_cfg;
#[doc = "FUNC13_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func13_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func13_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func13_in_sel_cfg`] module"]
pub type FUNC13_IN_SEL_CFG = crate::Reg<func13_in_sel_cfg::FUNC13_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func13_in_sel_cfg;
#[doc = "FUNC14_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func14_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func14_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func14_in_sel_cfg`] module"]
pub type FUNC14_IN_SEL_CFG = crate::Reg<func14_in_sel_cfg::FUNC14_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func14_in_sel_cfg;
#[doc = "FUNC15_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func15_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func15_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func15_in_sel_cfg`] module"]
pub type FUNC15_IN_SEL_CFG = crate::Reg<func15_in_sel_cfg::FUNC15_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func15_in_sel_cfg;
#[doc = "FUNC16_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func16_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func16_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func16_in_sel_cfg`] module"]
pub type FUNC16_IN_SEL_CFG = crate::Reg<func16_in_sel_cfg::FUNC16_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func16_in_sel_cfg;
#[doc = "FUNC17_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func17_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func17_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func17_in_sel_cfg`] module"]
pub type FUNC17_IN_SEL_CFG = crate::Reg<func17_in_sel_cfg::FUNC17_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func17_in_sel_cfg;
#[doc = "FUNC18_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func18_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func18_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func18_in_sel_cfg`] module"]
pub type FUNC18_IN_SEL_CFG = crate::Reg<func18_in_sel_cfg::FUNC18_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func18_in_sel_cfg;
#[doc = "FUNC19_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func19_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func19_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func19_in_sel_cfg`] module"]
pub type FUNC19_IN_SEL_CFG = crate::Reg<func19_in_sel_cfg::FUNC19_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func19_in_sel_cfg;
#[doc = "FUNC20_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func20_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func20_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func20_in_sel_cfg`] module"]
pub type FUNC20_IN_SEL_CFG = crate::Reg<func20_in_sel_cfg::FUNC20_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func20_in_sel_cfg;
#[doc = "FUNC21_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func21_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func21_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func21_in_sel_cfg`] module"]
pub type FUNC21_IN_SEL_CFG = crate::Reg<func21_in_sel_cfg::FUNC21_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func21_in_sel_cfg;
#[doc = "FUNC22_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func22_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func22_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func22_in_sel_cfg`] module"]
pub type FUNC22_IN_SEL_CFG = crate::Reg<func22_in_sel_cfg::FUNC22_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func22_in_sel_cfg;
#[doc = "FUNC23_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func23_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func23_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func23_in_sel_cfg`] module"]
pub type FUNC23_IN_SEL_CFG = crate::Reg<func23_in_sel_cfg::FUNC23_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func23_in_sel_cfg;
#[doc = "FUNC24_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func24_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func24_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func24_in_sel_cfg`] module"]
pub type FUNC24_IN_SEL_CFG = crate::Reg<func24_in_sel_cfg::FUNC24_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func24_in_sel_cfg;
#[doc = "FUNC25_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func25_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func25_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func25_in_sel_cfg`] module"]
pub type FUNC25_IN_SEL_CFG = crate::Reg<func25_in_sel_cfg::FUNC25_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func25_in_sel_cfg;
#[doc = "FUNC26_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func26_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func26_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func26_in_sel_cfg`] module"]
pub type FUNC26_IN_SEL_CFG = crate::Reg<func26_in_sel_cfg::FUNC26_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func26_in_sel_cfg;
#[doc = "FUNC27_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func27_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func27_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func27_in_sel_cfg`] module"]
pub type FUNC27_IN_SEL_CFG = crate::Reg<func27_in_sel_cfg::FUNC27_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func27_in_sel_cfg;
#[doc = "FUNC28_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func28_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func28_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func28_in_sel_cfg`] module"]
pub type FUNC28_IN_SEL_CFG = crate::Reg<func28_in_sel_cfg::FUNC28_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func28_in_sel_cfg;
#[doc = "FUNC29_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func29_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func29_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func29_in_sel_cfg`] module"]
pub type FUNC29_IN_SEL_CFG = crate::Reg<func29_in_sel_cfg::FUNC29_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func29_in_sel_cfg;
#[doc = "FUNC30_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func30_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func30_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func30_in_sel_cfg`] module"]
pub type FUNC30_IN_SEL_CFG = crate::Reg<func30_in_sel_cfg::FUNC30_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func30_in_sel_cfg;
#[doc = "FUNC31_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func31_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func31_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func31_in_sel_cfg`] module"]
pub type FUNC31_IN_SEL_CFG = crate::Reg<func31_in_sel_cfg::FUNC31_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func31_in_sel_cfg;
#[doc = "FUNC32_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func32_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func32_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func32_in_sel_cfg`] module"]
pub type FUNC32_IN_SEL_CFG = crate::Reg<func32_in_sel_cfg::FUNC32_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func32_in_sel_cfg;
#[doc = "FUNC33_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func33_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func33_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func33_in_sel_cfg`] module"]
pub type FUNC33_IN_SEL_CFG = crate::Reg<func33_in_sel_cfg::FUNC33_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func33_in_sel_cfg;
#[doc = "FUNC34_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func34_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func34_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func34_in_sel_cfg`] module"]
pub type FUNC34_IN_SEL_CFG = crate::Reg<func34_in_sel_cfg::FUNC34_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func34_in_sel_cfg;
#[doc = "FUNC35_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func35_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func35_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func35_in_sel_cfg`] module"]
pub type FUNC35_IN_SEL_CFG = crate::Reg<func35_in_sel_cfg::FUNC35_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func35_in_sel_cfg;
#[doc = "FUNC36_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func36_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func36_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func36_in_sel_cfg`] module"]
pub type FUNC36_IN_SEL_CFG = crate::Reg<func36_in_sel_cfg::FUNC36_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func36_in_sel_cfg;
#[doc = "FUNC37_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func37_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func37_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func37_in_sel_cfg`] module"]
pub type FUNC37_IN_SEL_CFG = crate::Reg<func37_in_sel_cfg::FUNC37_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func37_in_sel_cfg;
#[doc = "FUNC38_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func38_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func38_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func38_in_sel_cfg`] module"]
pub type FUNC38_IN_SEL_CFG = crate::Reg<func38_in_sel_cfg::FUNC38_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func38_in_sel_cfg;
#[doc = "FUNC39_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func39_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func39_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func39_in_sel_cfg`] module"]
pub type FUNC39_IN_SEL_CFG = crate::Reg<func39_in_sel_cfg::FUNC39_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func39_in_sel_cfg;
#[doc = "FUNC40_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func40_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func40_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func40_in_sel_cfg`] module"]
pub type FUNC40_IN_SEL_CFG = crate::Reg<func40_in_sel_cfg::FUNC40_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func40_in_sel_cfg;
#[doc = "FUNC41_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func41_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func41_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func41_in_sel_cfg`] module"]
pub type FUNC41_IN_SEL_CFG = crate::Reg<func41_in_sel_cfg::FUNC41_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func41_in_sel_cfg;
#[doc = "FUNC42_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func42_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func42_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func42_in_sel_cfg`] module"]
pub type FUNC42_IN_SEL_CFG = crate::Reg<func42_in_sel_cfg::FUNC42_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func42_in_sel_cfg;
#[doc = "FUNC43_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func43_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func43_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func43_in_sel_cfg`] module"]
pub type FUNC43_IN_SEL_CFG = crate::Reg<func43_in_sel_cfg::FUNC43_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func43_in_sel_cfg;
#[doc = "FUNC44_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func44_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func44_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func44_in_sel_cfg`] module"]
pub type FUNC44_IN_SEL_CFG = crate::Reg<func44_in_sel_cfg::FUNC44_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func44_in_sel_cfg;
#[doc = "FUNC45_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func45_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func45_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func45_in_sel_cfg`] module"]
pub type FUNC45_IN_SEL_CFG = crate::Reg<func45_in_sel_cfg::FUNC45_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func45_in_sel_cfg;
#[doc = "FUNC47_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func47_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func47_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func47_in_sel_cfg`] module"]
pub type FUNC47_IN_SEL_CFG = crate::Reg<func47_in_sel_cfg::FUNC47_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func47_in_sel_cfg;
#[doc = "FUNC48_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func48_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func48_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func48_in_sel_cfg`] module"]
pub type FUNC48_IN_SEL_CFG = crate::Reg<func48_in_sel_cfg::FUNC48_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func48_in_sel_cfg;
#[doc = "FUNC49_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func49_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func49_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func49_in_sel_cfg`] module"]
pub type FUNC49_IN_SEL_CFG = crate::Reg<func49_in_sel_cfg::FUNC49_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func49_in_sel_cfg;
#[doc = "FUNC50_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func50_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func50_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func50_in_sel_cfg`] module"]
pub type FUNC50_IN_SEL_CFG = crate::Reg<func50_in_sel_cfg::FUNC50_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func50_in_sel_cfg;
#[doc = "FUNC51_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func51_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func51_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func51_in_sel_cfg`] module"]
pub type FUNC51_IN_SEL_CFG = crate::Reg<func51_in_sel_cfg::FUNC51_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func51_in_sel_cfg;
#[doc = "FUNC52_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func52_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func52_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func52_in_sel_cfg`] module"]
pub type FUNC52_IN_SEL_CFG = crate::Reg<func52_in_sel_cfg::FUNC52_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func52_in_sel_cfg;
#[doc = "FUNC53_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func53_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func53_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func53_in_sel_cfg`] module"]
pub type FUNC53_IN_SEL_CFG = crate::Reg<func53_in_sel_cfg::FUNC53_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func53_in_sel_cfg;
#[doc = "FUNC54_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func54_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func54_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func54_in_sel_cfg`] module"]
pub type FUNC54_IN_SEL_CFG = crate::Reg<func54_in_sel_cfg::FUNC54_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func54_in_sel_cfg;
#[doc = "FUNC55_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func55_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func55_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func55_in_sel_cfg`] module"]
pub type FUNC55_IN_SEL_CFG = crate::Reg<func55_in_sel_cfg::FUNC55_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func55_in_sel_cfg;
#[doc = "FUNC56_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func56_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func56_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func56_in_sel_cfg`] module"]
pub type FUNC56_IN_SEL_CFG = crate::Reg<func56_in_sel_cfg::FUNC56_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func56_in_sel_cfg;
#[doc = "FUNC57_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func57_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func57_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func57_in_sel_cfg`] module"]
pub type FUNC57_IN_SEL_CFG = crate::Reg<func57_in_sel_cfg::FUNC57_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func57_in_sel_cfg;
#[doc = "FUNC58_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func58_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func58_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func58_in_sel_cfg`] module"]
pub type FUNC58_IN_SEL_CFG = crate::Reg<func58_in_sel_cfg::FUNC58_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func58_in_sel_cfg;
#[doc = "FUNC59_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func59_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func59_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func59_in_sel_cfg`] module"]
pub type FUNC59_IN_SEL_CFG = crate::Reg<func59_in_sel_cfg::FUNC59_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func59_in_sel_cfg;
#[doc = "FUNC60_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func60_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func60_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func60_in_sel_cfg`] module"]
pub type FUNC60_IN_SEL_CFG = crate::Reg<func60_in_sel_cfg::FUNC60_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func60_in_sel_cfg;
#[doc = "FUNC61_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func61_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func61_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func61_in_sel_cfg`] module"]
pub type FUNC61_IN_SEL_CFG = crate::Reg<func61_in_sel_cfg::FUNC61_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func61_in_sel_cfg;
#[doc = "FUNC62_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func62_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func62_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func62_in_sel_cfg`] module"]
pub type FUNC62_IN_SEL_CFG = crate::Reg<func62_in_sel_cfg::FUNC62_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func62_in_sel_cfg;
#[doc = "FUNC63_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func63_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func63_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func63_in_sel_cfg`] module"]
pub type FUNC63_IN_SEL_CFG = crate::Reg<func63_in_sel_cfg::FUNC63_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func63_in_sel_cfg;
#[doc = "FUNC64_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func64_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func64_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func64_in_sel_cfg`] module"]
pub type FUNC64_IN_SEL_CFG = crate::Reg<func64_in_sel_cfg::FUNC64_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func64_in_sel_cfg;
#[doc = "FUNC65_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func65_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func65_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func65_in_sel_cfg`] module"]
pub type FUNC65_IN_SEL_CFG = crate::Reg<func65_in_sel_cfg::FUNC65_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func65_in_sel_cfg;
#[doc = "FUNC66_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func66_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func66_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func66_in_sel_cfg`] module"]
pub type FUNC66_IN_SEL_CFG = crate::Reg<func66_in_sel_cfg::FUNC66_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func66_in_sel_cfg;
#[doc = "FUNC68_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func68_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func68_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func68_in_sel_cfg`] module"]
pub type FUNC68_IN_SEL_CFG = crate::Reg<func68_in_sel_cfg::FUNC68_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func68_in_sel_cfg;
#[doc = "FUNC69_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func69_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func69_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func69_in_sel_cfg`] module"]
pub type FUNC69_IN_SEL_CFG = crate::Reg<func69_in_sel_cfg::FUNC69_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func69_in_sel_cfg;
#[doc = "FUNC70_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func70_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func70_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func70_in_sel_cfg`] module"]
pub type FUNC70_IN_SEL_CFG = crate::Reg<func70_in_sel_cfg::FUNC70_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func70_in_sel_cfg;
#[doc = "FUNC71_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func71_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func71_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func71_in_sel_cfg`] module"]
pub type FUNC71_IN_SEL_CFG = crate::Reg<func71_in_sel_cfg::FUNC71_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func71_in_sel_cfg;
#[doc = "FUNC74_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func74_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func74_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func74_in_sel_cfg`] module"]
pub type FUNC74_IN_SEL_CFG = crate::Reg<func74_in_sel_cfg::FUNC74_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func74_in_sel_cfg;
#[doc = "FUNC75_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func75_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func75_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func75_in_sel_cfg`] module"]
pub type FUNC75_IN_SEL_CFG = crate::Reg<func75_in_sel_cfg::FUNC75_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func75_in_sel_cfg;
#[doc = "FUNC76_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func76_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func76_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func76_in_sel_cfg`] module"]
pub type FUNC76_IN_SEL_CFG = crate::Reg<func76_in_sel_cfg::FUNC76_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func76_in_sel_cfg;
#[doc = "FUNC77_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func77_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func77_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func77_in_sel_cfg`] module"]
pub type FUNC77_IN_SEL_CFG = crate::Reg<func77_in_sel_cfg::FUNC77_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func77_in_sel_cfg;
#[doc = "FUNC78_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func78_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func78_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func78_in_sel_cfg`] module"]
pub type FUNC78_IN_SEL_CFG = crate::Reg<func78_in_sel_cfg::FUNC78_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func78_in_sel_cfg;
#[doc = "FUNC80_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func80_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func80_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func80_in_sel_cfg`] module"]
pub type FUNC80_IN_SEL_CFG = crate::Reg<func80_in_sel_cfg::FUNC80_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func80_in_sel_cfg;
#[doc = "FUNC83_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func83_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func83_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func83_in_sel_cfg`] module"]
pub type FUNC83_IN_SEL_CFG = crate::Reg<func83_in_sel_cfg::FUNC83_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func83_in_sel_cfg;
#[doc = "FUNC86_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func86_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func86_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func86_in_sel_cfg`] module"]
pub type FUNC86_IN_SEL_CFG = crate::Reg<func86_in_sel_cfg::FUNC86_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func86_in_sel_cfg;
#[doc = "FUNC89_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func89_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func89_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func89_in_sel_cfg`] module"]
pub type FUNC89_IN_SEL_CFG = crate::Reg<func89_in_sel_cfg::FUNC89_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func89_in_sel_cfg;
#[doc = "FUNC90_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func90_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func90_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func90_in_sel_cfg`] module"]
pub type FUNC90_IN_SEL_CFG = crate::Reg<func90_in_sel_cfg::FUNC90_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func90_in_sel_cfg;
#[doc = "FUNC91_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func91_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func91_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func91_in_sel_cfg`] module"]
pub type FUNC91_IN_SEL_CFG = crate::Reg<func91_in_sel_cfg::FUNC91_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func91_in_sel_cfg;
#[doc = "FUNC92_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func92_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func92_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func92_in_sel_cfg`] module"]
pub type FUNC92_IN_SEL_CFG = crate::Reg<func92_in_sel_cfg::FUNC92_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func92_in_sel_cfg;
#[doc = "FUNC93_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func93_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func93_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func93_in_sel_cfg`] module"]
pub type FUNC93_IN_SEL_CFG = crate::Reg<func93_in_sel_cfg::FUNC93_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func93_in_sel_cfg;
#[doc = "FUNC94_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func94_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func94_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func94_in_sel_cfg`] module"]
pub type FUNC94_IN_SEL_CFG = crate::Reg<func94_in_sel_cfg::FUNC94_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func94_in_sel_cfg;
#[doc = "FUNC95_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func95_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func95_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func95_in_sel_cfg`] module"]
pub type FUNC95_IN_SEL_CFG = crate::Reg<func95_in_sel_cfg::FUNC95_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func95_in_sel_cfg;
#[doc = "FUNC96_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func96_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func96_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func96_in_sel_cfg`] module"]
pub type FUNC96_IN_SEL_CFG = crate::Reg<func96_in_sel_cfg::FUNC96_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func96_in_sel_cfg;
#[doc = "FUNC97_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func97_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func97_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func97_in_sel_cfg`] module"]
pub type FUNC97_IN_SEL_CFG = crate::Reg<func97_in_sel_cfg::FUNC97_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func97_in_sel_cfg;
#[doc = "FUNC98_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func98_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func98_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func98_in_sel_cfg`] module"]
pub type FUNC98_IN_SEL_CFG = crate::Reg<func98_in_sel_cfg::FUNC98_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func98_in_sel_cfg;
#[doc = "FUNC99_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func99_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func99_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func99_in_sel_cfg`] module"]
pub type FUNC99_IN_SEL_CFG = crate::Reg<func99_in_sel_cfg::FUNC99_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func99_in_sel_cfg;
#[doc = "FUNC100_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func100_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func100_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func100_in_sel_cfg`] module"]
pub type FUNC100_IN_SEL_CFG = crate::Reg<func100_in_sel_cfg::FUNC100_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func100_in_sel_cfg;
#[doc = "FUNC101_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func101_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func101_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func101_in_sel_cfg`] module"]
pub type FUNC101_IN_SEL_CFG = crate::Reg<func101_in_sel_cfg::FUNC101_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func101_in_sel_cfg;
#[doc = "FUNC102_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func102_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func102_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func102_in_sel_cfg`] module"]
pub type FUNC102_IN_SEL_CFG = crate::Reg<func102_in_sel_cfg::FUNC102_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func102_in_sel_cfg;
#[doc = "FUNC103_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func103_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func103_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func103_in_sel_cfg`] module"]
pub type FUNC103_IN_SEL_CFG = crate::Reg<func103_in_sel_cfg::FUNC103_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func103_in_sel_cfg;
#[doc = "FUNC104_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func104_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func104_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func104_in_sel_cfg`] module"]
pub type FUNC104_IN_SEL_CFG = crate::Reg<func104_in_sel_cfg::FUNC104_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func104_in_sel_cfg;
#[doc = "FUNC105_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func105_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func105_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func105_in_sel_cfg`] module"]
pub type FUNC105_IN_SEL_CFG = crate::Reg<func105_in_sel_cfg::FUNC105_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func105_in_sel_cfg;
#[doc = "FUNC106_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func106_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func106_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func106_in_sel_cfg`] module"]
pub type FUNC106_IN_SEL_CFG = crate::Reg<func106_in_sel_cfg::FUNC106_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func106_in_sel_cfg;
#[doc = "FUNC107_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func107_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func107_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func107_in_sel_cfg`] module"]
pub type FUNC107_IN_SEL_CFG = crate::Reg<func107_in_sel_cfg::FUNC107_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func107_in_sel_cfg;
#[doc = "FUNC108_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func108_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func108_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func108_in_sel_cfg`] module"]
pub type FUNC108_IN_SEL_CFG = crate::Reg<func108_in_sel_cfg::FUNC108_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func108_in_sel_cfg;
#[doc = "FUNC109_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func109_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func109_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func109_in_sel_cfg`] module"]
pub type FUNC109_IN_SEL_CFG = crate::Reg<func109_in_sel_cfg::FUNC109_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func109_in_sel_cfg;
#[doc = "FUNC110_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func110_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func110_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func110_in_sel_cfg`] module"]
pub type FUNC110_IN_SEL_CFG = crate::Reg<func110_in_sel_cfg::FUNC110_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func110_in_sel_cfg;
#[doc = "FUNC111_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func111_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func111_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func111_in_sel_cfg`] module"]
pub type FUNC111_IN_SEL_CFG = crate::Reg<func111_in_sel_cfg::FUNC111_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func111_in_sel_cfg;
#[doc = "FUNC112_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func112_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func112_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func112_in_sel_cfg`] module"]
pub type FUNC112_IN_SEL_CFG = crate::Reg<func112_in_sel_cfg::FUNC112_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func112_in_sel_cfg;
#[doc = "FUNC113_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func113_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func113_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func113_in_sel_cfg`] module"]
pub type FUNC113_IN_SEL_CFG = crate::Reg<func113_in_sel_cfg::FUNC113_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func113_in_sel_cfg;
#[doc = "FUNC114_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func114_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func114_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func114_in_sel_cfg`] module"]
pub type FUNC114_IN_SEL_CFG = crate::Reg<func114_in_sel_cfg::FUNC114_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func114_in_sel_cfg;
#[doc = "FUNC117_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func117_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func117_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func117_in_sel_cfg`] module"]
pub type FUNC117_IN_SEL_CFG = crate::Reg<func117_in_sel_cfg::FUNC117_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func117_in_sel_cfg;
#[doc = "FUNC118_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func118_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func118_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func118_in_sel_cfg`] module"]
pub type FUNC118_IN_SEL_CFG = crate::Reg<func118_in_sel_cfg::FUNC118_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func118_in_sel_cfg;
#[doc = "FUNC126_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func126_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func126_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func126_in_sel_cfg`] module"]
pub type FUNC126_IN_SEL_CFG = crate::Reg<func126_in_sel_cfg::FUNC126_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func126_in_sel_cfg;
#[doc = "FUNC127_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func127_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func127_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func127_in_sel_cfg`] module"]
pub type FUNC127_IN_SEL_CFG = crate::Reg<func127_in_sel_cfg::FUNC127_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func127_in_sel_cfg;
#[doc = "FUNC128_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func128_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func128_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func128_in_sel_cfg`] module"]
pub type FUNC128_IN_SEL_CFG = crate::Reg<func128_in_sel_cfg::FUNC128_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func128_in_sel_cfg;
#[doc = "FUNC129_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func129_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func129_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func129_in_sel_cfg`] module"]
pub type FUNC129_IN_SEL_CFG = crate::Reg<func129_in_sel_cfg::FUNC129_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func129_in_sel_cfg;
#[doc = "FUNC130_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func130_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func130_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func130_in_sel_cfg`] module"]
pub type FUNC130_IN_SEL_CFG = crate::Reg<func130_in_sel_cfg::FUNC130_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func130_in_sel_cfg;
#[doc = "FUNC131_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func131_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func131_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func131_in_sel_cfg`] module"]
pub type FUNC131_IN_SEL_CFG = crate::Reg<func131_in_sel_cfg::FUNC131_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func131_in_sel_cfg;
#[doc = "FUNC132_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func132_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func132_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func132_in_sel_cfg`] module"]
pub type FUNC132_IN_SEL_CFG = crate::Reg<func132_in_sel_cfg::FUNC132_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func132_in_sel_cfg;
#[doc = "FUNC133_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func133_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func133_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func133_in_sel_cfg`] module"]
pub type FUNC133_IN_SEL_CFG = crate::Reg<func133_in_sel_cfg::FUNC133_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func133_in_sel_cfg;
#[doc = "FUNC134_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func134_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func134_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func134_in_sel_cfg`] module"]
pub type FUNC134_IN_SEL_CFG = crate::Reg<func134_in_sel_cfg::FUNC134_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func134_in_sel_cfg;
#[doc = "FUNC135_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func135_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func135_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func135_in_sel_cfg`] module"]
pub type FUNC135_IN_SEL_CFG = crate::Reg<func135_in_sel_cfg::FUNC135_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func135_in_sel_cfg;
#[doc = "FUNC136_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func136_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func136_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func136_in_sel_cfg`] module"]
pub type FUNC136_IN_SEL_CFG = crate::Reg<func136_in_sel_cfg::FUNC136_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func136_in_sel_cfg;
#[doc = "FUNC137_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func137_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func137_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func137_in_sel_cfg`] module"]
pub type FUNC137_IN_SEL_CFG = crate::Reg<func137_in_sel_cfg::FUNC137_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func137_in_sel_cfg;
#[doc = "FUNC138_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func138_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func138_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func138_in_sel_cfg`] module"]
pub type FUNC138_IN_SEL_CFG = crate::Reg<func138_in_sel_cfg::FUNC138_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func138_in_sel_cfg;
#[doc = "FUNC139_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func139_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func139_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func139_in_sel_cfg`] module"]
pub type FUNC139_IN_SEL_CFG = crate::Reg<func139_in_sel_cfg::FUNC139_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func139_in_sel_cfg;
#[doc = "FUNC140_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func140_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func140_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func140_in_sel_cfg`] module"]
pub type FUNC140_IN_SEL_CFG = crate::Reg<func140_in_sel_cfg::FUNC140_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func140_in_sel_cfg;
#[doc = "FUNC141_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func141_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func141_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func141_in_sel_cfg`] module"]
pub type FUNC141_IN_SEL_CFG = crate::Reg<func141_in_sel_cfg::FUNC141_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func141_in_sel_cfg;
#[doc = "FUNC142_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func142_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func142_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func142_in_sel_cfg`] module"]
pub type FUNC142_IN_SEL_CFG = crate::Reg<func142_in_sel_cfg::FUNC142_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func142_in_sel_cfg;
#[doc = "FUNC143_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func143_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func143_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func143_in_sel_cfg`] module"]
pub type FUNC143_IN_SEL_CFG = crate::Reg<func143_in_sel_cfg::FUNC143_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func143_in_sel_cfg;
#[doc = "FUNC144_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func144_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func144_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func144_in_sel_cfg`] module"]
pub type FUNC144_IN_SEL_CFG = crate::Reg<func144_in_sel_cfg::FUNC144_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func144_in_sel_cfg;
#[doc = "FUNC145_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func145_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func145_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func145_in_sel_cfg`] module"]
pub type FUNC145_IN_SEL_CFG = crate::Reg<func145_in_sel_cfg::FUNC145_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func145_in_sel_cfg;
#[doc = "FUNC146_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func146_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func146_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func146_in_sel_cfg`] module"]
pub type FUNC146_IN_SEL_CFG = crate::Reg<func146_in_sel_cfg::FUNC146_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func146_in_sel_cfg;
#[doc = "FUNC147_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func147_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func147_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func147_in_sel_cfg`] module"]
pub type FUNC147_IN_SEL_CFG = crate::Reg<func147_in_sel_cfg::FUNC147_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func147_in_sel_cfg;
#[doc = "FUNC148_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func148_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func148_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func148_in_sel_cfg`] module"]
pub type FUNC148_IN_SEL_CFG = crate::Reg<func148_in_sel_cfg::FUNC148_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func148_in_sel_cfg;
#[doc = "FUNC149_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func149_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func149_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func149_in_sel_cfg`] module"]
pub type FUNC149_IN_SEL_CFG = crate::Reg<func149_in_sel_cfg::FUNC149_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func149_in_sel_cfg;
#[doc = "FUNC150_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func150_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func150_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func150_in_sel_cfg`] module"]
pub type FUNC150_IN_SEL_CFG = crate::Reg<func150_in_sel_cfg::FUNC150_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func150_in_sel_cfg;
#[doc = "FUNC151_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func151_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func151_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func151_in_sel_cfg`] module"]
pub type FUNC151_IN_SEL_CFG = crate::Reg<func151_in_sel_cfg::FUNC151_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func151_in_sel_cfg;
#[doc = "FUNC152_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func152_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func152_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func152_in_sel_cfg`] module"]
pub type FUNC152_IN_SEL_CFG = crate::Reg<func152_in_sel_cfg::FUNC152_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func152_in_sel_cfg;
#[doc = "FUNC153_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func153_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func153_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func153_in_sel_cfg`] module"]
pub type FUNC153_IN_SEL_CFG = crate::Reg<func153_in_sel_cfg::FUNC153_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func153_in_sel_cfg;
#[doc = "FUNC154_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func154_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func154_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func154_in_sel_cfg`] module"]
pub type FUNC154_IN_SEL_CFG = crate::Reg<func154_in_sel_cfg::FUNC154_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func154_in_sel_cfg;
#[doc = "FUNC155_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func155_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func155_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func155_in_sel_cfg`] module"]
pub type FUNC155_IN_SEL_CFG = crate::Reg<func155_in_sel_cfg::FUNC155_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func155_in_sel_cfg;
#[doc = "FUNC156_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func156_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func156_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func156_in_sel_cfg`] module"]
pub type FUNC156_IN_SEL_CFG = crate::Reg<func156_in_sel_cfg::FUNC156_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func156_in_sel_cfg;
#[doc = "FUNC158_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func158_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func158_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func158_in_sel_cfg`] module"]
pub type FUNC158_IN_SEL_CFG = crate::Reg<func158_in_sel_cfg::FUNC158_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func158_in_sel_cfg;
#[doc = "FUNC159_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func159_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func159_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func159_in_sel_cfg`] module"]
pub type FUNC159_IN_SEL_CFG = crate::Reg<func159_in_sel_cfg::FUNC159_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func159_in_sel_cfg;
#[doc = "FUNC160_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func160_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func160_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func160_in_sel_cfg`] module"]
pub type FUNC160_IN_SEL_CFG = crate::Reg<func160_in_sel_cfg::FUNC160_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func160_in_sel_cfg;
#[doc = "FUNC161_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func161_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func161_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func161_in_sel_cfg`] module"]
pub type FUNC161_IN_SEL_CFG = crate::Reg<func161_in_sel_cfg::FUNC161_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func161_in_sel_cfg;
#[doc = "FUNC162_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func162_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func162_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func162_in_sel_cfg`] module"]
pub type FUNC162_IN_SEL_CFG = crate::Reg<func162_in_sel_cfg::FUNC162_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func162_in_sel_cfg;
#[doc = "FUNC163_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func163_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func163_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func163_in_sel_cfg`] module"]
pub type FUNC163_IN_SEL_CFG = crate::Reg<func163_in_sel_cfg::FUNC163_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func163_in_sel_cfg;
#[doc = "FUNC164_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func164_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func164_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func164_in_sel_cfg`] module"]
pub type FUNC164_IN_SEL_CFG = crate::Reg<func164_in_sel_cfg::FUNC164_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func164_in_sel_cfg;
#[doc = "FUNC165_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func165_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func165_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func165_in_sel_cfg`] module"]
pub type FUNC165_IN_SEL_CFG = crate::Reg<func165_in_sel_cfg::FUNC165_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func165_in_sel_cfg;
#[doc = "FUNC166_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func166_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func166_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func166_in_sel_cfg`] module"]
pub type FUNC166_IN_SEL_CFG = crate::Reg<func166_in_sel_cfg::FUNC166_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func166_in_sel_cfg;
#[doc = "FUNC167_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func167_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func167_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func167_in_sel_cfg`] module"]
pub type FUNC167_IN_SEL_CFG = crate::Reg<func167_in_sel_cfg::FUNC167_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func167_in_sel_cfg;
#[doc = "FUNC168_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func168_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func168_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func168_in_sel_cfg`] module"]
pub type FUNC168_IN_SEL_CFG = crate::Reg<func168_in_sel_cfg::FUNC168_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func168_in_sel_cfg;
#[doc = "FUNC169_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func169_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func169_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func169_in_sel_cfg`] module"]
pub type FUNC169_IN_SEL_CFG = crate::Reg<func169_in_sel_cfg::FUNC169_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func169_in_sel_cfg;
#[doc = "FUNC170_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func170_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func170_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func170_in_sel_cfg`] module"]
pub type FUNC170_IN_SEL_CFG = crate::Reg<func170_in_sel_cfg::FUNC170_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func170_in_sel_cfg;
#[doc = "FUNC171_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func171_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func171_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func171_in_sel_cfg`] module"]
pub type FUNC171_IN_SEL_CFG = crate::Reg<func171_in_sel_cfg::FUNC171_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func171_in_sel_cfg;
#[doc = "FUNC172_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func172_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func172_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func172_in_sel_cfg`] module"]
pub type FUNC172_IN_SEL_CFG = crate::Reg<func172_in_sel_cfg::FUNC172_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func172_in_sel_cfg;
#[doc = "FUNC173_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func173_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func173_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func173_in_sel_cfg`] module"]
pub type FUNC173_IN_SEL_CFG = crate::Reg<func173_in_sel_cfg::FUNC173_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func173_in_sel_cfg;
#[doc = "FUNC174_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func174_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func174_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func174_in_sel_cfg`] module"]
pub type FUNC174_IN_SEL_CFG = crate::Reg<func174_in_sel_cfg::FUNC174_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func174_in_sel_cfg;
#[doc = "FUNC175_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func175_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func175_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func175_in_sel_cfg`] module"]
pub type FUNC175_IN_SEL_CFG = crate::Reg<func175_in_sel_cfg::FUNC175_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func175_in_sel_cfg;
#[doc = "FUNC176_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func176_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func176_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func176_in_sel_cfg`] module"]
pub type FUNC176_IN_SEL_CFG = crate::Reg<func176_in_sel_cfg::FUNC176_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func176_in_sel_cfg;
#[doc = "FUNC177_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func177_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func177_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func177_in_sel_cfg`] module"]
pub type FUNC177_IN_SEL_CFG = crate::Reg<func177_in_sel_cfg::FUNC177_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func177_in_sel_cfg;
#[doc = "FUNC178_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func178_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func178_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func178_in_sel_cfg`] module"]
pub type FUNC178_IN_SEL_CFG = crate::Reg<func178_in_sel_cfg::FUNC178_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func178_in_sel_cfg;
#[doc = "FUNC179_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func179_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func179_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func179_in_sel_cfg`] module"]
pub type FUNC179_IN_SEL_CFG = crate::Reg<func179_in_sel_cfg::FUNC179_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func179_in_sel_cfg;
#[doc = "FUNC180_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func180_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func180_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func180_in_sel_cfg`] module"]
pub type FUNC180_IN_SEL_CFG = crate::Reg<func180_in_sel_cfg::FUNC180_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func180_in_sel_cfg;
#[doc = "FUNC181_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func181_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func181_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func181_in_sel_cfg`] module"]
pub type FUNC181_IN_SEL_CFG = crate::Reg<func181_in_sel_cfg::FUNC181_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func181_in_sel_cfg;
#[doc = "FUNC182_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func182_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func182_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func182_in_sel_cfg`] module"]
pub type FUNC182_IN_SEL_CFG = crate::Reg<func182_in_sel_cfg::FUNC182_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func182_in_sel_cfg;
#[doc = "FUNC183_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func183_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func183_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func183_in_sel_cfg`] module"]
pub type FUNC183_IN_SEL_CFG = crate::Reg<func183_in_sel_cfg::FUNC183_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func183_in_sel_cfg;
#[doc = "FUNC184_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func184_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func184_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func184_in_sel_cfg`] module"]
pub type FUNC184_IN_SEL_CFG = crate::Reg<func184_in_sel_cfg::FUNC184_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func184_in_sel_cfg;
#[doc = "FUNC185_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func185_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func185_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func185_in_sel_cfg`] module"]
pub type FUNC185_IN_SEL_CFG = crate::Reg<func185_in_sel_cfg::FUNC185_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func185_in_sel_cfg;
#[doc = "FUNC186_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func186_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func186_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func186_in_sel_cfg`] module"]
pub type FUNC186_IN_SEL_CFG = crate::Reg<func186_in_sel_cfg::FUNC186_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func186_in_sel_cfg;
#[doc = "FUNC187_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func187_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func187_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func187_in_sel_cfg`] module"]
pub type FUNC187_IN_SEL_CFG = crate::Reg<func187_in_sel_cfg::FUNC187_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func187_in_sel_cfg;
#[doc = "FUNC188_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func188_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func188_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func188_in_sel_cfg`] module"]
pub type FUNC188_IN_SEL_CFG = crate::Reg<func188_in_sel_cfg::FUNC188_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func188_in_sel_cfg;
#[doc = "FUNC189_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func189_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func189_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func189_in_sel_cfg`] module"]
pub type FUNC189_IN_SEL_CFG = crate::Reg<func189_in_sel_cfg::FUNC189_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func189_in_sel_cfg;
#[doc = "FUNC190_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func190_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func190_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func190_in_sel_cfg`] module"]
pub type FUNC190_IN_SEL_CFG = crate::Reg<func190_in_sel_cfg::FUNC190_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func190_in_sel_cfg;
#[doc = "FUNC191_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func191_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func191_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func191_in_sel_cfg`] module"]
pub type FUNC191_IN_SEL_CFG = crate::Reg<func191_in_sel_cfg::FUNC191_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func191_in_sel_cfg;
#[doc = "FUNC192_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func192_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func192_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func192_in_sel_cfg`] module"]
pub type FUNC192_IN_SEL_CFG = crate::Reg<func192_in_sel_cfg::FUNC192_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func192_in_sel_cfg;
#[doc = "FUNC193_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func193_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func193_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func193_in_sel_cfg`] module"]
pub type FUNC193_IN_SEL_CFG = crate::Reg<func193_in_sel_cfg::FUNC193_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func193_in_sel_cfg;
#[doc = "FUNC194_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func194_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func194_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func194_in_sel_cfg`] module"]
pub type FUNC194_IN_SEL_CFG = crate::Reg<func194_in_sel_cfg::FUNC194_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func194_in_sel_cfg;
#[doc = "FUNC195_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func195_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func195_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func195_in_sel_cfg`] module"]
pub type FUNC195_IN_SEL_CFG = crate::Reg<func195_in_sel_cfg::FUNC195_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func195_in_sel_cfg;
#[doc = "FUNC196_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func196_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func196_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func196_in_sel_cfg`] module"]
pub type FUNC196_IN_SEL_CFG = crate::Reg<func196_in_sel_cfg::FUNC196_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func196_in_sel_cfg;
#[doc = "FUNC197_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func197_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func197_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func197_in_sel_cfg`] module"]
pub type FUNC197_IN_SEL_CFG = crate::Reg<func197_in_sel_cfg::FUNC197_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func197_in_sel_cfg;
#[doc = "FUNC198_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func198_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func198_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func198_in_sel_cfg`] module"]
pub type FUNC198_IN_SEL_CFG = crate::Reg<func198_in_sel_cfg::FUNC198_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func198_in_sel_cfg;
#[doc = "FUNC199_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func199_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func199_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func199_in_sel_cfg`] module"]
pub type FUNC199_IN_SEL_CFG = crate::Reg<func199_in_sel_cfg::FUNC199_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func199_in_sel_cfg;
#[doc = "FUNC200_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func200_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func200_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func200_in_sel_cfg`] module"]
pub type FUNC200_IN_SEL_CFG = crate::Reg<func200_in_sel_cfg::FUNC200_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func200_in_sel_cfg;
#[doc = "FUNC201_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func201_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func201_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func201_in_sel_cfg`] module"]
pub type FUNC201_IN_SEL_CFG = crate::Reg<func201_in_sel_cfg::FUNC201_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func201_in_sel_cfg;
#[doc = "FUNC202_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func202_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func202_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func202_in_sel_cfg`] module"]
pub type FUNC202_IN_SEL_CFG = crate::Reg<func202_in_sel_cfg::FUNC202_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func202_in_sel_cfg;
#[doc = "FUNC203_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func203_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func203_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func203_in_sel_cfg`] module"]
pub type FUNC203_IN_SEL_CFG = crate::Reg<func203_in_sel_cfg::FUNC203_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func203_in_sel_cfg;
#[doc = "FUNC214_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func214_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func214_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func214_in_sel_cfg`] module"]
pub type FUNC214_IN_SEL_CFG = crate::Reg<func214_in_sel_cfg::FUNC214_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func214_in_sel_cfg;
#[doc = "FUNC215_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func215_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func215_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func215_in_sel_cfg`] module"]
pub type FUNC215_IN_SEL_CFG = crate::Reg<func215_in_sel_cfg::FUNC215_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func215_in_sel_cfg;
#[doc = "FUNC216_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func216_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func216_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func216_in_sel_cfg`] module"]
pub type FUNC216_IN_SEL_CFG = crate::Reg<func216_in_sel_cfg::FUNC216_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func216_in_sel_cfg;
#[doc = "FUNC217_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func217_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func217_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func217_in_sel_cfg`] module"]
pub type FUNC217_IN_SEL_CFG = crate::Reg<func217_in_sel_cfg::FUNC217_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func217_in_sel_cfg;
#[doc = "FUNC218_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func218_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func218_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func218_in_sel_cfg`] module"]
pub type FUNC218_IN_SEL_CFG = crate::Reg<func218_in_sel_cfg::FUNC218_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func218_in_sel_cfg;
#[doc = "FUNC219_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func219_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func219_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func219_in_sel_cfg`] module"]
pub type FUNC219_IN_SEL_CFG = crate::Reg<func219_in_sel_cfg::FUNC219_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func219_in_sel_cfg;
#[doc = "FUNC220_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func220_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func220_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func220_in_sel_cfg`] module"]
pub type FUNC220_IN_SEL_CFG = crate::Reg<func220_in_sel_cfg::FUNC220_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func220_in_sel_cfg;
#[doc = "FUNC221_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func221_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func221_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func221_in_sel_cfg`] module"]
pub type FUNC221_IN_SEL_CFG = crate::Reg<func221_in_sel_cfg::FUNC221_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func221_in_sel_cfg;
#[doc = "FUNC222_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func222_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func222_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func222_in_sel_cfg`] module"]
pub type FUNC222_IN_SEL_CFG = crate::Reg<func222_in_sel_cfg::FUNC222_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func222_in_sel_cfg;
#[doc = "FUNC223_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func223_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func223_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func223_in_sel_cfg`] module"]
pub type FUNC223_IN_SEL_CFG = crate::Reg<func223_in_sel_cfg::FUNC223_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func223_in_sel_cfg;
#[doc = "FUNC224_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func224_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func224_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func224_in_sel_cfg`] module"]
pub type FUNC224_IN_SEL_CFG = crate::Reg<func224_in_sel_cfg::FUNC224_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func224_in_sel_cfg;
#[doc = "FUNC225_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func225_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func225_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func225_in_sel_cfg`] module"]
pub type FUNC225_IN_SEL_CFG = crate::Reg<func225_in_sel_cfg::FUNC225_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func225_in_sel_cfg;
#[doc = "FUNC226_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func226_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func226_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func226_in_sel_cfg`] module"]
pub type FUNC226_IN_SEL_CFG = crate::Reg<func226_in_sel_cfg::FUNC226_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func226_in_sel_cfg;
#[doc = "FUNC227_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func227_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func227_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func227_in_sel_cfg`] module"]
pub type FUNC227_IN_SEL_CFG = crate::Reg<func227_in_sel_cfg::FUNC227_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func227_in_sel_cfg;
#[doc = "FUNC228_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func228_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func228_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func228_in_sel_cfg`] module"]
pub type FUNC228_IN_SEL_CFG = crate::Reg<func228_in_sel_cfg::FUNC228_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func228_in_sel_cfg;
#[doc = "FUNC229_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func229_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func229_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func229_in_sel_cfg`] module"]
pub type FUNC229_IN_SEL_CFG = crate::Reg<func229_in_sel_cfg::FUNC229_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func229_in_sel_cfg;
#[doc = "FUNC230_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func230_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func230_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func230_in_sel_cfg`] module"]
pub type FUNC230_IN_SEL_CFG = crate::Reg<func230_in_sel_cfg::FUNC230_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func230_in_sel_cfg;
#[doc = "FUNC231_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func231_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func231_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func231_in_sel_cfg`] module"]
pub type FUNC231_IN_SEL_CFG = crate::Reg<func231_in_sel_cfg::FUNC231_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func231_in_sel_cfg;
#[doc = "FUNC232_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func232_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func232_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func232_in_sel_cfg`] module"]
pub type FUNC232_IN_SEL_CFG = crate::Reg<func232_in_sel_cfg::FUNC232_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func232_in_sel_cfg;
#[doc = "FUNC233_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func233_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func233_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func233_in_sel_cfg`] module"]
pub type FUNC233_IN_SEL_CFG = crate::Reg<func233_in_sel_cfg::FUNC233_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func233_in_sel_cfg;
#[doc = "FUNC234_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func234_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func234_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func234_in_sel_cfg`] module"]
pub type FUNC234_IN_SEL_CFG = crate::Reg<func234_in_sel_cfg::FUNC234_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func234_in_sel_cfg;
#[doc = "FUNC235_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func235_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func235_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func235_in_sel_cfg`] module"]
pub type FUNC235_IN_SEL_CFG = crate::Reg<func235_in_sel_cfg::FUNC235_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func235_in_sel_cfg;
#[doc = "FUNC236_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func236_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func236_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func236_in_sel_cfg`] module"]
pub type FUNC236_IN_SEL_CFG = crate::Reg<func236_in_sel_cfg::FUNC236_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func236_in_sel_cfg;
#[doc = "FUNC237_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func237_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func237_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func237_in_sel_cfg`] module"]
pub type FUNC237_IN_SEL_CFG = crate::Reg<func237_in_sel_cfg::FUNC237_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func237_in_sel_cfg;
#[doc = "FUNC238_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func238_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func238_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func238_in_sel_cfg`] module"]
pub type FUNC238_IN_SEL_CFG = crate::Reg<func238_in_sel_cfg::FUNC238_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func238_in_sel_cfg;
#[doc = "FUNC239_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func239_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func239_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func239_in_sel_cfg`] module"]
pub type FUNC239_IN_SEL_CFG = crate::Reg<func239_in_sel_cfg::FUNC239_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func239_in_sel_cfg;
#[doc = "FUNC240_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func240_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func240_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func240_in_sel_cfg`] module"]
pub type FUNC240_IN_SEL_CFG = crate::Reg<func240_in_sel_cfg::FUNC240_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func240_in_sel_cfg;
#[doc = "FUNC241_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func241_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func241_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func241_in_sel_cfg`] module"]
pub type FUNC241_IN_SEL_CFG = crate::Reg<func241_in_sel_cfg::FUNC241_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func241_in_sel_cfg;
#[doc = "FUNC242_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func242_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func242_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func242_in_sel_cfg`] module"]
pub type FUNC242_IN_SEL_CFG = crate::Reg<func242_in_sel_cfg::FUNC242_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func242_in_sel_cfg;
#[doc = "FUNC243_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func243_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func243_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func243_in_sel_cfg`] module"]
pub type FUNC243_IN_SEL_CFG = crate::Reg<func243_in_sel_cfg::FUNC243_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func243_in_sel_cfg;
#[doc = "FUNC244_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func244_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func244_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func244_in_sel_cfg`] module"]
pub type FUNC244_IN_SEL_CFG = crate::Reg<func244_in_sel_cfg::FUNC244_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func244_in_sel_cfg;
#[doc = "FUNC245_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func245_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func245_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func245_in_sel_cfg`] module"]
pub type FUNC245_IN_SEL_CFG = crate::Reg<func245_in_sel_cfg::FUNC245_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func245_in_sel_cfg;
#[doc = "FUNC246_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func246_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func246_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func246_in_sel_cfg`] module"]
pub type FUNC246_IN_SEL_CFG = crate::Reg<func246_in_sel_cfg::FUNC246_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func246_in_sel_cfg;
#[doc = "FUNC247_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func247_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func247_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func247_in_sel_cfg`] module"]
pub type FUNC247_IN_SEL_CFG = crate::Reg<func247_in_sel_cfg::FUNC247_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func247_in_sel_cfg;
#[doc = "FUNC248_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func248_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func248_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func248_in_sel_cfg`] module"]
pub type FUNC248_IN_SEL_CFG = crate::Reg<func248_in_sel_cfg::FUNC248_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func248_in_sel_cfg;
#[doc = "FUNC249_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func249_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func249_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func249_in_sel_cfg`] module"]
pub type FUNC249_IN_SEL_CFG = crate::Reg<func249_in_sel_cfg::FUNC249_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func249_in_sel_cfg;
#[doc = "FUNC250_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func250_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func250_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func250_in_sel_cfg`] module"]
pub type FUNC250_IN_SEL_CFG = crate::Reg<func250_in_sel_cfg::FUNC250_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func250_in_sel_cfg;
#[doc = "FUNC251_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func251_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func251_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func251_in_sel_cfg`] module"]
pub type FUNC251_IN_SEL_CFG = crate::Reg<func251_in_sel_cfg::FUNC251_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func251_in_sel_cfg;
#[doc = "FUNC252_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func252_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func252_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func252_in_sel_cfg`] module"]
pub type FUNC252_IN_SEL_CFG = crate::Reg<func252_in_sel_cfg::FUNC252_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func252_in_sel_cfg;
#[doc = "FUNC253_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func253_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func253_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func253_in_sel_cfg`] module"]
pub type FUNC253_IN_SEL_CFG = crate::Reg<func253_in_sel_cfg::FUNC253_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func253_in_sel_cfg;
#[doc = "FUNC254_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func254_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func254_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func254_in_sel_cfg`] module"]
pub type FUNC254_IN_SEL_CFG = crate::Reg<func254_in_sel_cfg::FUNC254_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func254_in_sel_cfg;
#[doc = "FUNC255_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func255_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func255_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func255_in_sel_cfg`] module"]
pub type FUNC255_IN_SEL_CFG = crate::Reg<func255_in_sel_cfg::FUNC255_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func255_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: GPIO output function select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func_out_sel_cfg;
#[doc = "INTR_2 (r) register accessor: GPIO interrupt 2 status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_2`] module"]
pub type INTR_2 = crate::Reg<intr_2::INTR_2_SPEC>;
#[doc = "GPIO interrupt 2 status register for GPIO0-31"]
pub mod intr_2;
#[doc = "INTR1_2 (r) register accessor: GPIO interrupt 2 status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr1_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr1_2`] module"]
pub type INTR1_2 = crate::Reg<intr1_2::INTR1_2_SPEC>;
#[doc = "GPIO interrupt 2 status register for GPIO32-56"]
pub mod intr1_2;
#[doc = "INTR_3 (r) register accessor: GPIO interrupt 3 status register for GPIO0-31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_3`] module"]
pub type INTR_3 = crate::Reg<intr_3::INTR_3_SPEC>;
#[doc = "GPIO interrupt 3 status register for GPIO0-31"]
pub mod intr_3;
#[doc = "INTR1_3 (r) register accessor: GPIO interrupt 3 status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr1_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr1_3`] module"]
pub type INTR1_3 = crate::Reg<intr1_3::INTR1_3_SPEC>;
#[doc = "GPIO interrupt 3 status register for GPIO32-56"]
pub mod intr1_3;
#[doc = "CLOCK_GATE (rw) register accessor: GPIO clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "INT_RAW (rw) register accessor: analog comparator interrupt raw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "analog comparator interrupt raw"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: analog comparator interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "analog comparator interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: analog comparator interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "analog comparator interrupt enable"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: analog comparator interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "analog comparator interrupt clear"]
pub mod int_clr;
#[doc = "ZERO_DET0_FILTER_CNT (rw) register accessor: GPIO analog comparator zero detect filter count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_det0_filter_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zero_det0_filter_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det0_filter_cnt`] module"]
pub type ZERO_DET0_FILTER_CNT = crate::Reg<zero_det0_filter_cnt::ZERO_DET0_FILTER_CNT_SPEC>;
#[doc = "GPIO analog comparator zero detect filter count"]
pub mod zero_det0_filter_cnt;
#[doc = "ZERO_DET1_FILTER_CNT (rw) register accessor: GPIO analog comparator zero detect filter count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_det1_filter_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zero_det1_filter_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det1_filter_cnt`] module"]
pub type ZERO_DET1_FILTER_CNT = crate::Reg<zero_det1_filter_cnt::ZERO_DET1_FILTER_CNT_SPEC>;
#[doc = "GPIO analog comparator zero detect filter count"]
pub mod zero_det1_filter_cnt;
#[doc = "SEND_SEQ (rw) register accessor: High speed sdio pad bist send sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`send_seq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send_seq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@send_seq`] module"]
pub type SEND_SEQ = crate::Reg<send_seq::SEND_SEQ_SPEC>;
#[doc = "High speed sdio pad bist send sequence"]
pub mod send_seq;
#[doc = "RECIVE_SEQ (r) register accessor: High speed sdio pad bist recive sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`recive_seq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recive_seq`] module"]
pub type RECIVE_SEQ = crate::Reg<recive_seq::RECIVE_SEQ_SPEC>;
#[doc = "High speed sdio pad bist recive sequence"]
pub mod recive_seq;
#[doc = "BISTIN_SEL (rw) register accessor: High speed sdio pad bist in pad sel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bistin_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bistin_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bistin_sel`] module"]
pub type BISTIN_SEL = crate::Reg<bistin_sel::BISTIN_SEL_SPEC>;
#[doc = "High speed sdio pad bist in pad sel"]
pub mod bistin_sel;
#[doc = "BIST_CTRL (rw) register accessor: High speed sdio pad bist control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bist_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bist_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bist_ctrl`] module"]
pub type BIST_CTRL = crate::Reg<bist_ctrl::BIST_CTRL_SPEC>;
#[doc = "High speed sdio pad bist control"]
pub mod bist_ctrl;
#[doc = "DATE (rw) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod date;
