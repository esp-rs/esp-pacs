#[doc = "Register `SPI_MEM_DOUT_MODE` reader"]
pub struct R(crate::R<SPI_MEM_DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_DOUT_MODE` writer"]
pub struct W(crate::W<SPI_MEM_DOUT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_DOUT_MODE_SPEC>;
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
impl From<crate::W<SPI_MEM_DOUT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_DOUT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_DOUT0_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT0_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DOUT0_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT0_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DOUT_MODE_SPEC, bool, 0>;
#[doc = "Field `SPI_MEM_DOUT1_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT1_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DOUT1_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT1_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DOUT_MODE_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_DOUT2_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DOUT2_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT2_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DOUT_MODE_SPEC, bool, 2>;
#[doc = "Field `SPI_MEM_DOUT3_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT3_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_DOUT3_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SPI_MEM_DOUT3_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_DOUT_MODE_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout0_mode(&self) -> SPI_MEM_DOUT0_MODE_R {
        SPI_MEM_DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout1_mode(&self) -> SPI_MEM_DOUT1_MODE_R {
        SPI_MEM_DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout2_mode(&self) -> SPI_MEM_DOUT2_MODE_R {
        SPI_MEM_DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout3_mode(&self) -> SPI_MEM_DOUT3_MODE_R {
        SPI_MEM_DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout0_mode(&mut self) -> SPI_MEM_DOUT0_MODE_W {
        SPI_MEM_DOUT0_MODE_W::new(self)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout1_mode(&mut self) -> SPI_MEM_DOUT1_MODE_W {
        SPI_MEM_DOUT1_MODE_W::new(self)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout2_mode(&mut self) -> SPI_MEM_DOUT2_MODE_W {
        SPI_MEM_DOUT2_MODE_W::new(self)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_mem_dout3_mode(&mut self) -> SPI_MEM_DOUT3_MODE_W {
        SPI_MEM_DOUT3_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 output delay mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_dout_mode](index.html) module"]
pub struct SPI_MEM_DOUT_MODE_SPEC;
impl crate::RegisterSpec for SPI_MEM_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_dout_mode::R](R) reader structure"]
impl crate::Readable for SPI_MEM_DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_dout_mode::W](W) writer structure"]
impl crate::Writable for SPI_MEM_DOUT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_DOUT_MODE to value 0"]
impl crate::Resettable for SPI_MEM_DOUT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
