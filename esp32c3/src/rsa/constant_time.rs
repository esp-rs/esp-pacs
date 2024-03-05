#[doc = "Register `CONSTANT_TIME` reader"]
pub type R = crate::R<CONSTANT_TIME_SPEC>;
#[doc = "Register `CONSTANT_TIME` writer"]
pub type W = crate::W<CONSTANT_TIME_SPEC>;
#[doc = "Field `CONSTANT_TIME` reader - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
pub type CONSTANT_TIME_R = crate::BitReader;
#[doc = "Field `CONSTANT_TIME` writer - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
pub type CONSTANT_TIME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    #[inline(always)]
    pub fn constant_time(&self) -> CONSTANT_TIME_R {
        CONSTANT_TIME_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONSTANT_TIME")
            .field(
                "constant_time",
                &format_args!("{}", self.constant_time().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONSTANT_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    #[inline(always)]
    #[must_use]
    pub fn constant_time(&mut self) -> CONSTANT_TIME_W<CONSTANT_TIME_SPEC> {
        CONSTANT_TIME_W::new(self, 0)
    }
}
#[doc = "RSA constant time option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`constant_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`constant_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONSTANT_TIME_SPEC;
impl crate::RegisterSpec for CONSTANT_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`constant_time::R`](R) reader structure"]
impl crate::Readable for CONSTANT_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`constant_time::W`](W) writer structure"]
impl crate::Writable for CONSTANT_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONSTANT_TIME to value 0x01"]
impl crate::Resettable for CONSTANT_TIME_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
