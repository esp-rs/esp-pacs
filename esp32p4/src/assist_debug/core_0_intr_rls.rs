#[doc = "Register `CORE_0_INTR_RLS` reader"]
pub type R = crate::R<CORE_0_INTR_RLS_SPEC>;
#[doc = "Register `CORE_0_INTR_RLS` writer"]
pub type W = crate::W<CORE_0_INTR_RLS_SPEC>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` reader - Core0 dram0 area0 read monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_0_RD_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` writer - Core0 dram0 area0 read monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_0_RD_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` reader - Core0 dram0 area0 write monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_0_WR_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` writer - Core0 dram0 area0 write monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_0_WR_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RLS` reader - Core0 dram0 area1 read monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_1_RD_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RLS` writer - Core0 dram0 area1 read monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_1_RD_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RLS` reader - Core0 dram0 area1 write monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_1_WR_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RLS` writer - Core0 dram0 area1 write monitor interrupt enable"]
pub type CORE_0_AREA_DRAM0_1_WR_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RLS` reader - Core0 PIF area0 read monitor interrupt enable"]
pub type CORE_0_AREA_PIF_0_RD_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RLS` writer - Core0 PIF area0 read monitor interrupt enable"]
pub type CORE_0_AREA_PIF_0_RD_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RLS` reader - Core0 PIF area0 write monitor interrupt enable"]
pub type CORE_0_AREA_PIF_0_WR_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RLS` writer - Core0 PIF area0 write monitor interrupt enable"]
pub type CORE_0_AREA_PIF_0_WR_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RLS` reader - Core0 PIF area1 read monitor interrupt enable"]
pub type CORE_0_AREA_PIF_1_RD_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RLS` writer - Core0 PIF area1 read monitor interrupt enable"]
pub type CORE_0_AREA_PIF_1_RD_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RLS` reader - Core0 PIF area1 write monitor interrupt enable"]
pub type CORE_0_AREA_PIF_1_WR_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RLS` writer - Core0 PIF area1 write monitor interrupt enable"]
pub type CORE_0_AREA_PIF_1_WR_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_RLS` reader - Core0 stackpoint underflow monitor interrupt enable"]
pub type CORE_0_SP_SPILL_MIN_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_RLS` writer - Core0 stackpoint underflow monitor interrupt enable"]
pub type CORE_0_SP_SPILL_MIN_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_RLS` reader - Core0 stackpoint overflow monitor interrupt enable"]
pub type CORE_0_SP_SPILL_MAX_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_RLS` writer - Core0 stackpoint overflow monitor interrupt enable"]
pub type CORE_0_SP_SPILL_MAX_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RLS` reader - IBUS busy monitor interrupt enable"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RLS` writer - IBUS busy monitor interrupt enable"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RLS` reader - DBUS busy monitor interrupt enbale"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R = crate::BitReader;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RLS` writer - DBUS busy monitor interrupt enbale"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&self) -> CORE_0_AREA_DRAM0_0_RD_RLS_R {
        CORE_0_AREA_DRAM0_0_RD_RLS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&self) -> CORE_0_AREA_DRAM0_0_WR_RLS_R {
        CORE_0_AREA_DRAM0_0_WR_RLS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(&self) -> CORE_0_AREA_DRAM0_1_RD_RLS_R {
        CORE_0_AREA_DRAM0_1_RD_RLS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(&self) -> CORE_0_AREA_DRAM0_1_WR_RLS_R {
        CORE_0_AREA_DRAM0_1_WR_RLS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&self) -> CORE_0_AREA_PIF_0_RD_RLS_R {
        CORE_0_AREA_PIF_0_RD_RLS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&self) -> CORE_0_AREA_PIF_0_WR_RLS_R {
        CORE_0_AREA_PIF_0_WR_RLS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&self) -> CORE_0_AREA_PIF_1_RD_RLS_R {
        CORE_0_AREA_PIF_1_RD_RLS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&self) -> CORE_0_AREA_PIF_1_WR_RLS_R {
        CORE_0_AREA_PIF_1_WR_RLS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&self) -> CORE_0_SP_SPILL_MIN_RLS_R {
        CORE_0_SP_SPILL_MIN_RLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&self) -> CORE_0_SP_SPILL_MAX_RLS_R {
        CORE_0_SP_SPILL_MAX_RLS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_RLS")
            .field(
                "core_0_area_dram0_0_rd_rls",
                &self.core_0_area_dram0_0_rd_rls(),
            )
            .field(
                "core_0_area_dram0_0_wr_rls",
                &self.core_0_area_dram0_0_wr_rls(),
            )
            .field(
                "core_0_area_dram0_1_rd_rls",
                &self.core_0_area_dram0_1_rd_rls(),
            )
            .field(
                "core_0_area_dram0_1_wr_rls",
                &self.core_0_area_dram0_1_wr_rls(),
            )
            .field("core_0_area_pif_0_rd_rls", &self.core_0_area_pif_0_rd_rls())
            .field("core_0_area_pif_0_wr_rls", &self.core_0_area_pif_0_wr_rls())
            .field("core_0_area_pif_1_rd_rls", &self.core_0_area_pif_1_rd_rls())
            .field("core_0_area_pif_1_wr_rls", &self.core_0_area_pif_1_wr_rls())
            .field("core_0_sp_spill_min_rls", &self.core_0_sp_spill_min_rls())
            .field("core_0_sp_spill_max_rls", &self.core_0_sp_spill_max_rls())
            .field(
                "core_0_iram0_exception_monitor_rls",
                &self.core_0_iram0_exception_monitor_rls(),
            )
            .field(
                "core_0_dram0_exception_monitor_rls",
                &self.core_0_dram0_exception_monitor_rls(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_RD_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_DRAM0_0_RD_RLS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_WR_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_DRAM0_0_WR_RLS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_1_RD_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_DRAM0_1_RD_RLS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_1_WR_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_DRAM0_1_WR_RLS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&mut self) -> CORE_0_AREA_PIF_0_RD_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_PIF_0_RD_RLS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&mut self) -> CORE_0_AREA_PIF_0_WR_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_PIF_0_WR_RLS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&mut self) -> CORE_0_AREA_PIF_1_RD_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_PIF_1_RD_RLS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&mut self) -> CORE_0_AREA_PIF_1_WR_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_AREA_PIF_1_WR_RLS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Core0 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&mut self) -> CORE_0_SP_SPILL_MIN_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_SP_SPILL_MIN_RLS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Core0 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&mut self) -> CORE_0_SP_SPILL_MAX_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_SP_SPILL_MAX_RLS_W::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(
        &mut self,
    ) -> CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(
        &mut self,
    ) -> CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W<CORE_0_INTR_RLS_SPEC> {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W::new(self, 11)
    }
}
#[doc = "core0 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_rls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_rls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_RLS_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_rls::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_intr_rls::W`](W) writer structure"]
impl crate::Writable for CORE_0_INTR_RLS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_INTR_RLS to value 0"]
impl crate::Resettable for CORE_0_INTR_RLS_SPEC {
    const RESET_VALUE: u32 = 0;
}
