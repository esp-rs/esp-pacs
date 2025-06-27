#[doc = "Register `TIME1_THRESHOLD` reader"]
pub type R = crate::R<TIME1_THRESHOLD_SPEC>;
#[doc = "Register `TIME1_THRESHOLD` writer"]
pub type W = crate::W<TIME1_THRESHOLD_SPEC>;
#[doc = "Field `TIMER1_THRESHOLD` reader - "]
pub type TIMER1_THRESHOLD_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER1_THRESHOLD` writer - "]
pub type TIMER1_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer1_threshold(&self) -> TIMER1_THRESHOLD_R {
        TIMER1_THRESHOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME1_THRESHOLD")
            .field("timer1_threshold", &self.timer1_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer1_threshold(&mut self) -> TIMER1_THRESHOLD_W<TIME1_THRESHOLD_SPEC> {
        TIMER1_THRESHOLD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`time1_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time1_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME1_THRESHOLD_SPEC;
impl crate::RegisterSpec for TIME1_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time1_threshold::R`](R) reader structure"]
impl crate::Readable for TIME1_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time1_threshold::W`](W) writer structure"]
impl crate::Writable for TIME1_THRESHOLD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIME1_THRESHOLD to value 0"]
impl crate::Resettable for TIME1_THRESHOLD_SPEC {}
