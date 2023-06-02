#[doc = "Register `HAINTMSK` reader"]
pub struct R(crate::R<HAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HAINTMSK` writer"]
pub struct W(crate::W<HAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HAINTMSK_SPEC>;
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
impl From<crate::W<HAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAINTMSK` reader - "]
pub type HAINTMSK_R = crate::FieldReader;
#[doc = "Field `HAINTMSK` writer - "]
pub type HAINTMSK_W<'a, const O: u8> = crate::FieldWriter<'a, HAINTMSK_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn haintmsk(&self) -> HAINTMSK_R {
        HAINTMSK_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINTMSK")
            .field("haintmsk", &format_args!("{}", self.haintmsk().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HAINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn haintmsk(&mut self) -> HAINTMSK_W<0> {
        HAINTMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haintmsk](index.html) module"]
pub struct HAINTMSK_SPEC;
impl crate::RegisterSpec for HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haintmsk::R](R) reader structure"]
impl crate::Readable for HAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [haintmsk::W](W) writer structure"]
impl crate::Writable for HAINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HAINTMSK to value 0"]
impl crate::Resettable for HAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
