#[doc = "Register `SPI_MISC` reader"]
pub type R = crate::R<SPI_MISC_SPEC>;
#[doc = "Register `SPI_MISC` writer"]
pub type W = crate::W<SPI_MISC_SPEC>;
#[doc = "Field `SPI_CS0_DIS` reader - Configures whether or not to disable SPI_CS0 pin.\\\\ 0: SPI_CS0 signal is from/to SPI_CS0 pin.\\\\ 1: Disable SPI_CS0 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS0_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS0_DIS` writer - Configures whether or not to disable SPI_CS0 pin.\\\\ 0: SPI_CS0 signal is from/to SPI_CS0 pin.\\\\ 1: Disable SPI_CS0 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS1_DIS` reader - Configures whether or not to disable SPI_CS1 pin.\\\\ 0: SPI_CS1 signal is from/to SPI_CS1 pin.\\\\ 1: Disable SPI_CS1 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS1_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS1_DIS` writer - Configures whether or not to disable SPI_CS1 pin.\\\\ 0: SPI_CS1 signal is from/to SPI_CS1 pin.\\\\ 1: Disable SPI_CS1 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS2_DIS` reader - Configures whether or not to disable SPI_CS2 pin.\\\\ 0: SPI_CS2 signal is from/to SPI_CS2 pin.\\\\ 1: Disable SPI_CS2 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS2_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS2_DIS` writer - Configures whether or not to disable SPI_CS2 pin.\\\\ 0: SPI_CS2 signal is from/to SPI_CS2 pin.\\\\ 1: Disable SPI_CS2 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS2_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS3_DIS` reader - Configures whether or not to disable SPI_CS3 pin.\\\\ 0: SPI_CS3 signal is from/to SPI_CS3 pin.\\\\ 1: Disable SPI_CS3 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS3_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS3_DIS` writer - Configures whether or not to disable SPI_CS3 pin.\\\\ 0: SPI_CS3 signal is from/to SPI_CS3 pin.\\\\ 1: Disable SPI_CS3 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS3_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS4_DIS` reader - Configures whether or not to disable SPI_CS4 pin.\\\\ 0: SPI_CS4 signal is from/to SPI_CS4 pin.\\\\ 1: Disable SPI_CS4 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS4_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS4_DIS` writer - Configures whether or not to disable SPI_CS4 pin.\\\\ 0: SPI_CS4 signal is from/to SPI_CS4 pin.\\\\ 1: Disable SPI_CS4 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS4_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS5_DIS` reader - Configures whether or not to disable SPI_CS5 pin.\\\\ 0: SPI_CS5 signal is from/to SPI_CS5 pin.\\\\ 1: Disable SPI_CS5 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS5_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS5_DIS` writer - Configures whether or not to disable SPI_CS5 pin.\\\\ 0: SPI_CS5 signal is from/to SPI_CS5 pin.\\\\ 1: Disable SPI_CS5 pin.\\\\ Can be configured in CONF state."]
pub type SPI_CS5_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_DIS` reader - Configures whether or not to disable SPI_CLK output.\\\\ 0: Enable\\\\ 1: Disable\\\\ Can be configured in CONF state."]
pub type SPI_CK_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CK_DIS` writer - Configures whether or not to disable SPI_CLK output.\\\\ 0: Enable\\\\ 1: Disable\\\\ Can be configured in CONF state."]
pub type SPI_CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MASTER_CS_POL` reader - Configures the polarity of SPI_CS7 (7 = 0-5) line in master transfer.\\\\ 0: SPI_CS7 is low active.\\\\ 1: SPI_CS7 is high active.\\\\ Can be configured in CONF state."]
pub type SPI_MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `SPI_MASTER_CS_POL` writer - Configures the polarity of SPI_CS7 (7 = 0-5) line in master transfer.\\\\ 0: SPI_CS7 is low active.\\\\ 1: SPI_CS7 is high active.\\\\ Can be configured in CONF state."]
pub type SPI_MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLK_DATA_DTR_EN` reader - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub type SPI_CLK_DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_DATA_DTR_EN` reader - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub type SPI_DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_ADDR_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub type SPI_ADDR_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_CMD_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub type SPI_CMD_DTR_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLAVE_CS_POL` reader - Configures whether or not invert SPI slave input CS polarity.\\\\ 0: Not change\\\\ 1: Invert\\\\ Can be configured in CONF state."]
pub type SPI_SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `SPI_SLAVE_CS_POL` writer - Configures whether or not invert SPI slave input CS polarity.\\\\ 0: Not change\\\\ 1: Invert\\\\ Can be configured in CONF state."]
pub type SPI_SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DQS_IDLE_EDGE` reader - The default value of spi_dqs. Can be configured in CONF state."]
pub type SPI_DQS_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_CK_IDLE_EDGE` reader - Configures the level of SPI_CLK line when GP-SPI2 is in idle.\\\\ 0: Low\\\\ 1: High\\\\ Can be configured in CONF state."]
pub type SPI_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_CK_IDLE_EDGE` writer - Configures the level of SPI_CLK line when GP-SPI2 is in idle.\\\\ 0: Low\\\\ 1: High\\\\ Can be configured in CONF state."]
pub type SPI_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_KEEP_ACTIVE` reader - Configures whether or not to keep the SPI_CS line low.\\\\ 0: Not keep low\\\\ 1: Keep low\\\\ Can be configured in CONF state."]
pub type SPI_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_CS_KEEP_ACTIVE` writer - Configures whether or not to keep the SPI_CS line low.\\\\ 0: Not keep low\\\\ 1: Keep low\\\\ Can be configured in CONF state."]
pub type SPI_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_QUAD_DIN_PIN_SWAP` reader - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
pub type SPI_QUAD_DIN_PIN_SWAP_R = crate::BitReader;
#[doc = "Field `SPI_QUAD_DIN_PIN_SWAP` writer - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
pub type SPI_QUAD_DIN_PIN_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to disable SPI_CS0 pin.\\\\ 0: SPI_CS0 signal is from/to SPI_CS0 pin.\\\\ 1: Disable SPI_CS0 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> SPI_CS0_DIS_R {
        SPI_CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to disable SPI_CS1 pin.\\\\ 0: SPI_CS1 signal is from/to SPI_CS1 pin.\\\\ 1: Disable SPI_CS1 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> SPI_CS1_DIS_R {
        SPI_CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to disable SPI_CS2 pin.\\\\ 0: SPI_CS2 signal is from/to SPI_CS2 pin.\\\\ 1: Disable SPI_CS2 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> SPI_CS2_DIS_R {
        SPI_CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to disable SPI_CS3 pin.\\\\ 0: SPI_CS3 signal is from/to SPI_CS3 pin.\\\\ 1: Disable SPI_CS3 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs3_dis(&self) -> SPI_CS3_DIS_R {
        SPI_CS3_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to disable SPI_CS4 pin.\\\\ 0: SPI_CS4 signal is from/to SPI_CS4 pin.\\\\ 1: Disable SPI_CS4 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs4_dis(&self) -> SPI_CS4_DIS_R {
        SPI_CS4_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to disable SPI_CS5 pin.\\\\ 0: SPI_CS5 signal is from/to SPI_CS5 pin.\\\\ 1: Disable SPI_CS5 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs5_dis(&self) -> SPI_CS5_DIS_R {
        SPI_CS5_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to disable SPI_CLK output.\\\\ 0: Enable\\\\ 1: Disable\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_dis(&self) -> SPI_CK_DIS_R {
        SPI_CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Configures the polarity of SPI_CS7 (7 = 0-5) line in master transfer.\\\\ 0: SPI_CS7 is low active.\\\\ 1: SPI_CS7 is high active.\\\\ Can be configured in CONF state."]
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
    #[doc = "Bit 23 - Configures whether or not invert SPI slave input CS polarity.\\\\ 0: Not change\\\\ 1: Invert\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&self) -> SPI_SLAVE_CS_POL_R {
        SPI_SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dqs_idle_edge(&self) -> SPI_DQS_IDLE_EDGE_R {
        SPI_DQS_IDLE_EDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures the level of SPI_CLK line when GP-SPI2 is in idle.\\\\ 0: Low\\\\ 1: High\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&self) -> SPI_CK_IDLE_EDGE_R {
        SPI_CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether or not to keep the SPI_CS line low.\\\\ 0: Not keep low\\\\ 1: Keep low\\\\ Can be configured in CONF state."]
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
            .field("spi_cs0_dis", &self.spi_cs0_dis())
            .field("spi_cs1_dis", &self.spi_cs1_dis())
            .field("spi_cs2_dis", &self.spi_cs2_dis())
            .field("spi_cs3_dis", &self.spi_cs3_dis())
            .field("spi_cs4_dis", &self.spi_cs4_dis())
            .field("spi_cs5_dis", &self.spi_cs5_dis())
            .field("spi_ck_dis", &self.spi_ck_dis())
            .field("spi_master_cs_pol", &self.spi_master_cs_pol())
            .field("spi_clk_data_dtr_en", &self.spi_clk_data_dtr_en())
            .field("spi_data_dtr_en", &self.spi_data_dtr_en())
            .field("spi_addr_dtr_en", &self.spi_addr_dtr_en())
            .field("spi_cmd_dtr_en", &self.spi_cmd_dtr_en())
            .field("spi_slave_cs_pol", &self.spi_slave_cs_pol())
            .field("spi_dqs_idle_edge", &self.spi_dqs_idle_edge())
            .field("spi_ck_idle_edge", &self.spi_ck_idle_edge())
            .field("spi_cs_keep_active", &self.spi_cs_keep_active())
            .field("spi_quad_din_pin_swap", &self.spi_quad_din_pin_swap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to disable SPI_CS0 pin.\\\\ 0: SPI_CS0 signal is from/to SPI_CS0 pin.\\\\ 1: Disable SPI_CS0 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs0_dis(&mut self) -> SPI_CS0_DIS_W<SPI_MISC_SPEC> {
        SPI_CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to disable SPI_CS1 pin.\\\\ 0: SPI_CS1 signal is from/to SPI_CS1 pin.\\\\ 1: Disable SPI_CS1 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs1_dis(&mut self) -> SPI_CS1_DIS_W<SPI_MISC_SPEC> {
        SPI_CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to disable SPI_CS2 pin.\\\\ 0: SPI_CS2 signal is from/to SPI_CS2 pin.\\\\ 1: Disable SPI_CS2 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs2_dis(&mut self) -> SPI_CS2_DIS_W<SPI_MISC_SPEC> {
        SPI_CS2_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to disable SPI_CS3 pin.\\\\ 0: SPI_CS3 signal is from/to SPI_CS3 pin.\\\\ 1: Disable SPI_CS3 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs3_dis(&mut self) -> SPI_CS3_DIS_W<SPI_MISC_SPEC> {
        SPI_CS3_DIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to disable SPI_CS4 pin.\\\\ 0: SPI_CS4 signal is from/to SPI_CS4 pin.\\\\ 1: Disable SPI_CS4 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs4_dis(&mut self) -> SPI_CS4_DIS_W<SPI_MISC_SPEC> {
        SPI_CS4_DIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to disable SPI_CS5 pin.\\\\ 0: SPI_CS5 signal is from/to SPI_CS5 pin.\\\\ 1: Disable SPI_CS5 pin.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs5_dis(&mut self) -> SPI_CS5_DIS_W<SPI_MISC_SPEC> {
        SPI_CS5_DIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to disable SPI_CLK output.\\\\ 0: Enable\\\\ 1: Disable\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_dis(&mut self) -> SPI_CK_DIS_W<SPI_MISC_SPEC> {
        SPI_CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - Configures the polarity of SPI_CS7 (7 = 0-5) line in master transfer.\\\\ 0: SPI_CS7 is low active.\\\\ 1: SPI_CS7 is high active.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_master_cs_pol(&mut self) -> SPI_MASTER_CS_POL_W<SPI_MISC_SPEC> {
        SPI_MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 23 - Configures whether or not invert SPI slave input CS polarity.\\\\ 0: Not change\\\\ 1: Invert\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&mut self) -> SPI_SLAVE_CS_POL_W<SPI_MISC_SPEC> {
        SPI_SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 29 - Configures the level of SPI_CLK line when GP-SPI2 is in idle.\\\\ 0: Low\\\\ 1: High\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&mut self) -> SPI_CK_IDLE_EDGE_W<SPI_MISC_SPEC> {
        SPI_CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to keep the SPI_CS line low.\\\\ 0: Not keep low\\\\ 1: Keep low\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_keep_active(&mut self) -> SPI_CS_KEEP_ACTIVE_W<SPI_MISC_SPEC> {
        SPI_CS_KEEP_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_quad_din_pin_swap(&mut self) -> SPI_QUAD_DIN_PIN_SWAP_W<SPI_MISC_SPEC> {
        SPI_QUAD_DIN_PIN_SWAP_W::new(self, 31)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MISC_SPEC;
impl crate::RegisterSpec for SPI_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_misc::R`](R) reader structure"]
impl crate::Readable for SPI_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_misc::W`](W) writer structure"]
impl crate::Writable for SPI_MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MISC to value 0x3e"]
impl crate::Resettable for SPI_MISC_SPEC {
    const RESET_VALUE: u32 = 0x3e;
}
