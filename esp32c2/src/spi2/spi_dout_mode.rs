#[doc = "Register `SPI_DOUT_MODE` reader"]
pub struct R(crate::R<SPI_DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_DOUT_MODE` writer"]
pub struct W(crate::W<SPI_DOUT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_DOUT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_DOUT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_DOUT0_MODE` reader - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT0_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT0_MODE` writer - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT0_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 0>;
#[doc = "Field `SPI_DOUT1_MODE` reader - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT1_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT1_MODE` writer - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT1_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 1>;
#[doc = "Field `SPI_DOUT2_MODE` reader - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT2_MODE` writer - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT2_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 2>;
#[doc = "Field `SPI_DOUT3_MODE` reader - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT3_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT3_MODE` writer - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT3_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 3>;
#[doc = "Field `SPI_DOUT4_MODE` reader - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT4_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT4_MODE` writer - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT4_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 4>;
#[doc = "Field `SPI_DOUT5_MODE` reader - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT5_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT5_MODE` writer - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT5_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 5>;
#[doc = "Field `SPI_DOUT6_MODE` reader - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT6_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT6_MODE` writer - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT6_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 6>;
#[doc = "Field `SPI_DOUT7_MODE` reader - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT7_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DOUT7_MODE` writer - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_DOUT7_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 7>;
#[doc = "Field `SPI_D_DQS_MODE` reader - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_D_DQS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_D_DQS_MODE` writer - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type SPI_D_DQS_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_DOUT_MODE_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout0_mode(&self) -> SPI_DOUT0_MODE_R {
        SPI_DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout1_mode(&self) -> SPI_DOUT1_MODE_R {
        SPI_DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout2_mode(&self) -> SPI_DOUT2_MODE_R {
        SPI_DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
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
impl W {
    #[doc = "Bit 0 - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout0_mode(&mut self) -> SPI_DOUT0_MODE_W {
        SPI_DOUT0_MODE_W::new(self)
    }
    #[doc = "Bit 1 - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout1_mode(&mut self) -> SPI_DOUT1_MODE_W {
        SPI_DOUT1_MODE_W::new(self)
    }
    #[doc = "Bit 2 - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout2_mode(&mut self) -> SPI_DOUT2_MODE_W {
        SPI_DOUT2_MODE_W::new(self)
    }
    #[doc = "Bit 3 - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout3_mode(&mut self) -> SPI_DOUT3_MODE_W {
        SPI_DOUT3_MODE_W::new(self)
    }
    #[doc = "Bit 4 - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout4_mode(&mut self) -> SPI_DOUT4_MODE_W {
        SPI_DOUT4_MODE_W::new(self)
    }
    #[doc = "Bit 5 - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout5_mode(&mut self) -> SPI_DOUT5_MODE_W {
        SPI_DOUT5_MODE_W::new(self)
    }
    #[doc = "Bit 6 - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout6_mode(&mut self) -> SPI_DOUT6_MODE_W {
        SPI_DOUT6_MODE_W::new(self)
    }
    #[doc = "Bit 7 - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dout7_mode(&mut self) -> SPI_DOUT7_MODE_W {
        SPI_DOUT7_MODE_W::new(self)
    }
    #[doc = "Bit 8 - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_d_dqs_mode(&mut self) -> SPI_D_DQS_MODE_W {
        SPI_D_DQS_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI output delay mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dout_mode](index.html) module"]
pub struct SPI_DOUT_MODE_SPEC;
impl crate::RegisterSpec for SPI_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_dout_mode::R](R) reader structure"]
impl crate::Readable for SPI_DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_dout_mode::W](W) writer structure"]
impl crate::Writable for SPI_DOUT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_DOUT_MODE to value 0"]
impl crate::Resettable for SPI_DOUT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
