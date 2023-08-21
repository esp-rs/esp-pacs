#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - IO_MUX_CONF"]
    pub io_mux_conf: IO_MUX_CONF,
    #[doc = "0x04 - IO_MUX_MTDI"]
    pub io_mux_mtdi: IO_MUX_MTDI,
    #[doc = "0x08 - IO_MUX_MTCK"]
    pub io_mux_mtck: IO_MUX_MTCK,
    #[doc = "0x0c - IO_MUX_MTMS"]
    pub io_mux_mtms: IO_MUX_MTMS,
    #[doc = "0x10 - IO_MUX_MTDO"]
    pub io_mux_mtdo: IO_MUX_MTDO,
    #[doc = "0x14 - IO_MUX_U0RXD"]
    pub io_mux_u0rxd: IO_MUX_U0RXD,
    #[doc = "0x18 - IO_MUX_U0TXD"]
    pub io_mux_u0txd: IO_MUX_U0TXD,
    #[doc = "0x1c - IO_MUX_SD_CLK"]
    pub io_mux_sd_clk: IO_MUX_SD_CLK,
    #[doc = "0x20 - IO_MUX_SD_DATA0"]
    pub io_mux_sd_data0: IO_MUX_SD_DATA0,
    #[doc = "0x24 - IO_MUX_SD_DATA1"]
    pub io_mux_sd_data1: IO_MUX_SD_DATA1,
    #[doc = "0x28 - IO_MUX_SD_DATA2"]
    pub io_mux_sd_data2: IO_MUX_SD_DATA2,
    #[doc = "0x2c - IO_MUX_SD_DATA3"]
    pub io_mux_sd_data3: IO_MUX_SD_DATA3,
    #[doc = "0x30 - IO_MUX_SD_CMD"]
    pub io_mux_sd_cmd: IO_MUX_SD_CMD,
    #[doc = "0x34 - IO_MUX_GPIO0"]
    pub io_mux_gpio0: IO_MUX_GPIO0,
    #[doc = "0x38 - IO_MUX_GPIO2"]
    pub io_mux_gpio2: IO_MUX_GPIO2,
    #[doc = "0x3c - IO_MUX_GPIO4"]
    pub io_mux_gpio4: IO_MUX_GPIO4,
    #[doc = "0x40 - IO_MUX_GPIO5"]
    pub io_mux_gpio5: IO_MUX_GPIO5,
}
#[doc = "IO_MUX_CONF (rw) register accessor: IO_MUX_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_conf`] module"]
pub type IO_MUX_CONF = crate::Reg<io_mux_conf::IO_MUX_CONF_SPEC>;
#[doc = "IO_MUX_CONF"]
pub mod io_mux_conf;
#[doc = "IO_MUX_MTDI (rw) register accessor: IO_MUX_MTDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_mtdi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_mtdi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_mtdi`] module"]
pub type IO_MUX_MTDI = crate::Reg<io_mux_mtdi::IO_MUX_MTDI_SPEC>;
#[doc = "IO_MUX_MTDI"]
pub mod io_mux_mtdi;
#[doc = "IO_MUX_MTCK (rw) register accessor: IO_MUX_MTCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_mtck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_mtck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_mtck`] module"]
pub type IO_MUX_MTCK = crate::Reg<io_mux_mtck::IO_MUX_MTCK_SPEC>;
#[doc = "IO_MUX_MTCK"]
pub mod io_mux_mtck;
#[doc = "IO_MUX_MTMS (rw) register accessor: IO_MUX_MTMS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_mtms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_mtms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_mtms`] module"]
pub type IO_MUX_MTMS = crate::Reg<io_mux_mtms::IO_MUX_MTMS_SPEC>;
#[doc = "IO_MUX_MTMS"]
pub mod io_mux_mtms;
#[doc = "IO_MUX_MTDO (rw) register accessor: IO_MUX_MTDO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_mtdo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_mtdo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_mtdo`] module"]
pub type IO_MUX_MTDO = crate::Reg<io_mux_mtdo::IO_MUX_MTDO_SPEC>;
#[doc = "IO_MUX_MTDO"]
pub mod io_mux_mtdo;
#[doc = "IO_MUX_U0RXD (rw) register accessor: IO_MUX_U0RXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_u0rxd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_u0rxd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_u0rxd`] module"]
pub type IO_MUX_U0RXD = crate::Reg<io_mux_u0rxd::IO_MUX_U0RXD_SPEC>;
#[doc = "IO_MUX_U0RXD"]
pub mod io_mux_u0rxd;
#[doc = "IO_MUX_U0TXD (rw) register accessor: IO_MUX_U0TXD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_u0txd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_u0txd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_u0txd`] module"]
pub type IO_MUX_U0TXD = crate::Reg<io_mux_u0txd::IO_MUX_U0TXD_SPEC>;
#[doc = "IO_MUX_U0TXD"]
pub mod io_mux_u0txd;
#[doc = "IO_MUX_SD_CLK (rw) register accessor: IO_MUX_SD_CLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_sd_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_sd_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_sd_clk`] module"]
pub type IO_MUX_SD_CLK = crate::Reg<io_mux_sd_clk::IO_MUX_SD_CLK_SPEC>;
#[doc = "IO_MUX_SD_CLK"]
pub mod io_mux_sd_clk;
#[doc = "IO_MUX_SD_DATA0 (rw) register accessor: IO_MUX_SD_DATA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_sd_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_sd_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_sd_data0`] module"]
pub type IO_MUX_SD_DATA0 = crate::Reg<io_mux_sd_data0::IO_MUX_SD_DATA0_SPEC>;
#[doc = "IO_MUX_SD_DATA0"]
pub mod io_mux_sd_data0;
#[doc = "IO_MUX_SD_DATA1 (rw) register accessor: IO_MUX_SD_DATA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_sd_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_sd_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_sd_data1`] module"]
pub type IO_MUX_SD_DATA1 = crate::Reg<io_mux_sd_data1::IO_MUX_SD_DATA1_SPEC>;
#[doc = "IO_MUX_SD_DATA1"]
pub mod io_mux_sd_data1;
#[doc = "IO_MUX_SD_DATA2 (rw) register accessor: IO_MUX_SD_DATA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_sd_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_sd_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_sd_data2`] module"]
pub type IO_MUX_SD_DATA2 = crate::Reg<io_mux_sd_data2::IO_MUX_SD_DATA2_SPEC>;
#[doc = "IO_MUX_SD_DATA2"]
pub mod io_mux_sd_data2;
#[doc = "IO_MUX_SD_DATA3 (rw) register accessor: IO_MUX_SD_DATA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_sd_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_sd_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_sd_data3`] module"]
pub type IO_MUX_SD_DATA3 = crate::Reg<io_mux_sd_data3::IO_MUX_SD_DATA3_SPEC>;
#[doc = "IO_MUX_SD_DATA3"]
pub mod io_mux_sd_data3;
#[doc = "IO_MUX_SD_CMD (rw) register accessor: IO_MUX_SD_CMD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_sd_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_sd_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_sd_cmd`] module"]
pub type IO_MUX_SD_CMD = crate::Reg<io_mux_sd_cmd::IO_MUX_SD_CMD_SPEC>;
#[doc = "IO_MUX_SD_CMD"]
pub mod io_mux_sd_cmd;
#[doc = "IO_MUX_GPIO0 (rw) register accessor: IO_MUX_GPIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_gpio0`] module"]
pub type IO_MUX_GPIO0 = crate::Reg<io_mux_gpio0::IO_MUX_GPIO0_SPEC>;
#[doc = "IO_MUX_GPIO0"]
pub mod io_mux_gpio0;
#[doc = "IO_MUX_GPIO2 (rw) register accessor: IO_MUX_GPIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_gpio2`] module"]
pub type IO_MUX_GPIO2 = crate::Reg<io_mux_gpio2::IO_MUX_GPIO2_SPEC>;
#[doc = "IO_MUX_GPIO2"]
pub mod io_mux_gpio2;
#[doc = "IO_MUX_GPIO4 (rw) register accessor: IO_MUX_GPIO4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_gpio4`] module"]
pub type IO_MUX_GPIO4 = crate::Reg<io_mux_gpio4::IO_MUX_GPIO4_SPEC>;
#[doc = "IO_MUX_GPIO4"]
pub mod io_mux_gpio4;
#[doc = "IO_MUX_GPIO5 (rw) register accessor: IO_MUX_GPIO5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux_gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux_gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io_mux_gpio5`] module"]
pub type IO_MUX_GPIO5 = crate::Reg<io_mux_gpio5::IO_MUX_GPIO5_SPEC>;
#[doc = "IO_MUX_GPIO5"]
pub mod io_mux_gpio5;
