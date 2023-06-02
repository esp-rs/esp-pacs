#[doc = "Register `GLITCH_FILTER_CH%s` reader"]
pub struct R(crate::R<GLITCH_FILTER_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLITCH_FILTER_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLITCH_FILTER_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLITCH_FILTER_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLITCH_FILTER_CH%s` writer"]
pub struct W(crate::W<GLITCH_FILTER_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLITCH_FILTER_CH_SPEC>;
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
impl From<crate::W<GLITCH_FILTER_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLITCH_FILTER_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_CH0_EN` reader - Glitch Filter channel enable bit."]
pub type FILTER_CH0_EN_R = crate::BitReader;
#[doc = "Field `FILTER_CH0_EN` writer - Glitch Filter channel enable bit."]
pub type FILTER_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, GLITCH_FILTER_CH_SPEC, O>;
#[doc = "Field `FILTER_CH0_INPUT_IO_NUM` reader - Glitch Filter input io number."]
pub type FILTER_CH0_INPUT_IO_NUM_R = crate::FieldReader;
#[doc = "Field `FILTER_CH0_INPUT_IO_NUM` writer - Glitch Filter input io number."]
pub type FILTER_CH0_INPUT_IO_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, GLITCH_FILTER_CH_SPEC, 6, O>;
#[doc = "Field `FILTER_CH0_WINDOW_THRES` reader - Glitch Filter window threshold."]
pub type FILTER_CH0_WINDOW_THRES_R = crate::FieldReader;
#[doc = "Field `FILTER_CH0_WINDOW_THRES` writer - Glitch Filter window threshold."]
pub type FILTER_CH0_WINDOW_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, GLITCH_FILTER_CH_SPEC, 6, O>;
#[doc = "Field `FILTER_CH0_WINDOW_WIDTH` reader - Glitch Filter window width."]
pub type FILTER_CH0_WINDOW_WIDTH_R = crate::FieldReader;
#[doc = "Field `FILTER_CH0_WINDOW_WIDTH` writer - Glitch Filter window width."]
pub type FILTER_CH0_WINDOW_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, GLITCH_FILTER_CH_SPEC, 6, O>;
impl R {
    #[doc = "Bit 0 - Glitch Filter channel enable bit."]
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> FILTER_CH0_EN_R {
        FILTER_CH0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Glitch Filter input io number."]
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> FILTER_CH0_INPUT_IO_NUM_R {
        FILTER_CH0_INPUT_IO_NUM_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:12 - Glitch Filter window threshold."]
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> FILTER_CH0_WINDOW_THRES_R {
        FILTER_CH0_WINDOW_THRES_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - Glitch Filter window width."]
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> FILTER_CH0_WINDOW_WIDTH_R {
        FILTER_CH0_WINDOW_WIDTH_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLITCH_FILTER_CH")
            .field(
                "filter_ch0_en",
                &format_args!("{}", self.filter_ch0_en().bit()),
            )
            .field(
                "filter_ch0_input_io_num",
                &format_args!("{}", self.filter_ch0_input_io_num().bits()),
            )
            .field(
                "filter_ch0_window_thres",
                &format_args!("{}", self.filter_ch0_window_thres().bits()),
            )
            .field(
                "filter_ch0_window_width",
                &format_args!("{}", self.filter_ch0_window_width().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GLITCH_FILTER_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Glitch Filter channel enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn filter_ch0_en(&mut self) -> FILTER_CH0_EN_W<0> {
        FILTER_CH0_EN_W::new(self)
    }
    #[doc = "Bits 1:6 - Glitch Filter input io number."]
    #[inline(always)]
    #[must_use]
    pub fn filter_ch0_input_io_num(&mut self) -> FILTER_CH0_INPUT_IO_NUM_W<1> {
        FILTER_CH0_INPUT_IO_NUM_W::new(self)
    }
    #[doc = "Bits 7:12 - Glitch Filter window threshold."]
    #[inline(always)]
    #[must_use]
    pub fn filter_ch0_window_thres(&mut self) -> FILTER_CH0_WINDOW_THRES_W<7> {
        FILTER_CH0_WINDOW_THRES_W::new(self)
    }
    #[doc = "Bits 13:18 - Glitch Filter window width."]
    #[inline(always)]
    #[must_use]
    pub fn filter_ch0_window_width(&mut self) -> FILTER_CH0_WINDOW_WIDTH_W<13> {
        FILTER_CH0_WINDOW_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Glitch Filter Configure Register of Channel%s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glitch_filter_ch](index.html) module"]
pub struct GLITCH_FILTER_CH_SPEC;
impl crate::RegisterSpec for GLITCH_FILTER_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glitch_filter_ch::R](R) reader structure"]
impl crate::Readable for GLITCH_FILTER_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glitch_filter_ch::W](W) writer structure"]
impl crate::Writable for GLITCH_FILTER_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLITCH_FILTER_CH%s to value 0"]
impl crate::Resettable for GLITCH_FILTER_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
