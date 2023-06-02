#[doc = "Register `FRC2_ALARM` reader"]
pub struct R(crate::R<FRC2_ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRC2_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRC2_ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRC2_ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRC2_ALARM` writer"]
pub struct W(crate::W<FRC2_ALARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRC2_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FRC2_ALARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRC2_ALARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frc2_alarm` reader - the alarm value for the counter"]
pub type FRC2_ALARM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `frc2_alarm` writer - the alarm value for the counter"]
pub type FRC2_ALARM_W<'a, const O: u8> = crate::FieldWriter<'a, FRC2_ALARM_SPEC, 32, O, u32, u32>;
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
    pub fn frc2_alarm(&mut self) -> FRC2_ALARM_W<0> {
        FRC2_ALARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the alarm value for the counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frc2_alarm](index.html) module"]
pub struct FRC2_ALARM_SPEC;
impl crate::RegisterSpec for FRC2_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frc2_alarm::R](R) reader structure"]
impl crate::Readable for FRC2_ALARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frc2_alarm::W](W) writer structure"]
impl crate::Writable for FRC2_ALARM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_ALARM to value 0"]
impl crate::Resettable for FRC2_ALARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
