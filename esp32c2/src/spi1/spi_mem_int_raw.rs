#[doc = "Register `SPI_MEM_INT_RAW` reader"]
pub struct R(crate::R<SPI_MEM_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_PER_END_INT_RAW` reader - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed. 0: Others."]
pub type SPI_MEM_PER_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_PES_END_INT_RAW` reader - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended. 0: Others."]
pub type SPI_MEM_PES_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_WPE_END_INT_RAW` reader - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type SPI_MEM_WPE_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_RAW` reader - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type SPI_MEM_BROWN_OUT_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_raw(&self) -> SPI_MEM_PER_END_INT_RAW_R {
        SPI_MEM_PER_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_raw(&self) -> SPI_MEM_PES_END_INT_RAW_R {
        SPI_MEM_PES_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_raw(&self) -> SPI_MEM_WPE_END_INT_RAW_R {
        SPI_MEM_WPE_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_raw(&self) -> SPI_MEM_SLV_ST_END_INT_RAW_R {
        SPI_MEM_SLV_ST_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_raw(&self) -> SPI_MEM_MST_ST_END_INT_RAW_R {
        SPI_MEM_MST_ST_END_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_raw(&self) -> SPI_MEM_BROWN_OUT_INT_RAW_R {
        SPI_MEM_BROWN_OUT_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "SPI1 interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_raw](index.html) module"]
pub struct SPI_MEM_INT_RAW_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_int_raw::R](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_INT_RAW to value 0"]
impl crate::Resettable for SPI_MEM_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
