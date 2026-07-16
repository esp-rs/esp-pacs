#[doc = "Register `DEBUG_MODE` reader"]
pub type R = crate::R<DEBUG_MODE_SPEC>;
#[doc = "Field `DEBUG_MODE` reader - Represents whether RISC-V CPU (HP CPU) is in debugging mode.\\\\ 1: In debugging mode\\\\ 0: Not in debugging mode\\\\"]
pub type DEBUG_MODE_R = crate::BitReader;
#[doc = "Field `DEBUG_MODULE_ACTIVE` reader - Represents the status of the RISC-V CPU (HP CPU) debug module.\\\\ 1: Active status\\\\ Other: Inactive status\\\\"]
pub type DEBUG_MODULE_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether RISC-V CPU (HP CPU) is in debugging mode.\\\\ 1: In debugging mode\\\\ 0: Not in debugging mode\\\\"]
    #[inline(always)]
    pub fn debug_mode(&self) -> DEBUG_MODE_R {
        DEBUG_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents the status of the RISC-V CPU (HP CPU) debug module.\\\\ 1: Active status\\\\ Other: Inactive status\\\\"]
    #[inline(always)]
    pub fn debug_module_active(&self) -> DEBUG_MODULE_ACTIVE_R {
        DEBUG_MODULE_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_MODE")
            .field("debug_mode", &self.debug_mode())
            .field("debug_module_active", &self.debug_module_active())
            .finish()
    }
}
#[doc = "cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_MODE_SPEC;
impl crate::RegisterSpec for DEBUG_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_mode::R`](R) reader structure"]
impl crate::Readable for DEBUG_MODE_SPEC {}
#[doc = "`reset()` method sets DEBUG_MODE to value 0"]
impl crate::Resettable for DEBUG_MODE_SPEC {}
