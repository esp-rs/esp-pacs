#[doc = "Register `MST_AWQOS_REG0` reader"]
pub type R = crate::R<MST_AWQOS_REG0_SPEC>;
#[doc = "Register `MST_AWQOS_REG0` writer"]
pub type W = crate::W<MST_AWQOS_REG0_SPEC>;
#[doc = "Field `REG_CPU_AWQOS` reader - "]
pub type REG_CPU_AWQOS_R = crate::FieldReader;
#[doc = "Field `REG_CPU_AWQOS` writer - "]
pub type REG_CPU_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_CACHE_AWQOS` reader - "]
pub type REG_CACHE_AWQOS_R = crate::FieldReader;
#[doc = "Field `REG_CACHE_AWQOS` writer - "]
pub type REG_CACHE_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_DMA2D_AWQOS` reader - "]
pub type REG_DMA2D_AWQOS_R = crate::FieldReader;
#[doc = "Field `REG_DMA2D_AWQOS` writer - "]
pub type REG_DMA2D_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_PDMA_INT_AWQOS` reader - "]
pub type REG_PDMA_INT_AWQOS_R = crate::FieldReader;
#[doc = "Field `REG_PDMA_INT_AWQOS` writer - "]
pub type REG_PDMA_INT_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_GMAC_AWQOS` reader - "]
pub type REG_GMAC_AWQOS_R = crate::FieldReader;
#[doc = "Field `REG_GMAC_AWQOS` writer - "]
pub type REG_GMAC_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_AXI_SDMMC_INT_AWQOS` reader - "]
pub type REG_AXI_SDMMC_INT_AWQOS_R = crate::FieldReader;
#[doc = "Field `REG_AXI_SDMMC_INT_AWQOS` writer - "]
pub type REG_AXI_SDMMC_INT_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_cpu_awqos(&self) -> REG_CPU_AWQOS_R {
        REG_CPU_AWQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_cache_awqos(&self) -> REG_CACHE_AWQOS_R {
        REG_CACHE_AWQOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_dma2d_awqos(&self) -> REG_DMA2D_AWQOS_R {
        REG_DMA2D_AWQOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reg_pdma_int_awqos(&self) -> REG_PDMA_INT_AWQOS_R {
        REG_PDMA_INT_AWQOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_gmac_awqos(&self) -> REG_GMAC_AWQOS_R {
        REG_GMAC_AWQOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_axi_sdmmc_int_awqos(&self) -> REG_AXI_SDMMC_INT_AWQOS_R {
        REG_AXI_SDMMC_INT_AWQOS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MST_AWQOS_REG0")
            .field("reg_cpu_awqos", &self.reg_cpu_awqos())
            .field("reg_cache_awqos", &self.reg_cache_awqos())
            .field("reg_dma2d_awqos", &self.reg_dma2d_awqos())
            .field("reg_pdma_int_awqos", &self.reg_pdma_int_awqos())
            .field("reg_gmac_awqos", &self.reg_gmac_awqos())
            .field("reg_axi_sdmmc_int_awqos", &self.reg_axi_sdmmc_int_awqos())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_cpu_awqos(&mut self) -> REG_CPU_AWQOS_W<'_, MST_AWQOS_REG0_SPEC> {
        REG_CPU_AWQOS_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_cache_awqos(&mut self) -> REG_CACHE_AWQOS_W<'_, MST_AWQOS_REG0_SPEC> {
        REG_CACHE_AWQOS_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_dma2d_awqos(&mut self) -> REG_DMA2D_AWQOS_W<'_, MST_AWQOS_REG0_SPEC> {
        REG_DMA2D_AWQOS_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reg_pdma_int_awqos(&mut self) -> REG_PDMA_INT_AWQOS_W<'_, MST_AWQOS_REG0_SPEC> {
        REG_PDMA_INT_AWQOS_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_gmac_awqos(&mut self) -> REG_GMAC_AWQOS_W<'_, MST_AWQOS_REG0_SPEC> {
        REG_GMAC_AWQOS_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_axi_sdmmc_int_awqos(
        &mut self,
    ) -> REG_AXI_SDMMC_INT_AWQOS_W<'_, MST_AWQOS_REG0_SPEC> {
        REG_AXI_SDMMC_INT_AWQOS_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mst_awqos_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mst_awqos_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MST_AWQOS_REG0_SPEC;
impl crate::RegisterSpec for MST_AWQOS_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mst_awqos_reg0::R`](R) reader structure"]
impl crate::Readable for MST_AWQOS_REG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mst_awqos_reg0::W`](W) writer structure"]
impl crate::Writable for MST_AWQOS_REG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MST_AWQOS_REG0 to value 0"]
impl crate::Resettable for MST_AWQOS_REG0_SPEC {}
