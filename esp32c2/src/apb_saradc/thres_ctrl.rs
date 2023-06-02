#[doc = "Register `THRES_CTRL` reader"]
pub struct R(crate::R<THRES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES_CTRL` writer"]
pub struct W(crate::W<THRES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES_CTRL_SPEC>;
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
impl From<crate::W<THRES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES_ALL_EN` reader - Need add description"]
pub type THRES_ALL_EN_R = crate::BitReader;
#[doc = "Field `THRES_ALL_EN` writer - Need add description"]
pub type THRES_ALL_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
#[doc = "Field `THRES3_EN` reader - Need add description"]
pub type THRES3_EN_R = crate::BitReader;
#[doc = "Field `THRES3_EN` writer - Need add description"]
pub type THRES3_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
#[doc = "Field `THRES2_EN` reader - Need add description"]
pub type THRES2_EN_R = crate::BitReader;
#[doc = "Field `THRES2_EN` writer - Need add description"]
pub type THRES2_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
#[doc = "Field `THRES1_EN` reader - Need add description"]
pub type THRES1_EN_R = crate::BitReader;
#[doc = "Field `THRES1_EN` writer - Need add description"]
pub type THRES1_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
#[doc = "Field `THRES0_EN` reader - Need add description"]
pub type THRES0_EN_R = crate::BitReader;
#[doc = "Field `THRES0_EN` writer - Need add description"]
pub type THRES0_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    pub fn thres_all_en(&self) -> THRES_ALL_EN_R {
        THRES_ALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    pub fn thres3_en(&self) -> THRES3_EN_R {
        THRES3_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn thres2_en(&self) -> THRES2_EN_R {
        THRES2_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn thres1_en(&self) -> THRES1_EN_R {
        THRES1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn thres0_en(&self) -> THRES0_EN_R {
        THRES0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field(
                "thres_all_en",
                &format_args!("{}", self.thres_all_en().bit()),
            )
            .field("thres3_en", &format_args!("{}", self.thres3_en().bit()))
            .field("thres2_en", &format_args!("{}", self.thres2_en().bit()))
            .field("thres1_en", &format_args!("{}", self.thres1_en().bit()))
            .field("thres0_en", &format_args!("{}", self.thres0_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THRES_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn thres_all_en(&mut self) -> THRES_ALL_EN_W<27> {
        THRES_ALL_EN_W::new(self)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn thres3_en(&mut self) -> THRES3_EN_W<28> {
        THRES3_EN_W::new(self)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn thres2_en(&mut self) -> THRES2_EN_W<29> {
        THRES2_EN_W::new(self)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_en(&mut self) -> THRES1_EN_W<30> {
        THRES1_EN_W::new(self)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_en(&mut self) -> THRES0_EN_W<31> {
        THRES0_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres_ctrl](index.html) module"]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres_ctrl::R](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres_ctrl::W](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
