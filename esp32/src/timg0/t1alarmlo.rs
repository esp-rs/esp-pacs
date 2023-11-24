#[doc = "Register `T1ALARMLO` reader"]
pub type R = crate::R<T1ALARMLO_SPEC>;
#[doc = "Register `T1ALARMLO` writer"]
pub type W = crate::W<T1ALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
pub type ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1ALARMLO")
            .field("alarm_lo", &format_args!("{}", self.alarm_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1ALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<T1ALARMLO_SPEC> {
        ALARM_LO_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1alarmlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1alarmlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1ALARMLO_SPEC;
impl crate::RegisterSpec for T1ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1alarmlo::R`](R) reader structure"]
impl crate::Readable for T1ALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t1alarmlo::W`](W) writer structure"]
impl crate::Writable for T1ALARMLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1ALARMLO to value 0"]
impl crate::Resettable for T1ALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
