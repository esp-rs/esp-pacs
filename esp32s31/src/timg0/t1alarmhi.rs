#[doc = "Register `T1ALARMHI` reader"]
pub type R = crate::R<T1ALARMHI_SPEC>;
#[doc = "Register `T1ALARMHI` writer"]
pub type W = crate::W<T1ALARMHI_SPEC>;
#[doc = "Field `T_ALARM_HI` reader - Configures the high 22 bits of timer 1 alarm trigger time-base counter value. Valid only when TIMG_T1_ALARM_EN is 1. \\\\ Measurement unit: T1_clk \\\\"]
pub type T_ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `T_ALARM_HI` writer - Configures the high 22 bits of timer 1 alarm trigger time-base counter value. Valid only when TIMG_T1_ALARM_EN is 1. \\\\ Measurement unit: T1_clk \\\\"]
pub type T_ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Configures the high 22 bits of timer 1 alarm trigger time-base counter value. Valid only when TIMG_T1_ALARM_EN is 1. \\\\ Measurement unit: T1_clk \\\\"]
    #[inline(always)]
    pub fn t_alarm_hi(&self) -> T_ALARM_HI_R {
        T_ALARM_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1ALARMHI")
            .field("t_alarm_hi", &self.t_alarm_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Configures the high 22 bits of timer 1 alarm trigger time-base counter value. Valid only when TIMG_T1_ALARM_EN is 1. \\\\ Measurement unit: T1_clk \\\\"]
    #[inline(always)]
    pub fn t_alarm_hi(&mut self) -> T_ALARM_HI_W<'_, T1ALARMHI_SPEC> {
        T_ALARM_HI_W::new(self, 0)
    }
}
#[doc = "Timer 1 alarm value, high bits\n\nYou can [`read`](crate::Reg::read) this register and get [`t1alarmhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t1alarmhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1ALARMHI_SPEC;
impl crate::RegisterSpec for T1ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1alarmhi::R`](R) reader structure"]
impl crate::Readable for T1ALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t1alarmhi::W`](W) writer structure"]
impl crate::Writable for T1ALARMHI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T1ALARMHI to value 0"]
impl crate::Resettable for T1ALARMHI_SPEC {}
