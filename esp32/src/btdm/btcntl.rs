#[doc = "Register `BTCNTL` reader"]
pub type R = crate::R<BTCNTL_SPEC>;
#[doc = "Register `BTCNTL` writer"]
pub type W = crate::W<BTCNTL_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BR/EDR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`btcntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btcntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTCNTL_SPEC;
impl crate::RegisterSpec for BTCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btcntl::R`](R) reader structure"]
impl crate::Readable for BTCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btcntl::W`](W) writer structure"]
impl crate::Writable for BTCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTCNTL to value 0"]
impl crate::Resettable for BTCNTL_SPEC {}
