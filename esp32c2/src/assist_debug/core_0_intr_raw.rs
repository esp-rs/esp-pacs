#[doc = "Register `CORE_0_INTR_RAW` reader"]
pub type R = crate::R<CORE_0_INTR_RAW_SPEC>;
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
            .field("core_0_sp_spill_min_raw", &self.core_0_sp_spill_min_raw())
            .field("core_0_sp_spill_max_raw", &self.core_0_sp_spill_max_raw())
            .finish()
    }
}
#[doc = "core0 monitor interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_RAW_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_raw::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RAW_SPEC {}
#[doc = "`reset()` method sets CORE_0_INTR_RAW to value 0"]
impl crate::Resettable for CORE_0_INTR_RAW_SPEC {}
