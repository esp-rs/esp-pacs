#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `READ_DONE_INT_ST` reader - The status signal for read_done interrupt."]
pub type READ_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `PGM_DONE_INT_ST` reader - The status signal for pgm_done interrupt."]
pub type PGM_DONE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status signal for read_done interrupt."]
    #[inline(always)]
    pub fn read_done_int_st(&self) -> READ_DONE_INT_ST_R {
        READ_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status signal for pgm_done interrupt."]
    #[inline(always)]
    pub fn pgm_done_int_st(&self) -> PGM_DONE_INT_ST_R {
        PGM_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "read_done_int_st",
                &format_args!("{}", self.read_done_int_st().bit()),
            )
            .field(
                "pgm_done_int_st",
                &format_args!("{}", self.pgm_done_int_st().bit()),
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
#[doc = "eFuse interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
