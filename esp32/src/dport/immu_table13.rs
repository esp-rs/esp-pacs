#[doc = "Register `IMMU_TABLE13` reader"]
pub struct R(crate::R<IMMU_TABLE13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMMU_TABLE13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMMU_TABLE13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMMU_TABLE13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMMU_TABLE13` writer"]
pub struct W(crate::W<IMMU_TABLE13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMMU_TABLE13_SPEC>;
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
impl From<crate::W<IMMU_TABLE13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMMU_TABLE13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMMU_TABLE13` reader - "]
pub type IMMU_TABLE13_R = crate::FieldReader;
#[doc = "Field `IMMU_TABLE13` writer - "]
pub type IMMU_TABLE13_W<'a, const O: u8> = crate::FieldWriter<'a, IMMU_TABLE13_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table13(&self) -> IMMU_TABLE13_R {
        IMMU_TABLE13_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE13")
            .field(
                "immu_table13",
                &format_args!("{}", self.immu_table13().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMMU_TABLE13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn immu_table13(&mut self) -> IMMU_TABLE13_W<0> {
        IMMU_TABLE13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [immu_table13](index.html) module"]
pub struct IMMU_TABLE13_SPEC;
impl crate::RegisterSpec for IMMU_TABLE13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [immu_table13::R](R) reader structure"]
impl crate::Readable for IMMU_TABLE13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [immu_table13::W](W) writer structure"]
impl crate::Writable for IMMU_TABLE13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMMU_TABLE13 to value 0x0d"]
impl crate::Resettable for IMMU_TABLE13_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
