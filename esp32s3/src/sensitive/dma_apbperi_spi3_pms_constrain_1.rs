#[doc = "Register `DMA_APBPERI_SPI3_PMS_CONSTRAIN_1` reader"]
pub type R = crate::R<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Register `DMA_APBPERI_SPI3_PMS_CONSTRAIN_1` writer"]
pub type W = crate::W<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC>;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0` reader - spi3's permission(store,load) in data region0 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0` writer - spi3's permission(store,load) in data region0 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1` reader - spi3's permission(store,load) in data region1 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1` writer - spi3's permission(store,load) in data region1 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2` reader - spi3's permission(store,load) in data region2 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2` writer - spi3's permission(store,load) in data region2 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3` reader - spi3's permission(store,load) in data region3 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3` writer - spi3's permission(store,load) in data region3 of SRAM"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` reader - spi3's permission(store,load) in dcache data sram block0"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` writer - spi3's permission(store,load) in dcache data sram block0"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<'a, REG> =
    crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` reader - spi3's permission(store,load) in dcache data sram block1"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` writer - spi3's permission(store,load) in dcache data sram block1"]
pub type DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<'a, REG> =
    crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - spi3's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_0(
        &self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0_R {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - spi3's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_1(
        &self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1_R {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - spi3's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_2(
        &self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2_R {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - spi3's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_3(
        &self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3_R {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - spi3's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    pub fn dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_0(
        &self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R::new(
            ((self.bits >> 8) & 3) as u8,
        )
    }
    #[doc = "Bits 10:11 - spi3's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    pub fn dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_1(
        &self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R::new(
            ((self.bits >> 10) & 3) as u8,
        )
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_SPI3_PMS_CONSTRAIN_1")
            .field(
                "dma_apbperi_spi3_pms_constrain_sram_pms_0",
                &self.dma_apbperi_spi3_pms_constrain_sram_pms_0(),
            )
            .field(
                "dma_apbperi_spi3_pms_constrain_sram_pms_1",
                &self.dma_apbperi_spi3_pms_constrain_sram_pms_1(),
            )
            .field(
                "dma_apbperi_spi3_pms_constrain_sram_pms_2",
                &self.dma_apbperi_spi3_pms_constrain_sram_pms_2(),
            )
            .field(
                "dma_apbperi_spi3_pms_constrain_sram_pms_3",
                &self.dma_apbperi_spi3_pms_constrain_sram_pms_3(),
            )
            .field(
                "dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_0",
                &self.dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_0(),
            )
            .field(
                "dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_1",
                &self.dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - spi3's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0_W<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC> {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - spi3's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1_W<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC> {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - spi3's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_2(
        &mut self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2_W<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC> {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - spi3's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi3_pms_constrain_sram_pms_3(
        &mut self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3_W<DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC> {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_PMS_3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - spi3's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC,
    > {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - spi3's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi3_pms_constrain_sram_cachedataarray_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC,
    > {
        DMA_APBPERI_SPI3_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W::new(self, 10)
    }
}
#[doc = "spi3 dma permission configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_spi3_pms_constrain_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_spi3_pms_constrain_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_apbperi_spi3_pms_constrain_1::R`](R) reader structure"]
impl crate::Readable for DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_apbperi_spi3_pms_constrain_1::W`](W) writer structure"]
impl crate::Writable for DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_APBPERI_SPI3_PMS_CONSTRAIN_1 to value 0x0fff"]
impl crate::Resettable for DMA_APBPERI_SPI3_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
