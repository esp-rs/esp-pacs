#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Otp debuger block0 data register1."]
    pub wr_dis: WR_DIS,
    #[doc = "0x04 - Otp debuger block0 data register2."]
    pub blk0_backup1_w1: BLK0_BACKUP1_W1,
    #[doc = "0x08 - Otp debuger block0 data register3."]
    pub blk0_backup1_w2: BLK0_BACKUP1_W2,
    #[doc = "0x0c - Otp debuger block0 data register4."]
    pub blk0_backup1_w3: BLK0_BACKUP1_W3,
    #[doc = "0x10 - Otp debuger block0 data register5."]
    pub blk0_backup1_w4: BLK0_BACKUP1_W4,
    #[doc = "0x14 - Otp debuger block0 data register6."]
    pub blk0_backup1_w5: BLK0_BACKUP1_W5,
    #[doc = "0x18 - Otp debuger block0 data register7."]
    pub blk0_backup2_w1: BLK0_BACKUP2_W1,
    #[doc = "0x1c - Otp debuger block0 data register8."]
    pub blk0_backup2_w2: BLK0_BACKUP2_W2,
    #[doc = "0x20 - Otp debuger block0 data register9."]
    pub blk0_backup2_w3: BLK0_BACKUP2_W3,
    #[doc = "0x24 - Otp debuger block0 data register10."]
    pub blk0_backup2_w4: BLK0_BACKUP2_W4,
    #[doc = "0x28 - Otp debuger block0 data register11."]
    pub blk0_backup2_w5: BLK0_BACKUP2_W5,
    #[doc = "0x2c - Otp debuger block0 data register12."]
    pub blk0_backup3_w1: BLK0_BACKUP3_W1,
    #[doc = "0x30 - Otp debuger block0 data register13."]
    pub blk0_backup3_w2: BLK0_BACKUP3_W2,
    #[doc = "0x34 - Otp debuger block0 data register14."]
    pub blk0_backup3_w3: BLK0_BACKUP3_W3,
    #[doc = "0x38 - Otp debuger block0 data register15."]
    pub blk0_backup3_w4: BLK0_BACKUP3_W4,
    #[doc = "0x3c - Otp debuger block0 data register16."]
    pub blk0_backup3_w5: BLK0_BACKUP3_W5,
    #[doc = "0x40 - Otp debuger block0 data register17."]
    pub blk0_backup4_w1: BLK0_BACKUP4_W1,
    #[doc = "0x44 - Otp debuger block0 data register18."]
    pub blk0_backup4_w2: BLK0_BACKUP4_W2,
    #[doc = "0x48 - Otp debuger block0 data register19."]
    pub blk0_backup4_w3: BLK0_BACKUP4_W3,
    #[doc = "0x4c - Otp debuger block0 data register20."]
    pub blk0_backup4_w4: BLK0_BACKUP4_W4,
    #[doc = "0x50 - Otp debuger block0 data register21."]
    pub blk0_backup4_w5: BLK0_BACKUP4_W5,
    #[doc = "0x54 - Otp debuger block1 data register1."]
    pub blk1_w1: BLK1_W1,
    #[doc = "0x58 - Otp debuger block1 data register2."]
    pub blk1_w2: BLK1_W2,
    #[doc = "0x5c - Otp debuger block1 data register3."]
    pub blk1_w3: BLK1_W3,
    #[doc = "0x60 - Otp debuger block1 data register4."]
    pub blk1_w4: BLK1_W4,
    #[doc = "0x64 - Otp debuger block1 data register5."]
    pub blk1_w5: BLK1_W5,
    #[doc = "0x68 - Otp debuger block1 data register6."]
    pub blk1_w6: BLK1_W6,
    #[doc = "0x6c - Otp debuger block1 data register7."]
    pub blk1_w7: BLK1_W7,
    #[doc = "0x70 - Otp debuger block1 data register8."]
    pub blk1_w8: BLK1_W8,
    #[doc = "0x74 - Otp debuger block1 data register9."]
    pub blk1_w9: BLK1_W9,
    #[doc = "0x78 - Otp debuger block2 data register1."]
    pub blk2_w1: BLK2_W1,
    #[doc = "0x7c - Otp debuger block2 data register2."]
    pub blk2_w2: BLK2_W2,
    #[doc = "0x80 - Otp debuger block2 data register3."]
    pub blk2_w3: BLK2_W3,
    #[doc = "0x84 - Otp debuger block2 data register4."]
    pub blk2_w4: BLK2_W4,
    #[doc = "0x88 - Otp debuger block2 data register5."]
    pub blk2_w5: BLK2_W5,
    #[doc = "0x8c - Otp debuger block2 data register6."]
    pub blk2_w6: BLK2_W6,
    #[doc = "0x90 - Otp debuger block2 data register7."]
    pub blk2_w7: BLK2_W7,
    #[doc = "0x94 - Otp debuger block2 data register8."]
    pub blk2_w8: BLK2_W8,
    #[doc = "0x98 - Otp debuger block2 data register9."]
    pub blk2_w9: BLK2_W9,
    #[doc = "0x9c - Otp debuger block2 data register10."]
    pub blk2_w10: BLK2_W10,
    #[doc = "0xa0 - Otp debuger block2 data register11."]
    pub blk2_w11: BLK2_W11,
    #[doc = "0xa4 - Otp debuger block3 data register1."]
    pub blk3_w1: BLK3_W1,
    #[doc = "0xa8 - Otp debuger block3 data register2."]
    pub blk3_w2: BLK3_W2,
    #[doc = "0xac - Otp debuger block3 data register3."]
    pub blk3_w3: BLK3_W3,
    #[doc = "0xb0 - Otp debuger block3 data register4."]
    pub blk3_w4: BLK3_W4,
    #[doc = "0xb4 - Otp debuger block3 data register5."]
    pub blk3_w5: BLK3_W5,
    #[doc = "0xb8 - Otp debuger block3 data register6."]
    pub blk3_w6: BLK3_W6,
    #[doc = "0xbc - Otp debuger block3 data register7."]
    pub blk3_w7: BLK3_W7,
    #[doc = "0xc0 - Otp debuger block3 data register8."]
    pub blk3_w8: BLK3_W8,
    #[doc = "0xc4 - Otp debuger block3 data register9."]
    pub blk3_w9: BLK3_W9,
    #[doc = "0xc8 - Otp debuger block3 data register10."]
    pub blk3_w10: BLK3_W10,
    #[doc = "0xcc - Otp debuger block3 data register11."]
    pub blk3_w11: BLK3_W11,
    #[doc = "0xd0 - Otp debuger block4 data register1."]
    pub blk4_w1: BLK4_W1,
    #[doc = "0xd4 - Otp debuger block4 data register2."]
    pub blk4_w2: BLK4_W2,
    #[doc = "0xd8 - Otp debuger block4 data register3."]
    pub blk4_w3: BLK4_W3,
    #[doc = "0xdc - Otp debuger block4 data register4."]
    pub blk4_w4: BLK4_W4,
    #[doc = "0xe0 - Otp debuger block4 data register5."]
    pub blk4_w5: BLK4_W5,
    #[doc = "0xe4 - Otp debuger block4 data register6."]
    pub blk4_w6: BLK4_W6,
    #[doc = "0xe8 - Otp debuger block4 data register7."]
    pub blk4_w7: BLK4_W7,
    #[doc = "0xec - Otp debuger block4 data register8."]
    pub blk4_w8: BLK4_W8,
    #[doc = "0xf0 - Otp debuger block4 data register9."]
    pub blk4_w9: BLK4_W9,
    #[doc = "0xf4 - Otp debuger block4 data registe10."]
    pub blk4_w10: BLK4_W10,
    #[doc = "0xf8 - Otp debuger block4 data register11."]
    pub blk4_w11: BLK4_W11,
    #[doc = "0xfc - Otp debuger block5 data register1."]
    pub blk5_w1: BLK5_W1,
    #[doc = "0x100 - Otp debuger block5 data register2."]
    pub blk5_w2: BLK5_W2,
    #[doc = "0x104 - Otp debuger block5 data register3."]
    pub blk5_w3: BLK5_W3,
    #[doc = "0x108 - Otp debuger block5 data register4."]
    pub blk5_w4: BLK5_W4,
    #[doc = "0x10c - Otp debuger block5 data register5."]
    pub blk5_w5: BLK5_W5,
    #[doc = "0x110 - Otp debuger block5 data register6."]
    pub blk5_w6: BLK5_W6,
    #[doc = "0x114 - Otp debuger block5 data register7."]
    pub blk5_w7: BLK5_W7,
    #[doc = "0x118 - Otp debuger block5 data register8."]
    pub blk5_w8: BLK5_W8,
    #[doc = "0x11c - Otp debuger block5 data register9."]
    pub blk5_w9: BLK5_W9,
    #[doc = "0x120 - Otp debuger block5 data register10."]
    pub blk5_w10: BLK5_W10,
    #[doc = "0x124 - Otp debuger block5 data register11."]
    pub blk5_w11: BLK5_W11,
    #[doc = "0x128 - Otp debuger block6 data register1."]
    pub blk6_w1: BLK6_W1,
    #[doc = "0x12c - Otp debuger block6 data register2."]
    pub blk6_w2: BLK6_W2,
    #[doc = "0x130 - Otp debuger block6 data register3."]
    pub blk6_w3: BLK6_W3,
    #[doc = "0x134 - Otp debuger block6 data register4."]
    pub blk6_w4: BLK6_W4,
    #[doc = "0x138 - Otp debuger block6 data register5."]
    pub blk6_w5: BLK6_W5,
    #[doc = "0x13c - Otp debuger block6 data register6."]
    pub blk6_w6: BLK6_W6,
    #[doc = "0x140 - Otp debuger block6 data register7."]
    pub blk6_w7: BLK6_W7,
    #[doc = "0x144 - Otp debuger block6 data register8."]
    pub blk6_w8: BLK6_W8,
    #[doc = "0x148 - Otp debuger block6 data register9."]
    pub blk6_w9: BLK6_W9,
    #[doc = "0x14c - Otp debuger block6 data register10."]
    pub blk6_w10: BLK6_W10,
    #[doc = "0x150 - Otp debuger block6 data register11."]
    pub blk6_w11: BLK6_W11,
    #[doc = "0x154 - Otp debuger block7 data register1."]
    pub blk7_w1: BLK7_W1,
    #[doc = "0x158 - Otp debuger block7 data register2."]
    pub blk7_w2: BLK7_W2,
    #[doc = "0x15c - Otp debuger block7 data register3."]
    pub blk7_w3: BLK7_W3,
    #[doc = "0x160 - Otp debuger block7 data register4."]
    pub blk7_w4: BLK7_W4,
    #[doc = "0x164 - Otp debuger block7 data register5."]
    pub blk7_w5: BLK7_W5,
    #[doc = "0x168 - Otp debuger block7 data register6."]
    pub blk7_w6: BLK7_W6,
    #[doc = "0x16c - Otp debuger block7 data register7."]
    pub blk7_w7: BLK7_W7,
    #[doc = "0x170 - Otp debuger block7 data register8."]
    pub blk7_w8: BLK7_W8,
    #[doc = "0x174 - Otp debuger block7 data register9."]
    pub blk7_w9: BLK7_W9,
    #[doc = "0x178 - Otp debuger block7 data register10."]
    pub blk7_w10: BLK7_W10,
    #[doc = "0x17c - Otp debuger block7 data register11."]
    pub blk7_w11: BLK7_W11,
    #[doc = "0x180 - Otp debuger block8 data register1."]
    pub blk8_w1: BLK8_W1,
    #[doc = "0x184 - Otp debuger block8 data register2."]
    pub blk8_w2: BLK8_W2,
    #[doc = "0x188 - Otp debuger block8 data register3."]
    pub blk8_w3: BLK8_W3,
    #[doc = "0x18c - Otp debuger block8 data register4."]
    pub blk8_w4: BLK8_W4,
    #[doc = "0x190 - Otp debuger block8 data register5."]
    pub blk8_w5: BLK8_W5,
    #[doc = "0x194 - Otp debuger block8 data register6."]
    pub blk8_w6: BLK8_W6,
    #[doc = "0x198 - Otp debuger block8 data register7."]
    pub blk8_w7: BLK8_W7,
    #[doc = "0x19c - Otp debuger block8 data register8."]
    pub blk8_w8: BLK8_W8,
    #[doc = "0x1a0 - Otp debuger block8 data register9."]
    pub blk8_w9: BLK8_W9,
    #[doc = "0x1a4 - Otp debuger block8 data register10."]
    pub blk8_w10: BLK8_W10,
    #[doc = "0x1a8 - Otp debuger block8 data register11."]
    pub blk8_w11: BLK8_W11,
    #[doc = "0x1ac - Otp debuger block9 data register1."]
    pub blk9_w1: BLK9_W1,
    #[doc = "0x1b0 - Otp debuger block9 data register2."]
    pub blk9_w2: BLK9_W2,
    #[doc = "0x1b4 - Otp debuger block9 data register3."]
    pub blk9_w3: BLK9_W3,
    #[doc = "0x1b8 - Otp debuger block9 data register4."]
    pub blk9_w4: BLK9_W4,
    #[doc = "0x1bc - Otp debuger block9 data register5."]
    pub blk9_w5: BLK9_W5,
    #[doc = "0x1c0 - Otp debuger block9 data register6."]
    pub blk9_w6: BLK9_W6,
    #[doc = "0x1c4 - Otp debuger block9 data register7."]
    pub blk9_w7: BLK9_W7,
    #[doc = "0x1c8 - Otp debuger block9 data register8."]
    pub blk9_w8: BLK9_W8,
    #[doc = "0x1cc - Otp debuger block9 data register9."]
    pub blk9_w9: BLK9_W9,
    #[doc = "0x1d0 - Otp debuger block9 data register10."]
    pub blk9_w10: BLK9_W10,
    #[doc = "0x1d4 - Otp debuger block9 data register11."]
    pub blk9_w11: BLK9_W11,
    #[doc = "0x1d8 - Otp debuger block10 data register1."]
    pub blk10_w1: BLK10_W1,
    #[doc = "0x1dc - Otp debuger block10 data register2."]
    pub blk10_w2: BLK10_W2,
    #[doc = "0x1e0 - Otp debuger block10 data register3."]
    pub blk10_w3: BLK10_W3,
    #[doc = "0x1e4 - Otp debuger block10 data register4."]
    pub blk10_w4: BLK10_W4,
    #[doc = "0x1e8 - Otp debuger block10 data register5."]
    pub blk10_w5: BLK10_W5,
    #[doc = "0x1ec - Otp debuger block10 data register6."]
    pub blk10_w6: BLK10_W6,
    #[doc = "0x1f0 - Otp debuger block10 data register7."]
    pub blk10_w7: BLK10_W7,
    #[doc = "0x1f4 - Otp debuger block10 data register8."]
    pub blk10_w8: BLK10_W8,
    #[doc = "0x1f8 - Otp debuger block10 data register9."]
    pub blk10_w9: BLK10_W9,
    #[doc = "0x1fc - Otp debuger block10 data register10."]
    pub blk10_w10: BLK10_W10,
    #[doc = "0x200 - Otp debuger block10 data register11."]
    pub blk10_w11: BLK10_W11,
    #[doc = "0x204 - Otp debuger clk_en configuration register."]
    pub clk: CLK,
    #[doc = "0x208 - Otp_debuger apb2otp enable configuration register."]
    pub apb2otp_en: APB2OTP_EN,
    #[doc = "0x20c - eFuse version register."]
    pub date: DATE,
}
#[doc = "WR_DIS (r) register accessor: an alias for `Reg<WR_DIS_SPEC>`"]
pub type WR_DIS = crate::Reg<wr_dis::WR_DIS_SPEC>;
#[doc = "Otp debuger block0 data register1."]
pub mod wr_dis;
#[doc = "BLK0_BACKUP1_W1 (r) register accessor: an alias for `Reg<BLK0_BACKUP1_W1_SPEC>`"]
pub type BLK0_BACKUP1_W1 = crate::Reg<blk0_backup1_w1::BLK0_BACKUP1_W1_SPEC>;
#[doc = "Otp debuger block0 data register2."]
pub mod blk0_backup1_w1;
#[doc = "BLK0_BACKUP1_W2 (r) register accessor: an alias for `Reg<BLK0_BACKUP1_W2_SPEC>`"]
pub type BLK0_BACKUP1_W2 = crate::Reg<blk0_backup1_w2::BLK0_BACKUP1_W2_SPEC>;
#[doc = "Otp debuger block0 data register3."]
pub mod blk0_backup1_w2;
#[doc = "BLK0_BACKUP1_W3 (r) register accessor: an alias for `Reg<BLK0_BACKUP1_W3_SPEC>`"]
pub type BLK0_BACKUP1_W3 = crate::Reg<blk0_backup1_w3::BLK0_BACKUP1_W3_SPEC>;
#[doc = "Otp debuger block0 data register4."]
pub mod blk0_backup1_w3;
#[doc = "BLK0_BACKUP1_W4 (r) register accessor: an alias for `Reg<BLK0_BACKUP1_W4_SPEC>`"]
pub type BLK0_BACKUP1_W4 = crate::Reg<blk0_backup1_w4::BLK0_BACKUP1_W4_SPEC>;
#[doc = "Otp debuger block0 data register5."]
pub mod blk0_backup1_w4;
#[doc = "BLK0_BACKUP1_W5 (r) register accessor: an alias for `Reg<BLK0_BACKUP1_W5_SPEC>`"]
pub type BLK0_BACKUP1_W5 = crate::Reg<blk0_backup1_w5::BLK0_BACKUP1_W5_SPEC>;
#[doc = "Otp debuger block0 data register6."]
pub mod blk0_backup1_w5;
#[doc = "BLK0_BACKUP2_W1 (r) register accessor: an alias for `Reg<BLK0_BACKUP2_W1_SPEC>`"]
pub type BLK0_BACKUP2_W1 = crate::Reg<blk0_backup2_w1::BLK0_BACKUP2_W1_SPEC>;
#[doc = "Otp debuger block0 data register7."]
pub mod blk0_backup2_w1;
#[doc = "BLK0_BACKUP2_W2 (r) register accessor: an alias for `Reg<BLK0_BACKUP2_W2_SPEC>`"]
pub type BLK0_BACKUP2_W2 = crate::Reg<blk0_backup2_w2::BLK0_BACKUP2_W2_SPEC>;
#[doc = "Otp debuger block0 data register8."]
pub mod blk0_backup2_w2;
#[doc = "BLK0_BACKUP2_W3 (r) register accessor: an alias for `Reg<BLK0_BACKUP2_W3_SPEC>`"]
pub type BLK0_BACKUP2_W3 = crate::Reg<blk0_backup2_w3::BLK0_BACKUP2_W3_SPEC>;
#[doc = "Otp debuger block0 data register9."]
pub mod blk0_backup2_w3;
#[doc = "BLK0_BACKUP2_W4 (r) register accessor: an alias for `Reg<BLK0_BACKUP2_W4_SPEC>`"]
pub type BLK0_BACKUP2_W4 = crate::Reg<blk0_backup2_w4::BLK0_BACKUP2_W4_SPEC>;
#[doc = "Otp debuger block0 data register10."]
pub mod blk0_backup2_w4;
#[doc = "BLK0_BACKUP2_W5 (r) register accessor: an alias for `Reg<BLK0_BACKUP2_W5_SPEC>`"]
pub type BLK0_BACKUP2_W5 = crate::Reg<blk0_backup2_w5::BLK0_BACKUP2_W5_SPEC>;
#[doc = "Otp debuger block0 data register11."]
pub mod blk0_backup2_w5;
#[doc = "BLK0_BACKUP3_W1 (r) register accessor: an alias for `Reg<BLK0_BACKUP3_W1_SPEC>`"]
pub type BLK0_BACKUP3_W1 = crate::Reg<blk0_backup3_w1::BLK0_BACKUP3_W1_SPEC>;
#[doc = "Otp debuger block0 data register12."]
pub mod blk0_backup3_w1;
#[doc = "BLK0_BACKUP3_W2 (r) register accessor: an alias for `Reg<BLK0_BACKUP3_W2_SPEC>`"]
pub type BLK0_BACKUP3_W2 = crate::Reg<blk0_backup3_w2::BLK0_BACKUP3_W2_SPEC>;
#[doc = "Otp debuger block0 data register13."]
pub mod blk0_backup3_w2;
#[doc = "BLK0_BACKUP3_W3 (r) register accessor: an alias for `Reg<BLK0_BACKUP3_W3_SPEC>`"]
pub type BLK0_BACKUP3_W3 = crate::Reg<blk0_backup3_w3::BLK0_BACKUP3_W3_SPEC>;
#[doc = "Otp debuger block0 data register14."]
pub mod blk0_backup3_w3;
#[doc = "BLK0_BACKUP3_W4 (r) register accessor: an alias for `Reg<BLK0_BACKUP3_W4_SPEC>`"]
pub type BLK0_BACKUP3_W4 = crate::Reg<blk0_backup3_w4::BLK0_BACKUP3_W4_SPEC>;
#[doc = "Otp debuger block0 data register15."]
pub mod blk0_backup3_w4;
#[doc = "BLK0_BACKUP3_W5 (r) register accessor: an alias for `Reg<BLK0_BACKUP3_W5_SPEC>`"]
pub type BLK0_BACKUP3_W5 = crate::Reg<blk0_backup3_w5::BLK0_BACKUP3_W5_SPEC>;
#[doc = "Otp debuger block0 data register16."]
pub mod blk0_backup3_w5;
#[doc = "BLK0_BACKUP4_W1 (r) register accessor: an alias for `Reg<BLK0_BACKUP4_W1_SPEC>`"]
pub type BLK0_BACKUP4_W1 = crate::Reg<blk0_backup4_w1::BLK0_BACKUP4_W1_SPEC>;
#[doc = "Otp debuger block0 data register17."]
pub mod blk0_backup4_w1;
#[doc = "BLK0_BACKUP4_W2 (r) register accessor: an alias for `Reg<BLK0_BACKUP4_W2_SPEC>`"]
pub type BLK0_BACKUP4_W2 = crate::Reg<blk0_backup4_w2::BLK0_BACKUP4_W2_SPEC>;
#[doc = "Otp debuger block0 data register18."]
pub mod blk0_backup4_w2;
#[doc = "BLK0_BACKUP4_W3 (r) register accessor: an alias for `Reg<BLK0_BACKUP4_W3_SPEC>`"]
pub type BLK0_BACKUP4_W3 = crate::Reg<blk0_backup4_w3::BLK0_BACKUP4_W3_SPEC>;
#[doc = "Otp debuger block0 data register19."]
pub mod blk0_backup4_w3;
#[doc = "BLK0_BACKUP4_W4 (r) register accessor: an alias for `Reg<BLK0_BACKUP4_W4_SPEC>`"]
pub type BLK0_BACKUP4_W4 = crate::Reg<blk0_backup4_w4::BLK0_BACKUP4_W4_SPEC>;
#[doc = "Otp debuger block0 data register20."]
pub mod blk0_backup4_w4;
#[doc = "BLK0_BACKUP4_W5 (r) register accessor: an alias for `Reg<BLK0_BACKUP4_W5_SPEC>`"]
pub type BLK0_BACKUP4_W5 = crate::Reg<blk0_backup4_w5::BLK0_BACKUP4_W5_SPEC>;
#[doc = "Otp debuger block0 data register21."]
pub mod blk0_backup4_w5;
#[doc = "BLK1_W1 (r) register accessor: an alias for `Reg<BLK1_W1_SPEC>`"]
pub type BLK1_W1 = crate::Reg<blk1_w1::BLK1_W1_SPEC>;
#[doc = "Otp debuger block1 data register1."]
pub mod blk1_w1;
#[doc = "BLK1_W2 (r) register accessor: an alias for `Reg<BLK1_W2_SPEC>`"]
pub type BLK1_W2 = crate::Reg<blk1_w2::BLK1_W2_SPEC>;
#[doc = "Otp debuger block1 data register2."]
pub mod blk1_w2;
#[doc = "BLK1_W3 (r) register accessor: an alias for `Reg<BLK1_W3_SPEC>`"]
pub type BLK1_W3 = crate::Reg<blk1_w3::BLK1_W3_SPEC>;
#[doc = "Otp debuger block1 data register3."]
pub mod blk1_w3;
#[doc = "BLK1_W4 (r) register accessor: an alias for `Reg<BLK1_W4_SPEC>`"]
pub type BLK1_W4 = crate::Reg<blk1_w4::BLK1_W4_SPEC>;
#[doc = "Otp debuger block1 data register4."]
pub mod blk1_w4;
#[doc = "BLK1_W5 (r) register accessor: an alias for `Reg<BLK1_W5_SPEC>`"]
pub type BLK1_W5 = crate::Reg<blk1_w5::BLK1_W5_SPEC>;
#[doc = "Otp debuger block1 data register5."]
pub mod blk1_w5;
#[doc = "BLK1_W6 (r) register accessor: an alias for `Reg<BLK1_W6_SPEC>`"]
pub type BLK1_W6 = crate::Reg<blk1_w6::BLK1_W6_SPEC>;
#[doc = "Otp debuger block1 data register6."]
pub mod blk1_w6;
#[doc = "BLK1_W7 (r) register accessor: an alias for `Reg<BLK1_W7_SPEC>`"]
pub type BLK1_W7 = crate::Reg<blk1_w7::BLK1_W7_SPEC>;
#[doc = "Otp debuger block1 data register7."]
pub mod blk1_w7;
#[doc = "BLK1_W8 (r) register accessor: an alias for `Reg<BLK1_W8_SPEC>`"]
pub type BLK1_W8 = crate::Reg<blk1_w8::BLK1_W8_SPEC>;
#[doc = "Otp debuger block1 data register8."]
pub mod blk1_w8;
#[doc = "BLK1_W9 (r) register accessor: an alias for `Reg<BLK1_W9_SPEC>`"]
pub type BLK1_W9 = crate::Reg<blk1_w9::BLK1_W9_SPEC>;
#[doc = "Otp debuger block1 data register9."]
pub mod blk1_w9;
#[doc = "BLK2_W1 (r) register accessor: an alias for `Reg<BLK2_W1_SPEC>`"]
pub type BLK2_W1 = crate::Reg<blk2_w1::BLK2_W1_SPEC>;
#[doc = "Otp debuger block2 data register1."]
pub mod blk2_w1;
#[doc = "BLK2_W2 (r) register accessor: an alias for `Reg<BLK2_W2_SPEC>`"]
pub type BLK2_W2 = crate::Reg<blk2_w2::BLK2_W2_SPEC>;
#[doc = "Otp debuger block2 data register2."]
pub mod blk2_w2;
#[doc = "BLK2_W3 (r) register accessor: an alias for `Reg<BLK2_W3_SPEC>`"]
pub type BLK2_W3 = crate::Reg<blk2_w3::BLK2_W3_SPEC>;
#[doc = "Otp debuger block2 data register3."]
pub mod blk2_w3;
#[doc = "BLK2_W4 (r) register accessor: an alias for `Reg<BLK2_W4_SPEC>`"]
pub type BLK2_W4 = crate::Reg<blk2_w4::BLK2_W4_SPEC>;
#[doc = "Otp debuger block2 data register4."]
pub mod blk2_w4;
#[doc = "BLK2_W5 (r) register accessor: an alias for `Reg<BLK2_W5_SPEC>`"]
pub type BLK2_W5 = crate::Reg<blk2_w5::BLK2_W5_SPEC>;
#[doc = "Otp debuger block2 data register5."]
pub mod blk2_w5;
#[doc = "BLK2_W6 (r) register accessor: an alias for `Reg<BLK2_W6_SPEC>`"]
pub type BLK2_W6 = crate::Reg<blk2_w6::BLK2_W6_SPEC>;
#[doc = "Otp debuger block2 data register6."]
pub mod blk2_w6;
#[doc = "BLK2_W7 (r) register accessor: an alias for `Reg<BLK2_W7_SPEC>`"]
pub type BLK2_W7 = crate::Reg<blk2_w7::BLK2_W7_SPEC>;
#[doc = "Otp debuger block2 data register7."]
pub mod blk2_w7;
#[doc = "BLK2_W8 (r) register accessor: an alias for `Reg<BLK2_W8_SPEC>`"]
pub type BLK2_W8 = crate::Reg<blk2_w8::BLK2_W8_SPEC>;
#[doc = "Otp debuger block2 data register8."]
pub mod blk2_w8;
#[doc = "BLK2_W9 (r) register accessor: an alias for `Reg<BLK2_W9_SPEC>`"]
pub type BLK2_W9 = crate::Reg<blk2_w9::BLK2_W9_SPEC>;
#[doc = "Otp debuger block2 data register9."]
pub mod blk2_w9;
#[doc = "BLK2_W10 (r) register accessor: an alias for `Reg<BLK2_W10_SPEC>`"]
pub type BLK2_W10 = crate::Reg<blk2_w10::BLK2_W10_SPEC>;
#[doc = "Otp debuger block2 data register10."]
pub mod blk2_w10;
#[doc = "BLK2_W11 (r) register accessor: an alias for `Reg<BLK2_W11_SPEC>`"]
pub type BLK2_W11 = crate::Reg<blk2_w11::BLK2_W11_SPEC>;
#[doc = "Otp debuger block2 data register11."]
pub mod blk2_w11;
#[doc = "BLK3_W1 (r) register accessor: an alias for `Reg<BLK3_W1_SPEC>`"]
pub type BLK3_W1 = crate::Reg<blk3_w1::BLK3_W1_SPEC>;
#[doc = "Otp debuger block3 data register1."]
pub mod blk3_w1;
#[doc = "BLK3_W2 (r) register accessor: an alias for `Reg<BLK3_W2_SPEC>`"]
pub type BLK3_W2 = crate::Reg<blk3_w2::BLK3_W2_SPEC>;
#[doc = "Otp debuger block3 data register2."]
pub mod blk3_w2;
#[doc = "BLK3_W3 (r) register accessor: an alias for `Reg<BLK3_W3_SPEC>`"]
pub type BLK3_W3 = crate::Reg<blk3_w3::BLK3_W3_SPEC>;
#[doc = "Otp debuger block3 data register3."]
pub mod blk3_w3;
#[doc = "BLK3_W4 (r) register accessor: an alias for `Reg<BLK3_W4_SPEC>`"]
pub type BLK3_W4 = crate::Reg<blk3_w4::BLK3_W4_SPEC>;
#[doc = "Otp debuger block3 data register4."]
pub mod blk3_w4;
#[doc = "BLK3_W5 (r) register accessor: an alias for `Reg<BLK3_W5_SPEC>`"]
pub type BLK3_W5 = crate::Reg<blk3_w5::BLK3_W5_SPEC>;
#[doc = "Otp debuger block3 data register5."]
pub mod blk3_w5;
#[doc = "BLK3_W6 (r) register accessor: an alias for `Reg<BLK3_W6_SPEC>`"]
pub type BLK3_W6 = crate::Reg<blk3_w6::BLK3_W6_SPEC>;
#[doc = "Otp debuger block3 data register6."]
pub mod blk3_w6;
#[doc = "BLK3_W7 (r) register accessor: an alias for `Reg<BLK3_W7_SPEC>`"]
pub type BLK3_W7 = crate::Reg<blk3_w7::BLK3_W7_SPEC>;
#[doc = "Otp debuger block3 data register7."]
pub mod blk3_w7;
#[doc = "BLK3_W8 (r) register accessor: an alias for `Reg<BLK3_W8_SPEC>`"]
pub type BLK3_W8 = crate::Reg<blk3_w8::BLK3_W8_SPEC>;
#[doc = "Otp debuger block3 data register8."]
pub mod blk3_w8;
#[doc = "BLK3_W9 (r) register accessor: an alias for `Reg<BLK3_W9_SPEC>`"]
pub type BLK3_W9 = crate::Reg<blk3_w9::BLK3_W9_SPEC>;
#[doc = "Otp debuger block3 data register9."]
pub mod blk3_w9;
#[doc = "BLK3_W10 (r) register accessor: an alias for `Reg<BLK3_W10_SPEC>`"]
pub type BLK3_W10 = crate::Reg<blk3_w10::BLK3_W10_SPEC>;
#[doc = "Otp debuger block3 data register10."]
pub mod blk3_w10;
#[doc = "BLK3_W11 (r) register accessor: an alias for `Reg<BLK3_W11_SPEC>`"]
pub type BLK3_W11 = crate::Reg<blk3_w11::BLK3_W11_SPEC>;
#[doc = "Otp debuger block3 data register11."]
pub mod blk3_w11;
#[doc = "BLK4_W1 (r) register accessor: an alias for `Reg<BLK4_W1_SPEC>`"]
pub type BLK4_W1 = crate::Reg<blk4_w1::BLK4_W1_SPEC>;
#[doc = "Otp debuger block4 data register1."]
pub mod blk4_w1;
#[doc = "BLK4_W2 (r) register accessor: an alias for `Reg<BLK4_W2_SPEC>`"]
pub type BLK4_W2 = crate::Reg<blk4_w2::BLK4_W2_SPEC>;
#[doc = "Otp debuger block4 data register2."]
pub mod blk4_w2;
#[doc = "BLK4_W3 (r) register accessor: an alias for `Reg<BLK4_W3_SPEC>`"]
pub type BLK4_W3 = crate::Reg<blk4_w3::BLK4_W3_SPEC>;
#[doc = "Otp debuger block4 data register3."]
pub mod blk4_w3;
#[doc = "BLK4_W4 (r) register accessor: an alias for `Reg<BLK4_W4_SPEC>`"]
pub type BLK4_W4 = crate::Reg<blk4_w4::BLK4_W4_SPEC>;
#[doc = "Otp debuger block4 data register4."]
pub mod blk4_w4;
#[doc = "BLK4_W5 (r) register accessor: an alias for `Reg<BLK4_W5_SPEC>`"]
pub type BLK4_W5 = crate::Reg<blk4_w5::BLK4_W5_SPEC>;
#[doc = "Otp debuger block4 data register5."]
pub mod blk4_w5;
#[doc = "BLK4_W6 (r) register accessor: an alias for `Reg<BLK4_W6_SPEC>`"]
pub type BLK4_W6 = crate::Reg<blk4_w6::BLK4_W6_SPEC>;
#[doc = "Otp debuger block4 data register6."]
pub mod blk4_w6;
#[doc = "BLK4_W7 (r) register accessor: an alias for `Reg<BLK4_W7_SPEC>`"]
pub type BLK4_W7 = crate::Reg<blk4_w7::BLK4_W7_SPEC>;
#[doc = "Otp debuger block4 data register7."]
pub mod blk4_w7;
#[doc = "BLK4_W8 (r) register accessor: an alias for `Reg<BLK4_W8_SPEC>`"]
pub type BLK4_W8 = crate::Reg<blk4_w8::BLK4_W8_SPEC>;
#[doc = "Otp debuger block4 data register8."]
pub mod blk4_w8;
#[doc = "BLK4_W9 (r) register accessor: an alias for `Reg<BLK4_W9_SPEC>`"]
pub type BLK4_W9 = crate::Reg<blk4_w9::BLK4_W9_SPEC>;
#[doc = "Otp debuger block4 data register9."]
pub mod blk4_w9;
#[doc = "BLK4_W10 (r) register accessor: an alias for `Reg<BLK4_W10_SPEC>`"]
pub type BLK4_W10 = crate::Reg<blk4_w10::BLK4_W10_SPEC>;
#[doc = "Otp debuger block4 data registe10."]
pub mod blk4_w10;
#[doc = "BLK4_W11 (r) register accessor: an alias for `Reg<BLK4_W11_SPEC>`"]
pub type BLK4_W11 = crate::Reg<blk4_w11::BLK4_W11_SPEC>;
#[doc = "Otp debuger block4 data register11."]
pub mod blk4_w11;
#[doc = "BLK5_W1 (r) register accessor: an alias for `Reg<BLK5_W1_SPEC>`"]
pub type BLK5_W1 = crate::Reg<blk5_w1::BLK5_W1_SPEC>;
#[doc = "Otp debuger block5 data register1."]
pub mod blk5_w1;
#[doc = "BLK5_W2 (r) register accessor: an alias for `Reg<BLK5_W2_SPEC>`"]
pub type BLK5_W2 = crate::Reg<blk5_w2::BLK5_W2_SPEC>;
#[doc = "Otp debuger block5 data register2."]
pub mod blk5_w2;
#[doc = "BLK5_W3 (r) register accessor: an alias for `Reg<BLK5_W3_SPEC>`"]
pub type BLK5_W3 = crate::Reg<blk5_w3::BLK5_W3_SPEC>;
#[doc = "Otp debuger block5 data register3."]
pub mod blk5_w3;
#[doc = "BLK5_W4 (r) register accessor: an alias for `Reg<BLK5_W4_SPEC>`"]
pub type BLK5_W4 = crate::Reg<blk5_w4::BLK5_W4_SPEC>;
#[doc = "Otp debuger block5 data register4."]
pub mod blk5_w4;
#[doc = "BLK5_W5 (r) register accessor: an alias for `Reg<BLK5_W5_SPEC>`"]
pub type BLK5_W5 = crate::Reg<blk5_w5::BLK5_W5_SPEC>;
#[doc = "Otp debuger block5 data register5."]
pub mod blk5_w5;
#[doc = "BLK5_W6 (r) register accessor: an alias for `Reg<BLK5_W6_SPEC>`"]
pub type BLK5_W6 = crate::Reg<blk5_w6::BLK5_W6_SPEC>;
#[doc = "Otp debuger block5 data register6."]
pub mod blk5_w6;
#[doc = "BLK5_W7 (r) register accessor: an alias for `Reg<BLK5_W7_SPEC>`"]
pub type BLK5_W7 = crate::Reg<blk5_w7::BLK5_W7_SPEC>;
#[doc = "Otp debuger block5 data register7."]
pub mod blk5_w7;
#[doc = "BLK5_W8 (r) register accessor: an alias for `Reg<BLK5_W8_SPEC>`"]
pub type BLK5_W8 = crate::Reg<blk5_w8::BLK5_W8_SPEC>;
#[doc = "Otp debuger block5 data register8."]
pub mod blk5_w8;
#[doc = "BLK5_W9 (r) register accessor: an alias for `Reg<BLK5_W9_SPEC>`"]
pub type BLK5_W9 = crate::Reg<blk5_w9::BLK5_W9_SPEC>;
#[doc = "Otp debuger block5 data register9."]
pub mod blk5_w9;
#[doc = "BLK5_W10 (r) register accessor: an alias for `Reg<BLK5_W10_SPEC>`"]
pub type BLK5_W10 = crate::Reg<blk5_w10::BLK5_W10_SPEC>;
#[doc = "Otp debuger block5 data register10."]
pub mod blk5_w10;
#[doc = "BLK5_W11 (r) register accessor: an alias for `Reg<BLK5_W11_SPEC>`"]
pub type BLK5_W11 = crate::Reg<blk5_w11::BLK5_W11_SPEC>;
#[doc = "Otp debuger block5 data register11."]
pub mod blk5_w11;
#[doc = "BLK6_W1 (r) register accessor: an alias for `Reg<BLK6_W1_SPEC>`"]
pub type BLK6_W1 = crate::Reg<blk6_w1::BLK6_W1_SPEC>;
#[doc = "Otp debuger block6 data register1."]
pub mod blk6_w1;
#[doc = "BLK6_W2 (r) register accessor: an alias for `Reg<BLK6_W2_SPEC>`"]
pub type BLK6_W2 = crate::Reg<blk6_w2::BLK6_W2_SPEC>;
#[doc = "Otp debuger block6 data register2."]
pub mod blk6_w2;
#[doc = "BLK6_W3 (r) register accessor: an alias for `Reg<BLK6_W3_SPEC>`"]
pub type BLK6_W3 = crate::Reg<blk6_w3::BLK6_W3_SPEC>;
#[doc = "Otp debuger block6 data register3."]
pub mod blk6_w3;
#[doc = "BLK6_W4 (r) register accessor: an alias for `Reg<BLK6_W4_SPEC>`"]
pub type BLK6_W4 = crate::Reg<blk6_w4::BLK6_W4_SPEC>;
#[doc = "Otp debuger block6 data register4."]
pub mod blk6_w4;
#[doc = "BLK6_W5 (r) register accessor: an alias for `Reg<BLK6_W5_SPEC>`"]
pub type BLK6_W5 = crate::Reg<blk6_w5::BLK6_W5_SPEC>;
#[doc = "Otp debuger block6 data register5."]
pub mod blk6_w5;
#[doc = "BLK6_W6 (r) register accessor: an alias for `Reg<BLK6_W6_SPEC>`"]
pub type BLK6_W6 = crate::Reg<blk6_w6::BLK6_W6_SPEC>;
#[doc = "Otp debuger block6 data register6."]
pub mod blk6_w6;
#[doc = "BLK6_W7 (r) register accessor: an alias for `Reg<BLK6_W7_SPEC>`"]
pub type BLK6_W7 = crate::Reg<blk6_w7::BLK6_W7_SPEC>;
#[doc = "Otp debuger block6 data register7."]
pub mod blk6_w7;
#[doc = "BLK6_W8 (r) register accessor: an alias for `Reg<BLK6_W8_SPEC>`"]
pub type BLK6_W8 = crate::Reg<blk6_w8::BLK6_W8_SPEC>;
#[doc = "Otp debuger block6 data register8."]
pub mod blk6_w8;
#[doc = "BLK6_W9 (r) register accessor: an alias for `Reg<BLK6_W9_SPEC>`"]
pub type BLK6_W9 = crate::Reg<blk6_w9::BLK6_W9_SPEC>;
#[doc = "Otp debuger block6 data register9."]
pub mod blk6_w9;
#[doc = "BLK6_W10 (r) register accessor: an alias for `Reg<BLK6_W10_SPEC>`"]
pub type BLK6_W10 = crate::Reg<blk6_w10::BLK6_W10_SPEC>;
#[doc = "Otp debuger block6 data register10."]
pub mod blk6_w10;
#[doc = "BLK6_W11 (r) register accessor: an alias for `Reg<BLK6_W11_SPEC>`"]
pub type BLK6_W11 = crate::Reg<blk6_w11::BLK6_W11_SPEC>;
#[doc = "Otp debuger block6 data register11."]
pub mod blk6_w11;
#[doc = "BLK7_W1 (r) register accessor: an alias for `Reg<BLK7_W1_SPEC>`"]
pub type BLK7_W1 = crate::Reg<blk7_w1::BLK7_W1_SPEC>;
#[doc = "Otp debuger block7 data register1."]
pub mod blk7_w1;
#[doc = "BLK7_W2 (r) register accessor: an alias for `Reg<BLK7_W2_SPEC>`"]
pub type BLK7_W2 = crate::Reg<blk7_w2::BLK7_W2_SPEC>;
#[doc = "Otp debuger block7 data register2."]
pub mod blk7_w2;
#[doc = "BLK7_W3 (r) register accessor: an alias for `Reg<BLK7_W3_SPEC>`"]
pub type BLK7_W3 = crate::Reg<blk7_w3::BLK7_W3_SPEC>;
#[doc = "Otp debuger block7 data register3."]
pub mod blk7_w3;
#[doc = "BLK7_W4 (r) register accessor: an alias for `Reg<BLK7_W4_SPEC>`"]
pub type BLK7_W4 = crate::Reg<blk7_w4::BLK7_W4_SPEC>;
#[doc = "Otp debuger block7 data register4."]
pub mod blk7_w4;
#[doc = "BLK7_W5 (r) register accessor: an alias for `Reg<BLK7_W5_SPEC>`"]
pub type BLK7_W5 = crate::Reg<blk7_w5::BLK7_W5_SPEC>;
#[doc = "Otp debuger block7 data register5."]
pub mod blk7_w5;
#[doc = "BLK7_W6 (r) register accessor: an alias for `Reg<BLK7_W6_SPEC>`"]
pub type BLK7_W6 = crate::Reg<blk7_w6::BLK7_W6_SPEC>;
#[doc = "Otp debuger block7 data register6."]
pub mod blk7_w6;
#[doc = "BLK7_W7 (r) register accessor: an alias for `Reg<BLK7_W7_SPEC>`"]
pub type BLK7_W7 = crate::Reg<blk7_w7::BLK7_W7_SPEC>;
#[doc = "Otp debuger block7 data register7."]
pub mod blk7_w7;
#[doc = "BLK7_W8 (r) register accessor: an alias for `Reg<BLK7_W8_SPEC>`"]
pub type BLK7_W8 = crate::Reg<blk7_w8::BLK7_W8_SPEC>;
#[doc = "Otp debuger block7 data register8."]
pub mod blk7_w8;
#[doc = "BLK7_W9 (r) register accessor: an alias for `Reg<BLK7_W9_SPEC>`"]
pub type BLK7_W9 = crate::Reg<blk7_w9::BLK7_W9_SPEC>;
#[doc = "Otp debuger block7 data register9."]
pub mod blk7_w9;
#[doc = "BLK7_W10 (r) register accessor: an alias for `Reg<BLK7_W10_SPEC>`"]
pub type BLK7_W10 = crate::Reg<blk7_w10::BLK7_W10_SPEC>;
#[doc = "Otp debuger block7 data register10."]
pub mod blk7_w10;
#[doc = "BLK7_W11 (r) register accessor: an alias for `Reg<BLK7_W11_SPEC>`"]
pub type BLK7_W11 = crate::Reg<blk7_w11::BLK7_W11_SPEC>;
#[doc = "Otp debuger block7 data register11."]
pub mod blk7_w11;
#[doc = "BLK8_W1 (r) register accessor: an alias for `Reg<BLK8_W1_SPEC>`"]
pub type BLK8_W1 = crate::Reg<blk8_w1::BLK8_W1_SPEC>;
#[doc = "Otp debuger block8 data register1."]
pub mod blk8_w1;
#[doc = "BLK8_W2 (r) register accessor: an alias for `Reg<BLK8_W2_SPEC>`"]
pub type BLK8_W2 = crate::Reg<blk8_w2::BLK8_W2_SPEC>;
#[doc = "Otp debuger block8 data register2."]
pub mod blk8_w2;
#[doc = "BLK8_W3 (r) register accessor: an alias for `Reg<BLK8_W3_SPEC>`"]
pub type BLK8_W3 = crate::Reg<blk8_w3::BLK8_W3_SPEC>;
#[doc = "Otp debuger block8 data register3."]
pub mod blk8_w3;
#[doc = "BLK8_W4 (r) register accessor: an alias for `Reg<BLK8_W4_SPEC>`"]
pub type BLK8_W4 = crate::Reg<blk8_w4::BLK8_W4_SPEC>;
#[doc = "Otp debuger block8 data register4."]
pub mod blk8_w4;
#[doc = "BLK8_W5 (r) register accessor: an alias for `Reg<BLK8_W5_SPEC>`"]
pub type BLK8_W5 = crate::Reg<blk8_w5::BLK8_W5_SPEC>;
#[doc = "Otp debuger block8 data register5."]
pub mod blk8_w5;
#[doc = "BLK8_W6 (r) register accessor: an alias for `Reg<BLK8_W6_SPEC>`"]
pub type BLK8_W6 = crate::Reg<blk8_w6::BLK8_W6_SPEC>;
#[doc = "Otp debuger block8 data register6."]
pub mod blk8_w6;
#[doc = "BLK8_W7 (r) register accessor: an alias for `Reg<BLK8_W7_SPEC>`"]
pub type BLK8_W7 = crate::Reg<blk8_w7::BLK8_W7_SPEC>;
#[doc = "Otp debuger block8 data register7."]
pub mod blk8_w7;
#[doc = "BLK8_W8 (r) register accessor: an alias for `Reg<BLK8_W8_SPEC>`"]
pub type BLK8_W8 = crate::Reg<blk8_w8::BLK8_W8_SPEC>;
#[doc = "Otp debuger block8 data register8."]
pub mod blk8_w8;
#[doc = "BLK8_W9 (r) register accessor: an alias for `Reg<BLK8_W9_SPEC>`"]
pub type BLK8_W9 = crate::Reg<blk8_w9::BLK8_W9_SPEC>;
#[doc = "Otp debuger block8 data register9."]
pub mod blk8_w9;
#[doc = "BLK8_W10 (r) register accessor: an alias for `Reg<BLK8_W10_SPEC>`"]
pub type BLK8_W10 = crate::Reg<blk8_w10::BLK8_W10_SPEC>;
#[doc = "Otp debuger block8 data register10."]
pub mod blk8_w10;
#[doc = "BLK8_W11 (r) register accessor: an alias for `Reg<BLK8_W11_SPEC>`"]
pub type BLK8_W11 = crate::Reg<blk8_w11::BLK8_W11_SPEC>;
#[doc = "Otp debuger block8 data register11."]
pub mod blk8_w11;
#[doc = "BLK9_W1 (r) register accessor: an alias for `Reg<BLK9_W1_SPEC>`"]
pub type BLK9_W1 = crate::Reg<blk9_w1::BLK9_W1_SPEC>;
#[doc = "Otp debuger block9 data register1."]
pub mod blk9_w1;
#[doc = "BLK9_W2 (r) register accessor: an alias for `Reg<BLK9_W2_SPEC>`"]
pub type BLK9_W2 = crate::Reg<blk9_w2::BLK9_W2_SPEC>;
#[doc = "Otp debuger block9 data register2."]
pub mod blk9_w2;
#[doc = "BLK9_W3 (r) register accessor: an alias for `Reg<BLK9_W3_SPEC>`"]
pub type BLK9_W3 = crate::Reg<blk9_w3::BLK9_W3_SPEC>;
#[doc = "Otp debuger block9 data register3."]
pub mod blk9_w3;
#[doc = "BLK9_W4 (r) register accessor: an alias for `Reg<BLK9_W4_SPEC>`"]
pub type BLK9_W4 = crate::Reg<blk9_w4::BLK9_W4_SPEC>;
#[doc = "Otp debuger block9 data register4."]
pub mod blk9_w4;
#[doc = "BLK9_W5 (r) register accessor: an alias for `Reg<BLK9_W5_SPEC>`"]
pub type BLK9_W5 = crate::Reg<blk9_w5::BLK9_W5_SPEC>;
#[doc = "Otp debuger block9 data register5."]
pub mod blk9_w5;
#[doc = "BLK9_W6 (r) register accessor: an alias for `Reg<BLK9_W6_SPEC>`"]
pub type BLK9_W6 = crate::Reg<blk9_w6::BLK9_W6_SPEC>;
#[doc = "Otp debuger block9 data register6."]
pub mod blk9_w6;
#[doc = "BLK9_W7 (r) register accessor: an alias for `Reg<BLK9_W7_SPEC>`"]
pub type BLK9_W7 = crate::Reg<blk9_w7::BLK9_W7_SPEC>;
#[doc = "Otp debuger block9 data register7."]
pub mod blk9_w7;
#[doc = "BLK9_W8 (r) register accessor: an alias for `Reg<BLK9_W8_SPEC>`"]
pub type BLK9_W8 = crate::Reg<blk9_w8::BLK9_W8_SPEC>;
#[doc = "Otp debuger block9 data register8."]
pub mod blk9_w8;
#[doc = "BLK9_W9 (r) register accessor: an alias for `Reg<BLK9_W9_SPEC>`"]
pub type BLK9_W9 = crate::Reg<blk9_w9::BLK9_W9_SPEC>;
#[doc = "Otp debuger block9 data register9."]
pub mod blk9_w9;
#[doc = "BLK9_W10 (r) register accessor: an alias for `Reg<BLK9_W10_SPEC>`"]
pub type BLK9_W10 = crate::Reg<blk9_w10::BLK9_W10_SPEC>;
#[doc = "Otp debuger block9 data register10."]
pub mod blk9_w10;
#[doc = "BLK9_W11 (r) register accessor: an alias for `Reg<BLK9_W11_SPEC>`"]
pub type BLK9_W11 = crate::Reg<blk9_w11::BLK9_W11_SPEC>;
#[doc = "Otp debuger block9 data register11."]
pub mod blk9_w11;
#[doc = "BLK10_W1 (r) register accessor: an alias for `Reg<BLK10_W1_SPEC>`"]
pub type BLK10_W1 = crate::Reg<blk10_w1::BLK10_W1_SPEC>;
#[doc = "Otp debuger block10 data register1."]
pub mod blk10_w1;
#[doc = "BLK10_W2 (r) register accessor: an alias for `Reg<BLK10_W2_SPEC>`"]
pub type BLK10_W2 = crate::Reg<blk10_w2::BLK10_W2_SPEC>;
#[doc = "Otp debuger block10 data register2."]
pub mod blk10_w2;
#[doc = "BLK10_W3 (r) register accessor: an alias for `Reg<BLK10_W3_SPEC>`"]
pub type BLK10_W3 = crate::Reg<blk10_w3::BLK10_W3_SPEC>;
#[doc = "Otp debuger block10 data register3."]
pub mod blk10_w3;
#[doc = "BLK10_W4 (r) register accessor: an alias for `Reg<BLK10_W4_SPEC>`"]
pub type BLK10_W4 = crate::Reg<blk10_w4::BLK10_W4_SPEC>;
#[doc = "Otp debuger block10 data register4."]
pub mod blk10_w4;
#[doc = "BLK10_W5 (r) register accessor: an alias for `Reg<BLK10_W5_SPEC>`"]
pub type BLK10_W5 = crate::Reg<blk10_w5::BLK10_W5_SPEC>;
#[doc = "Otp debuger block10 data register5."]
pub mod blk10_w5;
#[doc = "BLK10_W6 (r) register accessor: an alias for `Reg<BLK10_W6_SPEC>`"]
pub type BLK10_W6 = crate::Reg<blk10_w6::BLK10_W6_SPEC>;
#[doc = "Otp debuger block10 data register6."]
pub mod blk10_w6;
#[doc = "BLK10_W7 (r) register accessor: an alias for `Reg<BLK10_W7_SPEC>`"]
pub type BLK10_W7 = crate::Reg<blk10_w7::BLK10_W7_SPEC>;
#[doc = "Otp debuger block10 data register7."]
pub mod blk10_w7;
#[doc = "BLK10_W8 (r) register accessor: an alias for `Reg<BLK10_W8_SPEC>`"]
pub type BLK10_W8 = crate::Reg<blk10_w8::BLK10_W8_SPEC>;
#[doc = "Otp debuger block10 data register8."]
pub mod blk10_w8;
#[doc = "BLK10_W9 (r) register accessor: an alias for `Reg<BLK10_W9_SPEC>`"]
pub type BLK10_W9 = crate::Reg<blk10_w9::BLK10_W9_SPEC>;
#[doc = "Otp debuger block10 data register9."]
pub mod blk10_w9;
#[doc = "BLK10_W10 (r) register accessor: an alias for `Reg<BLK10_W10_SPEC>`"]
pub type BLK10_W10 = crate::Reg<blk10_w10::BLK10_W10_SPEC>;
#[doc = "Otp debuger block10 data register10."]
pub mod blk10_w10;
#[doc = "BLK10_W11 (r) register accessor: an alias for `Reg<BLK10_W11_SPEC>`"]
pub type BLK10_W11 = crate::Reg<blk10_w11::BLK10_W11_SPEC>;
#[doc = "Otp debuger block10 data register11."]
pub mod blk10_w11;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Otp debuger clk_en configuration register."]
pub mod clk;
#[doc = "APB2OTP_EN (rw) register accessor: an alias for `Reg<APB2OTP_EN_SPEC>`"]
pub type APB2OTP_EN = crate::Reg<apb2otp_en::APB2OTP_EN_SPEC>;
#[doc = "Otp_debuger apb2otp enable configuration register."]
pub mod apb2otp_en;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "eFuse version register."]
pub mod date;
