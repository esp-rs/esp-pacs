#[doc = "Register `LACTUPDATE` writer"]
pub type W = crate::W<LACTUPDATE_SPEC>;
#[doc = "Field `UPDATE` writer - "]
pub type UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTUPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<'_, LACTUPDATE_SPEC> {
        UPDATE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactupdate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTUPDATE_SPEC;
impl crate::RegisterSpec for LACTUPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lactupdate::W`](W) writer structure"]
impl crate::Writable for LACTUPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LACTUPDATE to value 0"]
impl crate::Resettable for LACTUPDATE_SPEC {}
