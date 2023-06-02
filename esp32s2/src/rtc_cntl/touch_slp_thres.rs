#[doc = "Register `TOUCH_SLP_THRES` reader"]
pub struct R(crate::R<TOUCH_SLP_THRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_SLP_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_SLP_THRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_SLP_THRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_SLP_THRES` writer"]
pub struct W(crate::W<TOUCH_SLP_THRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_SLP_THRES_SPEC>;
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
impl From<crate::W<TOUCH_SLP_THRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_SLP_THRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SLP_TH` reader - Set the threshold for touch sleep pad."]
pub type TOUCH_SLP_TH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOUCH_SLP_TH` writer - Set the threshold for touch sleep pad."]
pub type TOUCH_SLP_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, TOUCH_SLP_THRES_SPEC, 22, O, u32, u32>;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` reader - Enable the proximity mode of touch sleep pad."]
pub type TOUCH_SLP_APPROACH_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` writer - Enable the proximity mode of touch sleep pad."]
pub type TOUCH_SLP_APPROACH_EN_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_SLP_THRES_SPEC, O>;
#[doc = "Field `TOUCH_SLP_PAD` reader - Select sleep pad."]
pub type TOUCH_SLP_PAD_R = crate::FieldReader;
#[doc = "Field `TOUCH_SLP_PAD` writer - Select sleep pad."]
pub type TOUCH_SLP_PAD_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_SLP_THRES_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:21 - Set the threshold for touch sleep pad."]
    #[inline(always)]
    pub fn touch_slp_th(&self) -> TOUCH_SLP_TH_R {
        TOUCH_SLP_TH_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 26 - Enable the proximity mode of touch sleep pad."]
    #[inline(always)]
    pub fn touch_slp_approach_en(&self) -> TOUCH_SLP_APPROACH_EN_R {
        TOUCH_SLP_APPROACH_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Select sleep pad."]
    #[inline(always)]
    pub fn touch_slp_pad(&self) -> TOUCH_SLP_PAD_R {
        TOUCH_SLP_PAD_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_SLP_THRES")
            .field(
                "touch_slp_th",
                &format_args!("{}", self.touch_slp_th().bits()),
            )
            .field(
                "touch_slp_approach_en",
                &format_args!("{}", self.touch_slp_approach_en().bit()),
            )
            .field(
                "touch_slp_pad",
                &format_args!("{}", self.touch_slp_pad().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_SLP_THRES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Set the threshold for touch sleep pad."]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_th(&mut self) -> TOUCH_SLP_TH_W<0> {
        TOUCH_SLP_TH_W::new(self)
    }
    #[doc = "Bit 26 - Enable the proximity mode of touch sleep pad."]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_approach_en(&mut self) -> TOUCH_SLP_APPROACH_EN_W<26> {
        TOUCH_SLP_APPROACH_EN_W::new(self)
    }
    #[doc = "Bits 27:31 - Select sleep pad."]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_pad(&mut self) -> TOUCH_SLP_PAD_W<27> {
        TOUCH_SLP_PAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the settings of touch sleep pad\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_slp_thres](index.html) module"]
pub struct TOUCH_SLP_THRES_SPEC;
impl crate::RegisterSpec for TOUCH_SLP_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_slp_thres::R](R) reader structure"]
impl crate::Readable for TOUCH_SLP_THRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_slp_thres::W](W) writer structure"]
impl crate::Writable for TOUCH_SLP_THRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_SLP_THRES to value 0x7800_0000"]
impl crate::Resettable for TOUCH_SLP_THRES_SPEC {
    const RESET_VALUE: Self::Ux = 0x7800_0000;
}
