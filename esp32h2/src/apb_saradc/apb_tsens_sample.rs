#[doc = "Register `APB_TSENS_SAMPLE` reader"]
pub struct R(crate::R<APB_TSENS_SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_SAMPLE` writer"]
pub struct W(crate::W<APB_TSENS_SAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_SAMPLE_SPEC>;
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
impl From<crate::W<APB_TSENS_SAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_SAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_SAMPLE_RATE` reader - HW sample rate"]
pub type TSENS_SAMPLE_RATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSENS_SAMPLE_RATE` writer - HW sample rate"]
pub type TSENS_SAMPLE_RATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, APB_TSENS_SAMPLE_SPEC, 16, O, u16, u16>;
#[doc = "Field `TSENS_SAMPLE_EN` reader - HW sample en"]
pub type TSENS_SAMPLE_EN_R = crate::BitReader;
#[doc = "Field `TSENS_SAMPLE_EN` writer - HW sample en"]
pub type TSENS_SAMPLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_SAMPLE_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - HW sample rate"]
    #[inline(always)]
    pub fn tsens_sample_rate(&self) -> TSENS_SAMPLE_RATE_R {
        TSENS_SAMPLE_RATE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - HW sample en"]
    #[inline(always)]
    pub fn tsens_sample_en(&self) -> TSENS_SAMPLE_EN_R {
        TSENS_SAMPLE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_SAMPLE")
            .field(
                "tsens_sample_rate",
                &format_args!("{}", self.tsens_sample_rate().bits()),
            )
            .field(
                "tsens_sample_en",
                &format_args!("{}", self.tsens_sample_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_SAMPLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - HW sample rate"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_sample_rate(&mut self) -> TSENS_SAMPLE_RATE_W<0> {
        TSENS_SAMPLE_RATE_W::new(self)
    }
    #[doc = "Bit 16 - HW sample en"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_sample_en(&mut self) -> TSENS_SAMPLE_EN_W<16> {
        TSENS_SAMPLE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital tsens configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_sample](index.html) module"]
pub struct APB_TSENS_SAMPLE_SPEC;
impl crate::RegisterSpec for APB_TSENS_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_sample::R](R) reader structure"]
impl crate::Readable for APB_TSENS_SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_sample::W](W) writer structure"]
impl crate::Writable for APB_TSENS_SAMPLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_TSENS_SAMPLE to value 0x14"]
impl crate::Resettable for APB_TSENS_SAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
