#[doc = "Register `LACTALARMHI` reader"]
pub struct R(crate::R<LACTALARMHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTALARMHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTALARMHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTALARMHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTALARMHI` writer"]
pub struct W(crate::W<LACTALARMHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTALARMHI_SPEC>;
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
impl From<crate::W<LACTALARMHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTALARMHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_ALARM_HI` reader - "]
pub type LACT_ALARM_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LACT_ALARM_HI` writer - "]
pub type LACT_ALARM_HI_W<'a, const O: u8> =
    crate::FieldWriter<'a, LACTALARMHI_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_alarm_hi(&self) -> LACT_ALARM_HI_R {
        LACT_ALARM_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMHI")
            .field(
                "lact_alarm_hi",
                &format_args!("{}", self.lact_alarm_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_alarm_hi(&mut self) -> LACT_ALARM_HI_W<0> {
        LACT_ALARM_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactalarmhi](index.html) module"]
pub struct LACTALARMHI_SPEC;
impl crate::RegisterSpec for LACTALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactalarmhi::R](R) reader structure"]
impl crate::Readable for LACTALARMHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactalarmhi::W](W) writer structure"]
impl crate::Writable for LACTALARMHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTALARMHI to value 0"]
impl crate::Resettable for LACTALARMHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
