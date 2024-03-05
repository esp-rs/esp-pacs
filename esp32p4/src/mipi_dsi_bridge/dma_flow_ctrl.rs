#[doc = "Register `DMA_FLOW_CTRL` reader"]
pub type R = crate::R<DMA_FLOW_CTRL_SPEC>;
#[doc = "Register `DMA_FLOW_CTRL` writer"]
pub type W = crate::W<DMA_FLOW_CTRL_SPEC>;
#[doc = "Field `DSI_DMA_FLOW_CONTROLLER` reader - this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller"]
pub type DSI_DMA_FLOW_CONTROLLER_R = crate::BitReader;
#[doc = "Field `DSI_DMA_FLOW_CONTROLLER` writer - this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller"]
pub type DSI_DMA_FLOW_CONTROLLER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_FLOW_MULTIBLK_NUM` reader - this field configures the num of blocks when multi-blk is enable and dmac as flow controller"]
pub type DMA_FLOW_MULTIBLK_NUM_R = crate::FieldReader;
#[doc = "Field `DMA_FLOW_MULTIBLK_NUM` writer - this field configures the num of blocks when multi-blk is enable and dmac as flow controller"]
pub type DMA_FLOW_MULTIBLK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn dsi_dma_flow_controller(&self) -> DSI_DMA_FLOW_CONTROLLER_R {
        DSI_DMA_FLOW_CONTROLLER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - this field configures the num of blocks when multi-blk is enable and dmac as flow controller"]
    #[inline(always)]
    pub fn dma_flow_multiblk_num(&self) -> DMA_FLOW_MULTIBLK_NUM_R {
        DMA_FLOW_MULTIBLK_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_FLOW_CTRL")
            .field(
                "dsi_dma_flow_controller",
                &format_args!("{}", self.dsi_dma_flow_controller().bit()),
            )
            .field(
                "dma_flow_multiblk_num",
                &format_args!("{}", self.dma_flow_multiblk_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_FLOW_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_dma_flow_controller(&mut self) -> DSI_DMA_FLOW_CONTROLLER_W<DMA_FLOW_CTRL_SPEC> {
        DSI_DMA_FLOW_CONTROLLER_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - this field configures the num of blocks when multi-blk is enable and dmac as flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn dma_flow_multiblk_num(&mut self) -> DMA_FLOW_MULTIBLK_NUM_W<DMA_FLOW_CTRL_SPEC> {
        DMA_FLOW_MULTIBLK_NUM_W::new(self, 4)
    }
}
#[doc = "dsi_bridge dma flow controller register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_flow_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_flow_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_FLOW_CTRL_SPEC;
impl crate::RegisterSpec for DMA_FLOW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for DMA_FLOW_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for DMA_FLOW_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_FLOW_CTRL to value 0x11"]
impl crate::Resettable for DMA_FLOW_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
