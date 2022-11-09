#[doc = "Register `DOUT_MODE` reader"]
pub struct R(crate::R<DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOUT0_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type DOUT0_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DOUT1_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type DOUT1_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DOUT2_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type DOUT2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DOUT3_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type DOUT3_MODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SPI0 output delay mode control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout_mode](index.html) module"]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout_mode::R](R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
