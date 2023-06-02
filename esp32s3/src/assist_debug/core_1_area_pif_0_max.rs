#[doc = "Register `CORE_1_AREA_PIF_0_MAX` reader"]
pub struct R(crate::R<CORE_1_AREA_PIF_0_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_AREA_PIF_0_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_AREA_PIF_0_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_AREA_PIF_0_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_AREA_PIF_0_MAX` writer"]
pub struct W(crate::W<CORE_1_AREA_PIF_0_MAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_AREA_PIF_0_MAX_SPEC>;
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
impl From<crate::W<CORE_1_AREA_PIF_0_MAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_AREA_PIF_0_MAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_AREA_PIF_0_MAX` reader - Core1 PIF region0 end addr"]
pub type CORE_1_AREA_PIF_0_MAX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_1_AREA_PIF_0_MAX` writer - Core1 PIF region0 end addr"]
pub type CORE_1_AREA_PIF_0_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_AREA_PIF_0_MAX_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core1 PIF region0 end addr"]
    #[inline(always)]
    pub fn core_1_area_pif_0_max(&self) -> CORE_1_AREA_PIF_0_MAX_R {
        CORE_1_AREA_PIF_0_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_AREA_PIF_0_MAX")
            .field(
                "core_1_area_pif_0_max",
                &format_args!("{}", self.core_1_area_pif_0_max().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_AREA_PIF_0_MAX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core1 PIF region0 end addr"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_max(&mut self) -> CORE_1_AREA_PIF_0_MAX_W<0> {
        CORE_1_AREA_PIF_0_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 PIF region0 addr configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_area_pif_0_max](index.html) module"]
pub struct CORE_1_AREA_PIF_0_MAX_SPEC;
impl crate::RegisterSpec for CORE_1_AREA_PIF_0_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_area_pif_0_max::R](R) reader structure"]
impl crate::Readable for CORE_1_AREA_PIF_0_MAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_area_pif_0_max::W](W) writer structure"]
impl crate::Writable for CORE_1_AREA_PIF_0_MAX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_AREA_PIF_0_MAX to value 0"]
impl crate::Resettable for CORE_1_AREA_PIF_0_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
