#[doc = "Register `TIMER_ALARM` reader"]
pub type R = crate::R<TIMER_ALARM_SPEC>;
#[doc = "Register `TIMER_ALARM` writer"]
pub type W = crate::W<TIMER_ALARM_SPEC>;
#[doc = "Field `TIMER_ALARM` reader - "]
pub type TIMER_ALARM_R = crate::FieldReader;
#[doc = "Field `TIMER_ALARM` writer - "]
pub type TIMER_ALARM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_alarm(&self) -> TIMER_ALARM_R {
        TIMER_ALARM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_ALARM")
            .field("timer_alarm", &self.timer_alarm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_alarm(&mut self) -> TIMER_ALARM_W<'_, TIMER_ALARM_SPEC> {
        TIMER_ALARM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_alarm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_alarm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_ALARM_SPEC;
impl crate::RegisterSpec for TIMER_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_alarm::R`](R) reader structure"]
impl crate::Readable for TIMER_ALARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_alarm::W`](W) writer structure"]
impl crate::Writable for TIMER_ALARM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_ALARM to value 0"]
impl crate::Resettable for TIMER_ALARM_SPEC {}
