#[doc = "Register `DMA_APBPERI_PMS_MONITOR_3` reader"]
pub struct R(crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR` reader - dma_apbperi_pms_monitor_violate_status_wr"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R = crate::BitReader;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN` reader - dma_apbperi_pms_monitor_violate_status_byteen"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - dma_apbperi_pms_monitor_violate_status_wr"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_wr(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - dma_apbperi_pms_monitor_violate_status_byteen"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_byteen(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_PMS_MONITOR_3")
            .field(
                "dma_apbperi_pms_monitor_violate_status_wr",
                &format_args!("{}", self.dma_apbperi_pms_monitor_violate_status_wr().bit()),
            )
            .field(
                "dma_apbperi_pms_monitor_violate_status_byteen",
                &format_args!(
                    "{}",
                    self.dma_apbperi_pms_monitor_violate_status_byteen().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_APBPERI_PMS_MONITOR_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_pms_monitor_3](index.html) module"]
pub struct DMA_APBPERI_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_pms_monitor_3::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_APBPERI_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
