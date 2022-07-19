#[doc = "Register `SPI_MEM_DIN_MODE` reader"]
pub struct R(crate::R<SPI_MEM_DIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_DIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_DIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_DIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_DIN_MODE` writer"]
pub struct W(crate::W<SPI_MEM_DIN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_DIN_MODE_SPEC>;
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
impl From<crate::W<SPI_MEM_DIN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_DIN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN0_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_DIN0_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN0_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_DIN_MODE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SPI_MEM_DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN1_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_DIN1_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN1_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_DIN_MODE_SPEC, u8, u8, 2, 2>;
#[doc = "Field `SPI_MEM_DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN2_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_DIN2_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN2_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_DIN_MODE_SPEC, u8, u8, 2, 4>;
#[doc = "Field `SPI_MEM_DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN3_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_DIN3_MODE` writer - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_MEM_DIN3_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_DIN_MODE_SPEC, u8, u8, 2, 6>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din0_mode(&self) -> SPI_MEM_DIN0_MODE_R {
        SPI_MEM_DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din1_mode(&self) -> SPI_MEM_DIN1_MODE_R {
        SPI_MEM_DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din2_mode(&self) -> SPI_MEM_DIN2_MODE_R {
        SPI_MEM_DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din3_mode(&self) -> SPI_MEM_DIN3_MODE_R {
        SPI_MEM_DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din0_mode(&mut self) -> SPI_MEM_DIN0_MODE_W {
        SPI_MEM_DIN0_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din1_mode(&mut self) -> SPI_MEM_DIN1_MODE_W {
        SPI_MEM_DIN1_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din2_mode(&mut self) -> SPI_MEM_DIN2_MODE_W {
        SPI_MEM_DIN2_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_din3_mode(&mut self) -> SPI_MEM_DIN3_MODE_W {
        SPI_MEM_DIN3_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 input delay mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_din_mode](index.html) module"]
pub struct SPI_MEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_MEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_din_mode::R](R) reader structure"]
impl crate::Readable for SPI_MEM_DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_din_mode::W](W) writer structure"]
impl crate::Writable for SPI_MEM_DIN_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_DIN_MODE to value 0"]
impl crate::Resettable for SPI_MEM_DIN_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
