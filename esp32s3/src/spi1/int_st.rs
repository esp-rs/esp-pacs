#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `PER_END_INT_ST` reader - The status bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_INT_ST_R = crate::BitReader;
#[doc = "Field `PES_END_INT_ST` reader - The status bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_INT_ST_R = crate::BitReader;
#[doc = "Field `TOTAL_TRANS_END_INT_ST` reader - The status bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_INT_ST_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ST` reader - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_INT_ST_R = crate::BitReader;
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
    #[doc = "Bit 2 - The status bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    pub fn total_trans_end_int_st(&self) -> TOTAL_TRANS_END_INT_ST_R {
        TOTAL_TRANS_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out_int_st(&self) -> BROWN_OUT_INT_ST_R {
        BROWN_OUT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
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
                "total_trans_end_int_st",
                &format_args!("{}", self.total_trans_end_int_st().bit()),
            )
            .field(
                "brown_out_int_st",
                &format_args!("{}", self.brown_out_int_st().bit()),
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
