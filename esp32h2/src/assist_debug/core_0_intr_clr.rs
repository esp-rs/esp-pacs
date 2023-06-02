#[doc = "Register `CORE_0_INTR_CLR` writer"]
pub struct W(crate::W<CORE_0_INTR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_INTR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` writer - Core0 dram0 area0 read monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` writer - Core0 dram0 area0 write monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` writer - Core0 dram0 area1 read monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_1_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` writer - Core0 dram0 area1 write monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_1_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` writer - Core0 PIF area0 read monitor interrupt clr"]
pub type CORE_0_AREA_PIF_0_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` writer - Core0 PIF area0 write monitor interrupt clr"]
pub type CORE_0_AREA_PIF_0_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` writer - Core0 PIF area1 read monitor interrupt clr"]
pub type CORE_0_AREA_PIF_1_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` writer - Core0 PIF area1 write monitor interrupt clr"]
pub type CORE_0_AREA_PIF_1_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` writer - Core0 stackpoint underflow monitor interrupt clr"]
pub type CORE_0_SP_SPILL_MIN_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` writer - Core0 stackpoint overflow monitor interrupt clr"]
pub type CORE_0_SP_SPILL_MAX_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_0_RD_CLR_W<0> {
        CORE_0_AREA_DRAM0_0_RD_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_0_WR_CLR_W<1> {
        CORE_0_AREA_DRAM0_0_WR_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_1_RD_CLR_W<2> {
        CORE_0_AREA_DRAM0_1_RD_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_1_WR_CLR_W<3> {
        CORE_0_AREA_DRAM0_1_WR_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_rd_clr(&mut self) -> CORE_0_AREA_PIF_0_RD_CLR_W<4> {
        CORE_0_AREA_PIF_0_RD_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_wr_clr(&mut self) -> CORE_0_AREA_PIF_0_WR_CLR_W<5> {
        CORE_0_AREA_PIF_0_WR_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_rd_clr(&mut self) -> CORE_0_AREA_PIF_1_RD_CLR_W<6> {
        CORE_0_AREA_PIF_1_RD_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_wr_clr(&mut self) -> CORE_0_AREA_PIF_1_WR_CLR_W<7> {
        CORE_0_AREA_PIF_1_WR_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W<8> {
        CORE_0_SP_SPILL_MIN_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W<9> {
        CORE_0_SP_SPILL_MAX_CLR_W::new(self)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_iram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<10> {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W::new(self)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_dram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<11> {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor interrupt clr register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_clr](index.html) module"]
pub struct CORE_0_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core_0_intr_clr::W](W) writer structure"]
impl crate::Writable for CORE_0_INTR_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_INTR_CLR to value 0"]
impl crate::Resettable for CORE_0_INTR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
