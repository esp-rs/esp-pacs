#[doc = "Register `SPI_MEM_DIN_NUM` reader"]
pub struct R(crate::R<SPI_MEM_DIN_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_DIN_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_DIN_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_DIN_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_DIN_NUM` writer"]
pub struct W(crate::W<SPI_MEM_DIN_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_DIN_NUM_SPEC>;
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
impl From<crate::W<SPI_MEM_DIN_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_DIN_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_DIN0_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN0_NUM_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DIN0_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN0_NUM_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DIN_NUM_SPEC, bool, 0>;
#[doc = "Field `SPI_MEM_DIN1_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN1_NUM_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DIN1_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN1_NUM_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DIN_NUM_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_DIN2_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN2_NUM_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DIN2_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN2_NUM_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DIN_NUM_SPEC, bool, 2>;
#[doc = "Field `SPI_MEM_DIN3_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN3_NUM_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DIN3_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN3_NUM_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DIN_NUM_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din0_num(&self) -> SPI_MEM_DIN0_NUM_R {
        SPI_MEM_DIN0_NUM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din1_num(&self) -> SPI_MEM_DIN1_NUM_R {
        SPI_MEM_DIN1_NUM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din2_num(&self) -> SPI_MEM_DIN2_NUM_R {
        SPI_MEM_DIN2_NUM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din3_num(&self) -> SPI_MEM_DIN3_NUM_R {
        SPI_MEM_DIN3_NUM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din0_num(&mut self) -> SPI_MEM_DIN0_NUM_W {
        SPI_MEM_DIN0_NUM_W::new(self)
    }
    #[doc = "Bit 1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din1_num(&mut self) -> SPI_MEM_DIN1_NUM_W {
        SPI_MEM_DIN1_NUM_W::new(self)
    }
    #[doc = "Bit 2 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din2_num(&mut self) -> SPI_MEM_DIN2_NUM_W {
        SPI_MEM_DIN2_NUM_W::new(self)
    }
    #[doc = "Bit 3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din3_num(&mut self) -> SPI_MEM_DIN3_NUM_W {
        SPI_MEM_DIN3_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 input delay number control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_din_num](index.html) module"]
pub struct SPI_MEM_DIN_NUM_SPEC;
impl crate::RegisterSpec for SPI_MEM_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_din_num::R](R) reader structure"]
impl crate::Readable for SPI_MEM_DIN_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_din_num::W](W) writer structure"]
impl crate::Writable for SPI_MEM_DIN_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_DIN_NUM to value 0"]
impl crate::Resettable for SPI_MEM_DIN_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
