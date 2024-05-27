///Register `DMA_APBPERI_PMS_MONITOR_2` reader
pub type R = crate::R<DMA_APBPERI_PMS_MONITOR_2_SPEC>;
///Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR` reader - dma_apbperi_pms_monitor_violate_intr
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R = crate::BitReader;
///Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - dma_apbperi_pms_monitor_violate_status_world
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R = crate::FieldReader;
///Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - dma_apbperi_pms_monitor_violate_status_addr
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bit 0 - dma_apbperi_pms_monitor_violate_intr
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_intr(&self) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - dma_apbperi_pms_monitor_violate_status_world
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_world(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:26 - dma_apbperi_pms_monitor_violate_status_addr
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_addr(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new((self.bits >> 3) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_PMS_MONITOR_2")
            .field(
                "dma_apbperi_pms_monitor_violate_intr",
                &self.dma_apbperi_pms_monitor_violate_intr(),
            )
            .field(
                "dma_apbperi_pms_monitor_violate_status_world",
                &self.dma_apbperi_pms_monitor_violate_status_world(),
            )
            .field(
                "dma_apbperi_pms_monitor_violate_status_addr",
                &self.dma_apbperi_pms_monitor_violate_status_addr(),
            )
            .finish()
    }
}
/**SENSITIVE_DMA_APBPERI_PMS_MONITOR_2_REG

You can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_APBPERI_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_apbperi_pms_monitor_2::R`](R) reader structure
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_2_SPEC {}
///`reset()` method sets DMA_APBPERI_PMS_MONITOR_2 to value 0
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
