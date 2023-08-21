#[doc = "Register `LOG_MEM_ADDR_UPDATE` writer"]
pub type W = crate::W<LOG_MEM_ADDR_UPDATE_SPEC>;
#[doc = "Field `LOG_MEM_ADDR_UPDATE` writer - Set 1 to updata MEM_MONITOR_LOG_MEM_CURRENT_ADDR, when set 1, MEM_MONITOR_LOG_MEM_CURRENT_ADDR will update to MEM_MONITOR_LOG_MEM_START"]
pub type LOG_MEM_ADDR_UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn log_mem_addr_update(&mut self) -> LOG_MEM_ADDR_UPDATE_W<LOG_MEM_ADDR_UPDATE_SPEC, 0> {
        LOG_MEM_ADDR_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "writing address update\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_addr_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_MEM_ADDR_UPDATE_SPEC;
impl crate::RegisterSpec for LOG_MEM_ADDR_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`log_mem_addr_update::W`](W) writer structure"]
impl crate::Writable for LOG_MEM_ADDR_UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_MEM_ADDR_UPDATE to value 0"]
impl crate::Resettable for LOG_MEM_ADDR_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
