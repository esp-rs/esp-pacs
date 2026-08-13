#[doc = "Register `SPI_SMEM_DIN_HEX_MODE` reader"]
pub type R = crate::R<SPI_SMEM_DIN_HEX_MODE_SPEC>;
#[doc = "Field `SPI_SMEM_DIN08_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN08_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN09_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN09_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN10_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN10_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN11_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN11_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN12_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN12_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN13_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN13_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN14_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN14_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN15_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN15_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DINS_HEX_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DINS_HEX_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din08_mode(&self) -> SPI_SMEM_DIN08_MODE_R {
        SPI_SMEM_DIN08_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din09_mode(&self) -> SPI_SMEM_DIN09_MODE_R {
        SPI_SMEM_DIN09_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din10_mode(&self) -> SPI_SMEM_DIN10_MODE_R {
        SPI_SMEM_DIN10_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din11_mode(&self) -> SPI_SMEM_DIN11_MODE_R {
        SPI_SMEM_DIN11_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din12_mode(&self) -> SPI_SMEM_DIN12_MODE_R {
        SPI_SMEM_DIN12_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din13_mode(&self) -> SPI_SMEM_DIN13_MODE_R {
        SPI_SMEM_DIN13_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din14_mode(&self) -> SPI_SMEM_DIN14_MODE_R {
        SPI_SMEM_DIN14_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din15_mode(&self) -> SPI_SMEM_DIN15_MODE_R {
        SPI_SMEM_DIN15_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_dins_hex_mode(&self) -> SPI_SMEM_DINS_HEX_MODE_R {
        SPI_SMEM_DINS_HEX_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DIN_HEX_MODE")
            .field("spi_smem_din08_mode", &self.spi_smem_din08_mode())
            .field("spi_smem_din09_mode", &self.spi_smem_din09_mode())
            .field("spi_smem_din10_mode", &self.spi_smem_din10_mode())
            .field("spi_smem_din11_mode", &self.spi_smem_din11_mode())
            .field("spi_smem_din12_mode", &self.spi_smem_din12_mode())
            .field("spi_smem_din13_mode", &self.spi_smem_din13_mode())
            .field("spi_smem_din14_mode", &self.spi_smem_din14_mode())
            .field("spi_smem_din15_mode", &self.spi_smem_din15_mode())
            .field("spi_smem_dins_hex_mode", &self.spi_smem_dins_hex_mode())
            .finish()
    }
}
#[doc = "MSPI 16x external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_hex_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_DIN_HEX_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_din_hex_mode::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_HEX_MODE_SPEC {}
#[doc = "`reset()` method sets SPI_SMEM_DIN_HEX_MODE to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_HEX_MODE_SPEC {}
