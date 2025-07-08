#[doc = "Register `LOG_MON_ADDR_UPDATE_0` writer"]
pub type W = crate::W<LOG_MON_ADDR_UPDATE_0_SPEC>;
#[doc = "Field `LOG_MON_ADDR_CORE_UPDATE` writer - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the HP CPU bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
pub type LOG_MON_ADDR_CORE_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_MON_ADDR_ALL_UPDATE` writer - Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of all masters.1: Update\\\\ 0: Not update\\\\"]
pub type LOG_MON_ADDR_ALL_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MON_ADDR_UPDATE_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the HP CPU bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
    #[inline(always)]
    pub fn log_mon_addr_core_update(
        &mut self,
    ) -> LOG_MON_ADDR_CORE_UPDATE_W<LOG_MON_ADDR_UPDATE_0_SPEC> {
        LOG_MON_ADDR_CORE_UPDATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of all masters.1: Update\\\\ 0: Not update\\\\"]
    #[inline(always)]
    pub fn log_mon_addr_all_update(
        &mut self,
    ) -> LOG_MON_ADDR_ALL_UPDATE_W<LOG_MON_ADDR_UPDATE_0_SPEC> {
        LOG_MON_ADDR_ALL_UPDATE_W::new(self, 31)
    }
}
#[doc = "Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mon_addr_update_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_MON_ADDR_UPDATE_0_SPEC;
impl crate::RegisterSpec for LOG_MON_ADDR_UPDATE_0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`log_mon_addr_update_0::W`](W) writer structure"]
impl crate::Writable for LOG_MON_ADDR_UPDATE_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOG_MON_ADDR_UPDATE_0 to value 0"]
impl crate::Resettable for LOG_MON_ADDR_UPDATE_0_SPEC {}
