#[doc = "Register `PRO_CACHE_STATE` reader"]
pub struct R(crate::R<PRO_CACHE_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_ICACHE_STATE` reader - The bit is used to indicate icache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
pub type PRO_ICACHE_STATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_DCACHE_STATE` reader - The bit is used to indicate dcache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
pub type PRO_DCACHE_STATE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - The bit is used to indicate icache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
    #[inline(always)]
    pub fn pro_icache_state(&self) -> PRO_ICACHE_STATE_R {
        PRO_ICACHE_STATE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - The bit is used to indicate dcache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
    #[inline(always)]
    pub fn pro_dcache_state(&self) -> PRO_DCACHE_STATE_R {
        PRO_DCACHE_STATE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_STATE")
            .field(
                "pro_icache_state",
                &format_args!("{}", self.pro_icache_state().bits()),
            )
            .field(
                "pro_dcache_state",
                &format_args!("{}", self.pro_dcache_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_state](index.html) module"]
pub struct PRO_CACHE_STATE_SPEC;
impl crate::RegisterSpec for PRO_CACHE_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_state::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_CACHE_STATE to value 0"]
impl crate::Resettable for PRO_CACHE_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
