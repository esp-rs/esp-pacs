#[doc = "Register `CORE_1_DEBUG_MODE` reader"]
pub type R = crate::R<CORE_1_DEBUG_MODE_SPEC>;
#[doc = "Field `CORE_1_DEBUG_MODE` reader - cpu debug mode status, 1 means cpu enter debug mode."]
pub type CORE_1_DEBUG_MODE_R = crate::BitReader;
#[doc = "Field `CORE_1_DEBUG_MODULE_ACTIVE` reader - cpu debug_module active status"]
pub type CORE_1_DEBUG_MODULE_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cpu debug mode status, 1 means cpu enter debug mode."]
    #[inline(always)]
    pub fn core_1_debug_mode(&self) -> CORE_1_DEBUG_MODE_R {
        CORE_1_DEBUG_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu debug_module active status"]
    #[inline(always)]
    pub fn core_1_debug_module_active(&self) -> CORE_1_DEBUG_MODULE_ACTIVE_R {
        CORE_1_DEBUG_MODULE_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DEBUG_MODE")
            .field("core_1_debug_mode", &self.core_1_debug_mode())
            .field(
                "core_1_debug_module_active",
                &self.core_1_debug_module_active(),
            )
            .finish()
    }
}
#[doc = "cpu status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_debug_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_DEBUG_MODE_SPEC;
impl crate::RegisterSpec for CORE_1_DEBUG_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_debug_mode::R`](R) reader structure"]
impl crate::Readable for CORE_1_DEBUG_MODE_SPEC {}
#[doc = "`reset()` method sets CORE_1_DEBUG_MODE to value 0"]
impl crate::Resettable for CORE_1_DEBUG_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
