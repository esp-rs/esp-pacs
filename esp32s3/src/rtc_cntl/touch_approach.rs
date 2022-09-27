#[doc = "Register `TOUCH_APPROACH` reader"]
pub struct R(crate::R<TOUCH_APPROACH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_APPROACH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_APPROACH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_APPROACH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_APPROACH` writer"]
pub struct W(crate::W<TOUCH_APPROACH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_APPROACH_SPEC>;
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
impl From<crate::W<TOUCH_APPROACH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_APPROACH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SLP_CHANNEL_CLR` writer - clear touch slp channel"]
pub type TOUCH_SLP_CHANNEL_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TOUCH_APPROACH_SPEC, bool, O>;
#[doc = "Field `MEAS_TIME` reader - approach pads total meas times"]
pub type MEAS_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEAS_TIME` writer - approach pads total meas times"]
pub type MEAS_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOUCH_APPROACH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - approach pads total meas times"]
    #[inline(always)]
    pub fn meas_time(&self) -> MEAS_TIME_R {
        MEAS_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - clear touch slp channel"]
    #[inline(always)]
    pub fn touch_slp_channel_clr(&mut self) -> TOUCH_SLP_CHANNEL_CLR_W<23> {
        TOUCH_SLP_CHANNEL_CLR_W::new(self)
    }
    #[doc = "Bits 24:31 - approach pads total meas times"]
    #[inline(always)]
    pub fn meas_time(&mut self) -> MEAS_TIME_W<24> {
        MEAS_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_approach](index.html) module"]
pub struct TOUCH_APPROACH_SPEC;
impl crate::RegisterSpec for TOUCH_APPROACH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_approach::R](R) reader structure"]
impl crate::Readable for TOUCH_APPROACH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_approach::W](W) writer structure"]
impl crate::Writable for TOUCH_APPROACH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_APPROACH to value 0x5000_0000"]
impl crate::Resettable for TOUCH_APPROACH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000_0000
    }
}
