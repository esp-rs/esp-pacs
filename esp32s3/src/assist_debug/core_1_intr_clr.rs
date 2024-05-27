///Register `CORE_1_INTR_CLR` reader
pub type R = crate::R<CORE_1_INTR_CLR_SPEC>;
///Register `CORE_1_INTR_CLR` writer
pub type W = crate::W<CORE_1_INTR_CLR_SPEC>;
///Field `CORE_1_AREA_DRAM0_0_RD_CLR` reader - Core1 dram0 area0 read monitor interrupt clr
pub type CORE_1_AREA_DRAM0_0_RD_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_DRAM0_0_RD_CLR` writer - Core1 dram0 area0 read monitor interrupt clr
pub type CORE_1_AREA_DRAM0_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_DRAM0_0_WR_CLR` reader - Core1 dram0 area0 write monitor interrupt clr
pub type CORE_1_AREA_DRAM0_0_WR_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_DRAM0_0_WR_CLR` writer - Core1 dram0 area0 write monitor interrupt clr
pub type CORE_1_AREA_DRAM0_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_DRAM0_1_RD_CLR` reader - Core1 dram0 area1 read monitor interrupt clr
pub type CORE_1_AREA_DRAM0_1_RD_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_DRAM0_1_RD_CLR` writer - Core1 dram0 area1 read monitor interrupt clr
pub type CORE_1_AREA_DRAM0_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_DRAM0_1_WR_CLR` reader - Core1 dram0 area1 write monitor interrupt clr
pub type CORE_1_AREA_DRAM0_1_WR_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_DRAM0_1_WR_CLR` writer - Core1 dram0 area1 write monitor interrupt clr
pub type CORE_1_AREA_DRAM0_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_PIF_0_RD_CLR` reader - Core1 PIF area0 read monitor interrupt clr
pub type CORE_1_AREA_PIF_0_RD_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_PIF_0_RD_CLR` writer - Core1 PIF area0 read monitor interrupt clr
pub type CORE_1_AREA_PIF_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_PIF_0_WR_CLR` reader - Core1 PIF area0 write monitor interrupt clr
pub type CORE_1_AREA_PIF_0_WR_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_PIF_0_WR_CLR` writer - Core1 PIF area0 write monitor interrupt clr
pub type CORE_1_AREA_PIF_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_PIF_1_RD_CLR` reader - Core1 PIF area1 read monitor interrupt clr
pub type CORE_1_AREA_PIF_1_RD_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_PIF_1_RD_CLR` writer - Core1 PIF area1 read monitor interrupt clr
pub type CORE_1_AREA_PIF_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_AREA_PIF_1_WR_CLR` reader - Core1 PIF area1 write monitor interrupt clr
pub type CORE_1_AREA_PIF_1_WR_CLR_R = crate::BitReader;
///Field `CORE_1_AREA_PIF_1_WR_CLR` writer - Core1 PIF area1 write monitor interrupt clr
pub type CORE_1_AREA_PIF_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_SP_SPILL_MIN_CLR` reader - Core1 stackpoint overflow monitor interrupt clr
pub type CORE_1_SP_SPILL_MIN_CLR_R = crate::BitReader;
///Field `CORE_1_SP_SPILL_MIN_CLR` writer - Core1 stackpoint overflow monitor interrupt clr
pub type CORE_1_SP_SPILL_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_SP_SPILL_MAX_CLR` reader - Core1 stackpoint underflow monitor interrupt clr
pub type CORE_1_SP_SPILL_MAX_CLR_R = crate::BitReader;
///Field `CORE_1_SP_SPILL_MAX_CLR` writer - Core1 stackpoint underflow monitor interrupt clr
pub type CORE_1_SP_SPILL_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_IRAM0_EXCEPTION_MONITOR_CLR` reader - IBUS busy monitor interrupt clr
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
///Field `CORE_1_IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_DRAM0_EXCEPTION_MONITOR_CLR` reader - DBUS busy monitor interrupt clr
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
///Field `CORE_1_DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Core1 dram0 area0 read monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_clr(&self) -> CORE_1_AREA_DRAM0_0_RD_CLR_R {
        CORE_1_AREA_DRAM0_0_RD_CLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Core1 dram0 area0 write monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_clr(&self) -> CORE_1_AREA_DRAM0_0_WR_CLR_R {
        CORE_1_AREA_DRAM0_0_WR_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Core1 dram0 area1 read monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_clr(&self) -> CORE_1_AREA_DRAM0_1_RD_CLR_R {
        CORE_1_AREA_DRAM0_1_RD_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Core1 dram0 area1 write monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_clr(&self) -> CORE_1_AREA_DRAM0_1_WR_CLR_R {
        CORE_1_AREA_DRAM0_1_WR_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Core1 PIF area0 read monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_clr(&self) -> CORE_1_AREA_PIF_0_RD_CLR_R {
        CORE_1_AREA_PIF_0_RD_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Core1 PIF area0 write monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_clr(&self) -> CORE_1_AREA_PIF_0_WR_CLR_R {
        CORE_1_AREA_PIF_0_WR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Core1 PIF area1 read monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_clr(&self) -> CORE_1_AREA_PIF_1_RD_CLR_R {
        CORE_1_AREA_PIF_1_RD_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Core1 PIF area1 write monitor interrupt clr
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_clr(&self) -> CORE_1_AREA_PIF_1_WR_CLR_R {
        CORE_1_AREA_PIF_1_WR_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Core1 stackpoint overflow monitor interrupt clr
    #[inline(always)]
    pub fn core_1_sp_spill_min_clr(&self) -> CORE_1_SP_SPILL_MIN_CLR_R {
        CORE_1_SP_SPILL_MIN_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Core1 stackpoint underflow monitor interrupt clr
    #[inline(always)]
    pub fn core_1_sp_spill_max_clr(&self) -> CORE_1_SP_SPILL_MAX_CLR_R {
        CORE_1_SP_SPILL_MAX_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IBUS busy monitor interrupt clr
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_clr(&self) -> CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DBUS busy monitor interrupt clr
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_clr(&self) -> CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_INTR_CLR")
            .field(
                "core_1_area_dram0_0_rd_clr",
                &self.core_1_area_dram0_0_rd_clr(),
            )
            .field(
                "core_1_area_dram0_0_wr_clr",
                &self.core_1_area_dram0_0_wr_clr(),
            )
            .field(
                "core_1_area_dram0_1_rd_clr",
                &self.core_1_area_dram0_1_rd_clr(),
            )
            .field(
                "core_1_area_dram0_1_wr_clr",
                &self.core_1_area_dram0_1_wr_clr(),
            )
            .field("core_1_area_pif_0_rd_clr", &self.core_1_area_pif_0_rd_clr())
            .field("core_1_area_pif_0_wr_clr", &self.core_1_area_pif_0_wr_clr())
            .field("core_1_area_pif_1_rd_clr", &self.core_1_area_pif_1_rd_clr())
            .field("core_1_area_pif_1_wr_clr", &self.core_1_area_pif_1_wr_clr())
            .field("core_1_sp_spill_min_clr", &self.core_1_sp_spill_min_clr())
            .field("core_1_sp_spill_max_clr", &self.core_1_sp_spill_max_clr())
            .field(
                "core_1_iram0_exception_monitor_clr",
                &self.core_1_iram0_exception_monitor_clr(),
            )
            .field(
                "core_1_dram0_exception_monitor_clr",
                &self.core_1_dram0_exception_monitor_clr(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Core1 dram0 area0 read monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_rd_clr(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_0_RD_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_DRAM0_0_RD_CLR_W::new(self, 0)
    }
    ///Bit 1 - Core1 dram0 area0 write monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_wr_clr(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_0_WR_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_DRAM0_0_WR_CLR_W::new(self, 1)
    }
    ///Bit 2 - Core1 dram0 area1 read monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_1_rd_clr(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_1_RD_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_DRAM0_1_RD_CLR_W::new(self, 2)
    }
    ///Bit 3 - Core1 dram0 area1 write monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_1_wr_clr(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_1_WR_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_DRAM0_1_WR_CLR_W::new(self, 3)
    }
    ///Bit 4 - Core1 PIF area0 read monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_rd_clr(&mut self) -> CORE_1_AREA_PIF_0_RD_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_PIF_0_RD_CLR_W::new(self, 4)
    }
    ///Bit 5 - Core1 PIF area0 write monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_wr_clr(&mut self) -> CORE_1_AREA_PIF_0_WR_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_PIF_0_WR_CLR_W::new(self, 5)
    }
    ///Bit 6 - Core1 PIF area1 read monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_rd_clr(&mut self) -> CORE_1_AREA_PIF_1_RD_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_PIF_1_RD_CLR_W::new(self, 6)
    }
    ///Bit 7 - Core1 PIF area1 write monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_wr_clr(&mut self) -> CORE_1_AREA_PIF_1_WR_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_AREA_PIF_1_WR_CLR_W::new(self, 7)
    }
    ///Bit 8 - Core1 stackpoint overflow monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_spill_min_clr(&mut self) -> CORE_1_SP_SPILL_MIN_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_SP_SPILL_MIN_CLR_W::new(self, 8)
    }
    ///Bit 9 - Core1 stackpoint underflow monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_spill_max_clr(&mut self) -> CORE_1_SP_SPILL_MAX_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_SP_SPILL_MAX_CLR_W::new(self, 9)
    }
    ///Bit 10 - IBUS busy monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_iram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_IRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 10)
    }
    ///Bit 11 - DBUS busy monitor interrupt clr
    #[inline(always)]
    #[must_use]
    pub fn core_1_dram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_W<CORE_1_INTR_CLR_SPEC> {
        CORE_1_DRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 11)
    }
}
/**Core1 monitor interrupt clr register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_intr_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_intr_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_1_INTR_CLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_intr_clr::R`](R) reader structure
impl crate::Readable for CORE_1_INTR_CLR_SPEC {}
///`write(|w| ..)` method takes [`core_1_intr_clr::W`](W) writer structure
impl crate::Writable for CORE_1_INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_INTR_CLR to value 0
impl crate::Resettable for CORE_1_INTR_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
