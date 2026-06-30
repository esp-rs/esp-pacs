#[doc = "Register `SIGN_MEM[%s]` reader"]
pub type R = crate::R<SIGN_MEM_SPEC>;
#[doc = "Register `SIGN_MEM[%s]` writer"]
pub type W = crate::W<SIGN_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RMA signature r memory .\n\nYou can [`read`](crate::Reg::read) this register and get [`sign_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sign_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGN_MEM_SPEC;
impl crate::RegisterSpec for SIGN_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sign_mem::R`](R) reader structure"]
impl crate::Readable for SIGN_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sign_mem::W`](W) writer structure"]
impl crate::Writable for SIGN_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGN_MEM[%s] to value 0"]
impl crate::Resettable for SIGN_MEM_SPEC {}
