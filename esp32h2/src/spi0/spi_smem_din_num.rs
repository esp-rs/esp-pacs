#[doc = "Register `SPI_SMEM_DIN_NUM` reader"]
pub struct R(crate::R<SPI_SMEM_DIN_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_DIN_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_DIN_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_DIN_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_SMEM_DIN0_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN0_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN1_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN1_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN2_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN2_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN3_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN3_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN4_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN4_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN5_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN5_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN6_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN6_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN7_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_SMEM_DIN7_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DINS_NUM` reader - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
pub type SPI_SMEM_DINS_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din0_num(&self) -> SPI_SMEM_DIN0_NUM_R {
        SPI_SMEM_DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din1_num(&self) -> SPI_SMEM_DIN1_NUM_R {
        SPI_SMEM_DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din2_num(&self) -> SPI_SMEM_DIN2_NUM_R {
        SPI_SMEM_DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din3_num(&self) -> SPI_SMEM_DIN3_NUM_R {
        SPI_SMEM_DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din4_num(&self) -> SPI_SMEM_DIN4_NUM_R {
        SPI_SMEM_DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din5_num(&self) -> SPI_SMEM_DIN5_NUM_R {
        SPI_SMEM_DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din6_num(&self) -> SPI_SMEM_DIN6_NUM_R {
        SPI_SMEM_DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din7_num(&self) -> SPI_SMEM_DIN7_NUM_R {
        SPI_SMEM_DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    #[inline(always)]
    pub fn spi_smem_dins_num(&self) -> SPI_SMEM_DINS_NUM_R {
        SPI_SMEM_DINS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DIN_NUM")
            .field(
                "spi_smem_din0_num",
                &format_args!("{}", self.spi_smem_din0_num().bits()),
            )
            .field(
                "spi_smem_din1_num",
                &format_args!("{}", self.spi_smem_din1_num().bits()),
            )
            .field(
                "spi_smem_din2_num",
                &format_args!("{}", self.spi_smem_din2_num().bits()),
            )
            .field(
                "spi_smem_din3_num",
                &format_args!("{}", self.spi_smem_din3_num().bits()),
            )
            .field(
                "spi_smem_din4_num",
                &format_args!("{}", self.spi_smem_din4_num().bits()),
            )
            .field(
                "spi_smem_din5_num",
                &format_args!("{}", self.spi_smem_din5_num().bits()),
            )
            .field(
                "spi_smem_din6_num",
                &format_args!("{}", self.spi_smem_din6_num().bits()),
            )
            .field(
                "spi_smem_din7_num",
                &format_args!("{}", self.spi_smem_din7_num().bits()),
            )
            .field(
                "spi_smem_dins_num",
                &format_args!("{}", self.spi_smem_dins_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MSPI external RAM input timing delay number control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_din_num](index.html) module"]
pub struct SPI_SMEM_DIN_NUM_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_din_num::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_NUM to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
