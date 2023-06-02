#[doc = "Register `CORE_0_INTR_RAW` reader"]
pub struct R(crate::R<CORE_0_INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RAW` reader - reg_core_0_area_dram0_0_rd_raw"]
pub type CORE_0_AREA_DRAM0_0_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RAW` reader - reg_core_0_area_dram0_0_wr_raw"]
pub type CORE_0_AREA_DRAM0_0_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RAW` reader - reg_core_0_area_dram0_1_rd_raw"]
pub type CORE_0_AREA_DRAM0_1_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RAW` reader - reg_core_0_area_dram0_1_wr_raw"]
pub type CORE_0_AREA_DRAM0_1_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RAW` reader - reg_core_0_area_pif_0_rd_raw"]
pub type CORE_0_AREA_PIF_0_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RAW` reader - reg_core_0_area_pif_0_wr_raw"]
pub type CORE_0_AREA_PIF_0_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RAW` reader - reg_core_0_area_pif_1_rd_raw"]
pub type CORE_0_AREA_PIF_1_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RAW` reader - reg_core_0_area_pif_1_wr_raw"]
pub type CORE_0_AREA_PIF_1_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_RAW` reader - reg_core_0_sp_spill_min_raw"]
pub type CORE_0_SP_SPILL_MIN_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_RAW` reader - reg_core_0_sp_spill_max_raw"]
pub type CORE_0_SP_SPILL_MAX_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RAW` reader - reg_core_0_iram0_exception_monitor_raw"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RAW` reader - reg_core_0_dram0_exception_monitor_raw"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_core_0_area_dram0_0_rd_raw"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_raw(&self) -> CORE_0_AREA_DRAM0_0_RD_RAW_R {
        CORE_0_AREA_DRAM0_0_RD_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_core_0_area_dram0_0_wr_raw"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_raw(&self) -> CORE_0_AREA_DRAM0_0_WR_RAW_R {
        CORE_0_AREA_DRAM0_0_WR_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_core_0_area_dram0_1_rd_raw"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_raw(&self) -> CORE_0_AREA_DRAM0_1_RD_RAW_R {
        CORE_0_AREA_DRAM0_1_RD_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_core_0_area_dram0_1_wr_raw"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_raw(&self) -> CORE_0_AREA_DRAM0_1_WR_RAW_R {
        CORE_0_AREA_DRAM0_1_WR_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_core_0_area_pif_0_rd_raw"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_raw(&self) -> CORE_0_AREA_PIF_0_RD_RAW_R {
        CORE_0_AREA_PIF_0_RD_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_core_0_area_pif_0_wr_raw"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_raw(&self) -> CORE_0_AREA_PIF_0_WR_RAW_R {
        CORE_0_AREA_PIF_0_WR_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_core_0_area_pif_1_rd_raw"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_raw(&self) -> CORE_0_AREA_PIF_1_RD_RAW_R {
        CORE_0_AREA_PIF_1_RD_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_core_0_area_pif_1_wr_raw"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_raw(&self) -> CORE_0_AREA_PIF_1_WR_RAW_R {
        CORE_0_AREA_PIF_1_WR_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_core_0_sp_spill_min_raw"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_raw(&self) -> CORE_0_SP_SPILL_MIN_RAW_R {
        CORE_0_SP_SPILL_MIN_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_core_0_sp_spill_max_raw"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_raw(&self) -> CORE_0_SP_SPILL_MAX_RAW_R {
        CORE_0_SP_SPILL_MAX_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_core_0_iram0_exception_monitor_raw"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_raw(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_core_0_dram0_exception_monitor_raw"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_raw(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_RAW")
            .field(
                "core_0_area_dram0_0_rd_raw",
                &format_args!("{}", self.core_0_area_dram0_0_rd_raw().bit()),
            )
            .field(
                "core_0_area_dram0_0_wr_raw",
                &format_args!("{}", self.core_0_area_dram0_0_wr_raw().bit()),
            )
            .field(
                "core_0_area_dram0_1_rd_raw",
                &format_args!("{}", self.core_0_area_dram0_1_rd_raw().bit()),
            )
            .field(
                "core_0_area_dram0_1_wr_raw",
                &format_args!("{}", self.core_0_area_dram0_1_wr_raw().bit()),
            )
            .field(
                "core_0_area_pif_0_rd_raw",
                &format_args!("{}", self.core_0_area_pif_0_rd_raw().bit()),
            )
            .field(
                "core_0_area_pif_0_wr_raw",
                &format_args!("{}", self.core_0_area_pif_0_wr_raw().bit()),
            )
            .field(
                "core_0_area_pif_1_rd_raw",
                &format_args!("{}", self.core_0_area_pif_1_rd_raw().bit()),
            )
            .field(
                "core_0_area_pif_1_wr_raw",
                &format_args!("{}", self.core_0_area_pif_1_wr_raw().bit()),
            )
            .field(
                "core_0_sp_spill_min_raw",
                &format_args!("{}", self.core_0_sp_spill_min_raw().bit()),
            )
            .field(
                "core_0_sp_spill_max_raw",
                &format_args!("{}", self.core_0_sp_spill_max_raw().bit()),
            )
            .field(
                "core_0_iram0_exception_monitor_raw",
                &format_args!("{}", self.core_0_iram0_exception_monitor_raw().bit()),
            )
            .field(
                "core_0_dram0_exception_monitor_raw",
                &format_args!("{}", self.core_0_dram0_exception_monitor_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_raw](index.html) module"]
pub struct CORE_0_INTR_RAW_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_raw::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_INTR_RAW to value 0"]
impl crate::Resettable for CORE_0_INTR_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
