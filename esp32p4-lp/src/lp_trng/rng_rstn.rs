#[doc = "Register `RNG_RSTN` reader"]
pub type R = crate::R<RNG_RSTN_SPEC>;
#[doc = "Register `RNG_RSTN` writer"]
pub type W = crate::W<RNG_RSTN_SPEC>;
#[doc = "Field `RNG_RSTN` reader - enable rng system reset: 1: not reset, 0: reset"]
pub type RNG_RSTN_R = crate::BitReader;
#[doc = "Field `RNG_RSTN` writer - enable rng system reset: 1: not reset, 0: reset"]
pub type RNG_RSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable rng system reset: 1: not reset, 0: reset"]
    #[inline(always)]
    pub fn rng_rstn(&self) -> RNG_RSTN_R {
        RNG_RSTN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_RSTN")
            .field("rng_rstn", &self.rng_rstn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable rng system reset: 1: not reset, 0: reset"]
    #[inline(always)]
    pub fn rng_rstn(&mut self) -> RNG_RSTN_W<'_, RNG_RSTN_SPEC> {
        RNG_RSTN_W::new(self, 0)
    }
}
#[doc = "rng rstn register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_rstn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_rstn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_RSTN_SPEC;
impl crate::RegisterSpec for RNG_RSTN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_rstn::R`](R) reader structure"]
impl crate::Readable for RNG_RSTN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_rstn::W`](W) writer structure"]
impl crate::Writable for RNG_RSTN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_RSTN to value 0"]
impl crate::Resettable for RNG_RSTN_SPEC {}
