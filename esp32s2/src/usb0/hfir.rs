#[doc = "Register `HFIR` reader"]
pub struct R(crate::R<HFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFIR` writer"]
pub struct W(crate::W<HFIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFIR_SPEC>;
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
impl From<crate::W<HFIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRINT` reader - "]
pub type FRINT_R = crate::FieldReader<u16>;
#[doc = "Field `FRINT` writer - "]
pub type FRINT_W<'a, const O: u8> = crate::FieldWriter<'a, HFIR_SPEC, 16, O, u16>;
#[doc = "Field `HFIRRLDCTRL` reader - "]
pub type HFIRRLDCTRL_R = crate::BitReader;
#[doc = "Field `HFIRRLDCTRL` writer - "]
pub type HFIRRLDCTRL_W<'a, const O: u8> = crate::BitWriter<'a, HFIR_SPEC, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn frint(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hfirrldctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFIR")
            .field("frint", &format_args!("{}", self.frint().bits()))
            .field("hfirrldctrl", &format_args!("{}", self.hfirrldctrl().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HFIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn frint(&mut self) -> FRINT_W<0> {
        FRINT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn hfirrldctrl(&mut self) -> HFIRRLDCTRL_W<16> {
        HFIRRLDCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfir](index.html) module"]
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfir::R](R) reader structure"]
impl crate::Readable for HFIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfir::W](W) writer structure"]
impl crate::Writable for HFIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFIR to value 0x17d7"]
impl crate::Resettable for HFIR_SPEC {
    const RESET_VALUE: Self::Ux = 0x17d7;
}
