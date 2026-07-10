#[doc = "Register `ICM_MST_AWQOS` reader"]
pub type R = crate::R<ICM_MST_AWQOS_SPEC>;
#[doc = "Register `ICM_MST_AWQOS` writer"]
pub type W = crate::W<ICM_MST_AWQOS_SPEC>;
#[doc = "Field `CPU_AWQOS` reader - "]
pub type CPU_AWQOS_R = crate::FieldReader;
#[doc = "Field `CPU_AWQOS` writer - "]
pub type CPU_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CACHE_AWQOS` reader - "]
pub type CACHE_AWQOS_R = crate::FieldReader;
#[doc = "Field `CACHE_AWQOS` writer - "]
pub type CACHE_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMA2D_AWQOS` reader - "]
pub type DMA2D_AWQOS_R = crate::FieldReader;
#[doc = "Field `DMA2D_AWQOS` writer - "]
pub type DMA2D_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST1_AWQOS` reader - "]
pub type GDMA_MST1_AWQOS_R = crate::FieldReader;
#[doc = "Field `GDMA_MST1_AWQOS` writer - "]
pub type GDMA_MST1_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST2_AWQOS` reader - "]
pub type GDMA_MST2_AWQOS_R = crate::FieldReader;
#[doc = "Field `GDMA_MST2_AWQOS` writer - "]
pub type GDMA_MST2_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_DMA2D_M1_AWQOS` reader - "]
pub type H264_DMA2D_M1_AWQOS_R = crate::FieldReader;
#[doc = "Field `H264_DMA2D_M1_AWQOS` writer - "]
pub type H264_DMA2D_M1_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_DMA2D_M2_AWQOS` reader - "]
pub type H264_DMA2D_M2_AWQOS_R = crate::FieldReader;
#[doc = "Field `H264_DMA2D_M2_AWQOS` writer - "]
pub type H264_DMA2D_M2_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDMA_INT_AWQOS` reader - "]
pub type PDMA_INT_AWQOS_R = crate::FieldReader;
#[doc = "Field `PDMA_INT_AWQOS` writer - "]
pub type PDMA_INT_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_awqos(&self) -> CPU_AWQOS_R {
        CPU_AWQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_awqos(&self) -> CACHE_AWQOS_R {
        CACHE_AWQOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_awqos(&self) -> DMA2D_AWQOS_R {
        DMA2D_AWQOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_awqos(&self) -> GDMA_MST1_AWQOS_R {
        GDMA_MST1_AWQOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_awqos(&self) -> GDMA_MST2_AWQOS_R {
        GDMA_MST2_AWQOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_dma2d_m1_awqos(&self) -> H264_DMA2D_M1_AWQOS_R {
        H264_DMA2D_M1_AWQOS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_dma2d_m2_awqos(&self) -> H264_DMA2D_M2_AWQOS_R {
        H264_DMA2D_M2_AWQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pdma_int_awqos(&self) -> PDMA_INT_AWQOS_R {
        PDMA_INT_AWQOS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_MST_AWQOS")
            .field("cpu_awqos", &self.cpu_awqos())
            .field("cache_awqos", &self.cache_awqos())
            .field("dma2d_awqos", &self.dma2d_awqos())
            .field("gdma_mst1_awqos", &self.gdma_mst1_awqos())
            .field("gdma_mst2_awqos", &self.gdma_mst2_awqos())
            .field("h264_dma2d_m1_awqos", &self.h264_dma2d_m1_awqos())
            .field("h264_dma2d_m2_awqos", &self.h264_dma2d_m2_awqos())
            .field("pdma_int_awqos", &self.pdma_int_awqos())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_awqos(&mut self) -> CPU_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        CPU_AWQOS_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_awqos(&mut self) -> CACHE_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        CACHE_AWQOS_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_awqos(&mut self) -> DMA2D_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        DMA2D_AWQOS_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_awqos(&mut self) -> GDMA_MST1_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        GDMA_MST1_AWQOS_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_awqos(&mut self) -> GDMA_MST2_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        GDMA_MST2_AWQOS_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_dma2d_m1_awqos(&mut self) -> H264_DMA2D_M1_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        H264_DMA2D_M1_AWQOS_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_dma2d_m2_awqos(&mut self) -> H264_DMA2D_M2_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        H264_DMA2D_M2_AWQOS_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pdma_int_awqos(&mut self) -> PDMA_INT_AWQOS_W<'_, ICM_MST_AWQOS_SPEC> {
        PDMA_INT_AWQOS_W::new(self, 28)
    }
}
#[doc = "Master write QoS\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_awqos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_awqos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_MST_AWQOS_SPEC;
impl crate::RegisterSpec for ICM_MST_AWQOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_mst_awqos::R`](R) reader structure"]
impl crate::Readable for ICM_MST_AWQOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_mst_awqos::W`](W) writer structure"]
impl crate::Writable for ICM_MST_AWQOS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_MST_AWQOS to value 0"]
impl crate::Resettable for ICM_MST_AWQOS_SPEC {}
