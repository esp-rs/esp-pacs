#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TOTAL_TRANS_END_INT_ST` reader - The status bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_INT_ST_R = crate::BitReader;
#[doc = "Field `ECC_ERR_INT_ST` reader - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - The status bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    pub fn total_trans_end_int_st(&self) -> TOTAL_TRANS_END_INT_ST_R {
        TOTAL_TRANS_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_st(&self) -> ECC_ERR_INT_ST_R {
        ECC_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "total_trans_end_int_st",
                &format_args!("{}", self.total_trans_end_int_st().bit()),
            )
            .field(
                "ecc_err_int_st",
                &format_args!("{}", self.ecc_err_int_st().bit()),
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
    const RESET_VALUE: Self::Ux = 0;
}
