#[doc = "Register `INTR_RAW` reader"]
pub type R = crate::R<INTR_RAW_SPEC>;
#[doc = "Field `AREA_DRAM0_0_RD_RAW` reader - The raw interrupt status of read operations in region 0 by Data bus."]
pub type AREA_DRAM0_0_RD_RAW_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_0_WR_RAW` reader - The raw interrupt status of write operations in region 0 by Data bus."]
pub type AREA_DRAM0_0_WR_RAW_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_1_RD_RAW` reader - The raw interrupt status of read operations in region 1 by Data bus."]
pub type AREA_DRAM0_1_RD_RAW_R = crate::BitReader;
#[doc = "Field `AREA_DRAM0_1_WR_RAW` reader - The raw interrupt status of write operations in region 1 by Data bus."]
pub type AREA_DRAM0_1_WR_RAW_R = crate::BitReader;
#[doc = "Field `AREA_PIF_0_RD_RAW` reader - The raw interrupt status of read operations in region 0 by Peripheral bus."]
pub type AREA_PIF_0_RD_RAW_R = crate::BitReader;
#[doc = "Field `AREA_PIF_0_WR_RAW` reader - The raw interrupt status of write operations in region 0 by Peripheral bus."]
pub type AREA_PIF_0_WR_RAW_R = crate::BitReader;
#[doc = "Field `AREA_PIF_1_RD_RAW` reader - The raw interrupt status of read operations in region 1 by Peripheral bus."]
pub type AREA_PIF_1_RD_RAW_R = crate::BitReader;
#[doc = "Field `AREA_PIF_1_WR_RAW` reader - The raw interrupt status of write operations in region 1 by Peripheral bus."]
pub type AREA_PIF_1_WR_RAW_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MIN_RAW` reader - The raw interrupt status of SP exceeding the lower bound address of SP monitored region."]
pub type SP_SPILL_MIN_RAW_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MAX_RAW` reader - The raw interrupt status of SP exceeding the upper bound address of SP monitored region."]
pub type SP_SPILL_MAX_RAW_R = crate::BitReader;
#[doc = "Field `IRAM0_EXCEPTION_MONITOR_RAW` reader - IBUS busy monitor interrupt status"]
pub type IRAM0_EXCEPTION_MONITOR_RAW_R = crate::BitReader;
#[doc = "Field `DRAM0_EXCEPTION_MONITOR_RAW` reader - DBUS busy monitor initerrupt status"]
pub type DRAM0_EXCEPTION_MONITOR_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of read operations in region 0 by Data bus."]
    #[inline(always)]
    pub fn area_dram0_0_rd_raw(&self) -> AREA_DRAM0_0_RD_RAW_R {
        AREA_DRAM0_0_RD_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of write operations in region 0 by Data bus."]
    #[inline(always)]
    pub fn area_dram0_0_wr_raw(&self) -> AREA_DRAM0_0_WR_RAW_R {
        AREA_DRAM0_0_WR_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of read operations in region 1 by Data bus."]
    #[inline(always)]
    pub fn area_dram0_1_rd_raw(&self) -> AREA_DRAM0_1_RD_RAW_R {
        AREA_DRAM0_1_RD_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of write operations in region 1 by Data bus."]
    #[inline(always)]
    pub fn area_dram0_1_wr_raw(&self) -> AREA_DRAM0_1_WR_RAW_R {
        AREA_DRAM0_1_WR_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of read operations in region 0 by Peripheral bus."]
    #[inline(always)]
    pub fn area_pif_0_rd_raw(&self) -> AREA_PIF_0_RD_RAW_R {
        AREA_PIF_0_RD_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of write operations in region 0 by Peripheral bus."]
    #[inline(always)]
    pub fn area_pif_0_wr_raw(&self) -> AREA_PIF_0_WR_RAW_R {
        AREA_PIF_0_WR_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of read operations in region 1 by Peripheral bus."]
    #[inline(always)]
    pub fn area_pif_1_rd_raw(&self) -> AREA_PIF_1_RD_RAW_R {
        AREA_PIF_1_RD_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status of write operations in region 1 by Peripheral bus."]
    #[inline(always)]
    pub fn area_pif_1_wr_raw(&self) -> AREA_PIF_1_WR_RAW_R {
        AREA_PIF_1_WR_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status of SP exceeding the lower bound address of SP monitored region."]
    #[inline(always)]
    pub fn sp_spill_min_raw(&self) -> SP_SPILL_MIN_RAW_R {
        SP_SPILL_MIN_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status of SP exceeding the upper bound address of SP monitored region."]
    #[inline(always)]
    pub fn sp_spill_max_raw(&self) -> SP_SPILL_MAX_RAW_R {
        SP_SPILL_MAX_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt status"]
    #[inline(always)]
    pub fn iram0_exception_monitor_raw(&self) -> IRAM0_EXCEPTION_MONITOR_RAW_R {
        IRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor initerrupt status"]
    #[inline(always)]
    pub fn dram0_exception_monitor_raw(&self) -> DRAM0_EXCEPTION_MONITOR_RAW_R {
        DRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RAW")
            .field("area_dram0_0_rd_raw", &self.area_dram0_0_rd_raw())
            .field("area_dram0_0_wr_raw", &self.area_dram0_0_wr_raw())
            .field("area_dram0_1_rd_raw", &self.area_dram0_1_rd_raw())
            .field("area_dram0_1_wr_raw", &self.area_dram0_1_wr_raw())
            .field("area_pif_0_rd_raw", &self.area_pif_0_rd_raw())
            .field("area_pif_0_wr_raw", &self.area_pif_0_wr_raw())
            .field("area_pif_1_rd_raw", &self.area_pif_1_rd_raw())
            .field("area_pif_1_wr_raw", &self.area_pif_1_wr_raw())
            .field("sp_spill_min_raw", &self.sp_spill_min_raw())
            .field("sp_spill_max_raw", &self.sp_spill_max_raw())
            .field(
                "iram0_exception_monitor_raw",
                &self.iram0_exception_monitor_raw(),
            )
            .field(
                "dram0_exception_monitor_raw",
                &self.dram0_exception_monitor_raw(),
            )
            .finish()
    }
}
#[doc = "core0 monitor interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RAW_SPEC;
impl crate::RegisterSpec for INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw::R`](R) reader structure"]
impl crate::Readable for INTR_RAW_SPEC {}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for INTR_RAW_SPEC {}
