#[doc = "Register `DSCR` reader"]
pub type R = crate::R<DSCR_SPEC>;
#[doc = "Field `OUTLINK_DSCR` reader - The address of the next outlink descriptor address y."]
pub type OUTLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the next outlink descriptor address y."]
    #[inline(always)]
    pub fn outlink_dscr(&self) -> OUTLINK_DSCR_R {
        OUTLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCR")
            .field("outlink_dscr", &self.outlink_dscr())
            .finish()
    }
}
#[doc = "TX CHx next dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSCR_SPEC;
impl crate::RegisterSpec for DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr::R`](R) reader structure"]
impl crate::Readable for DSCR_SPEC {}
#[doc = "`reset()` method sets DSCR to value 0"]
impl crate::Resettable for DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
