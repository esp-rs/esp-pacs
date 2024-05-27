///Register `DMA_APBPERI_PMS_MONITOR_3` reader
pub type R = crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>;
///Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR` reader - recorded dma's write status when dma access violated permission, 1(write), 0(read)
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R = crate::BitReader;
///Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN` reader - recorded dma's byte enable status when dma access violated permission
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - recorded dma's write status when dma access violated permission, 1(write), 0(read)
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_wr(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:16 - recorded dma's byte enable status when dma access violated permission
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_byteen(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_PMS_MONITOR_3")
            .field(
                "dma_apbperi_pms_monitor_violate_status_wr",
                &self.dma_apbperi_pms_monitor_violate_status_wr(),
            )
            .field(
                "dma_apbperi_pms_monitor_violate_status_byteen",
                &self.dma_apbperi_pms_monitor_violate_status_byteen(),
            )
            .finish()
    }
}
/**dma permission monitor configuration register 3.

You can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_APBPERI_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_apbperi_pms_monitor_3::R`](R) reader structure
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_3_SPEC {}
///`reset()` method sets DMA_APBPERI_PMS_MONITOR_3 to value 0
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
