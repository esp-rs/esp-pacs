#[doc = "Register `SMEM_DOUT_HEX_MODE` reader"]
pub type R = crate::R<SMEM_DOUT_HEX_MODE_SPEC>;
#[doc = "Field `SMEM_DOUT08_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT08_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT09_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT09_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT10_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT10_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT11_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT11_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT12_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT12_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT13_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT13_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT14_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT14_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUT15_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUT15_MODE_R = crate::BitReader;
#[doc = "Field `SMEM_DOUTS_HEX_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
pub type SMEM_DOUTS_HEX_MODE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout08_mode(&self) -> SMEM_DOUT08_MODE_R {
        SMEM_DOUT08_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout09_mode(&self) -> SMEM_DOUT09_MODE_R {
        SMEM_DOUT09_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout10_mode(&self) -> SMEM_DOUT10_MODE_R {
        SMEM_DOUT10_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout11_mode(&self) -> SMEM_DOUT11_MODE_R {
        SMEM_DOUT11_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout12_mode(&self) -> SMEM_DOUT12_MODE_R {
        SMEM_DOUT12_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout13_mode(&self) -> SMEM_DOUT13_MODE_R {
        SMEM_DOUT13_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout14_mode(&self) -> SMEM_DOUT14_MODE_R {
        SMEM_DOUT14_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_dout15_mode(&self) -> SMEM_DOUT15_MODE_R {
        SMEM_DOUT15_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    #[inline(always)]
    pub fn smem_douts_hex_mode(&self) -> SMEM_DOUTS_HEX_MODE_R {
        SMEM_DOUTS_HEX_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DOUT_HEX_MODE")
            .field("smem_dout08_mode", &self.smem_dout08_mode())
            .field("smem_dout09_mode", &self.smem_dout09_mode())
            .field("smem_dout10_mode", &self.smem_dout10_mode())
            .field("smem_dout11_mode", &self.smem_dout11_mode())
            .field("smem_dout12_mode", &self.smem_dout12_mode())
            .field("smem_dout13_mode", &self.smem_dout13_mode())
            .field("smem_dout14_mode", &self.smem_dout14_mode())
            .field("smem_dout15_mode", &self.smem_dout15_mode())
            .field("smem_douts_hex_mode", &self.smem_douts_hex_mode())
            .finish()
    }
}
#[doc = "MSPI 16x external RAM output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_dout_hex_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DOUT_HEX_MODE_SPEC;
impl crate::RegisterSpec for SMEM_DOUT_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_dout_hex_mode::R`](R) reader structure"]
impl crate::Readable for SMEM_DOUT_HEX_MODE_SPEC {}
#[doc = "`reset()` method sets SMEM_DOUT_HEX_MODE to value 0"]
impl crate::Resettable for SMEM_DOUT_HEX_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
