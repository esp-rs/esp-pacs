#[doc = "Register `BLECOEXIFCNTL0` reader"]
pub type R = crate::R<BLECOEXIFCNTL0_SPEC>;
#[doc = "Register `BLECOEXIFCNTL0` writer"]
pub type W = crate::W<BLECOEXIFCNTL0_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interface Coexistance Control\n\nYou can [`read`](crate::Reg::read) this register and get [`blecoexifcntl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blecoexifcntl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLECOEXIFCNTL0_SPEC;
impl crate::RegisterSpec for BLECOEXIFCNTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blecoexifcntl0::R`](R) reader structure"]
impl crate::Readable for BLECOEXIFCNTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blecoexifcntl0::W`](W) writer structure"]
impl crate::Writable for BLECOEXIFCNTL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLECOEXIFCNTL0 to value 0"]
impl crate::Resettable for BLECOEXIFCNTL0_SPEC {}
