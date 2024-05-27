///Register `DIN_MODE` reader
pub type R = crate::R<DIN_MODE_SPEC>;
///Field `DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
pub type DIN0_MODE_R = crate::FieldReader;
///Field `DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
pub type DIN1_MODE_R = crate::FieldReader;
///Field `DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
pub type DIN2_MODE_R = crate::FieldReader;
///Field `DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
pub type DIN3_MODE_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_MODE")
            .field("din0_mode", &self.din0_mode())
            .field("din1_mode", &self.din1_mode())
            .field("din2_mode", &self.din2_mode())
            .field("din3_mode", &self.din3_mode())
            .finish()
    }
}
/**SPI0 input delay mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`din_mode::R`](R) reader structure
impl crate::Readable for DIN_MODE_SPEC {}
///`reset()` method sets DIN_MODE to value 0
impl crate::Resettable for DIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
