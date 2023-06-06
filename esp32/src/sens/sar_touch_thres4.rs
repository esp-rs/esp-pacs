#[doc = "Register `SAR_TOUCH_THRES4` reader"]
pub struct R(crate::R<SAR_TOUCH_THRES4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_THRES4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_THRES4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_THRES4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_THRES4` writer"]
pub struct W(crate::W<SAR_TOUCH_THRES4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_THRES4_SPEC>;
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
impl From<crate::W<SAR_TOUCH_THRES4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_THRES4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_OUT_TH7` reader - the threshold for touch pad 7"]
pub type TOUCH_OUT_TH7_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH7` writer - the threshold for touch pad 7"]
pub type TOUCH_OUT_TH7_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_THRES4_SPEC, 16, O, u16>;
#[doc = "Field `TOUCH_OUT_TH6` reader - the threshold for touch pad 6"]
pub type TOUCH_OUT_TH6_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH6` writer - the threshold for touch pad 6"]
pub type TOUCH_OUT_TH6_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_THRES4_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - the threshold for touch pad 7"]
    #[inline(always)]
    pub fn touch_out_th7(&self) -> TOUCH_OUT_TH7_R {
        TOUCH_OUT_TH7_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 6"]
    #[inline(always)]
    pub fn touch_out_th6(&self) -> TOUCH_OUT_TH6_R {
        TOUCH_OUT_TH6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES4")
            .field(
                "touch_out_th7",
                &format_args!("{}", self.touch_out_th7().bits()),
            )
            .field(
                "touch_out_th6",
                &format_args!("{}", self.touch_out_th6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - the threshold for touch pad 7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th7(&mut self) -> TOUCH_OUT_TH7_W<0> {
        TOUCH_OUT_TH7_W::new(self)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th6(&mut self) -> TOUCH_OUT_TH6_W<16> {
        TOUCH_OUT_TH6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_thres4](index.html) module"]
pub struct SAR_TOUCH_THRES4_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_thres4::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres4::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES4 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
