#[doc = "Register `_1_RXLINK_DSCR` reader"]
pub type R = crate::R<_1_RXLINK_DSCR_SPEC>;
#[doc = "Field `SLC1_RXLINK_DSCR` reader - "]
pub type SLC1_RXLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_rxlink_dscr(&self) -> SLC1_RXLINK_DSCR_R {
        SLC1_RXLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1_RXLINK_DSCR")
            .field(
                "slc1_rxlink_dscr",
                &format_args!("{}", self.slc1_rxlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_1_RXLINK_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_rxlink_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_RXLINK_DSCR_SPEC;
impl crate::RegisterSpec for _1_RXLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_rxlink_dscr::R`](R) reader structure"]
impl crate::Readable for _1_RXLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets _1_RXLINK_DSCR to value 0"]
impl crate::Resettable for _1_RXLINK_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
