#[doc = "Register `CORE_0_INTR_RAW` reader"]
pub struct R(crate::R<CORE_0_INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_RAW` reader - sp underlow monitor interrupt status register"]
pub type CORE_0_SP_SPILL_MIN_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_RAW` reader - sp overflow monitor interupt status register"]
pub type CORE_0_SP_SPILL_MAX_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sp underlow monitor interrupt status register"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_raw(&self) -> CORE_0_SP_SPILL_MIN_RAW_R {
        CORE_0_SP_SPILL_MIN_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sp overflow monitor interupt status register"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_raw(&self) -> CORE_0_SP_SPILL_MAX_RAW_R {
        CORE_0_SP_SPILL_MAX_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_RAW")
            .field(
                "core_0_sp_spill_min_raw",
                &format_args!("{}", self.core_0_sp_spill_min_raw().bit()),
            )
            .field(
                "core_0_sp_spill_max_raw",
                &format_args!("{}", self.core_0_sp_spill_max_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "core0 monitor interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_raw](index.html) module"]
pub struct CORE_0_INTR_RAW_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_raw::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_INTR_RAW to value 0"]
impl crate::Resettable for CORE_0_INTR_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
