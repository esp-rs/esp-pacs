#[doc = "Register `T1ALARMHI` reader"]
pub type R = crate::R<T1ALARMHI_SPEC>;
#[doc = "Register `T1ALARMHI` writer"]
pub type W = crate::W<T1ALARMHI_SPEC>;
#[doc = "Field `ALARM_HI` reader - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1ALARMHI")
            .field("alarm_hi", &format_args!("{}", self.alarm_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1ALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W<T1ALARMHI_SPEC> {
        ALARM_HI_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1alarmhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1alarmhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1ALARMHI_SPEC;
impl crate::RegisterSpec for T1ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1alarmhi::R`](R) reader structure"]
impl crate::Readable for T1ALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t1alarmhi::W`](W) writer structure"]
impl crate::Writable for T1ALARMHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T1ALARMHI to value 0"]
impl crate::Resettable for T1ALARMHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
