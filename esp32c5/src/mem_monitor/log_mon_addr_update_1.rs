#[doc = "Register `LOG_MON_ADDR_UPDATE_1` writer"]
pub type W = crate::W<LOG_MON_ADDR_UPDATE_1_SPEC>;
#[doc = "Field `LOG_MON_ADDR_DMA_0_UPDATE` writer - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_0 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
pub type LOG_MON_ADDR_DMA_0_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_MON_ADDR_DMA_1_UPDATE` writer - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_1 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
pub type LOG_MON_ADDR_DMA_1_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_MON_ADDR_DMA_2_UPDATE` writer - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_2 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
pub type LOG_MON_ADDR_DMA_2_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOG_MON_ADDR_DMA_3_UPDATE` writer - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_3 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
pub type LOG_MON_ADDR_DMA_3_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MON_ADDR_UPDATE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_0 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
    #[inline(always)]
    pub fn log_mon_addr_dma_0_update(
        &mut self,
    ) -> LOG_MON_ADDR_DMA_0_UPDATE_W<LOG_MON_ADDR_UPDATE_1_SPEC> {
        LOG_MON_ADDR_DMA_0_UPDATE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_1 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
    #[inline(always)]
    pub fn log_mon_addr_dma_1_update(
        &mut self,
    ) -> LOG_MON_ADDR_DMA_1_UPDATE_W<LOG_MON_ADDR_UPDATE_1_SPEC> {
        LOG_MON_ADDR_DMA_1_UPDATE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_2 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
    #[inline(always)]
    pub fn log_mon_addr_dma_2_update(
        &mut self,
    ) -> LOG_MON_ADDR_DMA_2_UPDATE_W<LOG_MON_ADDR_UPDATE_1_SPEC> {
        LOG_MON_ADDR_DMA_2_UPDATE_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Configures the monitored address space of the certain master. Bit\\[0\\]: Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the DMA_3 bus.1: Update\\\\ 0: Not update\\\\ Bit\\[7:1\\]: Reserved\\\\"]
    #[inline(always)]
    pub fn log_mon_addr_dma_3_update(
        &mut self,
    ) -> LOG_MON_ADDR_DMA_3_UPDATE_W<LOG_MON_ADDR_UPDATE_1_SPEC> {
        LOG_MON_ADDR_DMA_3_UPDATE_W::new(self, 24)
    }
}
#[doc = "Configures the address space of from MEM_MONITOR_LOG_MIN_REG to MEM_MONITOR_LOG_MAX_REG as the monitored address space of the certain master.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_mon_addr_update_1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_MON_ADDR_UPDATE_1_SPEC;
impl crate::RegisterSpec for LOG_MON_ADDR_UPDATE_1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`log_mon_addr_update_1::W`](W) writer structure"]
impl crate::Writable for LOG_MON_ADDR_UPDATE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOG_MON_ADDR_UPDATE_1 to value 0"]
impl crate::Resettable for LOG_MON_ADDR_UPDATE_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
