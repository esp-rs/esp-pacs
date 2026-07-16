#[doc = "Register `MST_ARQOS_REG0` reader"]
pub type R = crate::R<MST_ARQOS_REG0_SPEC>;
#[doc = "Register `MST_ARQOS_REG0` writer"]
pub type W = crate::W<MST_ARQOS_REG0_SPEC>;
#[doc = "Field `REG_CPU_ARQOS` reader - "]
pub type REG_CPU_ARQOS_R = crate::FieldReader;
#[doc = "Field `REG_CPU_ARQOS` writer - "]
pub type REG_CPU_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_CACHE_ARQOS` reader - "]
pub type REG_CACHE_ARQOS_R = crate::FieldReader;
#[doc = "Field `REG_CACHE_ARQOS` writer - "]
pub type REG_CACHE_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_DMA2D_ARQOS` reader - "]
pub type REG_DMA2D_ARQOS_R = crate::FieldReader;
#[doc = "Field `REG_DMA2D_ARQOS` writer - "]
pub type REG_DMA2D_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_AXI_PDMA_INT_ARQOS` reader - "]
pub type REG_AXI_PDMA_INT_ARQOS_R = crate::FieldReader;
#[doc = "Field `REG_AXI_PDMA_INT_ARQOS` writer - "]
pub type REG_AXI_PDMA_INT_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_GMAC_ARQOS` reader - "]
pub type REG_GMAC_ARQOS_R = crate::FieldReader;
#[doc = "Field `REG_GMAC_ARQOS` writer - "]
pub type REG_GMAC_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_AXI_SDMMC_INT_ARQOS` reader - "]
pub type REG_AXI_SDMMC_INT_ARQOS_R = crate::FieldReader;
#[doc = "Field `REG_AXI_SDMMC_INT_ARQOS` writer - "]
pub type REG_AXI_SDMMC_INT_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_cpu_arqos(&self) -> REG_CPU_ARQOS_R {
        REG_CPU_ARQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_cache_arqos(&self) -> REG_CACHE_ARQOS_R {
        REG_CACHE_ARQOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_dma2d_arqos(&self) -> REG_DMA2D_ARQOS_R {
        REG_DMA2D_ARQOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reg_axi_pdma_int_arqos(&self) -> REG_AXI_PDMA_INT_ARQOS_R {
        REG_AXI_PDMA_INT_ARQOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_gmac_arqos(&self) -> REG_GMAC_ARQOS_R {
        REG_GMAC_ARQOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_axi_sdmmc_int_arqos(&self) -> REG_AXI_SDMMC_INT_ARQOS_R {
        REG_AXI_SDMMC_INT_ARQOS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MST_ARQOS_REG0")
            .field("reg_cpu_arqos", &self.reg_cpu_arqos())
            .field("reg_cache_arqos", &self.reg_cache_arqos())
            .field("reg_dma2d_arqos", &self.reg_dma2d_arqos())
            .field("reg_axi_pdma_int_arqos", &self.reg_axi_pdma_int_arqos())
            .field("reg_gmac_arqos", &self.reg_gmac_arqos())
            .field("reg_axi_sdmmc_int_arqos", &self.reg_axi_sdmmc_int_arqos())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_cpu_arqos(&mut self) -> REG_CPU_ARQOS_W<'_, MST_ARQOS_REG0_SPEC> {
        REG_CPU_ARQOS_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_cache_arqos(&mut self) -> REG_CACHE_ARQOS_W<'_, MST_ARQOS_REG0_SPEC> {
        REG_CACHE_ARQOS_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_dma2d_arqos(&mut self) -> REG_DMA2D_ARQOS_W<'_, MST_ARQOS_REG0_SPEC> {
        REG_DMA2D_ARQOS_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reg_axi_pdma_int_arqos(&mut self) -> REG_AXI_PDMA_INT_ARQOS_W<'_, MST_ARQOS_REG0_SPEC> {
        REG_AXI_PDMA_INT_ARQOS_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_gmac_arqos(&mut self) -> REG_GMAC_ARQOS_W<'_, MST_ARQOS_REG0_SPEC> {
        REG_GMAC_ARQOS_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_axi_sdmmc_int_arqos(
        &mut self,
    ) -> REG_AXI_SDMMC_INT_ARQOS_W<'_, MST_ARQOS_REG0_SPEC> {
        REG_AXI_SDMMC_INT_ARQOS_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mst_arqos_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mst_arqos_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MST_ARQOS_REG0_SPEC;
impl crate::RegisterSpec for MST_ARQOS_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mst_arqos_reg0::R`](R) reader structure"]
impl crate::Readable for MST_ARQOS_REG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mst_arqos_reg0::W`](W) writer structure"]
impl crate::Writable for MST_ARQOS_REG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MST_ARQOS_REG0 to value 0"]
impl crate::Resettable for MST_ARQOS_REG0_SPEC {}
