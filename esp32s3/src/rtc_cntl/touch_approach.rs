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
pub type TOUCH_SLP_CHANNEL_CLR_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_APPROACH_SPEC, O>;
#[doc = "Field `TOUCH_APPROACH_MEAS_TIME` reader - approach pads total meas times"]
pub type TOUCH_APPROACH_MEAS_TIME_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_MEAS_TIME` writer - approach pads total meas times"]
pub type TOUCH_APPROACH_MEAS_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_APPROACH_SPEC, 8, O>;
impl R {
    #[doc = "Bits 24:31 - approach pads total meas times"]
    #[inline(always)]
    pub fn touch_approach_meas_time(&self) -> TOUCH_APPROACH_MEAS_TIME_R {
        TOUCH_APPROACH_MEAS_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_APPROACH")
            .field(
                "touch_approach_meas_time",
                &format_args!("{}", self.touch_approach_meas_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_APPROACH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 23 - clear touch slp channel"]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_channel_clr(&mut self) -> TOUCH_SLP_CHANNEL_CLR_W<23> {
        TOUCH_SLP_CHANNEL_CLR_W::new(self)
    }
    #[doc = "Bits 24:31 - approach pads total meas times"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_meas_time(&mut self) -> TOUCH_APPROACH_MEAS_TIME_W<24> {
        TOUCH_APPROACH_MEAS_TIME_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_APPROACH to value 0x5000_0000"]
impl crate::Resettable for TOUCH_APPROACH_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000_0000;
}
