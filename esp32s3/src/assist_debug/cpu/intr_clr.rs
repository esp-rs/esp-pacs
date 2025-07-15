#[doc = "Register `INTR_CLR` reader"]
pub type R = crate::R<INTR_CLR_SPEC>;
#[doc = "Register `INTR_CLR` writer"]
pub type W = crate::W<INTR_CLR_SPEC>;
#[doc = "Field `AREA_DRAM0_0_RD_CLR` reader - Core0 dram0 area0 read monitor interrupt clr"]
pub type AREA_DRAM0_0_RD_CLR_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_0_RD_CLR` writer - Core0 dram0 area0 read monitor interrupt clr"]
pub type AREA_DRAM0_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_DRAM0_0_WR_CLR` reader - Core0 dram0 area0 write monitor interrupt clr"]
pub type AREA_DRAM0_0_WR_CLR_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_0_WR_CLR` writer - Core0 dram0 area0 write monitor interrupt clr"]
pub type AREA_DRAM0_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_DRAM0_1_RD_CLR` reader - Core0 dram0 area1 read monitor interrupt clr"]
pub type AREA_DRAM0_1_RD_CLR_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_1_RD_CLR` writer - Core0 dram0 area1 read monitor interrupt clr"]
pub type AREA_DRAM0_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_DRAM0_1_WR_CLR` reader - Core0 dram0 area1 write monitor interrupt clr"]
pub type AREA_DRAM0_1_WR_CLR_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_1_WR_CLR` writer - Core0 dram0 area1 write monitor interrupt clr"]
pub type AREA_DRAM0_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_PIF_0_RD_CLR` reader - Core0 PIF area0 read monitor interrupt clr"]
pub type AREA_PIF_0_RD_CLR_R = crate::BitReader;
#[doc = "Field `AREA_PIF_0_RD_CLR` writer - Core0 PIF area0 read monitor interrupt clr"]
pub type AREA_PIF_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_PIF_0_WR_CLR` reader - Core0 PIF area0 write monitor interrupt clr"]
pub type AREA_PIF_0_WR_CLR_R = crate::BitReader;
#[doc = "Field `AREA_PIF_0_WR_CLR` writer - Core0 PIF area0 write monitor interrupt clr"]
pub type AREA_PIF_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_PIF_1_RD_CLR` reader - Core0 PIF area1 read monitor interrupt clr"]
pub type AREA_PIF_1_RD_CLR_R = crate::BitReader;
#[doc = "Field `AREA_PIF_1_RD_CLR` writer - Core0 PIF area1 read monitor interrupt clr"]
pub type AREA_PIF_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREA_PIF_1_WR_CLR` reader - Core0 PIF area1 write monitor interrupt clr"]
pub type AREA_PIF_1_WR_CLR_R = crate::BitReader;
#[doc = "Field `AREA_PIF_1_WR_CLR` writer - Core0 PIF area1 write monitor interrupt clr"]
pub type AREA_PIF_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP_SPILL_MIN_CLR` reader - Core0 stackpoint overflow monitor interrupt clr"]
pub type SP_SPILL_MIN_CLR_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MIN_CLR` writer - Core0 stackpoint overflow monitor interrupt clr"]
pub type SP_SPILL_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP_SPILL_MAX_CLR` reader - Core0 stackpoint underflow monitor interrupt clr"]
pub type SP_SPILL_MAX_CLR_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MAX_CLR` writer - Core0 stackpoint underflow monitor interrupt clr"]
pub type SP_SPILL_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRAM0_EXCEPTION_MONITOR_CLR` reader - IBUS busy monitor interrupt clr"]
pub type IRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
#[doc = "Field `IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr"]
pub type IRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM0_EXCEPTION_MONITOR_CLR` reader - DBUS busy monitor interrupt clr"]
pub type DRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
#[doc = "Field `DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr"]
pub type DRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_0_rd_clr(&self) -> AREA_DRAM0_0_RD_CLR_R {
        AREA_DRAM0_0_RD_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_0_wr_clr(&self) -> AREA_DRAM0_0_WR_CLR_R {
        AREA_DRAM0_0_WR_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_1_rd_clr(&self) -> AREA_DRAM0_1_RD_CLR_R {
        AREA_DRAM0_1_RD_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_1_wr_clr(&self) -> AREA_DRAM0_1_WR_CLR_R {
        AREA_DRAM0_1_WR_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_0_rd_clr(&self) -> AREA_PIF_0_RD_CLR_R {
        AREA_PIF_0_RD_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_0_wr_clr(&self) -> AREA_PIF_0_WR_CLR_R {
        AREA_PIF_0_WR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_1_rd_clr(&self) -> AREA_PIF_1_RD_CLR_R {
        AREA_PIF_1_RD_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_1_wr_clr(&self) -> AREA_PIF_1_WR_CLR_R {
        AREA_PIF_1_WR_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    pub fn sp_spill_min_clr(&self) -> SP_SPILL_MIN_CLR_R {
        SP_SPILL_MIN_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    pub fn sp_spill_max_clr(&self) -> SP_SPILL_MAX_CLR_R {
        SP_SPILL_MAX_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn iram0_exception_monitor_clr(&self) -> IRAM0_EXCEPTION_MONITOR_CLR_R {
        IRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn dram0_exception_monitor_clr(&self) -> DRAM0_EXCEPTION_MONITOR_CLR_R {
        DRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_CLR")
            .field("area_dram0_0_rd_clr", &self.area_dram0_0_rd_clr())
            .field("area_dram0_0_wr_clr", &self.area_dram0_0_wr_clr())
            .field("area_dram0_1_rd_clr", &self.area_dram0_1_rd_clr())
            .field("area_dram0_1_wr_clr", &self.area_dram0_1_wr_clr())
            .field("area_pif_0_rd_clr", &self.area_pif_0_rd_clr())
            .field("area_pif_0_wr_clr", &self.area_pif_0_wr_clr())
            .field("area_pif_1_rd_clr", &self.area_pif_1_rd_clr())
            .field("area_pif_1_wr_clr", &self.area_pif_1_wr_clr())
            .field("sp_spill_min_clr", &self.sp_spill_min_clr())
            .field("sp_spill_max_clr", &self.sp_spill_max_clr())
            .field(
                "iram0_exception_monitor_clr",
                &self.iram0_exception_monitor_clr(),
            )
            .field(
                "dram0_exception_monitor_clr",
                &self.dram0_exception_monitor_clr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_0_rd_clr(&mut self) -> AREA_DRAM0_0_RD_CLR_W<INTR_CLR_SPEC> {
        AREA_DRAM0_0_RD_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_0_wr_clr(&mut self) -> AREA_DRAM0_0_WR_CLR_W<INTR_CLR_SPEC> {
        AREA_DRAM0_0_WR_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_1_rd_clr(&mut self) -> AREA_DRAM0_1_RD_CLR_W<INTR_CLR_SPEC> {
        AREA_DRAM0_1_RD_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_dram0_1_wr_clr(&mut self) -> AREA_DRAM0_1_WR_CLR_W<INTR_CLR_SPEC> {
        AREA_DRAM0_1_WR_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_0_rd_clr(&mut self) -> AREA_PIF_0_RD_CLR_W<INTR_CLR_SPEC> {
        AREA_PIF_0_RD_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_0_wr_clr(&mut self) -> AREA_PIF_0_WR_CLR_W<INTR_CLR_SPEC> {
        AREA_PIF_0_WR_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_1_rd_clr(&mut self) -> AREA_PIF_1_RD_CLR_W<INTR_CLR_SPEC> {
        AREA_PIF_1_RD_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn area_pif_1_wr_clr(&mut self) -> AREA_PIF_1_WR_CLR_W<INTR_CLR_SPEC> {
        AREA_PIF_1_WR_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    pub fn sp_spill_min_clr(&mut self) -> SP_SPILL_MIN_CLR_W<INTR_CLR_SPEC> {
        SP_SPILL_MIN_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    pub fn sp_spill_max_clr(&mut self) -> SP_SPILL_MAX_CLR_W<INTR_CLR_SPEC> {
        SP_SPILL_MAX_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn iram0_exception_monitor_clr(&mut self) -> IRAM0_EXCEPTION_MONITOR_CLR_W<INTR_CLR_SPEC> {
        IRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn dram0_exception_monitor_clr(&mut self) -> DRAM0_EXCEPTION_MONITOR_CLR_W<INTR_CLR_SPEC> {
        DRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 11)
    }
}
#[doc = "core0 monitor interrupt clr register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_clr::R`](R) reader structure"]
impl crate::Readable for INTR_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_clr::W`](W) writer structure"]
impl crate::Writable for INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for INTR_CLR_SPEC {}
