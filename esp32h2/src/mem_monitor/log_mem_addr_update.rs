#[doc = "Register `LOG_MEM_ADDR_UPDATE` writer"]
pub struct W(crate::W<LOG_MEM_ADDR_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MEM_ADDR_UPDATE_SPEC>;
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
impl From<crate::W<LOG_MEM_ADDR_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MEM_ADDR_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_MEM_ADDR_UPDATE` writer - Set 1 to updata MEM_MONITOR_LOG_MEM_CURRENT_ADDR, when set 1, MEM_MONITOR_LOG_MEM_CURRENT_ADDR will update to MEM_MONITOR_LOG_MEM_START"]
pub type LOG_MEM_ADDR_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, LOG_MEM_ADDR_UPDATE_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MEM_ADDR_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to updata MEM_MONITOR_LOG_MEM_CURRENT_ADDR, when set 1, MEM_MONITOR_LOG_MEM_CURRENT_ADDR will update to MEM_MONITOR_LOG_MEM_START"]
    #[inline(always)]
    #[must_use]
    pub fn log_mem_addr_update(&mut self) -> LOG_MEM_ADDR_UPDATE_W<0> {
        LOG_MEM_ADDR_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "writing address update\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_addr_update](index.html) module"]
pub struct LOG_MEM_ADDR_UPDATE_SPEC;
impl crate::RegisterSpec for LOG_MEM_ADDR_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [log_mem_addr_update::W](W) writer structure"]
impl crate::Writable for LOG_MEM_ADDR_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_MEM_ADDR_UPDATE to value 0"]
impl crate::Resettable for LOG_MEM_ADDR_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
