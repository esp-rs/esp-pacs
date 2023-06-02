#[doc = "Register `TOUCH_DAC` reader"]
pub struct R(crate::R<TOUCH_DAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_DAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_DAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_DAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_DAC` writer"]
pub struct W(crate::W<TOUCH_DAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_DAC_SPEC>;
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
impl From<crate::W<TOUCH_DAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_DAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD9_DAC` reader - configure touch pad dac9"]
pub type TOUCH_PAD9_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD9_DAC` writer - configure touch pad dac9"]
pub type TOUCH_PAD9_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD8_DAC` reader - configure touch pad dac8"]
pub type TOUCH_PAD8_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD8_DAC` writer - configure touch pad dac8"]
pub type TOUCH_PAD8_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD7_DAC` reader - configure touch pad dac7"]
pub type TOUCH_PAD7_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD7_DAC` writer - configure touch pad dac7"]
pub type TOUCH_PAD7_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD6_DAC` reader - configure touch pad dac6"]
pub type TOUCH_PAD6_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD6_DAC` writer - configure touch pad dac6"]
pub type TOUCH_PAD6_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD5_DAC` reader - configure touch pad dac5"]
pub type TOUCH_PAD5_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD5_DAC` writer - configure touch pad dac5"]
pub type TOUCH_PAD5_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD4_DAC` reader - configure touch pad dac4"]
pub type TOUCH_PAD4_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD4_DAC` writer - configure touch pad dac4"]
pub type TOUCH_PAD4_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD3_DAC` reader - configure touch pad dac3"]
pub type TOUCH_PAD3_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD3_DAC` writer - configure touch pad dac3"]
pub type TOUCH_PAD3_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD2_DAC` reader - configure touch pad dac2"]
pub type TOUCH_PAD2_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD2_DAC` writer - configure touch pad dac2"]
pub type TOUCH_PAD2_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD1_DAC` reader - configure touch pad dac1"]
pub type TOUCH_PAD1_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD1_DAC` writer - configure touch pad dac1"]
pub type TOUCH_PAD1_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD0_DAC` reader - configure touch pad dac0"]
pub type TOUCH_PAD0_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD0_DAC` writer - configure touch pad dac0"]
pub type TOUCH_PAD0_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_DAC_SPEC, 3, O>;
impl R {
    #[doc = "Bits 2:4 - configure touch pad dac9"]
    #[inline(always)]
    pub fn touch_pad9_dac(&self) -> TOUCH_PAD9_DAC_R {
        TOUCH_PAD9_DAC_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - configure touch pad dac8"]
    #[inline(always)]
    pub fn touch_pad8_dac(&self) -> TOUCH_PAD8_DAC_R {
        TOUCH_PAD8_DAC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - configure touch pad dac7"]
    #[inline(always)]
    pub fn touch_pad7_dac(&self) -> TOUCH_PAD7_DAC_R {
        TOUCH_PAD7_DAC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - configure touch pad dac6"]
    #[inline(always)]
    pub fn touch_pad6_dac(&self) -> TOUCH_PAD6_DAC_R {
        TOUCH_PAD6_DAC_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - configure touch pad dac5"]
    #[inline(always)]
    pub fn touch_pad5_dac(&self) -> TOUCH_PAD5_DAC_R {
        TOUCH_PAD5_DAC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - configure touch pad dac4"]
    #[inline(always)]
    pub fn touch_pad4_dac(&self) -> TOUCH_PAD4_DAC_R {
        TOUCH_PAD4_DAC_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - configure touch pad dac3"]
    #[inline(always)]
    pub fn touch_pad3_dac(&self) -> TOUCH_PAD3_DAC_R {
        TOUCH_PAD3_DAC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - configure touch pad dac2"]
    #[inline(always)]
    pub fn touch_pad2_dac(&self) -> TOUCH_PAD2_DAC_R {
        TOUCH_PAD2_DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - configure touch pad dac1"]
    #[inline(always)]
    pub fn touch_pad1_dac(&self) -> TOUCH_PAD1_DAC_R {
        TOUCH_PAD1_DAC_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - configure touch pad dac0"]
    #[inline(always)]
    pub fn touch_pad0_dac(&self) -> TOUCH_PAD0_DAC_R {
        TOUCH_PAD0_DAC_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_DAC")
            .field(
                "touch_pad9_dac",
                &format_args!("{}", self.touch_pad9_dac().bits()),
            )
            .field(
                "touch_pad8_dac",
                &format_args!("{}", self.touch_pad8_dac().bits()),
            )
            .field(
                "touch_pad7_dac",
                &format_args!("{}", self.touch_pad7_dac().bits()),
            )
            .field(
                "touch_pad6_dac",
                &format_args!("{}", self.touch_pad6_dac().bits()),
            )
            .field(
                "touch_pad5_dac",
                &format_args!("{}", self.touch_pad5_dac().bits()),
            )
            .field(
                "touch_pad4_dac",
                &format_args!("{}", self.touch_pad4_dac().bits()),
            )
            .field(
                "touch_pad3_dac",
                &format_args!("{}", self.touch_pad3_dac().bits()),
            )
            .field(
                "touch_pad2_dac",
                &format_args!("{}", self.touch_pad2_dac().bits()),
            )
            .field(
                "touch_pad1_dac",
                &format_args!("{}", self.touch_pad1_dac().bits()),
            )
            .field(
                "touch_pad0_dac",
                &format_args!("{}", self.touch_pad0_dac().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_DAC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:4 - configure touch pad dac9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad9_dac(&mut self) -> TOUCH_PAD9_DAC_W<2> {
        TOUCH_PAD9_DAC_W::new(self)
    }
    #[doc = "Bits 5:7 - configure touch pad dac8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad8_dac(&mut self) -> TOUCH_PAD8_DAC_W<5> {
        TOUCH_PAD8_DAC_W::new(self)
    }
    #[doc = "Bits 8:10 - configure touch pad dac7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad7_dac(&mut self) -> TOUCH_PAD7_DAC_W<8> {
        TOUCH_PAD7_DAC_W::new(self)
    }
    #[doc = "Bits 11:13 - configure touch pad dac6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad6_dac(&mut self) -> TOUCH_PAD6_DAC_W<11> {
        TOUCH_PAD6_DAC_W::new(self)
    }
    #[doc = "Bits 14:16 - configure touch pad dac5"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad5_dac(&mut self) -> TOUCH_PAD5_DAC_W<14> {
        TOUCH_PAD5_DAC_W::new(self)
    }
    #[doc = "Bits 17:19 - configure touch pad dac4"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad4_dac(&mut self) -> TOUCH_PAD4_DAC_W<17> {
        TOUCH_PAD4_DAC_W::new(self)
    }
    #[doc = "Bits 20:22 - configure touch pad dac3"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad3_dac(&mut self) -> TOUCH_PAD3_DAC_W<20> {
        TOUCH_PAD3_DAC_W::new(self)
    }
    #[doc = "Bits 23:25 - configure touch pad dac2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad2_dac(&mut self) -> TOUCH_PAD2_DAC_W<23> {
        TOUCH_PAD2_DAC_W::new(self)
    }
    #[doc = "Bits 26:28 - configure touch pad dac1"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad1_dac(&mut self) -> TOUCH_PAD1_DAC_W<26> {
        TOUCH_PAD1_DAC_W::new(self)
    }
    #[doc = "Bits 29:31 - configure touch pad dac0"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_dac(&mut self) -> TOUCH_PAD0_DAC_W<29> {
        TOUCH_PAD0_DAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch dac\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_dac](index.html) module"]
pub struct TOUCH_DAC_SPEC;
impl crate::RegisterSpec for TOUCH_DAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_dac::R](R) reader structure"]
impl crate::Readable for TOUCH_DAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_dac::W](W) writer structure"]
impl crate::Writable for TOUCH_DAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_DAC to value 0"]
impl crate::Resettable for TOUCH_DAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
