#[doc = "Register `ALARMLO` reader"]
pub type R = crate::R<ALARMLO_SPEC>;
#[doc = "Register `ALARMLO` writer"]
pub type W = crate::W<ALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - Timer 0 time-base counter value lower 32 bits that will trigger the alarm"]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - Timer 0 time-base counter value lower 32 bits that will trigger the alarm"]
pub type ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 0 time-base counter value lower 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALARMLO")
            .field("alarm_lo", &self.alarm_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 0 time-base counter value lower 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<ALARMLO_SPEC> {
        ALARM_LO_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARMLO_SPEC;
impl crate::RegisterSpec for ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmlo::R`](R) reader structure"]
impl crate::Readable for ALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarmlo::W`](W) writer structure"]
impl crate::Writable for ALARMLO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARMLO to value 0"]
impl crate::Resettable for ALARMLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
