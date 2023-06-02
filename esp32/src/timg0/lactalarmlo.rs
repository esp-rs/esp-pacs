#[doc = "Register `LACTALARMLO` reader"]
pub struct R(crate::R<LACTALARMLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTALARMLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTALARMLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTALARMLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTALARMLO` writer"]
pub struct W(crate::W<LACTALARMLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTALARMLO_SPEC>;
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
impl From<crate::W<LACTALARMLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTALARMLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_ALARM_LO` reader - "]
pub type LACT_ALARM_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LACT_ALARM_LO` writer - "]
pub type LACT_ALARM_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, LACTALARMLO_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_alarm_lo(&self) -> LACT_ALARM_LO_R {
        LACT_ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMLO")
            .field(
                "lact_alarm_lo",
                &format_args!("{}", self.lact_alarm_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_alarm_lo(&mut self) -> LACT_ALARM_LO_W<0> {
        LACT_ALARM_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactalarmlo](index.html) module"]
pub struct LACTALARMLO_SPEC;
impl crate::RegisterSpec for LACTALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactalarmlo::R](R) reader structure"]
impl crate::Readable for LACTALARMLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactalarmlo::W](W) writer structure"]
impl crate::Writable for LACTALARMLO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTALARMLO to value 0"]
impl crate::Resettable for LACTALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
