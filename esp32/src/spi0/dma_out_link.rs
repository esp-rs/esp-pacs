///Register `DMA_OUT_LINK` reader
pub type R = crate::R<DMA_OUT_LINK_SPEC>;
///Register `DMA_OUT_LINK` writer
pub type W = crate::W<DMA_OUT_LINK_SPEC>;
///Field `OUTLINK_ADDR` reader - The address of the first outlink descriptor.
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
///Field `OUTLINK_ADDR` writer - The address of the first outlink descriptor.
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `OUTLINK_STOP` reader - Set the bit to stop to use outlink descriptor.
pub type OUTLINK_STOP_R = crate::BitReader;
///Field `OUTLINK_STOP` writer - Set the bit to stop to use outlink descriptor.
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTLINK_START` reader - Set the bit to start to use outlink descriptor.
pub type OUTLINK_START_R = crate::BitReader;
///Field `OUTLINK_START` writer - Set the bit to start to use outlink descriptor.
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTLINK_RESTART` reader - Set the bit to mount on new outlink descriptors.
pub type OUTLINK_RESTART_R = crate::BitReader;
///Field `OUTLINK_RESTART` writer - Set the bit to mount on new outlink descriptors.
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:19 - The address of the first outlink descriptor.
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 28 - Set the bit to stop to use outlink descriptor.
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Set the bit to start to use outlink descriptor.
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Set the bit to mount on new outlink descriptors.
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_LINK")
            .field("outlink_addr", &self.outlink_addr())
            .field("outlink_stop", &self.outlink_stop())
            .field("outlink_start", &self.outlink_start())
            .field("outlink_restart", &self.outlink_restart())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - The address of the first outlink descriptor.
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<DMA_OUT_LINK_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
    ///Bit 28 - Set the bit to stop to use outlink descriptor.
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<DMA_OUT_LINK_SPEC> {
        OUTLINK_STOP_W::new(self, 28)
    }
    ///Bit 29 - Set the bit to start to use outlink descriptor.
    #[inline(always)]
    #[must_use]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<DMA_OUT_LINK_SPEC> {
        OUTLINK_START_W::new(self, 29)
    }
    ///Bit 30 - Set the bit to mount on new outlink descriptors.
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<DMA_OUT_LINK_SPEC> {
        OUTLINK_RESTART_W::new(self, 30)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_OUT_LINK_SPEC;
impl crate::RegisterSpec for DMA_OUT_LINK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_out_link::R`](R) reader structure
impl crate::Readable for DMA_OUT_LINK_SPEC {}
///`write(|w| ..)` method takes [`dma_out_link::W`](W) writer structure
impl crate::Writable for DMA_OUT_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_OUT_LINK to value 0
impl crate::Resettable for DMA_OUT_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
