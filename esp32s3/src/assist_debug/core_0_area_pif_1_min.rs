#[doc = "Register `CORE_0_AREA_PIF_1_MIN` reader"]
pub struct R(crate::R<CORE_0_AREA_PIF_1_MIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_AREA_PIF_1_MIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_AREA_PIF_1_MIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_AREA_PIF_1_MIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_AREA_PIF_1_MIN` writer"]
pub struct W(crate::W<CORE_0_AREA_PIF_1_MIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_AREA_PIF_1_MIN_SPEC>;
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
impl From<crate::W<CORE_0_AREA_PIF_1_MIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_AREA_PIF_1_MIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_MIN` reader - Core0 PIF region1 start addr"]
pub type CORE_0_AREA_PIF_1_MIN_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_PIF_1_MIN` writer - Core0 PIF region1 start addr"]
pub type CORE_0_AREA_PIF_1_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_AREA_PIF_1_MIN_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Core0 PIF region1 start addr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_min(&self) -> CORE_0_AREA_PIF_1_MIN_R {
        CORE_0_AREA_PIF_1_MIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_AREA_PIF_1_MIN")
            .field(
                "core_0_area_pif_1_min",
                &format_args!("{}", self.core_0_area_pif_1_min().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_AREA_PIF_1_MIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core0 PIF region1 start addr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_min(&mut self) -> CORE_0_AREA_PIF_1_MIN_W<0> {
        CORE_0_AREA_PIF_1_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 PIF region1 addr configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_area_pif_1_min](index.html) module"]
pub struct CORE_0_AREA_PIF_1_MIN_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_PIF_1_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_area_pif_1_min::R](R) reader structure"]
impl crate::Readable for CORE_0_AREA_PIF_1_MIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_area_pif_1_min::W](W) writer structure"]
impl crate::Writable for CORE_0_AREA_PIF_1_MIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_AREA_PIF_1_MIN to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_AREA_PIF_1_MIN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
