#[doc = "Register `EMACADDR1LOW` reader"]
pub type R = crate::R<EMACADDR1LOW_SPEC>;
#[doc = "Register `EMACADDR1LOW` writer"]
pub type W = crate::W<EMACADDR1LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr1low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr1low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR1LOW_SPEC;
impl crate::RegisterSpec for EMACADDR1LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr1low::R`](R) reader structure"]
impl crate::Readable for EMACADDR1LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr1low::W`](W) writer structure"]
impl crate::Writable for EMACADDR1LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACADDR1LOW to value 0"]
impl crate::Resettable for EMACADDR1LOW_SPEC {}
