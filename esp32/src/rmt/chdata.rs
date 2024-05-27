#[doc = "Register `CH%sDATA` reader"]
pub type R = crate::R<CHDATA_SPEC>;
#[doc = "Register `CH%sDATA` writer"]
pub type W = crate::W<CHDATA_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDATA_SPEC;
impl crate::RegisterSpec for CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdata::R`](R) reader structure"]
impl crate::Readable for CHDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chdata::W`](W) writer structure"]
impl crate::Writable for CHDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%sDATA to value 0"]
impl crate::Resettable for CHDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
