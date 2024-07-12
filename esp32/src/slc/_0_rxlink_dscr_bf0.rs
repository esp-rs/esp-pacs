#[doc = "Register `_0_RXLINK_DSCR_BF0` reader"]
pub type R = crate::R<_0_RXLINK_DSCR_BF0_SPEC>;
#[doc = "Field `SLC0_RXLINK_DSCR_BF0` reader - "]
pub type SLC0_RXLINK_DSCR_BF0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_rxlink_dscr_bf0(&self) -> SLC0_RXLINK_DSCR_BF0_R {
        SLC0_RXLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_RXLINK_DSCR_BF0")
            .field("slc0_rxlink_dscr_bf0", &self.slc0_rxlink_dscr_bf0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_rxlink_dscr_bf0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_RXLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for _0_RXLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_rxlink_dscr_bf0::R`](R) reader structure"]
impl crate::Readable for _0_RXLINK_DSCR_BF0_SPEC {}
#[doc = "`reset()` method sets _0_RXLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for _0_RXLINK_DSCR_BF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
