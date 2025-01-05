#[doc = "Register `DUMMY` reader"]
pub type R = crate::R<DUMMY_SPEC>;
#[doc = "Register `DUMMY` writer"]
pub type W = crate::W<DUMMY_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUMMY_SPEC;
impl crate::RegisterSpec for DUMMY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy::R`](R) reader structure"]
impl crate::Readable for DUMMY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dummy::W`](W) writer structure"]
impl crate::Writable for DUMMY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DUMMY to value 0"]
impl crate::Resettable for DUMMY_SPEC {
    const RESET_VALUE: u32 = 0;
}
