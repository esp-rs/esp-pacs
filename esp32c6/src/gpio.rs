#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO bit select register"]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - GPIO output register for GPIO0-31"]
    pub out: OUT,
    #[doc = "0x08 - GPIO output set register for GPIO0-31"]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - GPIO output clear register for GPIO0-31"]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x10 - GPIO output register for GPIO32-34"]
    pub out1: OUT1,
    #[doc = "0x14 - GPIO output set register for GPIO32-34"]
    pub out1_w1ts: OUT1_W1TS,
    #[doc = "0x18 - GPIO output clear register for GPIO32-34"]
    pub out1_w1tc: OUT1_W1TC,
    #[doc = "0x1c - GPIO sdio select register"]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - GPIO output enable register for GPIO0-31"]
    pub enable: ENABLE,
    #[doc = "0x24 - GPIO output enable set register for GPIO0-31"]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - GPIO output enable clear register for GPIO0-31"]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x2c - GPIO output enable register for GPIO32-34"]
    pub enable1: ENABLE1,
    #[doc = "0x30 - GPIO output enable set register for GPIO32-34"]
    pub enable1_w1ts: ENABLE1_W1TS,
    #[doc = "0x34 - GPIO output enable clear register for GPIO32-34"]
    pub enable1_w1tc: ENABLE1_W1TC,
    #[doc = "0x38 - pad strapping register"]
    pub strap: STRAP,
    #[doc = "0x3c - GPIO input register for GPIO0-31"]
    pub in_: IN,
    #[doc = "0x40 - GPIO input register for GPIO32-34"]
    pub in1: IN1,
    #[doc = "0x44 - GPIO interrupt status register for GPIO0-31"]
    pub status: STATUS,
    #[doc = "0x48 - GPIO interrupt status set register for GPIO0-31"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - GPIO interrupt status clear register for GPIO0-31"]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x50 - GPIO interrupt status register for GPIO32-34"]
    pub status1: STATUS1,
    #[doc = "0x54 - GPIO interrupt status set register for GPIO32-34"]
    pub status1_w1ts: STATUS1_W1TS,
    #[doc = "0x58 - GPIO interrupt status clear register for GPIO32-34"]
    pub status1_w1tc: STATUS1_W1TC,
    #[doc = "0x5c - GPIO PRO_CPU interrupt status register for GPIO0-31"]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x60 - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x64 - GPIO CPUSDIO interrupt status register for GPIO0-31"]
    pub cpusdio_int: CPUSDIO_INT,
    #[doc = "0x68 - GPIO PRO_CPU interrupt status register for GPIO32-34"]
    pub pcpu_int1: PCPU_INT1,
    #[doc = "0x6c - GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-34"]
    pub pcpu_nmi_int1: PCPU_NMI_INT1,
    #[doc = "0x70 - GPIO CPUSDIO interrupt status register for GPIO32-34"]
    pub cpusdio_int1: CPUSDIO_INT1,
    #[doc = "0x74..0x100 - GPIO pin configuration register"]
    pub pin: [PIN; 35],
    _reserved30: [u8; 0x4c],
    #[doc = "0x14c - GPIO interrupt source register for GPIO0-31"]
    pub status_next: STATUS_NEXT,
    #[doc = "0x150 - GPIO interrupt source register for GPIO32-34"]
    pub status_next1: STATUS_NEXT1,
    #[doc = "0x154 - GPIO input function configuration register"]
    pub func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved33: [u8; 0x14],
    #[doc = "0x16c - GPIO input function configuration register"]
    pub func6_in_sel_cfg: FUNC6_IN_SEL_CFG,
    #[doc = "0x170 - GPIO input function configuration register"]
    pub func7_in_sel_cfg: FUNC7_IN_SEL_CFG,
    #[doc = "0x174 - GPIO input function configuration register"]
    pub func8_in_sel_cfg: FUNC8_IN_SEL_CFG,
    #[doc = "0x178 - GPIO input function configuration register"]
    pub func9_in_sel_cfg: FUNC9_IN_SEL_CFG,
    #[doc = "0x17c - GPIO input function configuration register"]
    pub func10_in_sel_cfg: FUNC10_IN_SEL_CFG,
    #[doc = "0x180 - GPIO input function configuration register"]
    pub func11_in_sel_cfg: FUNC11_IN_SEL_CFG,
    #[doc = "0x184 - GPIO input function configuration register"]
    pub func12_in_sel_cfg: FUNC12_IN_SEL_CFG,
    #[doc = "0x188 - GPIO input function configuration register"]
    pub func13_in_sel_cfg: FUNC13_IN_SEL_CFG,
    #[doc = "0x18c - GPIO input function configuration register"]
    pub func14_in_sel_cfg: FUNC14_IN_SEL_CFG,
    #[doc = "0x190 - GPIO input function configuration register"]
    pub func15_in_sel_cfg: FUNC15_IN_SEL_CFG,
    #[doc = "0x194 - GPIO input function configuration register"]
    pub func16_in_sel_cfg: FUNC16_IN_SEL_CFG,
    #[doc = "0x198 - GPIO input function configuration register"]
    pub func17_in_sel_cfg: FUNC17_IN_SEL_CFG,
    _reserved45: [u8; 0x04],
    #[doc = "0x1a0 - GPIO input function configuration register"]
    pub func19_in_sel_cfg: FUNC19_IN_SEL_CFG,
    _reserved46: [u8; 0x20],
    #[doc = "0x1c4 - GPIO input function configuration register"]
    pub func28_in_sel_cfg: FUNC28_IN_SEL_CFG,
    #[doc = "0x1c8 - GPIO input function configuration register"]
    pub func29_in_sel_cfg: FUNC29_IN_SEL_CFG,
    #[doc = "0x1cc - GPIO input function configuration register"]
    pub func30_in_sel_cfg: FUNC30_IN_SEL_CFG,
    #[doc = "0x1d0 - GPIO input function configuration register"]
    pub func31_in_sel_cfg: FUNC31_IN_SEL_CFG,
    #[doc = "0x1d4 - GPIO input function configuration register"]
    pub func32_in_sel_cfg: FUNC32_IN_SEL_CFG,
    #[doc = "0x1d8 - GPIO input function configuration register"]
    pub func33_in_sel_cfg: FUNC33_IN_SEL_CFG,
    #[doc = "0x1dc - GPIO input function configuration register"]
    pub func34_in_sel_cfg: FUNC34_IN_SEL_CFG,
    #[doc = "0x1e0 - GPIO input function configuration register"]
    pub func35_in_sel_cfg: FUNC35_IN_SEL_CFG,
    _reserved54: [u8; 0x10],
    #[doc = "0x1f4 - GPIO input function configuration register"]
    pub func40_in_sel_cfg: FUNC40_IN_SEL_CFG,
    #[doc = "0x1f8 - GPIO input function configuration register"]
    pub func41_in_sel_cfg: FUNC41_IN_SEL_CFG,
    #[doc = "0x1fc - GPIO input function configuration register"]
    pub func42_in_sel_cfg: FUNC42_IN_SEL_CFG,
    _reserved57: [u8; 0x08],
    #[doc = "0x208 - GPIO input function configuration register"]
    pub func45_in_sel_cfg: FUNC45_IN_SEL_CFG,
    #[doc = "0x20c - GPIO input function configuration register"]
    pub func46_in_sel_cfg: FUNC46_IN_SEL_CFG,
    #[doc = "0x210 - GPIO input function configuration register"]
    pub func47_in_sel_cfg: FUNC47_IN_SEL_CFG,
    #[doc = "0x214 - GPIO input function configuration register"]
    pub func48_in_sel_cfg: FUNC48_IN_SEL_CFG,
    #[doc = "0x218 - GPIO input function configuration register"]
    pub func49_in_sel_cfg: FUNC49_IN_SEL_CFG,
    #[doc = "0x21c - GPIO input function configuration register"]
    pub func50_in_sel_cfg: FUNC50_IN_SEL_CFG,
    #[doc = "0x220 - GPIO input function configuration register"]
    pub func51_in_sel_cfg: FUNC51_IN_SEL_CFG,
    #[doc = "0x224 - GPIO input function configuration register"]
    pub func52_in_sel_cfg: FUNC52_IN_SEL_CFG,
    #[doc = "0x228 - GPIO input function configuration register"]
    pub func53_in_sel_cfg: FUNC53_IN_SEL_CFG,
    #[doc = "0x22c - GPIO input function configuration register"]
    pub func54_in_sel_cfg: FUNC54_IN_SEL_CFG,
    #[doc = "0x230 - GPIO input function configuration register"]
    pub func55_in_sel_cfg: FUNC55_IN_SEL_CFG,
    #[doc = "0x234 - GPIO input function configuration register"]
    pub func56_in_sel_cfg: FUNC56_IN_SEL_CFG,
    #[doc = "0x238 - GPIO input function configuration register"]
    pub func57_in_sel_cfg: FUNC57_IN_SEL_CFG,
    #[doc = "0x23c - GPIO input function configuration register"]
    pub func58_in_sel_cfg: FUNC58_IN_SEL_CFG,
    #[doc = "0x240 - GPIO input function configuration register"]
    pub func59_in_sel_cfg: FUNC59_IN_SEL_CFG,
    #[doc = "0x244 - GPIO input function configuration register"]
    pub func60_in_sel_cfg: FUNC60_IN_SEL_CFG,
    #[doc = "0x248 - GPIO input function configuration register"]
    pub func61_in_sel_cfg: FUNC61_IN_SEL_CFG,
    #[doc = "0x24c - GPIO input function configuration register"]
    pub func62_in_sel_cfg: FUNC62_IN_SEL_CFG,
    #[doc = "0x250 - GPIO input function configuration register"]
    pub func63_in_sel_cfg: FUNC63_IN_SEL_CFG,
    #[doc = "0x254 - GPIO input function configuration register"]
    pub func64_in_sel_cfg: FUNC64_IN_SEL_CFG,
    #[doc = "0x258 - GPIO input function configuration register"]
    pub func65_in_sel_cfg: FUNC65_IN_SEL_CFG,
    #[doc = "0x25c - GPIO input function configuration register"]
    pub func66_in_sel_cfg: FUNC66_IN_SEL_CFG,
    #[doc = "0x260 - GPIO input function configuration register"]
    pub func67_in_sel_cfg: FUNC67_IN_SEL_CFG,
    #[doc = "0x264 - GPIO input function configuration register"]
    pub func68_in_sel_cfg: FUNC68_IN_SEL_CFG,
    #[doc = "0x268 - GPIO input function configuration register"]
    pub func69_in_sel_cfg: FUNC69_IN_SEL_CFG,
    #[doc = "0x26c - GPIO input function configuration register"]
    pub func70_in_sel_cfg: FUNC70_IN_SEL_CFG,
    #[doc = "0x270 - GPIO input function configuration register"]
    pub func71_in_sel_cfg: FUNC71_IN_SEL_CFG,
    #[doc = "0x274 - GPIO input function configuration register"]
    pub func72_in_sel_cfg: FUNC72_IN_SEL_CFG,
    #[doc = "0x278 - GPIO input function configuration register"]
    pub func73_in_sel_cfg: FUNC73_IN_SEL_CFG,
    _reserved86: [u8; 0x0c],
    #[doc = "0x288 - GPIO input function configuration register"]
    pub func77_in_sel_cfg: FUNC77_IN_SEL_CFG,
    _reserved87: [u8; 0x0c],
    #[doc = "0x298 - GPIO input function configuration register"]
    pub func81_in_sel_cfg: FUNC81_IN_SEL_CFG,
    #[doc = "0x29c - GPIO input function configuration register"]
    pub func82_in_sel_cfg: FUNC82_IN_SEL_CFG,
    _reserved89: [u8; 0x10],
    #[doc = "0x2b0 - GPIO input function configuration register"]
    pub func87_in_sel_cfg: FUNC87_IN_SEL_CFG,
    #[doc = "0x2b4 - GPIO input function configuration register"]
    pub func88_in_sel_cfg: FUNC88_IN_SEL_CFG,
    #[doc = "0x2b8 - GPIO input function configuration register"]
    pub func89_in_sel_cfg: FUNC89_IN_SEL_CFG,
    #[doc = "0x2bc - GPIO input function configuration register"]
    pub func90_in_sel_cfg: FUNC90_IN_SEL_CFG,
    #[doc = "0x2c0 - GPIO input function configuration register"]
    pub func91_in_sel_cfg: FUNC91_IN_SEL_CFG,
    #[doc = "0x2c4 - GPIO input function configuration register"]
    pub func92_in_sel_cfg: FUNC92_IN_SEL_CFG,
    #[doc = "0x2c8 - GPIO input function configuration register"]
    pub func93_in_sel_cfg: FUNC93_IN_SEL_CFG,
    #[doc = "0x2cc - GPIO input function configuration register"]
    pub func94_in_sel_cfg: FUNC94_IN_SEL_CFG,
    #[doc = "0x2d0 - GPIO input function configuration register"]
    pub func95_in_sel_cfg: FUNC95_IN_SEL_CFG,
    _reserved98: [u8; 0x04],
    #[doc = "0x2d8 - GPIO input function configuration register"]
    pub func97_in_sel_cfg: FUNC97_IN_SEL_CFG,
    #[doc = "0x2dc - GPIO input function configuration register"]
    pub func98_in_sel_cfg: FUNC98_IN_SEL_CFG,
    #[doc = "0x2e0 - GPIO input function configuration register"]
    pub func99_in_sel_cfg: FUNC99_IN_SEL_CFG,
    #[doc = "0x2e4 - GPIO input function configuration register"]
    pub func100_in_sel_cfg: FUNC100_IN_SEL_CFG,
    #[doc = "0x2e8 - GPIO input function configuration register"]
    pub func101_in_sel_cfg: FUNC101_IN_SEL_CFG,
    #[doc = "0x2ec - GPIO input function configuration register"]
    pub func102_in_sel_cfg: FUNC102_IN_SEL_CFG,
    #[doc = "0x2f0 - GPIO input function configuration register"]
    pub func103_in_sel_cfg: FUNC103_IN_SEL_CFG,
    #[doc = "0x2f4 - GPIO input function configuration register"]
    pub func104_in_sel_cfg: FUNC104_IN_SEL_CFG,
    #[doc = "0x2f8 - GPIO input function configuration register"]
    pub func105_in_sel_cfg: FUNC105_IN_SEL_CFG,
    #[doc = "0x2fc - GPIO input function configuration register"]
    pub func106_in_sel_cfg: FUNC106_IN_SEL_CFG,
    #[doc = "0x300 - GPIO input function configuration register"]
    pub func107_in_sel_cfg: FUNC107_IN_SEL_CFG,
    #[doc = "0x304 - GPIO input function configuration register"]
    pub func108_in_sel_cfg: FUNC108_IN_SEL_CFG,
    #[doc = "0x308 - GPIO input function configuration register"]
    pub func109_in_sel_cfg: FUNC109_IN_SEL_CFG,
    #[doc = "0x30c - GPIO input function configuration register"]
    pub func110_in_sel_cfg: FUNC110_IN_SEL_CFG,
    #[doc = "0x310 - GPIO input function configuration register"]
    pub func111_in_sel_cfg: FUNC111_IN_SEL_CFG,
    #[doc = "0x314 - GPIO input function configuration register"]
    pub func112_in_sel_cfg: FUNC112_IN_SEL_CFG,
    #[doc = "0x318 - GPIO input function configuration register"]
    pub func113_in_sel_cfg: FUNC113_IN_SEL_CFG,
    #[doc = "0x31c - GPIO input function configuration register"]
    pub func114_in_sel_cfg: FUNC114_IN_SEL_CFG,
    #[doc = "0x320 - GPIO input function configuration register"]
    pub func115_in_sel_cfg: FUNC115_IN_SEL_CFG,
    #[doc = "0x324 - GPIO input function configuration register"]
    pub func116_in_sel_cfg: FUNC116_IN_SEL_CFG,
    #[doc = "0x328 - GPIO input function configuration register"]
    pub func117_in_sel_cfg: FUNC117_IN_SEL_CFG,
    #[doc = "0x32c - GPIO input function configuration register"]
    pub func118_in_sel_cfg: FUNC118_IN_SEL_CFG,
    #[doc = "0x330 - GPIO input function configuration register"]
    pub func119_in_sel_cfg: FUNC119_IN_SEL_CFG,
    #[doc = "0x334 - GPIO input function configuration register"]
    pub func120_in_sel_cfg: FUNC120_IN_SEL_CFG,
    #[doc = "0x338 - GPIO input function configuration register"]
    pub func121_in_sel_cfg: FUNC121_IN_SEL_CFG,
    #[doc = "0x33c - GPIO input function configuration register"]
    pub func122_in_sel_cfg: FUNC122_IN_SEL_CFG,
    #[doc = "0x340 - GPIO input function configuration register"]
    pub func123_in_sel_cfg: FUNC123_IN_SEL_CFG,
    #[doc = "0x344 - GPIO input function configuration register"]
    pub func124_in_sel_cfg: FUNC124_IN_SEL_CFG,
    _reserved126: [u8; 0x020c],
    #[doc = "0x554..0x5e0 - GPIO output function select register"]
    pub func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 35],
    _reserved127: [u8; 0x4c],
    #[doc = "0x62c - GPIO clock gate register"]
    pub clock_gate: CLOCK_GATE,
    _reserved128: [u8; 0xcc],
    #[doc = "0x6fc - GPIO version register"]
    pub date: DATE,
}
#[doc = "BT_SELECT (rw) register accessor: an alias for `Reg<BT_SELECT_SPEC>`"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = "GPIO bit select register"]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register for GPIO0-31"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO0-31"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO0-31"]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: an alias for `Reg<OUT1_SPEC>`"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register for GPIO32-34"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: an alias for `Reg<OUT1_W1TS_SPEC>`"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO32-34"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: an alias for `Reg<OUT1_W1TC_SPEC>`"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO32-34"]
pub mod out1_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: an alias for `Reg<SDIO_SELECT_SPEC>`"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = "GPIO sdio select register"]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register for GPIO0-31"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO0-31"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO0-31"]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: an alias for `Reg<ENABLE1_SPEC>`"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register for GPIO32-34"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: an alias for `Reg<ENABLE1_W1TS_SPEC>`"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO32-34"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: an alias for `Reg<ENABLE1_W1TC_SPEC>`"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO32-34"]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: an alias for `Reg<STRAP_SPEC>`"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register for GPIO0-31"]
pub mod in_;
#[doc = "IN1 (r) register accessor: an alias for `Reg<IN1_SPEC>`"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register for GPIO32-34"]
pub mod in1;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register for GPIO0-31"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO0-31"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO0-31"]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: an alias for `Reg<STATUS1_SPEC>`"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register for GPIO32-34"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: an alias for `Reg<STATUS1_W1TS_SPEC>`"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO32-34"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: an alias for `Reg<STATUS1_W1TC_SPEC>`"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO32-34"]
pub mod status1_w1tc;
#[doc = "PCPU_INT (r) register accessor: an alias for `Reg<PCPU_INT_SPEC>`"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register for GPIO0-31"]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: an alias for `Reg<PCPU_NMI_INT_SPEC>`"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: an alias for `Reg<CPUSDIO_INT_SPEC>`"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register for GPIO0-31"]
pub mod cpusdio_int;
#[doc = "PCPU_INT1 (r) register accessor: an alias for `Reg<PCPU_INT1_SPEC>`"]
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register for GPIO32-34"]
pub mod pcpu_int1;
#[doc = "PCPU_NMI_INT1 (r) register accessor: an alias for `Reg<PCPU_NMI_INT1_SPEC>`"]
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-34"]
pub mod pcpu_nmi_int1;
#[doc = "CPUSDIO_INT1 (r) register accessor: an alias for `Reg<CPUSDIO_INT1_SPEC>`"]
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register for GPIO32-34"]
pub mod cpusdio_int1;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin;
#[doc = "STATUS_NEXT (r) register accessor: an alias for `Reg<STATUS_NEXT_SPEC>`"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register for GPIO0-31"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: an alias for `Reg<STATUS_NEXT1_SPEC>`"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register for GPIO32-34"]
pub mod status_next1;
#[doc = "FUNC0_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC0_IN_SEL_CFG_SPEC>`"]
pub type FUNC0_IN_SEL_CFG = crate::Reg<func0_in_sel_cfg::FUNC0_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func0_in_sel_cfg;
#[doc = "FUNC6_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC6_IN_SEL_CFG_SPEC>`"]
pub type FUNC6_IN_SEL_CFG = crate::Reg<func6_in_sel_cfg::FUNC6_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func6_in_sel_cfg;
#[doc = "FUNC7_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC7_IN_SEL_CFG_SPEC>`"]
pub type FUNC7_IN_SEL_CFG = crate::Reg<func7_in_sel_cfg::FUNC7_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func7_in_sel_cfg;
#[doc = "FUNC8_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC8_IN_SEL_CFG_SPEC>`"]
pub type FUNC8_IN_SEL_CFG = crate::Reg<func8_in_sel_cfg::FUNC8_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func8_in_sel_cfg;
#[doc = "FUNC9_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC9_IN_SEL_CFG_SPEC>`"]
pub type FUNC9_IN_SEL_CFG = crate::Reg<func9_in_sel_cfg::FUNC9_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func9_in_sel_cfg;
#[doc = "FUNC10_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC10_IN_SEL_CFG_SPEC>`"]
pub type FUNC10_IN_SEL_CFG = crate::Reg<func10_in_sel_cfg::FUNC10_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func10_in_sel_cfg;
#[doc = "FUNC11_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC11_IN_SEL_CFG_SPEC>`"]
pub type FUNC11_IN_SEL_CFG = crate::Reg<func11_in_sel_cfg::FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func11_in_sel_cfg;
#[doc = "FUNC12_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC12_IN_SEL_CFG_SPEC>`"]
pub type FUNC12_IN_SEL_CFG = crate::Reg<func12_in_sel_cfg::FUNC12_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func12_in_sel_cfg;
#[doc = "FUNC13_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC13_IN_SEL_CFG_SPEC>`"]
pub type FUNC13_IN_SEL_CFG = crate::Reg<func13_in_sel_cfg::FUNC13_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func13_in_sel_cfg;
#[doc = "FUNC14_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC14_IN_SEL_CFG_SPEC>`"]
pub type FUNC14_IN_SEL_CFG = crate::Reg<func14_in_sel_cfg::FUNC14_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func14_in_sel_cfg;
#[doc = "FUNC15_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC15_IN_SEL_CFG_SPEC>`"]
pub type FUNC15_IN_SEL_CFG = crate::Reg<func15_in_sel_cfg::FUNC15_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func15_in_sel_cfg;
#[doc = "FUNC16_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC16_IN_SEL_CFG_SPEC>`"]
pub type FUNC16_IN_SEL_CFG = crate::Reg<func16_in_sel_cfg::FUNC16_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func16_in_sel_cfg;
#[doc = "FUNC17_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC17_IN_SEL_CFG_SPEC>`"]
pub type FUNC17_IN_SEL_CFG = crate::Reg<func17_in_sel_cfg::FUNC17_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func17_in_sel_cfg;
#[doc = "FUNC19_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC19_IN_SEL_CFG_SPEC>`"]
pub type FUNC19_IN_SEL_CFG = crate::Reg<func19_in_sel_cfg::FUNC19_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func19_in_sel_cfg;
#[doc = "FUNC28_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC28_IN_SEL_CFG_SPEC>`"]
pub type FUNC28_IN_SEL_CFG = crate::Reg<func28_in_sel_cfg::FUNC28_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func28_in_sel_cfg;
#[doc = "FUNC29_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC29_IN_SEL_CFG_SPEC>`"]
pub type FUNC29_IN_SEL_CFG = crate::Reg<func29_in_sel_cfg::FUNC29_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func29_in_sel_cfg;
#[doc = "FUNC30_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC30_IN_SEL_CFG_SPEC>`"]
pub type FUNC30_IN_SEL_CFG = crate::Reg<func30_in_sel_cfg::FUNC30_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func30_in_sel_cfg;
#[doc = "FUNC31_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC31_IN_SEL_CFG_SPEC>`"]
pub type FUNC31_IN_SEL_CFG = crate::Reg<func31_in_sel_cfg::FUNC31_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func31_in_sel_cfg;
#[doc = "FUNC32_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC32_IN_SEL_CFG_SPEC>`"]
pub type FUNC32_IN_SEL_CFG = crate::Reg<func32_in_sel_cfg::FUNC32_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func32_in_sel_cfg;
#[doc = "FUNC33_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC33_IN_SEL_CFG_SPEC>`"]
pub type FUNC33_IN_SEL_CFG = crate::Reg<func33_in_sel_cfg::FUNC33_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func33_in_sel_cfg;
#[doc = "FUNC34_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC34_IN_SEL_CFG_SPEC>`"]
pub type FUNC34_IN_SEL_CFG = crate::Reg<func34_in_sel_cfg::FUNC34_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func34_in_sel_cfg;
#[doc = "FUNC35_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC35_IN_SEL_CFG_SPEC>`"]
pub type FUNC35_IN_SEL_CFG = crate::Reg<func35_in_sel_cfg::FUNC35_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func35_in_sel_cfg;
#[doc = "FUNC40_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC40_IN_SEL_CFG_SPEC>`"]
pub type FUNC40_IN_SEL_CFG = crate::Reg<func40_in_sel_cfg::FUNC40_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func40_in_sel_cfg;
#[doc = "FUNC41_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC41_IN_SEL_CFG_SPEC>`"]
pub type FUNC41_IN_SEL_CFG = crate::Reg<func41_in_sel_cfg::FUNC41_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func41_in_sel_cfg;
#[doc = "FUNC42_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC42_IN_SEL_CFG_SPEC>`"]
pub type FUNC42_IN_SEL_CFG = crate::Reg<func42_in_sel_cfg::FUNC42_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func42_in_sel_cfg;
#[doc = "FUNC45_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC45_IN_SEL_CFG_SPEC>`"]
pub type FUNC45_IN_SEL_CFG = crate::Reg<func45_in_sel_cfg::FUNC45_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func45_in_sel_cfg;
#[doc = "FUNC46_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC46_IN_SEL_CFG_SPEC>`"]
pub type FUNC46_IN_SEL_CFG = crate::Reg<func46_in_sel_cfg::FUNC46_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func46_in_sel_cfg;
#[doc = "FUNC47_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC47_IN_SEL_CFG_SPEC>`"]
pub type FUNC47_IN_SEL_CFG = crate::Reg<func47_in_sel_cfg::FUNC47_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func47_in_sel_cfg;
#[doc = "FUNC48_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC48_IN_SEL_CFG_SPEC>`"]
pub type FUNC48_IN_SEL_CFG = crate::Reg<func48_in_sel_cfg::FUNC48_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func48_in_sel_cfg;
#[doc = "FUNC49_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC49_IN_SEL_CFG_SPEC>`"]
pub type FUNC49_IN_SEL_CFG = crate::Reg<func49_in_sel_cfg::FUNC49_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func49_in_sel_cfg;
#[doc = "FUNC50_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC50_IN_SEL_CFG_SPEC>`"]
pub type FUNC50_IN_SEL_CFG = crate::Reg<func50_in_sel_cfg::FUNC50_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func50_in_sel_cfg;
#[doc = "FUNC51_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC51_IN_SEL_CFG_SPEC>`"]
pub type FUNC51_IN_SEL_CFG = crate::Reg<func51_in_sel_cfg::FUNC51_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func51_in_sel_cfg;
#[doc = "FUNC52_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC52_IN_SEL_CFG_SPEC>`"]
pub type FUNC52_IN_SEL_CFG = crate::Reg<func52_in_sel_cfg::FUNC52_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func52_in_sel_cfg;
#[doc = "FUNC53_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC53_IN_SEL_CFG_SPEC>`"]
pub type FUNC53_IN_SEL_CFG = crate::Reg<func53_in_sel_cfg::FUNC53_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func53_in_sel_cfg;
#[doc = "FUNC54_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC54_IN_SEL_CFG_SPEC>`"]
pub type FUNC54_IN_SEL_CFG = crate::Reg<func54_in_sel_cfg::FUNC54_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func54_in_sel_cfg;
#[doc = "FUNC55_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC55_IN_SEL_CFG_SPEC>`"]
pub type FUNC55_IN_SEL_CFG = crate::Reg<func55_in_sel_cfg::FUNC55_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func55_in_sel_cfg;
#[doc = "FUNC56_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC56_IN_SEL_CFG_SPEC>`"]
pub type FUNC56_IN_SEL_CFG = crate::Reg<func56_in_sel_cfg::FUNC56_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func56_in_sel_cfg;
#[doc = "FUNC57_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC57_IN_SEL_CFG_SPEC>`"]
pub type FUNC57_IN_SEL_CFG = crate::Reg<func57_in_sel_cfg::FUNC57_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func57_in_sel_cfg;
#[doc = "FUNC58_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC58_IN_SEL_CFG_SPEC>`"]
pub type FUNC58_IN_SEL_CFG = crate::Reg<func58_in_sel_cfg::FUNC58_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func58_in_sel_cfg;
#[doc = "FUNC59_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC59_IN_SEL_CFG_SPEC>`"]
pub type FUNC59_IN_SEL_CFG = crate::Reg<func59_in_sel_cfg::FUNC59_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func59_in_sel_cfg;
#[doc = "FUNC60_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC60_IN_SEL_CFG_SPEC>`"]
pub type FUNC60_IN_SEL_CFG = crate::Reg<func60_in_sel_cfg::FUNC60_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func60_in_sel_cfg;
#[doc = "FUNC61_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC61_IN_SEL_CFG_SPEC>`"]
pub type FUNC61_IN_SEL_CFG = crate::Reg<func61_in_sel_cfg::FUNC61_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func61_in_sel_cfg;
#[doc = "FUNC62_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC62_IN_SEL_CFG_SPEC>`"]
pub type FUNC62_IN_SEL_CFG = crate::Reg<func62_in_sel_cfg::FUNC62_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func62_in_sel_cfg;
#[doc = "FUNC63_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC63_IN_SEL_CFG_SPEC>`"]
pub type FUNC63_IN_SEL_CFG = crate::Reg<func63_in_sel_cfg::FUNC63_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func63_in_sel_cfg;
#[doc = "FUNC64_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC64_IN_SEL_CFG_SPEC>`"]
pub type FUNC64_IN_SEL_CFG = crate::Reg<func64_in_sel_cfg::FUNC64_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func64_in_sel_cfg;
#[doc = "FUNC65_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC65_IN_SEL_CFG_SPEC>`"]
pub type FUNC65_IN_SEL_CFG = crate::Reg<func65_in_sel_cfg::FUNC65_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func65_in_sel_cfg;
#[doc = "FUNC66_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC66_IN_SEL_CFG_SPEC>`"]
pub type FUNC66_IN_SEL_CFG = crate::Reg<func66_in_sel_cfg::FUNC66_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func66_in_sel_cfg;
#[doc = "FUNC67_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC67_IN_SEL_CFG_SPEC>`"]
pub type FUNC67_IN_SEL_CFG = crate::Reg<func67_in_sel_cfg::FUNC67_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func67_in_sel_cfg;
#[doc = "FUNC68_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC68_IN_SEL_CFG_SPEC>`"]
pub type FUNC68_IN_SEL_CFG = crate::Reg<func68_in_sel_cfg::FUNC68_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func68_in_sel_cfg;
#[doc = "FUNC69_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC69_IN_SEL_CFG_SPEC>`"]
pub type FUNC69_IN_SEL_CFG = crate::Reg<func69_in_sel_cfg::FUNC69_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func69_in_sel_cfg;
#[doc = "FUNC70_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC70_IN_SEL_CFG_SPEC>`"]
pub type FUNC70_IN_SEL_CFG = crate::Reg<func70_in_sel_cfg::FUNC70_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func70_in_sel_cfg;
#[doc = "FUNC71_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC71_IN_SEL_CFG_SPEC>`"]
pub type FUNC71_IN_SEL_CFG = crate::Reg<func71_in_sel_cfg::FUNC71_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func71_in_sel_cfg;
#[doc = "FUNC72_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC72_IN_SEL_CFG_SPEC>`"]
pub type FUNC72_IN_SEL_CFG = crate::Reg<func72_in_sel_cfg::FUNC72_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func72_in_sel_cfg;
#[doc = "FUNC73_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC73_IN_SEL_CFG_SPEC>`"]
pub type FUNC73_IN_SEL_CFG = crate::Reg<func73_in_sel_cfg::FUNC73_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func73_in_sel_cfg;
#[doc = "FUNC77_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC77_IN_SEL_CFG_SPEC>`"]
pub type FUNC77_IN_SEL_CFG = crate::Reg<func77_in_sel_cfg::FUNC77_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func77_in_sel_cfg;
#[doc = "FUNC81_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC81_IN_SEL_CFG_SPEC>`"]
pub type FUNC81_IN_SEL_CFG = crate::Reg<func81_in_sel_cfg::FUNC81_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func81_in_sel_cfg;
#[doc = "FUNC82_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC82_IN_SEL_CFG_SPEC>`"]
pub type FUNC82_IN_SEL_CFG = crate::Reg<func82_in_sel_cfg::FUNC82_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func82_in_sel_cfg;
#[doc = "FUNC87_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC87_IN_SEL_CFG_SPEC>`"]
pub type FUNC87_IN_SEL_CFG = crate::Reg<func87_in_sel_cfg::FUNC87_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func87_in_sel_cfg;
#[doc = "FUNC88_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC88_IN_SEL_CFG_SPEC>`"]
pub type FUNC88_IN_SEL_CFG = crate::Reg<func88_in_sel_cfg::FUNC88_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func88_in_sel_cfg;
#[doc = "FUNC89_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC89_IN_SEL_CFG_SPEC>`"]
pub type FUNC89_IN_SEL_CFG = crate::Reg<func89_in_sel_cfg::FUNC89_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func89_in_sel_cfg;
#[doc = "FUNC90_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC90_IN_SEL_CFG_SPEC>`"]
pub type FUNC90_IN_SEL_CFG = crate::Reg<func90_in_sel_cfg::FUNC90_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func90_in_sel_cfg;
#[doc = "FUNC91_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC91_IN_SEL_CFG_SPEC>`"]
pub type FUNC91_IN_SEL_CFG = crate::Reg<func91_in_sel_cfg::FUNC91_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func91_in_sel_cfg;
#[doc = "FUNC92_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC92_IN_SEL_CFG_SPEC>`"]
pub type FUNC92_IN_SEL_CFG = crate::Reg<func92_in_sel_cfg::FUNC92_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func92_in_sel_cfg;
#[doc = "FUNC93_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC93_IN_SEL_CFG_SPEC>`"]
pub type FUNC93_IN_SEL_CFG = crate::Reg<func93_in_sel_cfg::FUNC93_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func93_in_sel_cfg;
#[doc = "FUNC94_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC94_IN_SEL_CFG_SPEC>`"]
pub type FUNC94_IN_SEL_CFG = crate::Reg<func94_in_sel_cfg::FUNC94_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func94_in_sel_cfg;
#[doc = "FUNC95_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC95_IN_SEL_CFG_SPEC>`"]
pub type FUNC95_IN_SEL_CFG = crate::Reg<func95_in_sel_cfg::FUNC95_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func95_in_sel_cfg;
#[doc = "FUNC97_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC97_IN_SEL_CFG_SPEC>`"]
pub type FUNC97_IN_SEL_CFG = crate::Reg<func97_in_sel_cfg::FUNC97_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func97_in_sel_cfg;
#[doc = "FUNC98_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC98_IN_SEL_CFG_SPEC>`"]
pub type FUNC98_IN_SEL_CFG = crate::Reg<func98_in_sel_cfg::FUNC98_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func98_in_sel_cfg;
#[doc = "FUNC99_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC99_IN_SEL_CFG_SPEC>`"]
pub type FUNC99_IN_SEL_CFG = crate::Reg<func99_in_sel_cfg::FUNC99_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func99_in_sel_cfg;
#[doc = "FUNC100_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC100_IN_SEL_CFG_SPEC>`"]
pub type FUNC100_IN_SEL_CFG = crate::Reg<func100_in_sel_cfg::FUNC100_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func100_in_sel_cfg;
#[doc = "FUNC101_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC101_IN_SEL_CFG_SPEC>`"]
pub type FUNC101_IN_SEL_CFG = crate::Reg<func101_in_sel_cfg::FUNC101_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func101_in_sel_cfg;
#[doc = "FUNC102_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC102_IN_SEL_CFG_SPEC>`"]
pub type FUNC102_IN_SEL_CFG = crate::Reg<func102_in_sel_cfg::FUNC102_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func102_in_sel_cfg;
#[doc = "FUNC103_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC103_IN_SEL_CFG_SPEC>`"]
pub type FUNC103_IN_SEL_CFG = crate::Reg<func103_in_sel_cfg::FUNC103_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func103_in_sel_cfg;
#[doc = "FUNC104_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC104_IN_SEL_CFG_SPEC>`"]
pub type FUNC104_IN_SEL_CFG = crate::Reg<func104_in_sel_cfg::FUNC104_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func104_in_sel_cfg;
#[doc = "FUNC105_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC105_IN_SEL_CFG_SPEC>`"]
pub type FUNC105_IN_SEL_CFG = crate::Reg<func105_in_sel_cfg::FUNC105_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func105_in_sel_cfg;
#[doc = "FUNC106_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC106_IN_SEL_CFG_SPEC>`"]
pub type FUNC106_IN_SEL_CFG = crate::Reg<func106_in_sel_cfg::FUNC106_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func106_in_sel_cfg;
#[doc = "FUNC107_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC107_IN_SEL_CFG_SPEC>`"]
pub type FUNC107_IN_SEL_CFG = crate::Reg<func107_in_sel_cfg::FUNC107_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func107_in_sel_cfg;
#[doc = "FUNC108_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC108_IN_SEL_CFG_SPEC>`"]
pub type FUNC108_IN_SEL_CFG = crate::Reg<func108_in_sel_cfg::FUNC108_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func108_in_sel_cfg;
#[doc = "FUNC109_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC109_IN_SEL_CFG_SPEC>`"]
pub type FUNC109_IN_SEL_CFG = crate::Reg<func109_in_sel_cfg::FUNC109_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func109_in_sel_cfg;
#[doc = "FUNC110_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC110_IN_SEL_CFG_SPEC>`"]
pub type FUNC110_IN_SEL_CFG = crate::Reg<func110_in_sel_cfg::FUNC110_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func110_in_sel_cfg;
#[doc = "FUNC111_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC111_IN_SEL_CFG_SPEC>`"]
pub type FUNC111_IN_SEL_CFG = crate::Reg<func111_in_sel_cfg::FUNC111_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func111_in_sel_cfg;
#[doc = "FUNC112_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC112_IN_SEL_CFG_SPEC>`"]
pub type FUNC112_IN_SEL_CFG = crate::Reg<func112_in_sel_cfg::FUNC112_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func112_in_sel_cfg;
#[doc = "FUNC113_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC113_IN_SEL_CFG_SPEC>`"]
pub type FUNC113_IN_SEL_CFG = crate::Reg<func113_in_sel_cfg::FUNC113_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func113_in_sel_cfg;
#[doc = "FUNC114_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC114_IN_SEL_CFG_SPEC>`"]
pub type FUNC114_IN_SEL_CFG = crate::Reg<func114_in_sel_cfg::FUNC114_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func114_in_sel_cfg;
#[doc = "FUNC115_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC115_IN_SEL_CFG_SPEC>`"]
pub type FUNC115_IN_SEL_CFG = crate::Reg<func115_in_sel_cfg::FUNC115_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func115_in_sel_cfg;
#[doc = "FUNC116_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC116_IN_SEL_CFG_SPEC>`"]
pub type FUNC116_IN_SEL_CFG = crate::Reg<func116_in_sel_cfg::FUNC116_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func116_in_sel_cfg;
#[doc = "FUNC117_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC117_IN_SEL_CFG_SPEC>`"]
pub type FUNC117_IN_SEL_CFG = crate::Reg<func117_in_sel_cfg::FUNC117_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func117_in_sel_cfg;
#[doc = "FUNC118_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC118_IN_SEL_CFG_SPEC>`"]
pub type FUNC118_IN_SEL_CFG = crate::Reg<func118_in_sel_cfg::FUNC118_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func118_in_sel_cfg;
#[doc = "FUNC119_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC119_IN_SEL_CFG_SPEC>`"]
pub type FUNC119_IN_SEL_CFG = crate::Reg<func119_in_sel_cfg::FUNC119_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func119_in_sel_cfg;
#[doc = "FUNC120_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC120_IN_SEL_CFG_SPEC>`"]
pub type FUNC120_IN_SEL_CFG = crate::Reg<func120_in_sel_cfg::FUNC120_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func120_in_sel_cfg;
#[doc = "FUNC121_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC121_IN_SEL_CFG_SPEC>`"]
pub type FUNC121_IN_SEL_CFG = crate::Reg<func121_in_sel_cfg::FUNC121_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func121_in_sel_cfg;
#[doc = "FUNC122_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC122_IN_SEL_CFG_SPEC>`"]
pub type FUNC122_IN_SEL_CFG = crate::Reg<func122_in_sel_cfg::FUNC122_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func122_in_sel_cfg;
#[doc = "FUNC123_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC123_IN_SEL_CFG_SPEC>`"]
pub type FUNC123_IN_SEL_CFG = crate::Reg<func123_in_sel_cfg::FUNC123_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func123_in_sel_cfg;
#[doc = "FUNC124_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC124_IN_SEL_CFG_SPEC>`"]
pub type FUNC124_IN_SEL_CFG = crate::Reg<func124_in_sel_cfg::FUNC124_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func124_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC_OUT_SEL_CFG_SPEC>`"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func_out_sel_cfg;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod date;
