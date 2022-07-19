#[doc = "Register `APB_CTRL_DATE` reader"]
pub struct R(crate::R<APB_CTRL_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_CTRL_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_CTRL_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_CTRL_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_CTRL_DATE` writer"]
pub struct W(crate::W<APB_CTRL_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_CTRL_DATE_SPEC>;
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
impl From<crate::W<APB_CTRL_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_CTRL_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATE` reader - Need add description"]
pub type DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATE` writer - Need add description"]
pub type DATE_W<'a> = crate::FieldWriter<'a, u32, APB_CTRL_DATE_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add description"]
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
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_date](index.html) module"]
pub struct APB_CTRL_DATE_SPEC;
impl crate::RegisterSpec for APB_CTRL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_ctrl_date::R](R) reader structure"]
impl crate::Readable for APB_CTRL_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_date::W](W) writer structure"]
impl crate::Writable for APB_CTRL_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_CTRL_DATE to value 0x0210_7210"]
impl crate::Resettable for APB_CTRL_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_7210
    }
}
