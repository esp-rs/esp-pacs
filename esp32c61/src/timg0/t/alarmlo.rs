#[doc = "Register `ALARMLO` reader"]
pub type R = crate::R<ALARMLO_SPEC>;
#[doc = "Register `ALARMLO` writer"]
pub type W = crate::W<ALARMLO_SPEC>;
#[doc = "Field `T0_ALARM_LO` reader - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
pub type T0_ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `T0_ALARM_LO` writer - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
pub type T0_ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t0_alarm_lo(&self) -> T0_ALARM_LO_R {
        T0_ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALARMLO")
            .field("t0_alarm_lo", &self.t0_alarm_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t0_alarm_lo(&mut self) -> T0_ALARM_LO_W<'_, ALARMLO_SPEC> {
        T0_ALARM_LO_W::new(self, 0)
    }
}
#[doc = "Timer 0 alarm value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARMLO_SPEC;
impl crate::RegisterSpec for ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmlo::R`](R) reader structure"]
impl crate::Readable for ALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarmlo::W`](W) writer structure"]
impl crate::Writable for ALARMLO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALARMLO to value 0"]
impl crate::Resettable for ALARMLO_SPEC {}
