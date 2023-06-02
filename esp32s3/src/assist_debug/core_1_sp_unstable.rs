#[doc = "Register `CORE_1_SP_UNSTABLE` reader"]
pub struct R(crate::R<CORE_1_SP_UNSTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_SP_UNSTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_SP_UNSTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_SP_UNSTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_SP_UNSTABLE` writer"]
pub struct W(crate::W<CORE_1_SP_UNSTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_SP_UNSTABLE_SPEC>;
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
impl From<crate::W<CORE_1_SP_UNSTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_SP_UNSTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_SP_UNSTABLE` reader - unstable period when window change,during this period no check stackpointer"]
pub type CORE_1_SP_UNSTABLE_R = crate::FieldReader;
#[doc = "Field `CORE_1_SP_UNSTABLE` writer - unstable period when window change,during this period no check stackpointer"]
pub type CORE_1_SP_UNSTABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_SP_UNSTABLE_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - unstable period when window change,during this period no check stackpointer"]
    #[inline(always)]
    pub fn core_1_sp_unstable(&self) -> CORE_1_SP_UNSTABLE_R {
        CORE_1_SP_UNSTABLE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_SP_UNSTABLE")
            .field(
                "core_1_sp_unstable",
                &format_args!("{}", self.core_1_sp_unstable().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_SP_UNSTABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - unstable period when window change,during this period no check stackpointer"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_unstable(&mut self) -> CORE_1_SP_UNSTABLE_W<0> {
        CORE_1_SP_UNSTABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 sp unstable configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_sp_unstable](index.html) module"]
pub struct CORE_1_SP_UNSTABLE_SPEC;
impl crate::RegisterSpec for CORE_1_SP_UNSTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_sp_unstable::R](R) reader structure"]
impl crate::Readable for CORE_1_SP_UNSTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_sp_unstable::W](W) writer structure"]
impl crate::Writable for CORE_1_SP_UNSTABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_SP_UNSTABLE to value 0x0b"]
impl crate::Resettable for CORE_1_SP_UNSTABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
