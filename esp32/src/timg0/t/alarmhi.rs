#[doc = "Register `ALARMHI` reader"]
pub type R = crate::R<ALARMHI_SPEC>;
#[doc = "Register `ALARMHI` writer"]
pub type W = crate::W<ALARMHI_SPEC>;
#[doc = "Field `ALARM_HI` reader - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALARMHI")
            .field("alarm_hi", &self.alarm_hi().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W<ALARMHI_SPEC> {
        ALARM_HI_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARMHI_SPEC;
impl crate::RegisterSpec for ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmhi::R`](R) reader structure"]
impl crate::Readable for ALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarmhi::W`](W) writer structure"]
impl crate::Writable for ALARMHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARMHI to value 0"]
impl crate::Resettable for ALARMHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
