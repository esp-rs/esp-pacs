#[doc = "Register `EMACADDR2LOW` reader"]
pub type R = crate::R<EMACADDR2LOW_SPEC>;
#[doc = "Register `EMACADDR2LOW` writer"]
pub type W = crate::W<EMACADDR2LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr2low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr2low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR2LOW_SPEC;
impl crate::RegisterSpec for EMACADDR2LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr2low::R`](R) reader structure"]
impl crate::Readable for EMACADDR2LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr2low::W`](W) writer structure"]
impl crate::Writable for EMACADDR2LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACADDR2LOW to value 0"]
impl crate::Resettable for EMACADDR2LOW_SPEC {}
