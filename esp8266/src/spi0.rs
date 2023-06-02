#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
    pub spi_cmd: SPI_CMD,
    #[doc = "0x04 - In the master mode, it is the value of address in \"address\" phase."]
    pub spi_addr: SPI_ADDR,
    #[doc = "0x08 - SPI_CTRL"]
    pub spi_ctrl: SPI_CTRL,
    #[doc = "0x0c - "]
    pub spi_ctrl1: SPI_CTRL1,
    #[doc = "0x10 - In the slave mode, this register are the status register for the master to read out."]
    pub spi_rd_status: SPI_RD_STATUS,
    #[doc = "0x14 - spi_cs signal is delayed by 80MHz clock cycles"]
    pub spi_ctrl2: SPI_CTRL2,
    #[doc = "0x18 - In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
    pub spi_clock: SPI_CLOCK,
    #[doc = "0x1c - This bit enable the \"command\" phase of an operation."]
    pub spi_user: SPI_USER,
    #[doc = "0x20 - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
    pub spi_user1: SPI_USER1,
    #[doc = "0x24 - The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
    pub spi_user2: SPI_USER2,
    #[doc = "0x28 - In the slave mode, this register are the status register for the master to write into."]
    pub spi_wr_status: SPI_WR_STATUS,
    #[doc = "0x2c - 1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
    pub spi_pin: SPI_PIN,
    #[doc = "0x30 - It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
    pub spi_slave: SPI_SLAVE,
    #[doc = "0x34 - In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
    pub spi_slave1: SPI_SLAVE1,
    #[doc = "0x38 - In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
    pub spi_slave2: SPI_SLAVE2,
    #[doc = "0x3c - In slave mode, it is the value of \"write-status\" command"]
    pub spi_slave3: SPI_SLAVE3,
    #[doc = "0x40 - the data inside the buffer of the SPI module, word 0"]
    pub spi_w0: SPI_W0,
    #[doc = "0x44 - the data inside the buffer of the SPI module, word 1"]
    pub spi_w1: SPI_W1,
    #[doc = "0x48 - the data inside the buffer of the SPI module, word 2"]
    pub spi_w2: SPI_W2,
    #[doc = "0x4c - the data inside the buffer of the SPI module, word 3"]
    pub spi_w3: SPI_W3,
    #[doc = "0x50 - the data inside the buffer of the SPI module, word 4"]
    pub spi_w4: SPI_W4,
    #[doc = "0x54 - the data inside the buffer of the SPI module, word 5"]
    pub spi_w5: SPI_W5,
    #[doc = "0x58 - the data inside the buffer of the SPI module, word 6"]
    pub spi_w6: SPI_W6,
    #[doc = "0x5c - the data inside the buffer of the SPI module, word 7"]
    pub spi_w7: SPI_W7,
    #[doc = "0x60 - the data inside the buffer of the SPI module, word 8"]
    pub spi_w8: SPI_W8,
    #[doc = "0x64 - the data inside the buffer of the SPI module, word 9"]
    pub spi_w9: SPI_W9,
    #[doc = "0x68 - the data inside the buffer of the SPI module, word 10"]
    pub spi_w10: SPI_W10,
    #[doc = "0x6c - the data inside the buffer of the SPI module, word 11"]
    pub spi_w11: SPI_W11,
    #[doc = "0x70 - the data inside the buffer of the SPI module, word 12"]
    pub spi_w12: SPI_W12,
    #[doc = "0x74 - the data inside the buffer of the SPI module, word 13"]
    pub spi_w13: SPI_W13,
    #[doc = "0x78 - the data inside the buffer of the SPI module, word 14"]
    pub spi_w14: SPI_W14,
    #[doc = "0x7c - the data inside the buffer of the SPI module, word 15"]
    pub spi_w15: SPI_W15,
    _reserved32: [u8; 0x70],
    #[doc = "0xf0 - "]
    pub spi_ext0: SPI_EXT0,
    #[doc = "0xf4 - "]
    pub spi_ext1: SPI_EXT1,
    #[doc = "0xf8 - "]
    pub spi_ext2: SPI_EXT2,
    #[doc = "0xfc - This register is for two SPI masters to share the same cs, clock and data signals."]
    pub spi_ext3: SPI_EXT3,
}
#[doc = "SPI_CMD (rw) register accessor: an alias for `Reg<SPI_CMD_SPEC>`"]
pub type SPI_CMD = crate::Reg<spi_cmd::SPI_CMD_SPEC>;
#[doc = "In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
pub mod spi_cmd;
#[doc = "SPI_ADDR (rw) register accessor: an alias for `Reg<SPI_ADDR_SPEC>`"]
pub type SPI_ADDR = crate::Reg<spi_addr::SPI_ADDR_SPEC>;
#[doc = "In the master mode, it is the value of address in \"address\" phase."]
pub mod spi_addr;
#[doc = "SPI_CTRL (rw) register accessor: an alias for `Reg<SPI_CTRL_SPEC>`"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "SPI_CTRL"]
pub mod spi_ctrl;
#[doc = "SPI_RD_STATUS (rw) register accessor: an alias for `Reg<SPI_RD_STATUS_SPEC>`"]
pub type SPI_RD_STATUS = crate::Reg<spi_rd_status::SPI_RD_STATUS_SPEC>;
#[doc = "In the slave mode, this register are the status register for the master to read out."]
pub mod spi_rd_status;
#[doc = "SPI_CTRL2 (rw) register accessor: an alias for `Reg<SPI_CTRL2_SPEC>`"]
pub type SPI_CTRL2 = crate::Reg<spi_ctrl2::SPI_CTRL2_SPEC>;
#[doc = "spi_cs signal is delayed by 80MHz clock cycles"]
pub mod spi_ctrl2;
#[doc = "SPI_CLOCK (rw) register accessor: an alias for `Reg<SPI_CLOCK_SPEC>`"]
pub type SPI_CLOCK = crate::Reg<spi_clock::SPI_CLOCK_SPEC>;
#[doc = "In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
pub mod spi_clock;
#[doc = "SPI_USER (rw) register accessor: an alias for `Reg<SPI_USER_SPEC>`"]
pub type SPI_USER = crate::Reg<spi_user::SPI_USER_SPEC>;
#[doc = "This bit enable the \"command\" phase of an operation."]
pub mod spi_user;
#[doc = "SPI_USER1 (rw) register accessor: an alias for `Reg<SPI_USER1_SPEC>`"]
pub type SPI_USER1 = crate::Reg<spi_user1::SPI_USER1_SPEC>;
#[doc = "The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
pub mod spi_user1;
#[doc = "SPI_USER2 (rw) register accessor: an alias for `Reg<SPI_USER2_SPEC>`"]
pub type SPI_USER2 = crate::Reg<spi_user2::SPI_USER2_SPEC>;
#[doc = "The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
pub mod spi_user2;
#[doc = "SPI_WR_STATUS (rw) register accessor: an alias for `Reg<SPI_WR_STATUS_SPEC>`"]
pub type SPI_WR_STATUS = crate::Reg<spi_wr_status::SPI_WR_STATUS_SPEC>;
#[doc = "In the slave mode, this register are the status register for the master to write into."]
pub mod spi_wr_status;
#[doc = "SPI_PIN (rw) register accessor: an alias for `Reg<SPI_PIN_SPEC>`"]
pub type SPI_PIN = crate::Reg<spi_pin::SPI_PIN_SPEC>;
#[doc = "1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
pub mod spi_pin;
#[doc = "SPI_SLAVE (rw) register accessor: an alias for `Reg<SPI_SLAVE_SPEC>`"]
pub type SPI_SLAVE = crate::Reg<spi_slave::SPI_SLAVE_SPEC>;
#[doc = "It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
pub mod spi_slave;
#[doc = "SPI_SLAVE1 (rw) register accessor: an alias for `Reg<SPI_SLAVE1_SPEC>`"]
pub type SPI_SLAVE1 = crate::Reg<spi_slave1::SPI_SLAVE1_SPEC>;
#[doc = "In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
pub mod spi_slave1;
#[doc = "SPI_SLAVE2 (rw) register accessor: an alias for `Reg<SPI_SLAVE2_SPEC>`"]
pub type SPI_SLAVE2 = crate::Reg<spi_slave2::SPI_SLAVE2_SPEC>;
#[doc = "In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
pub mod spi_slave2;
#[doc = "SPI_SLAVE3 (rw) register accessor: an alias for `Reg<SPI_SLAVE3_SPEC>`"]
pub type SPI_SLAVE3 = crate::Reg<spi_slave3::SPI_SLAVE3_SPEC>;
#[doc = "In slave mode, it is the value of \"write-status\" command"]
pub mod spi_slave3;
#[doc = "SPI_EXT3 (rw) register accessor: an alias for `Reg<SPI_EXT3_SPEC>`"]
pub type SPI_EXT3 = crate::Reg<spi_ext3::SPI_EXT3_SPEC>;
#[doc = "This register is for two SPI masters to share the same cs, clock and data signals."]
pub mod spi_ext3;
#[doc = "SPI_W0 (rw) register accessor: an alias for `Reg<SPI_W0_SPEC>`"]
pub type SPI_W0 = crate::Reg<spi_w0::SPI_W0_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 0"]
pub mod spi_w0;
#[doc = "SPI_W1 (rw) register accessor: an alias for `Reg<SPI_W1_SPEC>`"]
pub type SPI_W1 = crate::Reg<spi_w1::SPI_W1_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 1"]
pub mod spi_w1;
#[doc = "SPI_W2 (rw) register accessor: an alias for `Reg<SPI_W2_SPEC>`"]
pub type SPI_W2 = crate::Reg<spi_w2::SPI_W2_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 2"]
pub mod spi_w2;
#[doc = "SPI_W3 (rw) register accessor: an alias for `Reg<SPI_W3_SPEC>`"]
pub type SPI_W3 = crate::Reg<spi_w3::SPI_W3_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 3"]
pub mod spi_w3;
#[doc = "SPI_W4 (rw) register accessor: an alias for `Reg<SPI_W4_SPEC>`"]
pub type SPI_W4 = crate::Reg<spi_w4::SPI_W4_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 4"]
pub mod spi_w4;
#[doc = "SPI_W5 (rw) register accessor: an alias for `Reg<SPI_W5_SPEC>`"]
pub type SPI_W5 = crate::Reg<spi_w5::SPI_W5_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 5"]
pub mod spi_w5;
#[doc = "SPI_W6 (rw) register accessor: an alias for `Reg<SPI_W6_SPEC>`"]
pub type SPI_W6 = crate::Reg<spi_w6::SPI_W6_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 6"]
pub mod spi_w6;
#[doc = "SPI_W7 (rw) register accessor: an alias for `Reg<SPI_W7_SPEC>`"]
pub type SPI_W7 = crate::Reg<spi_w7::SPI_W7_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 7"]
pub mod spi_w7;
#[doc = "SPI_W8 (rw) register accessor: an alias for `Reg<SPI_W8_SPEC>`"]
pub type SPI_W8 = crate::Reg<spi_w8::SPI_W8_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 8"]
pub mod spi_w8;
#[doc = "SPI_W9 (rw) register accessor: an alias for `Reg<SPI_W9_SPEC>`"]
pub type SPI_W9 = crate::Reg<spi_w9::SPI_W9_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 9"]
pub mod spi_w9;
#[doc = "SPI_W10 (rw) register accessor: an alias for `Reg<SPI_W10_SPEC>`"]
pub type SPI_W10 = crate::Reg<spi_w10::SPI_W10_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 10"]
pub mod spi_w10;
#[doc = "SPI_W11 (rw) register accessor: an alias for `Reg<SPI_W11_SPEC>`"]
pub type SPI_W11 = crate::Reg<spi_w11::SPI_W11_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 11"]
pub mod spi_w11;
#[doc = "SPI_W12 (rw) register accessor: an alias for `Reg<SPI_W12_SPEC>`"]
pub type SPI_W12 = crate::Reg<spi_w12::SPI_W12_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 12"]
pub mod spi_w12;
#[doc = "SPI_W13 (rw) register accessor: an alias for `Reg<SPI_W13_SPEC>`"]
pub type SPI_W13 = crate::Reg<spi_w13::SPI_W13_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 13"]
pub mod spi_w13;
#[doc = "SPI_W14 (rw) register accessor: an alias for `Reg<SPI_W14_SPEC>`"]
pub type SPI_W14 = crate::Reg<spi_w14::SPI_W14_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 14"]
pub mod spi_w14;
#[doc = "SPI_W15 (rw) register accessor: an alias for `Reg<SPI_W15_SPEC>`"]
pub type SPI_W15 = crate::Reg<spi_w15::SPI_W15_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 15"]
pub mod spi_w15;
#[doc = "SPI_CTRL1 (rw) register accessor: an alias for `Reg<SPI_CTRL1_SPEC>`"]
pub type SPI_CTRL1 = crate::Reg<spi_ctrl1::SPI_CTRL1_SPEC>;
#[doc = ""]
pub mod spi_ctrl1;
#[doc = "SPI_EXT0 (rw) register accessor: an alias for `Reg<SPI_EXT0_SPEC>`"]
pub type SPI_EXT0 = crate::Reg<spi_ext0::SPI_EXT0_SPEC>;
#[doc = ""]
pub mod spi_ext0;
#[doc = "SPI_EXT1 (rw) register accessor: an alias for `Reg<SPI_EXT1_SPEC>`"]
pub type SPI_EXT1 = crate::Reg<spi_ext1::SPI_EXT1_SPEC>;
#[doc = ""]
pub mod spi_ext1;
#[doc = "SPI_EXT2 (rw) register accessor: an alias for `Reg<SPI_EXT2_SPEC>`"]
pub type SPI_EXT2 = crate::Reg<spi_ext2::SPI_EXT2_SPEC>;
#[doc = ""]
pub mod spi_ext2;
