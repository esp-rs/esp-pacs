#[doc = "Register `PMU_INTR_MAP` reader"]
pub struct R(crate::R<PMU_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_INTR_MAP` writer"]
pub struct W(crate::W<PMU_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_INTR_MAP_SPEC>;
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
impl From<crate::W<PMU_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMU_INTR_MAP` reader - CORE0_PMU_INTR mapping register"]
pub type PMU_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PMU_INTR_MAP` writer - CORE0_PMU_INTR mapping register"]
pub type PMU_INTR_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, PMU_INTR_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - CORE0_PMU_INTR mapping register"]
    #[inline(always)]
    pub fn pmu_intr_map(&self) -> PMU_INTR_MAP_R {
        PMU_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_INTR_MAP")
            .field(
                "pmu_intr_map",
                &format_args!("{}", self.pmu_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMU_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_PMU_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_intr_map(&mut self) -> PMU_INTR_MAP_W<0> {
        PMU_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_intr_map](index.html) module"]
pub struct PMU_INTR_MAP_SPEC;
impl crate::RegisterSpec for PMU_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmu_intr_map::R](R) reader structure"]
impl crate::Readable for PMU_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_intr_map::W](W) writer structure"]
impl crate::Writable for PMU_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMU_INTR_MAP to value 0"]
impl crate::Resettable for PMU_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
