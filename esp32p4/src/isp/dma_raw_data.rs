#[doc = "Register `DMA_RAW_DATA` reader"]
pub type R = crate::R<DMA_RAW_DATA_SPEC>;
#[doc = "Register `DMA_RAW_DATA` writer"]
pub type W = crate::W<DMA_RAW_DATA_SPEC>;
#[doc = "Field `DMA_RAW_NUM_TOTAL` reader - this field configures the the number of 64bits in a frame"]
pub type DMA_RAW_NUM_TOTAL_R = crate::FieldReader<u32>;
#[doc = "Field `DMA_RAW_NUM_TOTAL` writer - this field configures the the number of 64bits in a frame"]
pub type DMA_RAW_NUM_TOTAL_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `DMA_RAW_NUM_TOTAL_SET` writer - write 1 to update reg_dma_raw_num_total"]
pub type DMA_RAW_NUM_TOTAL_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - this field configures the the number of 64bits in a frame"]
    #[inline(always)]
    pub fn dma_raw_num_total(&self) -> DMA_RAW_NUM_TOTAL_R {
        DMA_RAW_NUM_TOTAL_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RAW_DATA")
            .field("dma_raw_num_total", &self.dma_raw_num_total())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - this field configures the the number of 64bits in a frame"]
    #[inline(always)]
    pub fn dma_raw_num_total(&mut self) -> DMA_RAW_NUM_TOTAL_W<'_, DMA_RAW_DATA_SPEC> {
        DMA_RAW_NUM_TOTAL_W::new(self, 0)
    }
    #[doc = "Bit 31 - write 1 to update reg_dma_raw_num_total"]
    #[inline(always)]
    pub fn dma_raw_num_total_set(&mut self) -> DMA_RAW_NUM_TOTAL_SET_W<'_, DMA_RAW_DATA_SPEC> {
        DMA_RAW_NUM_TOTAL_SET_W::new(self, 31)
    }
}
#[doc = "isp dma source total raw number set register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RAW_DATA_SPEC;
impl crate::RegisterSpec for DMA_RAW_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_raw_data::R`](R) reader structure"]
impl crate::Readable for DMA_RAW_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_raw_data::W`](W) writer structure"]
impl crate::Writable for DMA_RAW_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RAW_DATA to value 0"]
impl crate::Resettable for DMA_RAW_DATA_SPEC {}
