#[doc = "Register `DVBUSDIS` reader"]
pub struct R(crate::R<DVBUSDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVBUSDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVBUSDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVBUSDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVBUSDIS` writer"]
pub struct W(crate::W<DVBUSDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVBUSDIS_SPEC>;
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
impl From<crate::W<DVBUSDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVBUSDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVBUSDIS` reader - "]
pub type DVBUSDIS_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSDIS` writer - "]
pub type DVBUSDIS_W<'a, const O: u8> = crate::FieldWriter<'a, DVBUSDIS_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dvbusdis(&self) -> DVBUSDIS_R {
        DVBUSDIS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSDIS")
            .field("dvbusdis", &format_args!("{}", self.dvbusdis().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DVBUSDIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn dvbusdis(&mut self) -> DVBUSDIS_W<0> {
        DVBUSDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbusdis](index.html) module"]
pub struct DVBUSDIS_SPEC;
impl crate::RegisterSpec for DVBUSDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvbusdis::R](R) reader structure"]
impl crate::Readable for DVBUSDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvbusdis::W](W) writer structure"]
impl crate::Writable for DVBUSDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVBUSDIS to value 0x17d7"]
impl crate::Resettable for DVBUSDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0x17d7;
}
