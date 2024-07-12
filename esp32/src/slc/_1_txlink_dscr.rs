#[doc = "Register `_1_TXLINK_DSCR` reader"]
pub type R = crate::R<_1_TXLINK_DSCR_SPEC>;
#[doc = "Field `SLC1_TXLINK_DSCR` reader - "]
pub type SLC1_TXLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_txlink_dscr(&self) -> SLC1_TXLINK_DSCR_R {
        SLC1_TXLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1_TXLINK_DSCR")
            .field("slc1_txlink_dscr", &self.slc1_txlink_dscr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_txlink_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_TXLINK_DSCR_SPEC;
impl crate::RegisterSpec for _1_TXLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_txlink_dscr::R`](R) reader structure"]
impl crate::Readable for _1_TXLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets _1_TXLINK_DSCR to value 0"]
impl crate::Resettable for _1_TXLINK_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
