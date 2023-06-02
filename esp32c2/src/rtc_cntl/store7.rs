#[doc = "Register `STORE7` reader"]
pub struct R(crate::R<STORE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE7` writer"]
pub struct W(crate::W<STORE7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE7_SPEC>;
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
impl From<crate::W<STORE7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH7` reader - Need add desc"]
pub type SCRATCH7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCRATCH7` writer - Need add desc"]
pub type SCRATCH7_W<'a, const O: u8> = crate::FieldWriter<'a, STORE7_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn scratch7(&self) -> SCRATCH7_R {
        SCRATCH7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE7")
            .field("scratch7", &format_args!("{}", self.scratch7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn scratch7(&mut self) -> SCRATCH7_W<0> {
        SCRATCH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store7](index.html) module"]
pub struct STORE7_SPEC;
impl crate::RegisterSpec for STORE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store7::R](R) reader structure"]
impl crate::Readable for STORE7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store7::W](W) writer structure"]
impl crate::Writable for STORE7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE7 to value 0"]
impl crate::Resettable for STORE7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
