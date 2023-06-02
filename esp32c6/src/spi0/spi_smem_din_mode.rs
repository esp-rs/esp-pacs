#[doc = "Register `SPI_SMEM_DIN_MODE` reader"]
pub struct R(crate::R<SPI_SMEM_DIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_DIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_DIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_DIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_SMEM_DIN0_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN0_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN1_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN1_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN2_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN2_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN3_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN3_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN4_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN4_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN5_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN5_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN6_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN6_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN7_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DIN7_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DINS_MODE` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DINS_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din0_mode(&self) -> SPI_SMEM_DIN0_MODE_R {
        SPI_SMEM_DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din1_mode(&self) -> SPI_SMEM_DIN1_MODE_R {
        SPI_SMEM_DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din2_mode(&self) -> SPI_SMEM_DIN2_MODE_R {
        SPI_SMEM_DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din3_mode(&self) -> SPI_SMEM_DIN3_MODE_R {
        SPI_SMEM_DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din4_mode(&self) -> SPI_SMEM_DIN4_MODE_R {
        SPI_SMEM_DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din5_mode(&self) -> SPI_SMEM_DIN5_MODE_R {
        SPI_SMEM_DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din6_mode(&self) -> SPI_SMEM_DIN6_MODE_R {
        SPI_SMEM_DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_din7_mode(&self) -> SPI_SMEM_DIN7_MODE_R {
        SPI_SMEM_DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_dins_mode(&self) -> SPI_SMEM_DINS_MODE_R {
        SPI_SMEM_DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DIN_MODE")
            .field(
                "spi_smem_din0_mode",
                &format_args!("{}", self.spi_smem_din0_mode().bits()),
            )
            .field(
                "spi_smem_din1_mode",
                &format_args!("{}", self.spi_smem_din1_mode().bits()),
            )
            .field(
                "spi_smem_din2_mode",
                &format_args!("{}", self.spi_smem_din2_mode().bits()),
            )
            .field(
                "spi_smem_din3_mode",
                &format_args!("{}", self.spi_smem_din3_mode().bits()),
            )
            .field(
                "spi_smem_din4_mode",
                &format_args!("{}", self.spi_smem_din4_mode().bits()),
            )
            .field(
                "spi_smem_din5_mode",
                &format_args!("{}", self.spi_smem_din5_mode().bits()),
            )
            .field(
                "spi_smem_din6_mode",
                &format_args!("{}", self.spi_smem_din6_mode().bits()),
            )
            .field(
                "spi_smem_din7_mode",
                &format_args!("{}", self.spi_smem_din7_mode().bits()),
            )
            .field(
                "spi_smem_dins_mode",
                &format_args!("{}", self.spi_smem_dins_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_DIN_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MSPI external RAM input timing delay mode control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_din_mode](index.html) module"]
pub struct SPI_SMEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_din_mode::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_MODE to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
