#[doc = "Register `SPI_MEM_TIMING_CALI` reader"]
pub struct R(crate::R<SPI_MEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_TIMING_CALI` writer"]
pub struct W(crate::W<SPI_MEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_TIMING_CALI_SPEC>;
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
impl From<crate::W<SPI_MEM_TIMING_CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_TIMING_CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_TIMING_CLK_ENA` reader - The bit is used to enable timing adjust clock for all reading operations."]
pub type SPI_MEM_TIMING_CLK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_TIMING_CLK_ENA` writer - The bit is used to enable timing adjust clock for all reading operations."]
pub type SPI_MEM_TIMING_CLK_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_TIMING_CALI_SPEC, bool, 0>;
#[doc = "Field `SPI_MEM_TIMING_CALI` reader - The bit is used to enable timing auto-calibration for all reading operations."]
pub type SPI_MEM_TIMING_CALI_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_TIMING_CALI` writer - The bit is used to enable timing auto-calibration for all reading operations."]
pub type SPI_MEM_TIMING_CALI_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_TIMING_CALI_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_EXTRA_DUMMY_CYCLELEN` reader - add extra dummy spi clock cycle length for spi clock calibration."]
pub type SPI_MEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_EXTRA_DUMMY_CYCLELEN` writer - add extra dummy spi clock cycle length for spi clock calibration."]
pub type SPI_MEM_EXTRA_DUMMY_CYCLELEN_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_TIMING_CALI_SPEC, u8, u8, 3, 2>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn spi_mem_timing_clk_ena(&self) -> SPI_MEM_TIMING_CLK_ENA_R {
        SPI_MEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn spi_mem_timing_cali(&self) -> SPI_MEM_TIMING_CALI_R {
        SPI_MEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn spi_mem_extra_dummy_cyclelen(&self) -> SPI_MEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_MEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn spi_mem_timing_clk_ena(&mut self) -> SPI_MEM_TIMING_CLK_ENA_W {
        SPI_MEM_TIMING_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn spi_mem_timing_cali(&mut self) -> SPI_MEM_TIMING_CALI_W {
        SPI_MEM_TIMING_CALI_W::new(self)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn spi_mem_extra_dummy_cyclelen(&mut self) -> SPI_MEM_EXTRA_DUMMY_CYCLELEN_W {
        SPI_MEM_EXTRA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 timing calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_timing_cali](index.html) module"]
pub struct SPI_MEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SPI_MEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_timing_cali::R](R) reader structure"]
impl crate::Readable for SPI_MEM_TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_timing_cali::W](W) writer structure"]
impl crate::Writable for SPI_MEM_TIMING_CALI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_TIMING_CALI to value 0"]
impl crate::Resettable for SPI_MEM_TIMING_CALI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
