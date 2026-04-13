#[doc = "Register `ICM_MST_ARQOS` reader"]
pub type R = crate::R<ICM_MST_ARQOS_SPEC>;
#[doc = "Register `ICM_MST_ARQOS` writer"]
pub type W = crate::W<ICM_MST_ARQOS_SPEC>;
#[doc = "Field `CPU_ARQOS` reader - "]
pub type CPU_ARQOS_R = crate::FieldReader;
#[doc = "Field `CPU_ARQOS` writer - "]
pub type CPU_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CACHE_ARQOS` reader - "]
pub type CACHE_ARQOS_R = crate::FieldReader;
#[doc = "Field `CACHE_ARQOS` writer - "]
pub type CACHE_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMA2D_ARQOS` reader - "]
pub type DMA2D_ARQOS_R = crate::FieldReader;
#[doc = "Field `DMA2D_ARQOS` writer - "]
pub type DMA2D_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST1_ARQOS` reader - "]
pub type GDMA_MST1_ARQOS_R = crate::FieldReader;
#[doc = "Field `GDMA_MST1_ARQOS` writer - "]
pub type GDMA_MST1_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GDMA_MST2_ARQOS` reader - "]
pub type GDMA_MST2_ARQOS_R = crate::FieldReader;
#[doc = "Field `GDMA_MST2_ARQOS` writer - "]
pub type GDMA_MST2_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_DMA2D_M1_ARQOS` reader - "]
pub type H264_DMA2D_M1_ARQOS_R = crate::FieldReader;
#[doc = "Field `H264_DMA2D_M1_ARQOS` writer - "]
pub type H264_DMA2D_M1_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H264_DMA2D_M2_ARQOS` reader - "]
pub type H264_DMA2D_M2_ARQOS_R = crate::FieldReader;
#[doc = "Field `H264_DMA2D_M2_ARQOS` writer - "]
pub type H264_DMA2D_M2_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AXI_PDMA_INT_ARQOS` reader - "]
pub type AXI_PDMA_INT_ARQOS_R = crate::FieldReader;
#[doc = "Field `AXI_PDMA_INT_ARQOS` writer - "]
pub type AXI_PDMA_INT_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_arqos(&self) -> CPU_ARQOS_R {
        CPU_ARQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_arqos(&self) -> CACHE_ARQOS_R {
        CACHE_ARQOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_arqos(&self) -> DMA2D_ARQOS_R {
        DMA2D_ARQOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_arqos(&self) -> GDMA_MST1_ARQOS_R {
        GDMA_MST1_ARQOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_arqos(&self) -> GDMA_MST2_ARQOS_R {
        GDMA_MST2_ARQOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_dma2d_m1_arqos(&self) -> H264_DMA2D_M1_ARQOS_R {
        H264_DMA2D_M1_ARQOS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_dma2d_m2_arqos(&self) -> H264_DMA2D_M2_ARQOS_R {
        H264_DMA2D_M2_ARQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn axi_pdma_int_arqos(&self) -> AXI_PDMA_INT_ARQOS_R {
        AXI_PDMA_INT_ARQOS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_MST_ARQOS")
            .field("cpu_arqos", &self.cpu_arqos())
            .field("cache_arqos", &self.cache_arqos())
            .field("dma2d_arqos", &self.dma2d_arqos())
            .field("gdma_mst1_arqos", &self.gdma_mst1_arqos())
            .field("gdma_mst2_arqos", &self.gdma_mst2_arqos())
            .field("h264_dma2d_m1_arqos", &self.h264_dma2d_m1_arqos())
            .field("h264_dma2d_m2_arqos", &self.h264_dma2d_m2_arqos())
            .field("axi_pdma_int_arqos", &self.axi_pdma_int_arqos())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cpu_arqos(&mut self) -> CPU_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        CPU_ARQOS_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cache_arqos(&mut self) -> CACHE_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        CACHE_ARQOS_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dma2d_arqos(&mut self) -> DMA2D_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        DMA2D_ARQOS_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gdma_mst1_arqos(&mut self) -> GDMA_MST1_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        GDMA_MST1_ARQOS_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gdma_mst2_arqos(&mut self) -> GDMA_MST2_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        GDMA_MST2_ARQOS_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn h264_dma2d_m1_arqos(&mut self) -> H264_DMA2D_M1_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        H264_DMA2D_M1_ARQOS_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn h264_dma2d_m2_arqos(&mut self) -> H264_DMA2D_M2_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        H264_DMA2D_M2_ARQOS_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn axi_pdma_int_arqos(&mut self) -> AXI_PDMA_INT_ARQOS_W<'_, ICM_MST_ARQOS_SPEC> {
        AXI_PDMA_INT_ARQOS_W::new(self, 28)
    }
}
#[doc = "Master read QoS\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_arqos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_arqos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_MST_ARQOS_SPEC;
impl crate::RegisterSpec for ICM_MST_ARQOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_mst_arqos::R`](R) reader structure"]
impl crate::Readable for ICM_MST_ARQOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_mst_arqos::W`](W) writer structure"]
impl crate::Writable for ICM_MST_ARQOS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_MST_ARQOS to value 0"]
impl crate::Resettable for ICM_MST_ARQOS_SPEC {}
