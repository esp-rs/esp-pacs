#[doc = "Register `RSTN` reader"]
pub type R = crate::R<RSTN_SPEC>;
#[doc = "Register `RSTN` writer"]
pub type W = crate::W<RSTN_SPEC>;
#[doc = "Field `RSTN` reader - enable rng system reset: 1: not reset, 0: reset"]
pub type RSTN_R = crate::BitReader;
#[doc = "Field `RSTN` writer - enable rng system reset: 1: not reset, 0: reset"]
pub type RSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable rng system reset: 1: not reset, 0: reset"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTN").field("rstn", &self.rstn()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable rng system reset: 1: not reset, 0: reset"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RSTN_W<'_, RSTN_SPEC> {
        RSTN_W::new(self, 0)
    }
}
#[doc = "rng rstn register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTN_SPEC;
impl crate::RegisterSpec for RSTN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstn::R`](R) reader structure"]
impl crate::Readable for RSTN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstn::W`](W) writer structure"]
impl crate::Writable for RSTN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTN to value 0x01"]
impl crate::Resettable for RSTN_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
