#[doc = "Register `DEBUG_MODE` reader"]
pub type R = crate::R<DEBUG_MODE_SPEC>;
#[doc = "Field `DEBUG_MODE` reader - cpu debug mode status, 1 means cpu enter debug mode."]
pub type DEBUG_MODE_R = crate::BitReader;
#[doc = "Field `DEBUG_MODULE_ACTIVE` reader - cpu debug_module active status"]
pub type DEBUG_MODULE_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cpu debug mode status, 1 means cpu enter debug mode."]
    #[inline(always)]
    pub fn debug_mode(&self) -> DEBUG_MODE_R {
        DEBUG_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu debug_module active status"]
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
