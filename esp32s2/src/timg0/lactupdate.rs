#[doc = "Register `LACTUPDATE` writer"]
pub type W = crate::W<LACTUPDATE_SPEC>;
#[doc = "Field `UPDATE` writer - Reserved."]
pub type UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTUPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<LACTUPDATE_SPEC> {
        UPDATE_W::new(self, 0)
    }
}
#[doc = "LACT update register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTUPDATE_SPEC;
impl crate::RegisterSpec for LACTUPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lactupdate::W`](W) writer structure"]
impl crate::Writable for LACTUPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTUPDATE to value 0"]
impl crate::Resettable for LACTUPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
