#[doc = "Register `C0RE_0_DEBUG_MODE` reader"]
pub struct R(crate::R<C0RE_0_DEBUG_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0RE_0_DEBUG_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0RE_0_DEBUG_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0RE_0_DEBUG_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DEBUG_MODE` reader - cpu debug mode status, 1 means cpu enter debug mode."]
pub type CORE_0_DEBUG_MODE_R = crate::BitReader;
#[doc = "Field `CORE_0_DEBUG_MODULE_ACTIVE` reader - cpu debug_module active status"]
pub type CORE_0_DEBUG_MODULE_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cpu debug mode status, 1 means cpu enter debug mode."]
    #[inline(always)]
    pub fn core_0_debug_mode(&self) -> CORE_0_DEBUG_MODE_R {
        CORE_0_DEBUG_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cpu debug_module active status"]
    #[inline(always)]
    pub fn core_0_debug_module_active(&self) -> CORE_0_DEBUG_MODULE_ACTIVE_R {
        CORE_0_DEBUG_MODULE_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C0RE_0_DEBUG_MODE")
            .field(
                "core_0_debug_mode",
                &format_args!("{}", self.core_0_debug_mode().bit()),
            )
            .field(
                "core_0_debug_module_active",
                &format_args!("{}", self.core_0_debug_module_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<C0RE_0_DEBUG_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "cpu status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0re_0_debug_mode](index.html) module"]
pub struct C0RE_0_DEBUG_MODE_SPEC;
impl crate::RegisterSpec for C0RE_0_DEBUG_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0re_0_debug_mode::R](R) reader structure"]
impl crate::Readable for C0RE_0_DEBUG_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C0RE_0_DEBUG_MODE to value 0"]
impl crate::Resettable for C0RE_0_DEBUG_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
