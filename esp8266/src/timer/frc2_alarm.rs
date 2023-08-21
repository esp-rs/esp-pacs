#[doc = "Register `FRC2_ALARM` reader"]
pub type R = crate::R<FRC2_ALARM_SPEC>;
#[doc = "Register `FRC2_ALARM` writer"]
pub type W = crate::W<FRC2_ALARM_SPEC>;
#[doc = "Field `frc2_alarm` reader - the alarm value for the counter"]
pub type FRC2_ALARM_R = crate::FieldReader<u32>;
#[doc = "Field `frc2_alarm` writer - the alarm value for the counter"]
pub type FRC2_ALARM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - the alarm value for the counter"]
    #[inline(always)]
    pub fn frc2_alarm(&self) -> FRC2_ALARM_R {
        FRC2_ALARM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC2_ALARM")
            .field("frc2_alarm", &format_args!("{}", self.frc2_alarm().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC2_ALARM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the alarm value for the counter"]
    #[inline(always)]
    #[must_use]
    pub fn frc2_alarm(&mut self) -> FRC2_ALARM_W<FRC2_ALARM_SPEC, 0> {
        FRC2_ALARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "the alarm value for the counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_alarm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_alarm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC2_ALARM_SPEC;
impl crate::RegisterSpec for FRC2_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc2_alarm::R`](R) reader structure"]
impl crate::Readable for FRC2_ALARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frc2_alarm::W`](W) writer structure"]
impl crate::Writable for FRC2_ALARM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_ALARM to value 0"]
impl crate::Resettable for FRC2_ALARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
