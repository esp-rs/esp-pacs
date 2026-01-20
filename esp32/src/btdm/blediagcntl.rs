#[doc = "Register `BLEDIAGCNTL` reader"]
pub type R = crate::R<BLEDIAGCNTL_SPEC>;
#[doc = "Register `BLEDIAGCNTL` writer"]
pub type W = crate::W<BLEDIAGCNTL_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Diagnostics control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blediagcntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blediagcntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEDIAGCNTL_SPEC;
impl crate::RegisterSpec for BLEDIAGCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blediagcntl::R`](R) reader structure"]
impl crate::Readable for BLEDIAGCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blediagcntl::W`](W) writer structure"]
impl crate::Writable for BLEDIAGCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEDIAGCNTL to value 0"]
impl crate::Resettable for BLEDIAGCNTL_SPEC {}
