#[doc = "Register `CORE_0_INTR_CLR` writer"]
pub type W = crate::W<CORE_0_INTR_CLR_SPEC>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` writer - Write 1 to clear the interrupt for read operations in region 0 by Data bus."]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` writer - Write 1 to clear the interrupt for write operations in region 0 by Data bus."]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` writer - Write 1 to clear the interrupt for read operations in region 1 by Data bus."]
pub type CORE_0_AREA_DRAM0_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` writer - Write 1 to clear the interrupt for write operations in region 1 by Data bus."]
pub type CORE_0_AREA_DRAM0_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` writer - Write 1 to clear the interrupt for read operations in region 0 by Peripheral bus."]
pub type CORE_0_AREA_PIF_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` writer - Write 1 to clear the interrupt for write operations in region 0 by Peripheral bus."]
pub type CORE_0_AREA_PIF_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` writer - Write 1 to clear the interrupt for read operations in region 1 by Peripheral bus."]
pub type CORE_0_AREA_PIF_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` writer - Write 1 to clear the interrupt for write operations in region 1 by Peripheral bus."]
pub type CORE_0_AREA_PIF_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` writer - Write 1 to clear the interrupt for SP exceeding the lower bound address of SP monitored region."]
pub type CORE_0_SP_SPILL_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` writer - Write 1 to clear the interrupt for SP exceeding the upper bound address of SP monitored region."]
pub type CORE_0_SP_SPILL_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear the interrupt for read operations in region 0 by Data bus."]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_0_RD_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear the interrupt for write operations in region 0 by Data bus."]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_0_WR_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear the interrupt for read operations in region 1 by Data bus."]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_1_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_1_RD_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear the interrupt for write operations in region 1 by Data bus."]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_1_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_1_WR_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear the interrupt for read operations in region 0 by Peripheral bus."]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_clr(&mut self) -> CORE_0_AREA_PIF_0_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_0_RD_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear the interrupt for write operations in region 0 by Peripheral bus."]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_clr(&mut self) -> CORE_0_AREA_PIF_0_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_0_WR_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear the interrupt for read operations in region 1 by Peripheral bus."]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_clr(&mut self) -> CORE_0_AREA_PIF_1_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_1_RD_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear the interrupt for write operations in region 1 by Peripheral bus."]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_clr(&mut self) -> CORE_0_AREA_PIF_1_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_1_WR_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear the interrupt for SP exceeding the lower bound address of SP monitored region."]
    #[inline(always)]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MIN_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to clear the interrupt for SP exceeding the upper bound address of SP monitored region."]
    #[inline(always)]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MAX_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 11)
    }
}
#[doc = "core0 monitor interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_0_intr_clr::W`](W) writer structure"]
impl crate::Writable for CORE_0_INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_INTR_CLR to value 0"]
impl crate::Resettable for CORE_0_INTR_CLR_SPEC {}
