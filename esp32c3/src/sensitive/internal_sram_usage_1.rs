#[doc = "Register `INTERNAL_SRAM_USAGE_1` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_1` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_1_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_USAGE_CPU_CACHE` reader - internal_sram_usage_cpu_cache"]
pub type INTERNAL_SRAM_USAGE_CPU_CACHE_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_USAGE_CPU_CACHE` writer - internal_sram_usage_cpu_cache"]
pub type INTERNAL_SRAM_USAGE_CPU_CACHE_W<'a, const O: u8> =
    crate::BitWriter<'a, INTERNAL_SRAM_USAGE_1_SPEC, O>;
#[doc = "Field `INTERNAL_SRAM_USAGE_CPU_SRAM` reader - internal_sram_usage_cpu_sram"]
pub type INTERNAL_SRAM_USAGE_CPU_SRAM_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_USAGE_CPU_SRAM` writer - internal_sram_usage_cpu_sram"]
pub type INTERNAL_SRAM_USAGE_CPU_SRAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, INTERNAL_SRAM_USAGE_1_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - internal_sram_usage_cpu_cache"]
    #[inline(always)]
    pub fn internal_sram_usage_cpu_cache(&self) -> INTERNAL_SRAM_USAGE_CPU_CACHE_R {
        INTERNAL_SRAM_USAGE_CPU_CACHE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - internal_sram_usage_cpu_sram"]
    #[inline(always)]
    pub fn internal_sram_usage_cpu_sram(&self) -> INTERNAL_SRAM_USAGE_CPU_SRAM_R {
        INTERNAL_SRAM_USAGE_CPU_SRAM_R::new(((self.bits >> 1) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_1")
            .field(
                "internal_sram_usage_cpu_cache",
                &format_args!("{}", self.internal_sram_usage_cpu_cache().bit()),
            )
            .field(
                "internal_sram_usage_cpu_sram",
                &format_args!("{}", self.internal_sram_usage_cpu_sram().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERNAL_SRAM_USAGE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - internal_sram_usage_cpu_cache"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_usage_cpu_cache(&mut self) -> INTERNAL_SRAM_USAGE_CPU_CACHE_W<0> {
        INTERNAL_SRAM_USAGE_CPU_CACHE_W::new(self)
    }
    #[doc = "Bits 1:3 - internal_sram_usage_cpu_sram"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_usage_cpu_sram(&mut self) -> INTERNAL_SRAM_USAGE_CPU_SRAM_W<1> {
        INTERNAL_SRAM_USAGE_CPU_SRAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_1](index.html) module"]
pub struct INTERNAL_SRAM_USAGE_1_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_1::R](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_1::W](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_1 to value 0x0f"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
