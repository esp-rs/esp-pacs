#[doc = "Register `T0ALARMLO` reader"]
pub type R = crate::R<T0ALARMLO_SPEC>;
#[doc = "Register `T0ALARMLO` writer"]
pub type W = crate::W<T0ALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0ALARMLO")
            .field("alarm_lo", &format_args!("{}", self.alarm_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0ALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<T0ALARMLO_SPEC> {
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
#[doc = "Timer %s alarm value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0alarmlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0alarmlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0ALARMLO_SPEC;
impl crate::RegisterSpec for T0ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0alarmlo::R`](R) reader structure"]
impl crate::Readable for T0ALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0alarmlo::W`](W) writer structure"]
impl crate::Writable for T0ALARMLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0ALARMLO to value 0"]
impl crate::Resettable for T0ALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
