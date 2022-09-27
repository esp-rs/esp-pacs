#[doc = "Register `SENSITIVE_REG_DATE` reader"]
pub struct R(crate::R<SENSITIVE_REG_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSITIVE_REG_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSITIVE_REG_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSITIVE_REG_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSITIVE_REG_DATE` writer"]
pub struct W(crate::W<SENSITIVE_REG_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSITIVE_REG_DATE_SPEC>;
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
impl From<crate::W<SENSITIVE_REG_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSITIVE_REG_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSITIVE_REG_DATE` reader - Need add description"]
pub type SENSITIVE_REG_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SENSITIVE_REG_DATE` writer - Need add description"]
pub type SENSITIVE_REG_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSITIVE_REG_DATE_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Need add description"]
    #[inline(always)]
    pub fn sensitive_reg_date(&self) -> SENSITIVE_REG_DATE_R {
        SENSITIVE_REG_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Need add description"]
    #[inline(always)]
    pub fn sensitive_reg_date(&mut self) -> SENSITIVE_REG_DATE_W<0> {
        SENSITIVE_REG_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensitive_reg_date](index.html) module"]
pub struct SENSITIVE_REG_DATE_SPEC;
impl crate::RegisterSpec for SENSITIVE_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensitive_reg_date::R](R) reader structure"]
impl crate::Readable for SENSITIVE_REG_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensitive_reg_date::W](W) writer structure"]
impl crate::Writable for SENSITIVE_REG_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSITIVE_REG_DATE to value 0x0210_6301"]
impl crate::Resettable for SENSITIVE_REG_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_6301
    }
}
