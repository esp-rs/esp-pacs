#[doc = "Register `OUTLINK_DSCR` reader"]
pub type R = crate::R<OUTLINK_DSCR_SPEC>;
#[doc = "Field `OUTLINK_DSCR` reader - "]
pub type OUTLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn outlink_dscr(&self) -> OUTLINK_DSCR_R {
        OUTLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTLINK_DSCR")
            .field("outlink_dscr", &self.outlink_dscr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTLINK_DSCR_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outlink_dscr::R`](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets OUTLINK_DSCR to value 0"]
impl crate::Resettable for OUTLINK_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
