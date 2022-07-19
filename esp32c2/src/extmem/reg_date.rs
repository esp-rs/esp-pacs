#[doc = "Register `REG_DATE` reader"]
pub struct R(crate::R<REG_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_DATE` writer"]
pub struct W(crate::W<REG_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_DATE_SPEC>;
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
impl From<crate::W<REG_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATE` reader - version information"]
pub type DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATE` writer - version information"]
pub type DATE_W<'a> = crate::FieldWriter<'a, u32, REG_DATE_SPEC, u32, u32, 28, 0>;
impl R {
    #[doc = "Bits 0:27 - version information"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - version information"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W {
        DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_date](index.html) module"]
pub struct REG_DATE_SPEC;
impl crate::RegisterSpec for REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_date::R](R) reader structure"]
impl crate::Readable for REG_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_date::W](W) writer structure"]
impl crate::Writable for REG_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_DATE to value 0x0210_7050"]
impl crate::Resettable for REG_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_7050
    }
}
