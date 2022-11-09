#[doc = "Register `DIN_MODE` reader"]
pub struct R(crate::R<DIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type DIN0_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type DIN1_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type DIN2_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type DIN3_MODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "SPI0 input delay mode control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_mode](index.html) module"]
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_mode::R](R) reader structure"]
impl crate::Readable for DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIN_MODE to value 0"]
impl crate::Resettable for DIN_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
