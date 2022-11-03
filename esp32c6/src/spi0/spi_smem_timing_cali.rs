#[doc = "Register `SPI_SMEM_TIMING_CALI` reader"]
pub struct R(crate::R<SPI_SMEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` reader - For sram, the bit is used to enable timing adjust clock for all reading operations."]
pub type SPI_SMEM_TIMING_CLK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SMEM_TIMING_CALI` reader - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
pub type SPI_SMEM_TIMING_CALI_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` reader - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
pub type SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_SMEM_DLL_TIMING_CALI` reader - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
pub type SPI_SMEM_DLL_TIMING_CALI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - For sram, the bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&self) -> SPI_SMEM_TIMING_CLK_ENA_R {
        SPI_SMEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&self) -> SPI_SMEM_TIMING_CALI_R {
        SPI_SMEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&self) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
    #[inline(always)]
    pub fn spi_smem_dll_timing_cali(&self) -> SPI_SMEM_DLL_TIMING_CALI_R {
        SPI_SMEM_DLL_TIMING_CALI_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "MSPI external RAM timing calibration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_timing_cali](index.html) module"]
pub struct SPI_SMEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SPI_SMEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_timing_cali::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_SMEM_TIMING_CALI to value 0x01"]
impl crate::Resettable for SPI_SMEM_TIMING_CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
