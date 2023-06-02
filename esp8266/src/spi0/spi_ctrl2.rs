#[doc = "Register `SPI_CTRL2` reader"]
pub struct R(crate::R<SPI_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL2` writer"]
pub struct W(crate::W<SPI_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL2_SPEC>;
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
impl From<crate::W<SPI_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_miso_delay_mode` reader - MISO signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
pub type SPI_MISO_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `spi_miso_delay_mode` writer - MISO signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
pub type SPI_MISO_DELAY_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL2_SPEC, 2, O>;
#[doc = "Field `spi_miso_delay_num` reader - MISO signals are delayed by 80MHz clock cycles"]
pub type SPI_MISO_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `spi_miso_delay_num` writer - MISO signals are delayed by 80MHz clock cycles"]
pub type SPI_MISO_DELAY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL2_SPEC, 3, O>;
#[doc = "Field `spi_mosi_delay_mode` reader - MOSI signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
pub type SPI_MOSI_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `spi_mosi_delay_mode` writer - MOSI signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
pub type SPI_MOSI_DELAY_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL2_SPEC, 2, O>;
#[doc = "Field `spi_mosi_delay_num` reader - MOSI signals are delayed by 80MHz clock cycles"]
pub type SPI_MOSI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `spi_mosi_delay_num` writer - MOSI signals are delayed by 80MHz clock cycles"]
pub type SPI_MOSI_DELAY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL2_SPEC, 3, O>;
#[doc = "Field `spi_cs_delay_mode` reader - spi_cs signal is delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
pub type SPI_CS_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `spi_cs_delay_mode` writer - spi_cs signal is delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
pub type SPI_CS_DELAY_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL2_SPEC, 2, O>;
#[doc = "Field `spi_cs_delay_num` reader - spi_cs signal is delayed by 80MHz clock cycles"]
pub type SPI_CS_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `spi_cs_delay_num` writer - spi_cs signal is delayed by 80MHz clock cycles"]
pub type SPI_CS_DELAY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL2_SPEC, 4, O>;
impl R {
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
    #[inline(always)]
    pub fn spi_miso_delay_mode(&self) -> SPI_MISO_DELAY_MODE_R {
        SPI_MISO_DELAY_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by 80MHz clock cycles"]
    #[inline(always)]
    pub fn spi_miso_delay_num(&self) -> SPI_MISO_DELAY_NUM_R {
        SPI_MISO_DELAY_NUM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
    #[inline(always)]
    pub fn spi_mosi_delay_mode(&self) -> SPI_MOSI_DELAY_MODE_R {
        SPI_MOSI_DELAY_MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by 80MHz clock cycles"]
    #[inline(always)]
    pub fn spi_mosi_delay_num(&self) -> SPI_MOSI_DELAY_NUM_R {
        SPI_MOSI_DELAY_NUM_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
    #[inline(always)]
    pub fn spi_cs_delay_mode(&self) -> SPI_CS_DELAY_MODE_R {
        SPI_CS_DELAY_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - spi_cs signal is delayed by 80MHz clock cycles"]
    #[inline(always)]
    pub fn spi_cs_delay_num(&self) -> SPI_CS_DELAY_NUM_R {
        SPI_CS_DELAY_NUM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CTRL2")
            .field(
                "spi_cs_delay_num",
                &format_args!("{}", self.spi_cs_delay_num().bits()),
            )
            .field(
                "spi_cs_delay_mode",
                &format_args!("{}", self.spi_cs_delay_mode().bits()),
            )
            .field(
                "spi_mosi_delay_num",
                &format_args!("{}", self.spi_mosi_delay_num().bits()),
            )
            .field(
                "spi_mosi_delay_mode",
                &format_args!("{}", self.spi_mosi_delay_mode().bits()),
            )
            .field(
                "spi_miso_delay_num",
                &format_args!("{}", self.spi_miso_delay_num().bits()),
            )
            .field(
                "spi_miso_delay_mode",
                &format_args!("{}", self.spi_miso_delay_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn spi_miso_delay_mode(&mut self) -> SPI_MISO_DELAY_MODE_W<16> {
        SPI_MISO_DELAY_MODE_W::new(self)
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by 80MHz clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn spi_miso_delay_num(&mut self) -> SPI_MISO_DELAY_NUM_W<18> {
        SPI_MISO_DELAY_NUM_W::new(self)
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mosi_delay_mode(&mut self) -> SPI_MOSI_DELAY_MODE_W<21> {
        SPI_MOSI_DELAY_MODE_W::new(self)
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by 80MHz clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mosi_delay_num(&mut self) -> SPI_MOSI_DELAY_NUM_W<23> {
        SPI_MOSI_DELAY_NUM_W::new(self)
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk. 0: zero; 1: half cycle; 2: one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_delay_mode(&mut self) -> SPI_CS_DELAY_MODE_W<26> {
        SPI_CS_DELAY_MODE_W::new(self)
    }
    #[doc = "Bits 28:31 - spi_cs signal is delayed by 80MHz clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_delay_num(&mut self) -> SPI_CS_DELAY_NUM_W<28> {
        SPI_CS_DELAY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_cs signal is delayed by 80MHz clock cycles\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl2](index.html) module"]
pub struct SPI_CTRL2_SPEC;
impl crate::RegisterSpec for SPI_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ctrl2::R](R) reader structure"]
impl crate::Readable for SPI_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl2::W](W) writer structure"]
impl crate::Writable for SPI_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CTRL2 to value 0"]
impl crate::Resettable for SPI_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
