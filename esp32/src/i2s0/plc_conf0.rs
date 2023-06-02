#[doc = "Register `PLC_CONF0` reader"]
pub struct R(crate::R<PLC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLC_CONF0` writer"]
pub struct W(crate::W<PLC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLC_CONF0_SPEC>;
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
impl From<crate::W<PLC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GOOD_PACK_MAX` reader - "]
pub type GOOD_PACK_MAX_R = crate::FieldReader;
#[doc = "Field `GOOD_PACK_MAX` writer - "]
pub type GOOD_PACK_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF0_SPEC, 6, O>;
#[doc = "Field `N_ERR_SEG` reader - "]
pub type N_ERR_SEG_R = crate::FieldReader;
#[doc = "Field `N_ERR_SEG` writer - "]
pub type N_ERR_SEG_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF0_SPEC, 3, O>;
#[doc = "Field `SHIFT_RATE` reader - "]
pub type SHIFT_RATE_R = crate::FieldReader;
#[doc = "Field `SHIFT_RATE` writer - "]
pub type SHIFT_RATE_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF0_SPEC, 3, O>;
#[doc = "Field `MAX_SLIDE_SAMPLE` reader - "]
pub type MAX_SLIDE_SAMPLE_R = crate::FieldReader;
#[doc = "Field `MAX_SLIDE_SAMPLE` writer - "]
pub type MAX_SLIDE_SAMPLE_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF0_SPEC, 8, O>;
#[doc = "Field `PACK_LEN_8K` reader - "]
pub type PACK_LEN_8K_R = crate::FieldReader;
#[doc = "Field `PACK_LEN_8K` writer - "]
pub type PACK_LEN_8K_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF0_SPEC, 5, O>;
#[doc = "Field `N_MIN_ERR` reader - "]
pub type N_MIN_ERR_R = crate::FieldReader;
#[doc = "Field `N_MIN_ERR` writer - "]
pub type N_MIN_ERR_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF0_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn good_pack_max(&self) -> GOOD_PACK_MAX_R {
        GOOD_PACK_MAX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn n_err_seg(&self) -> N_ERR_SEG_R {
        N_ERR_SEG_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn shift_rate(&self) -> SHIFT_RATE_R {
        SHIFT_RATE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn max_slide_sample(&self) -> MAX_SLIDE_SAMPLE_R {
        MAX_SLIDE_SAMPLE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pack_len_8k(&self) -> PACK_LEN_8K_R {
        PACK_LEN_8K_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn n_min_err(&self) -> N_MIN_ERR_R {
        N_MIN_ERR_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLC_CONF0")
            .field(
                "good_pack_max",
                &format_args!("{}", self.good_pack_max().bits()),
            )
            .field("n_err_seg", &format_args!("{}", self.n_err_seg().bits()))
            .field("shift_rate", &format_args!("{}", self.shift_rate().bits()))
            .field(
                "max_slide_sample",
                &format_args!("{}", self.max_slide_sample().bits()),
            )
            .field(
                "pack_len_8k",
                &format_args!("{}", self.pack_len_8k().bits()),
            )
            .field("n_min_err", &format_args!("{}", self.n_min_err().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn good_pack_max(&mut self) -> GOOD_PACK_MAX_W<0> {
        GOOD_PACK_MAX_W::new(self)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn n_err_seg(&mut self) -> N_ERR_SEG_W<6> {
        N_ERR_SEG_W::new(self)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn shift_rate(&mut self) -> SHIFT_RATE_W<9> {
        SHIFT_RATE_W::new(self)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    #[must_use]
    pub fn max_slide_sample(&mut self) -> MAX_SLIDE_SAMPLE_W<12> {
        MAX_SLIDE_SAMPLE_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn pack_len_8k(&mut self) -> PACK_LEN_8K_W<20> {
        PACK_LEN_8K_W::new(self)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn n_min_err(&mut self) -> N_MIN_ERR_W<25> {
        N_MIN_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc_conf0](index.html) module"]
pub struct PLC_CONF0_SPEC;
impl crate::RegisterSpec for PLC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plc_conf0::R](R) reader structure"]
impl crate::Readable for PLC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plc_conf0::W](W) writer structure"]
impl crate::Writable for PLC_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLC_CONF0 to value 0x08a8_0339"]
impl crate::Resettable for PLC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x08a8_0339;
}
