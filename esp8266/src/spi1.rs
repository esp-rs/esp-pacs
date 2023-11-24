#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    spi_cmd: SPI_CMD,
    spi_addr: SPI_ADDR,
    spi_ctrl: SPI_CTRL,
    spi_ctrl1: SPI_CTRL1,
    spi_rd_status: SPI_RD_STATUS,
    spi_ctrl2: SPI_CTRL2,
    spi_clock: SPI_CLOCK,
    spi_user: SPI_USER,
    spi_user1: SPI_USER1,
    spi_user2: SPI_USER2,
    spi_wr_status: SPI_WR_STATUS,
    spi_pin: SPI_PIN,
    spi_slave: SPI_SLAVE,
    spi_slave1: SPI_SLAVE1,
    spi_slave2: SPI_SLAVE2,
    spi_slave3: SPI_SLAVE3,
    spi_w0: SPI_W0,
    spi_w1: SPI_W1,
    spi_w2: SPI_W2,
    spi_w3: SPI_W3,
    spi_w4: SPI_W4,
    spi_w5: SPI_W5,
    spi_w6: SPI_W6,
    spi_w7: SPI_W7,
    spi_w8: SPI_W8,
    spi_w9: SPI_W9,
    spi_w10: SPI_W10,
    spi_w11: SPI_W11,
    spi_w12: SPI_W12,
    spi_w13: SPI_W13,
    spi_w14: SPI_W14,
    spi_w15: SPI_W15,
    _reserved32: [u8; 0x70],
    spi_ext0: SPI_EXT0,
    spi_ext1: SPI_EXT1,
    spi_ext2: SPI_EXT2,
    spi_ext3: SPI_EXT3,
}
impl RegisterBlock {
    #[doc = "0x00 - In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
    #[inline(always)]
    pub const fn spi_cmd(&self) -> &SPI_CMD {
        &self.spi_cmd
    }
    #[doc = "0x04 - In the master mode, it is the value of address in \"address\" phase."]
    #[inline(always)]
    pub const fn spi_addr(&self) -> &SPI_ADDR {
        &self.spi_addr
    }
    #[doc = "0x08 - SPI_CTRL"]
    #[inline(always)]
    pub const fn spi_ctrl(&self) -> &SPI_CTRL {
        &self.spi_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn spi_ctrl1(&self) -> &SPI_CTRL1 {
        &self.spi_ctrl1
    }
    #[doc = "0x10 - In the slave mode, this register are the status register for the master to read out."]
    #[inline(always)]
    pub const fn spi_rd_status(&self) -> &SPI_RD_STATUS {
        &self.spi_rd_status
    }
    #[doc = "0x14 - spi_cs signal is delayed by 80MHz clock cycles"]
    #[inline(always)]
    pub const fn spi_ctrl2(&self) -> &SPI_CTRL2 {
        &self.spi_ctrl2
    }
    #[doc = "0x18 - In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
    #[inline(always)]
    pub const fn spi_clock(&self) -> &SPI_CLOCK {
        &self.spi_clock
    }
    #[doc = "0x1c - This bit enable the \"command\" phase of an operation."]
    #[inline(always)]
    pub const fn spi_user(&self) -> &SPI_USER {
        &self.spi_user
    }
    #[doc = "0x20 - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub const fn spi_user1(&self) -> &SPI_USER1 {
        &self.spi_user1
    }
    #[doc = "0x24 - The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub const fn spi_user2(&self) -> &SPI_USER2 {
        &self.spi_user2
    }
    #[doc = "0x28 - In the slave mode, this register are the status register for the master to write into."]
    #[inline(always)]
    pub const fn spi_wr_status(&self) -> &SPI_WR_STATUS {
        &self.spi_wr_status
    }
    #[doc = "0x2c - 1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
    #[inline(always)]
    pub const fn spi_pin(&self) -> &SPI_PIN {
        &self.spi_pin
    }
    #[doc = "0x30 - It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
    #[inline(always)]
    pub const fn spi_slave(&self) -> &SPI_SLAVE {
        &self.spi_slave
    }
    #[doc = "0x34 - In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
    #[inline(always)]
    pub const fn spi_slave1(&self) -> &SPI_SLAVE1 {
        &self.spi_slave1
    }
    #[doc = "0x38 - In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
    #[inline(always)]
    pub const fn spi_slave2(&self) -> &SPI_SLAVE2 {
        &self.spi_slave2
    }
    #[doc = "0x3c - In slave mode, it is the value of \"write-status\" command"]
    #[inline(always)]
    pub const fn spi_slave3(&self) -> &SPI_SLAVE3 {
        &self.spi_slave3
    }
    #[doc = "0x40 - the data inside the buffer of the SPI module, word 0"]
    #[inline(always)]
    pub const fn spi_w0(&self) -> &SPI_W0 {
        &self.spi_w0
    }
    #[doc = "0x44 - the data inside the buffer of the SPI module, word 1"]
    #[inline(always)]
    pub const fn spi_w1(&self) -> &SPI_W1 {
        &self.spi_w1
    }
    #[doc = "0x48 - the data inside the buffer of the SPI module, word 2"]
    #[inline(always)]
    pub const fn spi_w2(&self) -> &SPI_W2 {
        &self.spi_w2
    }
    #[doc = "0x4c - the data inside the buffer of the SPI module, word 3"]
    #[inline(always)]
    pub const fn spi_w3(&self) -> &SPI_W3 {
        &self.spi_w3
    }
    #[doc = "0x50 - the data inside the buffer of the SPI module, word 4"]
    #[inline(always)]
    pub const fn spi_w4(&self) -> &SPI_W4 {
        &self.spi_w4
    }
    #[doc = "0x54 - the data inside the buffer of the SPI module, word 5"]
    #[inline(always)]
    pub const fn spi_w5(&self) -> &SPI_W5 {
        &self.spi_w5
    }
    #[doc = "0x58 - the data inside the buffer of the SPI module, word 6"]
    #[inline(always)]
    pub const fn spi_w6(&self) -> &SPI_W6 {
        &self.spi_w6
    }
    #[doc = "0x5c - the data inside the buffer of the SPI module, word 7"]
    #[inline(always)]
    pub const fn spi_w7(&self) -> &SPI_W7 {
        &self.spi_w7
    }
    #[doc = "0x60 - the data inside the buffer of the SPI module, word 8"]
    #[inline(always)]
    pub const fn spi_w8(&self) -> &SPI_W8 {
        &self.spi_w8
    }
    #[doc = "0x64 - the data inside the buffer of the SPI module, word 9"]
    #[inline(always)]
    pub const fn spi_w9(&self) -> &SPI_W9 {
        &self.spi_w9
    }
    #[doc = "0x68 - the data inside the buffer of the SPI module, word 10"]
    #[inline(always)]
    pub const fn spi_w10(&self) -> &SPI_W10 {
        &self.spi_w10
    }
    #[doc = "0x6c - the data inside the buffer of the SPI module, word 11"]
    #[inline(always)]
    pub const fn spi_w11(&self) -> &SPI_W11 {
        &self.spi_w11
    }
    #[doc = "0x70 - the data inside the buffer of the SPI module, word 12"]
    #[inline(always)]
    pub const fn spi_w12(&self) -> &SPI_W12 {
        &self.spi_w12
    }
    #[doc = "0x74 - the data inside the buffer of the SPI module, word 13"]
    #[inline(always)]
    pub const fn spi_w13(&self) -> &SPI_W13 {
        &self.spi_w13
    }
    #[doc = "0x78 - the data inside the buffer of the SPI module, word 14"]
    #[inline(always)]
    pub const fn spi_w14(&self) -> &SPI_W14 {
        &self.spi_w14
    }
    #[doc = "0x7c - the data inside the buffer of the SPI module, word 15"]
    #[inline(always)]
    pub const fn spi_w15(&self) -> &SPI_W15 {
        &self.spi_w15
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn spi_ext0(&self) -> &SPI_EXT0 {
        &self.spi_ext0
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn spi_ext1(&self) -> &SPI_EXT1 {
        &self.spi_ext1
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn spi_ext2(&self) -> &SPI_EXT2 {
        &self.spi_ext2
    }
    #[doc = "0xfc - This register is for two SPI masters to share the same cs, clock and data signals."]
    #[inline(always)]
    pub const fn spi_ext3(&self) -> &SPI_EXT3 {
        &self.spi_ext3
    }
}
#[doc = "SPI_CMD (rw) register accessor: In the master mode, it is the start bit of a single operation. Self-clear by hardware\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cmd`] module"]
pub type SPI_CMD = crate::Reg<spi_cmd::SPI_CMD_SPEC>;
#[doc = "In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
pub mod spi_cmd;
#[doc = "SPI_ADDR (rw) register accessor: In the master mode, it is the value of address in \"address\" phase.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_addr`] module"]
pub type SPI_ADDR = crate::Reg<spi_addr::SPI_ADDR_SPEC>;
#[doc = "In the master mode, it is the value of address in \"address\" phase."]
pub mod spi_addr;
#[doc = "SPI_CTRL (rw) register accessor: SPI_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl`] module"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "SPI_CTRL"]
pub mod spi_ctrl;
#[doc = "SPI_RD_STATUS (rw) register accessor: In the slave mode, this register are the status register for the master to read out.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rd_status`] module"]
pub type SPI_RD_STATUS = crate::Reg<spi_rd_status::SPI_RD_STATUS_SPEC>;
#[doc = "In the slave mode, this register are the status register for the master to read out."]
pub mod spi_rd_status;
#[doc = "SPI_CTRL2 (rw) register accessor: spi_cs signal is delayed by 80MHz clock cycles\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl2`] module"]
pub type SPI_CTRL2 = crate::Reg<spi_ctrl2::SPI_CTRL2_SPEC>;
#[doc = "spi_cs signal is delayed by 80MHz clock cycles"]
pub mod spi_ctrl2;
#[doc = "SPI_CLOCK (rw) register accessor: In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_clock`] module"]
pub type SPI_CLOCK = crate::Reg<spi_clock::SPI_CLOCK_SPEC>;
#[doc = "In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
pub mod spi_clock;
#[doc = "SPI_USER (rw) register accessor: This bit enable the \"command\" phase of an operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_user`] module"]
pub type SPI_USER = crate::Reg<spi_user::SPI_USER_SPEC>;
#[doc = "This bit enable the \"command\" phase of an operation."]
pub mod spi_user;
#[doc = "SPI_USER1 (rw) register accessor: The length in bits of \"address\" phase. The register value shall be (bit_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_user1`] module"]
pub type SPI_USER1 = crate::Reg<spi_user1::SPI_USER1_SPEC>;
#[doc = "The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
pub mod spi_user1;
#[doc = "SPI_USER2 (rw) register accessor: The length in bits of \"command\" phase. The register value shall be (bit_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_user2`] module"]
pub type SPI_USER2 = crate::Reg<spi_user2::SPI_USER2_SPEC>;
#[doc = "The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
pub mod spi_user2;
#[doc = "SPI_WR_STATUS (rw) register accessor: In the slave mode, this register are the status register for the master to write into.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_wr_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_wr_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_wr_status`] module"]
pub type SPI_WR_STATUS = crate::Reg<spi_wr_status::SPI_WR_STATUS_SPEC>;
#[doc = "In the slave mode, this register are the status register for the master to write into."]
pub mod spi_wr_status;
#[doc = "SPI_PIN (rw) register accessor: 1: disable CS2; 0: spi_cs signal is from/to CS2 pin\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_pin`] module"]
pub type SPI_PIN = crate::Reg<spi_pin::SPI_PIN_SPEC>;
#[doc = "1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
pub mod spi_pin;
#[doc = "SPI_SLAVE (rw) register accessor: It is the synchronous reset signal of the module. This bit is self-cleared by hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_slave`] module"]
pub type SPI_SLAVE = crate::Reg<spi_slave::SPI_SLAVE_SPEC>;
#[doc = "It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
pub mod spi_slave;
#[doc = "SPI_SLAVE1 (rw) register accessor: In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_slave1`] module"]
pub type SPI_SLAVE1 = crate::Reg<spi_slave1::SPI_SLAVE1_SPEC>;
#[doc = "In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
pub mod spi_slave1;
#[doc = "SPI_SLAVE2 (rw) register accessor: In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_slave2`] module"]
pub type SPI_SLAVE2 = crate::Reg<spi_slave2::SPI_SLAVE2_SPEC>;
#[doc = "In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
pub mod spi_slave2;
#[doc = "SPI_SLAVE3 (rw) register accessor: In slave mode, it is the value of \"write-status\" command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_slave3`] module"]
pub type SPI_SLAVE3 = crate::Reg<spi_slave3::SPI_SLAVE3_SPEC>;
#[doc = "In slave mode, it is the value of \"write-status\" command"]
pub mod spi_slave3;
#[doc = "SPI_EXT3 (rw) register accessor: This register is for two SPI masters to share the same cs, clock and data signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ext3`] module"]
pub type SPI_EXT3 = crate::Reg<spi_ext3::SPI_EXT3_SPEC>;
#[doc = "This register is for two SPI masters to share the same cs, clock and data signals."]
pub mod spi_ext3;
#[doc = "SPI_W0 (rw) register accessor: the data inside the buffer of the SPI module, word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w0`] module"]
pub type SPI_W0 = crate::Reg<spi_w0::SPI_W0_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 0"]
pub mod spi_w0;
#[doc = "SPI_W1 (rw) register accessor: the data inside the buffer of the SPI module, word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w1`] module"]
pub type SPI_W1 = crate::Reg<spi_w1::SPI_W1_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 1"]
pub mod spi_w1;
#[doc = "SPI_W2 (rw) register accessor: the data inside the buffer of the SPI module, word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w2`] module"]
pub type SPI_W2 = crate::Reg<spi_w2::SPI_W2_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 2"]
pub mod spi_w2;
#[doc = "SPI_W3 (rw) register accessor: the data inside the buffer of the SPI module, word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w3`] module"]
pub type SPI_W3 = crate::Reg<spi_w3::SPI_W3_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 3"]
pub mod spi_w3;
#[doc = "SPI_W4 (rw) register accessor: the data inside the buffer of the SPI module, word 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w4`] module"]
pub type SPI_W4 = crate::Reg<spi_w4::SPI_W4_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 4"]
pub mod spi_w4;
#[doc = "SPI_W5 (rw) register accessor: the data inside the buffer of the SPI module, word 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w5`] module"]
pub type SPI_W5 = crate::Reg<spi_w5::SPI_W5_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 5"]
pub mod spi_w5;
#[doc = "SPI_W6 (rw) register accessor: the data inside the buffer of the SPI module, word 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w6`] module"]
pub type SPI_W6 = crate::Reg<spi_w6::SPI_W6_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 6"]
pub mod spi_w6;
#[doc = "SPI_W7 (rw) register accessor: the data inside the buffer of the SPI module, word 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w7`] module"]
pub type SPI_W7 = crate::Reg<spi_w7::SPI_W7_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 7"]
pub mod spi_w7;
#[doc = "SPI_W8 (rw) register accessor: the data inside the buffer of the SPI module, word 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w8`] module"]
pub type SPI_W8 = crate::Reg<spi_w8::SPI_W8_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 8"]
pub mod spi_w8;
#[doc = "SPI_W9 (rw) register accessor: the data inside the buffer of the SPI module, word 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w9`] module"]
pub type SPI_W9 = crate::Reg<spi_w9::SPI_W9_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 9"]
pub mod spi_w9;
#[doc = "SPI_W10 (rw) register accessor: the data inside the buffer of the SPI module, word 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w10`] module"]
pub type SPI_W10 = crate::Reg<spi_w10::SPI_W10_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 10"]
pub mod spi_w10;
#[doc = "SPI_W11 (rw) register accessor: the data inside the buffer of the SPI module, word 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w11`] module"]
pub type SPI_W11 = crate::Reg<spi_w11::SPI_W11_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 11"]
pub mod spi_w11;
#[doc = "SPI_W12 (rw) register accessor: the data inside the buffer of the SPI module, word 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w12`] module"]
pub type SPI_W12 = crate::Reg<spi_w12::SPI_W12_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 12"]
pub mod spi_w12;
#[doc = "SPI_W13 (rw) register accessor: the data inside the buffer of the SPI module, word 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w13`] module"]
pub type SPI_W13 = crate::Reg<spi_w13::SPI_W13_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 13"]
pub mod spi_w13;
#[doc = "SPI_W14 (rw) register accessor: the data inside the buffer of the SPI module, word 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w14`] module"]
pub type SPI_W14 = crate::Reg<spi_w14::SPI_W14_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 14"]
pub mod spi_w14;
#[doc = "SPI_W15 (rw) register accessor: the data inside the buffer of the SPI module, word 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_w15`] module"]
pub type SPI_W15 = crate::Reg<spi_w15::SPI_W15_SPEC>;
#[doc = "the data inside the buffer of the SPI module, word 15"]
pub mod spi_w15;
#[doc = "SPI_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl1`] module"]
pub type SPI_CTRL1 = crate::Reg<spi_ctrl1::SPI_CTRL1_SPEC>;
#[doc = ""]
pub mod spi_ctrl1;
#[doc = "SPI_EXT0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ext0`] module"]
pub type SPI_EXT0 = crate::Reg<spi_ext0::SPI_EXT0_SPEC>;
#[doc = ""]
pub mod spi_ext0;
#[doc = "SPI_EXT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ext1`] module"]
pub type SPI_EXT1 = crate::Reg<spi_ext1::SPI_EXT1_SPEC>;
#[doc = ""]
pub mod spi_ext1;
#[doc = "SPI_EXT2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ext2`] module"]
pub type SPI_EXT2 = crate::Reg<spi_ext2::SPI_EXT2_SPEC>;
#[doc = ""]
pub mod spi_ext2;
