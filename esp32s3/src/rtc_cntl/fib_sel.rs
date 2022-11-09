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
#[doc = "Field `FIB_SEL` reader - No public"]
pub type FIB_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIB_SEL` writer - No public"]
pub type FIB_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIB_SEL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - No public"]
    #[inline(always)]
    pub fn fib_sel(&self) -> FIB_SEL_R {
        FIB_SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - No public"]
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
#[doc = "No public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fib_sel](index.html) module"]
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
