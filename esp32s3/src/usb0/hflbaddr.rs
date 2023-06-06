#[doc = "Register `HFLBADDR` reader"]
pub struct R(crate::R<HFLBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFLBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFLBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFLBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFLBADDR` writer"]
pub struct W(crate::W<HFLBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFLBADDR_SPEC>;
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
impl From<crate::W<HFLBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFLBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFLBADDR` reader - "]
pub type HFLBADDR_R = crate::FieldReader<u32>;
#[doc = "Field `HFLBADDR` writer - "]
pub type HFLBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, HFLBADDR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFLBADDR")
            .field("hflbaddr", &format_args!("{}", self.hflbaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HFLBADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<0> {
        HFLBADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hflbaddr](index.html) module"]
pub struct HFLBADDR_SPEC;
impl crate::RegisterSpec for HFLBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hflbaddr::R](R) reader structure"]
impl crate::Readable for HFLBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hflbaddr::W](W) writer structure"]
impl crate::Writable for HFLBADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFLBADDR to value 0"]
impl crate::Resettable for HFLBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
