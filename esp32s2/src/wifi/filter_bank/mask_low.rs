#[doc = "Register `MASK_LOW%s` reader"]
pub type R = crate::R<MASK_LOW_SPEC>;
#[doc = "Register `MASK_LOW%s` writer"]
pub type W = crate::W<MASK_LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "First 4 bytes of BSSID MAC address filter mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK_LOW_SPEC;
impl crate::RegisterSpec for MASK_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_low::R`](R) reader structure"]
impl crate::Readable for MASK_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask_low::W`](W) writer structure"]
impl crate::Writable for MASK_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASK_LOW%s to value 0"]
impl crate::Resettable for MASK_LOW_SPEC {}
