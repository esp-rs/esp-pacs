#[doc = "Register `T0ALARMLO` reader"]
pub type R = crate::R<T0ALARMLO_SPEC>;
#[doc = "Register `T0ALARMLO` writer"]
pub type W = crate::W<T0ALARMLO_SPEC>;
#[doc = "Field `T_ALARM_LO` reader - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
pub type T_ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `T_ALARM_LO` writer - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
pub type T_ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t_alarm_lo(&self) -> T_ALARM_LO_R {
        T_ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0ALARMLO")
            .field("t_alarm_lo", &self.t_alarm_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the low 32 bits of timer 0 alarm trigger time-base counter value. Valid only when TIMG_T0_ALARM_EN is 1. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t_alarm_lo(&mut self) -> T_ALARM_LO_W<'_, T0ALARMLO_SPEC> {
        T_ALARM_LO_W::new(self, 0)
    }
}
#[doc = "Timer 0 alarm value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`t0alarmlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0alarmlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0ALARMLO_SPEC;
impl crate::RegisterSpec for T0ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0alarmlo::R`](R) reader structure"]
impl crate::Readable for T0ALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0alarmlo::W`](W) writer structure"]
impl crate::Writable for T0ALARMLO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0ALARMLO to value 0"]
impl crate::Resettable for T0ALARMLO_SPEC {}
