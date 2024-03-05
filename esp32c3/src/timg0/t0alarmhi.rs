#[doc = "Register `T0ALARMHI` reader"]
pub type R = crate::R<T0ALARMHI_SPEC>;
#[doc = "Register `T0ALARMHI` writer"]
pub type W = crate::W<T0ALARMHI_SPEC>;
#[doc = "Field `ALARM_HI` reader - reg_t0_alarm_hi."]
pub type ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - reg_t0_alarm_hi."]
pub type ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - reg_t0_alarm_hi."]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0ALARMHI")
            .field("alarm_hi", &format_args!("{}", self.alarm_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0ALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - reg_t0_alarm_hi."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W<T0ALARMHI_SPEC> {
        ALARM_HI_W::new(self, 0)
    }
}
#[doc = "TIMG_T0ALARMHI_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0ALARMHI_SPEC;
impl crate::RegisterSpec for T0ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0alarmhi::R`](R) reader structure"]
impl crate::Readable for T0ALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0alarmhi::W`](W) writer structure"]
impl crate::Writable for T0ALARMHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0ALARMHI to value 0"]
impl crate::Resettable for T0ALARMHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
