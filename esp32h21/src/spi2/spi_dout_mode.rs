#[doc = "Register `SPI_DOUT_MODE` reader"]
pub type R = crate::R<SPI_DOUT_MODE_SPEC>;
#[doc = "Register `SPI_DOUT_MODE` writer"]
pub type W = crate::W<SPI_DOUT_MODE_SPEC>;
#[doc = "Field `SPI_DOUT0_MODE` reader - Configures the output mode for FSPID signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT0_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT0_MODE` writer - Configures the output mode for FSPID signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT0_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DOUT1_MODE` reader - Configures the output mode for FSPIQ signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT1_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT1_MODE` writer - Configures the output mode for FSPIQ signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT1_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DOUT2_MODE` reader - Configures the output mode for FSPIWP signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT2_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT2_MODE` writer - Configures the output mode for FSPIWP signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT2_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DOUT3_MODE` reader - Configures the output mode for FSPIHD signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT3_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT3_MODE` writer - Configures the output mode for FSPIHD signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
pub type SPI_DOUT3_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DOUT4_MODE` reader - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT4_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT5_MODE` reader - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT5_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT6_MODE` reader - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT6_MODE_R = crate::BitReader;
#[doc = "Field `SPI_DOUT7_MODE` reader - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT7_MODE_R = crate::BitReader;
#[doc = "Field `SPI_D_DQS_MODE` reader - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_D_DQS_MODE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures the output mode for FSPID signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout0_mode(&self) -> SPI_DOUT0_MODE_R {
        SPI_DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the output mode for FSPIQ signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout1_mode(&self) -> SPI_DOUT1_MODE_R {
        SPI_DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures the output mode for FSPIWP signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout2_mode(&self) -> SPI_DOUT2_MODE_R {
        SPI_DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the output mode for FSPIHD signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout3_mode(&self) -> SPI_DOUT3_MODE_R {
        SPI_DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout4_mode(&self) -> SPI_DOUT4_MODE_R {
        SPI_DOUT4_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout5_mode(&self) -> SPI_DOUT5_MODE_R {
        SPI_DOUT5_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout6_mode(&self) -> SPI_DOUT6_MODE_R {
        SPI_DOUT6_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout7_mode(&self) -> SPI_DOUT7_MODE_R {
        SPI_DOUT7_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_d_dqs_mode(&self) -> SPI_D_DQS_MODE_R {
        SPI_D_DQS_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DOUT_MODE")
            .field("spi_dout0_mode", &self.spi_dout0_mode())
            .field("spi_dout1_mode", &self.spi_dout1_mode())
            .field("spi_dout2_mode", &self.spi_dout2_mode())
            .field("spi_dout3_mode", &self.spi_dout3_mode())
            .field("spi_dout4_mode", &self.spi_dout4_mode())
            .field("spi_dout5_mode", &self.spi_dout5_mode())
            .field("spi_dout6_mode", &self.spi_dout6_mode())
            .field("spi_dout7_mode", &self.spi_dout7_mode())
            .field("spi_d_dqs_mode", &self.spi_d_dqs_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the output mode for FSPID signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout0_mode(&mut self) -> SPI_DOUT0_MODE_W<'_, SPI_DOUT_MODE_SPEC> {
        SPI_DOUT0_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the output mode for FSPIQ signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout1_mode(&mut self) -> SPI_DOUT1_MODE_W<'_, SPI_DOUT_MODE_SPEC> {
        SPI_DOUT1_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures the output mode for FSPIWP signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout2_mode(&mut self) -> SPI_DOUT2_MODE_W<'_, SPI_DOUT_MODE_SPEC> {
        SPI_DOUT2_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the output mode for FSPIHD signal.\\\\ 0: Output without delay\\\\ 1: Output with a delay of a SPI module clock cycle at its falling edge\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout3_mode(&mut self) -> SPI_DOUT3_MODE_W<'_, SPI_DOUT_MODE_SPEC> {
        SPI_DOUT3_MODE_W::new(self, 3)
    }
}
#[doc = "SPI output delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dout_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dout_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DOUT_MODE_SPEC;
impl crate::RegisterSpec for SPI_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dout_mode::R`](R) reader structure"]
impl crate::Readable for SPI_DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_dout_mode::W`](W) writer structure"]
impl crate::Writable for SPI_DOUT_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DOUT_MODE to value 0"]
impl crate::Resettable for SPI_DOUT_MODE_SPEC {}
