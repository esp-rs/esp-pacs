#[doc = "Register `STORE3` reader"]
pub struct R(crate::R<STORE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE3` writer"]
pub struct W(crate::W<STORE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE3_SPEC>;
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
impl From<crate::W<STORE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH3` reader - reserved register"]
pub type SCRATCH3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCRATCH3` writer - reserved register"]
pub type SCRATCH3_W<'a, const O: u8> = crate::FieldWriter<'a, STORE3_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch3(&self) -> SCRATCH3_R {
        SCRATCH3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE3")
            .field("scratch3", &format_args!("{}", self.scratch3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    #[must_use]
    pub fn scratch3(&mut self) -> SCRATCH3_W<0> {
        SCRATCH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store3](index.html) module"]
pub struct STORE3_SPEC;
impl crate::RegisterSpec for STORE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store3::R](R) reader structure"]
impl crate::Readable for STORE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store3::W](W) writer structure"]
impl crate::Writable for STORE3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE3 to value 0"]
impl crate::Resettable for STORE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
