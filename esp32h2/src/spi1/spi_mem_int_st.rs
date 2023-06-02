#[doc = "Register `SPI_MEM_INT_ST` reader"]
pub struct R(crate::R<SPI_MEM_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_PER_END_INT_ST` reader - The status bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PES_END_INT_ST` reader - The status bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WPE_END_INT_ST` reader - The status bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ST` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ST` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_ST` reader - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_per_end_int_st(&self) -> SPI_MEM_PER_END_INT_ST_R {
        SPI_MEM_PER_END_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pes_end_int_st(&self) -> SPI_MEM_PES_END_INT_ST_R {
        SPI_MEM_PES_END_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_wpe_end_int_st(&self) -> SPI_MEM_WPE_END_INT_ST_R {
        SPI_MEM_WPE_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_st(&self) -> SPI_MEM_SLV_ST_END_INT_ST_R {
        SPI_MEM_SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_st(&self) -> SPI_MEM_MST_ST_END_INT_ST_R {
        SPI_MEM_MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_brown_out_int_st(&self) -> SPI_MEM_BROWN_OUT_INT_ST_R {
        SPI_MEM_BROWN_OUT_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_ST")
            .field(
                "spi_mem_per_end_int_st",
                &format_args!("{}", self.spi_mem_per_end_int_st().bit()),
            )
            .field(
                "spi_mem_pes_end_int_st",
                &format_args!("{}", self.spi_mem_pes_end_int_st().bit()),
            )
            .field(
                "spi_mem_wpe_end_int_st",
                &format_args!("{}", self.spi_mem_wpe_end_int_st().bit()),
            )
            .field(
                "spi_mem_slv_st_end_int_st",
                &format_args!("{}", self.spi_mem_slv_st_end_int_st().bit()),
            )
            .field(
                "spi_mem_mst_st_end_int_st",
                &format_args!("{}", self.spi_mem_mst_st_end_int_st().bit()),
            )
            .field(
                "spi_mem_brown_out_int_st",
                &format_args!("{}", self.spi_mem_brown_out_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI1 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_st](index.html) module"]
pub struct SPI_MEM_INT_ST_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_int_st::R](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_INT_ST to value 0"]
impl crate::Resettable for SPI_MEM_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
