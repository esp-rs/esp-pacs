#[doc = "Register `CORE_1_INTR_ENA` reader"]
pub type R = crate::R<CORE_1_INTR_ENA_SPEC>;
#[doc = "Register `CORE_1_INTR_ENA` writer"]
pub type W = crate::W<CORE_1_INTR_ENA_SPEC>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_INTR_ENA` reader - Core1 dram0 area0 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_INTR_ENA` writer - Core1 dram0 area0 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_RD_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_INTR_ENA` reader - Core1 dram0 area0 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_INTR_ENA` writer - Core1 dram0 area0 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_WR_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_INTR_ENA` reader - Core1 dram0 area1 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_INTR_ENA` writer - Core1 dram0 area1 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_RD_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_INTR_ENA` reader - Core1 dram0 area1 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_INTR_ENA` writer - Core1 dram0 area1 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_WR_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_INTR_ENA` reader - Core1 PIF area0 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_INTR_ENA` writer - Core1 PIF area0 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_RD_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_INTR_ENA` reader - Core1 PIF area0 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_INTR_ENA` writer - Core1 PIF area0 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_WR_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_INTR_ENA` reader - Core1 PIF area1 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_INTR_ENA` writer - Core1 PIF area1 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_RD_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_INTR_ENA` reader - Core1 PIF area1 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_INTR_ENA` writer - Core1 PIF area1 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_WR_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MIN_INTR_ENA` reader - Core1 stackpoint overflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MIN_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MIN_INTR_ENA` writer - Core1 stackpoint overflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MIN_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MAX_INTR_ENA` reader - Core1 stackpoint underflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MAX_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MAX_INTR_ENA` writer - Core1 stackpoint underflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MAX_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA` reader - IBUS busy monitor interrupt enable"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA` writer - IBUS busy monitor interrupt enable"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA` reader - DBUS busy monitor interrupt enbale"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA` writer - DBUS busy monitor interrupt enbale"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_intr_ena(&self) -> CORE_1_AREA_DRAM0_0_RD_INTR_ENA_R {
        CORE_1_AREA_DRAM0_0_RD_INTR_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_intr_ena(&self) -> CORE_1_AREA_DRAM0_0_WR_INTR_ENA_R {
        CORE_1_AREA_DRAM0_0_WR_INTR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_intr_ena(&self) -> CORE_1_AREA_DRAM0_1_RD_INTR_ENA_R {
        CORE_1_AREA_DRAM0_1_RD_INTR_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_intr_ena(&self) -> CORE_1_AREA_DRAM0_1_WR_INTR_ENA_R {
        CORE_1_AREA_DRAM0_1_WR_INTR_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_intr_ena(&self) -> CORE_1_AREA_PIF_0_RD_INTR_ENA_R {
        CORE_1_AREA_PIF_0_RD_INTR_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_intr_ena(&self) -> CORE_1_AREA_PIF_0_WR_INTR_ENA_R {
        CORE_1_AREA_PIF_0_WR_INTR_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_intr_ena(&self) -> CORE_1_AREA_PIF_1_RD_INTR_ENA_R {
        CORE_1_AREA_PIF_1_RD_INTR_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_intr_ena(&self) -> CORE_1_AREA_PIF_1_WR_INTR_ENA_R {
        CORE_1_AREA_PIF_1_WR_INTR_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core1 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_intr_ena(&self) -> CORE_1_SP_SPILL_MIN_INTR_ENA_R {
        CORE_1_SP_SPILL_MIN_INTR_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core1 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_intr_ena(&self) -> CORE_1_SP_SPILL_MAX_INTR_ENA_R {
        CORE_1_SP_SPILL_MAX_INTR_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_intr_ena(
        &self,
    ) -> CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_R {
        CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_intr_ena(
        &self,
    ) -> CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_R {
        CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_INTR_ENA")
            .field(
                "core_1_area_dram0_0_rd_intr_ena",
                &self.core_1_area_dram0_0_rd_intr_ena(),
            )
            .field(
                "core_1_area_dram0_0_wr_intr_ena",
                &self.core_1_area_dram0_0_wr_intr_ena(),
            )
            .field(
                "core_1_area_dram0_1_rd_intr_ena",
                &self.core_1_area_dram0_1_rd_intr_ena(),
            )
            .field(
                "core_1_area_dram0_1_wr_intr_ena",
                &self.core_1_area_dram0_1_wr_intr_ena(),
            )
            .field(
                "core_1_area_pif_0_rd_intr_ena",
                &self.core_1_area_pif_0_rd_intr_ena(),
            )
            .field(
                "core_1_area_pif_0_wr_intr_ena",
                &self.core_1_area_pif_0_wr_intr_ena(),
            )
            .field(
                "core_1_area_pif_1_rd_intr_ena",
                &self.core_1_area_pif_1_rd_intr_ena(),
            )
            .field(
                "core_1_area_pif_1_wr_intr_ena",
                &self.core_1_area_pif_1_wr_intr_ena(),
            )
            .field(
                "core_1_sp_spill_min_intr_ena",
                &self.core_1_sp_spill_min_intr_ena(),
            )
            .field(
                "core_1_sp_spill_max_intr_ena",
                &self.core_1_sp_spill_max_intr_ena(),
            )
            .field(
                "core_1_iram0_exception_monitor_intr_ena",
                &self.core_1_iram0_exception_monitor_intr_ena(),
            )
            .field(
                "core_1_dram0_exception_monitor_intr_ena",
                &self.core_1_dram0_exception_monitor_intr_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_rd_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_0_RD_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_DRAM0_0_RD_INTR_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_wr_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_0_WR_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_DRAM0_0_WR_INTR_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_1_rd_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_1_RD_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_DRAM0_1_RD_INTR_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_1_wr_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_1_WR_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_DRAM0_1_WR_INTR_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_rd_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_PIF_0_RD_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_PIF_0_RD_INTR_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_wr_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_PIF_0_WR_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_PIF_0_WR_INTR_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_rd_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_PIF_1_RD_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_PIF_1_RD_INTR_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_wr_intr_ena(
        &mut self,
    ) -> CORE_1_AREA_PIF_1_WR_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_AREA_PIF_1_WR_INTR_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - Core1 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_spill_min_intr_ena(
        &mut self,
    ) -> CORE_1_SP_SPILL_MIN_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_SP_SPILL_MIN_INTR_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Core1 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_spill_max_intr_ena(
        &mut self,
    ) -> CORE_1_SP_SPILL_MAX_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_SP_SPILL_MAX_INTR_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_iram0_exception_monitor_intr_ena(
        &mut self,
    ) -> CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_dram0_exception_monitor_intr_ena(
        &mut self,
    ) -> CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_W<CORE_1_INTR_ENA_SPEC> {
        CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_W::new(self, 11)
    }
}
#[doc = "Core1 monitor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_intr_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_intr_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_INTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_1_INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_intr_ena::R`](R) reader structure"]
impl crate::Readable for CORE_1_INTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_intr_ena::W`](W) writer structure"]
impl crate::Writable for CORE_1_INTR_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_INTR_ENA to value 0"]
impl crate::Resettable for CORE_1_INTR_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
