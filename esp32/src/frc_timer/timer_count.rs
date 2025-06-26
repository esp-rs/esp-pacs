#[doc = "Register `TIMER_COUNT` reader"]
pub type R = crate::R<TIMER_COUNT_SPEC>;
#[doc = "Register `TIMER_COUNT` writer"]
pub type W = crate::W<TIMER_COUNT_SPEC>;
#[doc = "Field `TIMER_COUNT` reader - "]
pub type TIMER_COUNT_R = crate::FieldReader;
#[doc = "Field `TIMER_COUNT` writer - "]
pub type TIMER_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_count(&self) -> TIMER_COUNT_R {
        TIMER_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_COUNT")
            .field("timer_count", &self.timer_count())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_count(&mut self) -> TIMER_COUNT_W<TIMER_COUNT_SPEC> {
        TIMER_COUNT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_COUNT_SPEC;
impl crate::RegisterSpec for TIMER_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_count::R`](R) reader structure"]
impl crate::Readable for TIMER_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_count::W`](W) writer structure"]
impl crate::Writable for TIMER_COUNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_COUNT to value 0"]
impl crate::Resettable for TIMER_COUNT_SPEC {}
