#[doc = "Register `SAR_TOUCH_THRES13` reader"]
pub struct R(crate::R<SAR_TOUCH_THRES13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_THRES13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_THRES13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_THRES13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_THRES13` writer"]
pub struct W(crate::W<SAR_TOUCH_THRES13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_THRES13_SPEC>;
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
impl From<crate::W<SAR_TOUCH_THRES13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_THRES13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_TOUCH_OUT_TH13` reader - Finger threshold for touch pad 13"]
pub type SAR_TOUCH_OUT_TH13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH13` writer - Finger threshold for touch pad 13"]
pub type SAR_TOUCH_OUT_TH13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TOUCH_THRES13_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 13"]
    #[inline(always)]
    pub fn sar_touch_out_th13(&self) -> SAR_TOUCH_OUT_TH13_R {
        SAR_TOUCH_OUT_TH13_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 13"]
    #[inline(always)]
    pub fn sar_touch_out_th13(&mut self) -> SAR_TOUCH_OUT_TH13_W<0> {
        SAR_TOUCH_OUT_TH13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch thres of touch pad\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_thres13](index.html) module"]
pub struct SAR_TOUCH_THRES13_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_thres13::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres13::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES13 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
