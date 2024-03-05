#[doc = "Register `SPI_MISC` reader"]
pub type R = crate::R<SPI_MISC_SPEC>;
#[doc = "Register `SPI_MISC` writer"]
pub type W = crate::W<SPI_MISC_SPEC>;
#[doc = "Field `SPI_CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub type SPI_CS0_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub type SPI_CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub type SPI_CS1_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub type SPI_CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub type SPI_CS2_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub type SPI_CS2_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS3_DIS` reader - SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state."]
pub type SPI_CS3_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS3_DIS` writer - SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state."]
pub type SPI_CS3_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS4_DIS` reader - SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state."]
pub type SPI_CS4_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS4_DIS` writer - SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state."]
pub type SPI_CS4_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS5_DIS` reader - SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state."]
pub type SPI_CS5_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS5_DIS` writer - SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state."]
pub type SPI_CS5_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type SPI_CK_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type SPI_CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type SPI_MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `SPI_MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type SPI_MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLK_DATA_DTR_EN` reader - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub type SPI_CLK_DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_CLK_DATA_DTR_EN` writer - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub type SPI_CLK_DATA_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DATA_DTR_EN` reader - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub type SPI_DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_DATA_DTR_EN` writer - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub type SPI_DATA_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_ADDR_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub type SPI_ADDR_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_ADDR_DTR_EN` writer - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub type SPI_ADDR_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CMD_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub type SPI_CMD_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_CMD_DTR_EN` writer - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub type SPI_CMD_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SPI_SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `SPI_SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SPI_SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DQS_IDLE_EDGE` reader - The default value of spi_dqs. Can be configured in CONF state."]
pub type SPI_DQS_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_DQS_IDLE_EDGE` writer - The default value of spi_dqs. Can be configured in CONF state."]
pub type SPI_DQS_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type SPI_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type SPI_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type SPI_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type SPI_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_QUAD_DIN_PIN_SWAP` reader - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
pub type SPI_QUAD_DIN_PIN_SWAP_R = crate::BitReader;
#[doc = "Field `SPI_QUAD_DIN_PIN_SWAP` writer - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
pub type SPI_QUAD_DIN_PIN_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> SPI_CS0_DIS_R {
        SPI_CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> SPI_CS1_DIS_R {
        SPI_CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> SPI_CS2_DIS_R {
        SPI_CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs3_dis(&self) -> SPI_CS3_DIS_R {
        SPI_CS3_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs4_dis(&self) -> SPI_CS4_DIS_R {
        SPI_CS4_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs5_dis(&self) -> SPI_CS5_DIS_R {
        SPI_CS5_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_dis(&self) -> SPI_CK_DIS_R {
        SPI_CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_master_cs_pol(&self) -> SPI_MASTER_CS_POL_R {
        SPI_MASTER_CS_POL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    pub fn spi_clk_data_dtr_en(&self) -> SPI_CLK_DATA_DTR_EN_R {
        SPI_CLK_DATA_DTR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_data_dtr_en(&self) -> SPI_DATA_DTR_EN_R {
        SPI_DATA_DTR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_addr_dtr_en(&self) -> SPI_ADDR_DTR_EN_R {
        SPI_ADDR_DTR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cmd_dtr_en(&self) -> SPI_CMD_DTR_EN_R {
        SPI_CMD_DTR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&self) -> SPI_SLAVE_CS_POL_R {
        SPI_SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dqs_idle_edge(&self) -> SPI_DQS_IDLE_EDGE_R {
        SPI_DQS_IDLE_EDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&self) -> SPI_CK_IDLE_EDGE_R {
        SPI_CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_keep_active(&self) -> SPI_CS_KEEP_ACTIVE_R {
        SPI_CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_quad_din_pin_swap(&self) -> SPI_QUAD_DIN_PIN_SWAP_R {
        SPI_QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MISC")
            .field("spi_cs0_dis", &format_args!("{}", self.spi_cs0_dis().bit()))
            .field("spi_cs1_dis", &format_args!("{}", self.spi_cs1_dis().bit()))
            .field("spi_cs2_dis", &format_args!("{}", self.spi_cs2_dis().bit()))
            .field("spi_cs3_dis", &format_args!("{}", self.spi_cs3_dis().bit()))
            .field("spi_cs4_dis", &format_args!("{}", self.spi_cs4_dis().bit()))
            .field("spi_cs5_dis", &format_args!("{}", self.spi_cs5_dis().bit()))
            .field("spi_ck_dis", &format_args!("{}", self.spi_ck_dis().bit()))
            .field(
                "spi_master_cs_pol",
                &format_args!("{}", self.spi_master_cs_pol().bits()),
            )
            .field(
                "spi_clk_data_dtr_en",
                &format_args!("{}", self.spi_clk_data_dtr_en().bit()),
            )
            .field(
                "spi_data_dtr_en",
                &format_args!("{}", self.spi_data_dtr_en().bit()),
            )
            .field(
                "spi_addr_dtr_en",
                &format_args!("{}", self.spi_addr_dtr_en().bit()),
            )
            .field(
                "spi_cmd_dtr_en",
                &format_args!("{}", self.spi_cmd_dtr_en().bit()),
            )
            .field(
                "spi_slave_cs_pol",
                &format_args!("{}", self.spi_slave_cs_pol().bit()),
            )
            .field(
                "spi_dqs_idle_edge",
                &format_args!("{}", self.spi_dqs_idle_edge().bit()),
            )
            .field(
                "spi_ck_idle_edge",
                &format_args!("{}", self.spi_ck_idle_edge().bit()),
            )
            .field(
                "spi_cs_keep_active",
                &format_args!("{}", self.spi_cs_keep_active().bit()),
            )
            .field(
                "spi_quad_din_pin_swap",
                &format_args!("{}", self.spi_quad_din_pin_swap().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs0_dis(&mut self) -> SPI_CS0_DIS_W<SPI_MISC_SPEC> {
        SPI_CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs1_dis(&mut self) -> SPI_CS1_DIS_W<SPI_MISC_SPEC> {
        SPI_CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs2_dis(&mut self) -> SPI_CS2_DIS_W<SPI_MISC_SPEC> {
        SPI_CS2_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs3_dis(&mut self) -> SPI_CS3_DIS_W<SPI_MISC_SPEC> {
        SPI_CS3_DIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs4_dis(&mut self) -> SPI_CS4_DIS_W<SPI_MISC_SPEC> {
        SPI_CS4_DIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs5_dis(&mut self) -> SPI_CS5_DIS_W<SPI_MISC_SPEC> {
        SPI_CS5_DIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ck_dis(&mut self) -> SPI_CK_DIS_W<SPI_MISC_SPEC> {
        SPI_CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_master_cs_pol(&mut self) -> SPI_MASTER_CS_POL_W<SPI_MISC_SPEC> {
        SPI_MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_data_dtr_en(&mut self) -> SPI_CLK_DATA_DTR_EN_W<SPI_MISC_SPEC> {
        SPI_CLK_DATA_DTR_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_data_dtr_en(&mut self) -> SPI_DATA_DTR_EN_W<SPI_MISC_SPEC> {
        SPI_DATA_DTR_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_addr_dtr_en(&mut self) -> SPI_ADDR_DTR_EN_W<SPI_MISC_SPEC> {
        SPI_ADDR_DTR_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cmd_dtr_en(&mut self) -> SPI_CMD_DTR_EN_W<SPI_MISC_SPEC> {
        SPI_CMD_DTR_EN_W::new(self, 19)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slave_cs_pol(&mut self) -> SPI_SLAVE_CS_POL_W<SPI_MISC_SPEC> {
        SPI_SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dqs_idle_edge(&mut self) -> SPI_DQS_IDLE_EDGE_W<SPI_MISC_SPEC> {
        SPI_DQS_IDLE_EDGE_W::new(self, 24)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ck_idle_edge(&mut self) -> SPI_CK_IDLE_EDGE_W<SPI_MISC_SPEC> {
        SPI_CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_keep_active(&mut self) -> SPI_CS_KEEP_ACTIVE_W<SPI_MISC_SPEC> {
        SPI_CS_KEEP_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_quad_din_pin_swap(&mut self) -> SPI_QUAD_DIN_PIN_SWAP_W<SPI_MISC_SPEC> {
        SPI_QUAD_DIN_PIN_SWAP_W::new(self, 31)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MISC_SPEC;
impl crate::RegisterSpec for SPI_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_misc::R`](R) reader structure"]
impl crate::Readable for SPI_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_misc::W`](W) writer structure"]
impl crate::Writable for SPI_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MISC to value 0x3e"]
impl crate::Resettable for SPI_MISC_SPEC {
    const RESET_VALUE: u32 = 0x3e;
}
