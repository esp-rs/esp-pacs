#[doc = "Register `TIME0_THRESHOLD` reader"]
pub type R = crate::R<TIME0_THRESHOLD_SPEC>;
#[doc = "Register `TIME0_THRESHOLD` writer"]
pub type W = crate::W<TIME0_THRESHOLD_SPEC>;
#[doc = "Field `TIMER0_THRESHOLD` reader - "]
pub type TIMER0_THRESHOLD_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER0_THRESHOLD` writer - "]
pub type TIMER0_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer0_threshold(&self) -> TIMER0_THRESHOLD_R {
        TIMER0_THRESHOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME0_THRESHOLD")
            .field("timer0_threshold", &self.timer0_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer0_threshold(&mut self) -> TIMER0_THRESHOLD_W<TIME0_THRESHOLD_SPEC> {
        TIMER0_THRESHOLD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`time0_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time0_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME0_THRESHOLD_SPEC;
impl crate::RegisterSpec for TIME0_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time0_threshold::R`](R) reader structure"]
impl crate::Readable for TIME0_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time0_threshold::W`](W) writer structure"]
impl crate::Writable for TIME0_THRESHOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME0_THRESHOLD to value 0"]
impl crate::Resettable for TIME0_THRESHOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
