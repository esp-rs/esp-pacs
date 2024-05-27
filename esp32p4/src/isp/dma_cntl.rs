///Register `DMA_CNTL` reader
pub type R = crate::R<DMA_CNTL_SPEC>;
///Register `DMA_CNTL` writer
pub type W = crate::W<DMA_CNTL_SPEC>;
///Field `DMA_EN` writer - write 1 to triger dma to get 1 frame
pub type DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_UPDATE` reader - write 1 to update reg_dma_burst_len &amp; reg_dma_data_type
pub type DMA_UPDATE_R = crate::BitReader;
///Field `DMA_UPDATE` writer - write 1 to update reg_dma_burst_len &amp; reg_dma_data_type
pub type DMA_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_DATA_TYPE` reader - this field configures the idi data type for image data
pub type DMA_DATA_TYPE_R = crate::FieldReader;
///Field `DMA_DATA_TYPE` writer - this field configures the idi data type for image data
pub type DMA_DATA_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DMA_BURST_LEN` reader - this field configures dma burst len when data source is dma. set according to dma_msize, it is the number of 64bits in a dma transfer
pub type DMA_BURST_LEN_R = crate::FieldReader<u16>;
///Field `DMA_BURST_LEN` writer - this field configures dma burst len when data source is dma. set according to dma_msize, it is the number of 64bits in a dma transfer
pub type DMA_BURST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DMA_INTERVAL` reader - this field configures dma req interval, 12'b1: 1 cycle, 12'b11 2 cycle ...
pub type DMA_INTERVAL_R = crate::FieldReader<u16>;
///Field `DMA_INTERVAL` writer - this field configures dma req interval, 12'b1: 1 cycle, 12'b11 2 cycle ...
pub type DMA_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 1 - write 1 to update reg_dma_burst_len &amp; reg_dma_data_type
    #[inline(always)]
    pub fn dma_update(&self) -> DMA_UPDATE_R {
        DMA_UPDATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:7 - this field configures the idi data type for image data
    #[inline(always)]
    pub fn dma_data_type(&self) -> DMA_DATA_TYPE_R {
        DMA_DATA_TYPE_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    ///Bits 8:19 - this field configures dma burst len when data source is dma. set according to dma_msize, it is the number of 64bits in a dma transfer
    #[inline(always)]
    pub fn dma_burst_len(&self) -> DMA_BURST_LEN_R {
        DMA_BURST_LEN_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    ///Bits 20:31 - this field configures dma req interval, 12'b1: 1 cycle, 12'b11 2 cycle ...
    #[inline(always)]
    pub fn dma_interval(&self) -> DMA_INTERVAL_R {
        DMA_INTERVAL_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CNTL")
            .field("dma_update", &self.dma_update())
            .field("dma_data_type", &self.dma_data_type())
            .field("dma_burst_len", &self.dma_burst_len())
            .field("dma_interval", &self.dma_interval())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to triger dma to get 1 frame
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<DMA_CNTL_SPEC> {
        DMA_EN_W::new(self, 0)
    }
    ///Bit 1 - write 1 to update reg_dma_burst_len &amp; reg_dma_data_type
    #[inline(always)]
    #[must_use]
    pub fn dma_update(&mut self) -> DMA_UPDATE_W<DMA_CNTL_SPEC> {
        DMA_UPDATE_W::new(self, 1)
    }
    ///Bits 2:7 - this field configures the idi data type for image data
    #[inline(always)]
    #[must_use]
    pub fn dma_data_type(&mut self) -> DMA_DATA_TYPE_W<DMA_CNTL_SPEC> {
        DMA_DATA_TYPE_W::new(self, 2)
    }
    ///Bits 8:19 - this field configures dma burst len when data source is dma. set according to dma_msize, it is the number of 64bits in a dma transfer
    #[inline(always)]
    #[must_use]
    pub fn dma_burst_len(&mut self) -> DMA_BURST_LEN_W<DMA_CNTL_SPEC> {
        DMA_BURST_LEN_W::new(self, 8)
    }
    ///Bits 20:31 - this field configures dma req interval, 12'b1: 1 cycle, 12'b11 2 cycle ...
    #[inline(always)]
    #[must_use]
    pub fn dma_interval(&mut self) -> DMA_INTERVAL_W<DMA_CNTL_SPEC> {
        DMA_INTERVAL_W::new(self, 20)
    }
}
/**isp dma source trans control register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_CNTL_SPEC;
impl crate::RegisterSpec for DMA_CNTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_cntl::R`](R) reader structure
impl crate::Readable for DMA_CNTL_SPEC {}
///`write(|w| ..)` method takes [`dma_cntl::W`](W) writer structure
impl crate::Writable for DMA_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_CNTL to value 0x0010_80a8
impl crate::Resettable for DMA_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x0010_80a8;
}
