#[doc = "Register `BTINTCNTL` reader"]
pub type R = crate::R<BTINTCNTL_SPEC>;
#[doc = "Register `BTINTCNTL` writer"]
pub type W = crate::W<BTINTCNTL_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BR/EDR interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintcntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintcntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTINTCNTL_SPEC;
impl crate::RegisterSpec for BTINTCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btintcntl::R`](R) reader structure"]
impl crate::Readable for BTINTCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btintcntl::W`](W) writer structure"]
impl crate::Writable for BTINTCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTINTCNTL to value 0"]
impl crate::Resettable for BTINTCNTL_SPEC {}
