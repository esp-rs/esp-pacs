#[doc = "Register `INLINK_DSCR` reader"]
pub type R = crate::R<INLINK_DSCR_SPEC>;
#[doc = "Field `INLINK_DSCR` reader - "]
pub type INLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> INLINK_DSCR_R {
        INLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INLINK_DSCR")
            .field("inlink_dscr", &self.inlink_dscr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INLINK_DSCR_SPEC;
impl crate::RegisterSpec for INLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inlink_dscr::R`](R) reader structure"]
impl crate::Readable for INLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets INLINK_DSCR to value 0"]
impl crate::Resettable for INLINK_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
