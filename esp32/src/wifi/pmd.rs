#[doc = "Register `PMD%s` reader"]
pub type R = crate::R<PMD_SPEC>;
#[doc = "Register `PMD%s` writer"]
pub type W = crate::W<PMD_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMD_SPEC;
impl crate::RegisterSpec for PMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmd::R`](R) reader structure"]
impl crate::Readable for PMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmd::W`](W) writer structure"]
impl crate::Writable for PMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMD%s to value 0"]
impl crate::Resettable for PMD_SPEC {}
