#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `PER_END_INT_ST` reader - The status bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_INT_ST_R = crate::BitReader;
#[doc = "Field `PES_END_INT_ST` reader - The status bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_INT_ST_R = crate::BitReader;
#[doc = "Field `WPE_END_INT_ST` reader - The status bit for SPI_MEM_WPE_END_INT interrupt."]
pub type WPE_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_ST_END_INT_ST` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_ST_END_INT_ST` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_st(&self) -> PER_END_INT_ST_R {
        PER_END_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_st(&self) -> PES_END_INT_ST_R {
        PES_END_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end_int_st(&self) -> WPE_END_INT_ST_R {
        WPE_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_st(&self) -> SLV_ST_END_INT_ST_R {
        SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_st(&self) -> MST_ST_END_INT_ST_R {
        MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "per_end_int_st",
                &format_args!("{}", self.per_end_int_st().bit()),
            )
            .field(
                "pes_end_int_st",
                &format_args!("{}", self.pes_end_int_st().bit()),
            )
            .field(
                "wpe_end_int_st",
                &format_args!("{}", self.wpe_end_int_st().bit()),
            )
            .field(
                "slv_st_end_int_st",
                &format_args!("{}", self.slv_st_end_int_st().bit()),
            )
            .field(
                "mst_st_end_int_st",
                &format_args!("{}", self.mst_st_end_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI1 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
