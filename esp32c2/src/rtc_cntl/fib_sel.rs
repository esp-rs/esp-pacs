#[doc = "Register `FIB_SEL` reader"]
pub struct R(crate::R<FIB_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIB_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIB_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIB_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIB_SEL` writer"]
pub struct W(crate::W<FIB_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIB_SEL_SPEC>;
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
impl From<crate::W<FIB_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIB_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIB_SEL` reader - select use analog fib signal"]
pub type FIB_SEL_R = crate::FieldReader;
#[doc = "Field `FIB_SEL` writer - select use analog fib signal"]
pub type FIB_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, FIB_SEL_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - select use analog fib signal"]
    #[inline(always)]
    pub fn fib_sel(&self) -> FIB_SEL_R {
        FIB_SEL_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIB_SEL")
            .field("fib_sel", &format_args!("{}", self.fib_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIB_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - select use analog fib signal"]
    #[inline(always)]
    #[must_use]
    pub fn fib_sel(&mut self) -> FIB_SEL_W<0> {
        FIB_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fib_sel](index.html) module"]
pub struct FIB_SEL_SPEC;
impl crate::RegisterSpec for FIB_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fib_sel::R](R) reader structure"]
impl crate::Readable for FIB_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fib_sel::W](W) writer structure"]
impl crate::Writable for FIB_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIB_SEL to value 0x07"]
impl crate::Resettable for FIB_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
