#[doc = "Register `EX_DATE` reader"]
pub type R = crate::R<EX_DATE_SPEC>;
#[doc = "Register `EX_DATE` writer"]
pub type W = crate::W<EX_DATE_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ex_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ex_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EX_DATE_SPEC;
impl crate::RegisterSpec for EX_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ex_date::R`](R) reader structure"]
impl crate::Readable for EX_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ex_date::W`](W) writer structure"]
impl crate::Writable for EX_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EX_DATE to value 0"]
impl crate::Resettable for EX_DATE_SPEC {}
