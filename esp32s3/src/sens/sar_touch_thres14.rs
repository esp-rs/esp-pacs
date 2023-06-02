#[doc = "Register `SAR_TOUCH_THRES14` reader"]
pub struct R(crate::R<SAR_TOUCH_THRES14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_THRES14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_THRES14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_THRES14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_THRES14` writer"]
pub struct W(crate::W<SAR_TOUCH_THRES14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_THRES14_SPEC>;
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
impl From<crate::W<SAR_TOUCH_THRES14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_THRES14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_TOUCH_OUT_TH14` reader - Finger threshold for touch pad 14"]
pub type SAR_TOUCH_OUT_TH14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH14` writer - Finger threshold for touch pad 14"]
pub type SAR_TOUCH_OUT_TH14_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_THRES14_SPEC, 22, O, u32, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 14"]
    #[inline(always)]
    pub fn sar_touch_out_th14(&self) -> SAR_TOUCH_OUT_TH14_R {
        SAR_TOUCH_OUT_TH14_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES14")
            .field(
                "sar_touch_out_th14",
                &format_args!("{}", self.sar_touch_out_th14().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 14"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_out_th14(&mut self) -> SAR_TOUCH_OUT_TH14_W<0> {
        SAR_TOUCH_OUT_TH14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch thres of touch pad\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_thres14](index.html) module"]
pub struct SAR_TOUCH_THRES14_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_thres14::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres14::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES14 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
