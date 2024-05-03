#[doc = "Register `DMA_BLOCK_INTERVAL` reader"]
pub type R = crate::R<DMA_BLOCK_INTERVAL_SPEC>;
#[doc = "Register `DMA_BLOCK_INTERVAL` writer"]
pub type W = crate::W<DMA_BLOCK_INTERVAL_SPEC>;
#[doc = "Field `DMA_BLOCK_SLOT` reader - this field configures the max block_slot_cnt"]
pub type DMA_BLOCK_SLOT_R = crate::FieldReader<u16>;
#[doc = "Field `DMA_BLOCK_SLOT` writer - this field configures the max block_slot_cnt"]
pub type DMA_BLOCK_SLOT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DMA_BLOCK_INTERVAL` reader - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
pub type DMA_BLOCK_INTERVAL_R = crate::FieldReader<u32>;
#[doc = "Field `DMA_BLOCK_INTERVAL` writer - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
pub type DMA_BLOCK_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `RAW_NUM_TOTAL_AUTO_RELOAD` reader - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
pub type RAW_NUM_TOTAL_AUTO_RELOAD_R = crate::BitReader;
#[doc = "Field `RAW_NUM_TOTAL_AUTO_RELOAD` writer - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
pub type RAW_NUM_TOTAL_AUTO_RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - this field configures the max block_slot_cnt"]
    #[inline(always)]
    pub fn dma_block_slot(&self) -> DMA_BLOCK_SLOT_R {
        DMA_BLOCK_SLOT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:27 - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
    #[inline(always)]
    pub fn dma_block_interval(&self) -> DMA_BLOCK_INTERVAL_R {
        DMA_BLOCK_INTERVAL_R::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bit 28 - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn raw_num_total_auto_reload(&self) -> RAW_NUM_TOTAL_AUTO_RELOAD_R {
        RAW_NUM_TOTAL_AUTO_RELOAD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_BLOCK_INTERVAL")
            .field("dma_block_slot", &self.dma_block_slot().bits())
            .field("dma_block_interval", &self.dma_block_interval().bits())
            .field(
                "raw_num_total_auto_reload",
                &self.raw_num_total_auto_reload().bit(),
            )
            .field("en", &self.en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_BLOCK_INTERVAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - this field configures the max block_slot_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_block_slot(&mut self) -> DMA_BLOCK_SLOT_W<DMA_BLOCK_INTERVAL_SPEC> {
        DMA_BLOCK_SLOT_W::new(self, 0)
    }
    #[doc = "Bits 10:27 - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
    #[inline(always)]
    #[must_use]
    pub fn dma_block_interval(&mut self) -> DMA_BLOCK_INTERVAL_W<DMA_BLOCK_INTERVAL_SPEC> {
        DMA_BLOCK_INTERVAL_W::new(self, 10)
    }
    #[doc = "Bit 28 - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn raw_num_total_auto_reload(
        &mut self,
    ) -> RAW_NUM_TOTAL_AUTO_RELOAD_W<DMA_BLOCK_INTERVAL_SPEC> {
        RAW_NUM_TOTAL_AUTO_RELOAD_W::new(self, 28)
    }
    #[doc = "Bit 29 - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DMA_BLOCK_INTERVAL_SPEC> {
        EN_W::new(self, 29)
    }
}
#[doc = "dsi bridge dma block interval control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_block_interval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_block_interval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_BLOCK_INTERVAL_SPEC;
impl crate::RegisterSpec for DMA_BLOCK_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_block_interval::R`](R) reader structure"]
impl crate::Readable for DMA_BLOCK_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_block_interval::W`](W) writer structure"]
impl crate::Writable for DMA_BLOCK_INTERVAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_BLOCK_INTERVAL to value 0x3000_2409"]
impl crate::Resettable for DMA_BLOCK_INTERVAL_SPEC {
    const RESET_VALUE: u32 = 0x3000_2409;
}
