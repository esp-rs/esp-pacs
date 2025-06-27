#[doc = "Register `SPI_DMA_CHAN_SEL` reader"]
pub type R = crate::R<SPI_DMA_CHAN_SEL_SPEC>;
#[doc = "Register `SPI_DMA_CHAN_SEL` writer"]
pub type W = crate::W<SPI_DMA_CHAN_SEL_SPEC>;
#[doc = "Field `SPI1_DMA_CHAN_SEL` reader - "]
pub type SPI1_DMA_CHAN_SEL_R = crate::FieldReader;
#[doc = "Field `SPI1_DMA_CHAN_SEL` writer - "]
pub type SPI1_DMA_CHAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI2_DMA_CHAN_SEL` reader - "]
pub type SPI2_DMA_CHAN_SEL_R = crate::FieldReader;
#[doc = "Field `SPI2_DMA_CHAN_SEL` writer - "]
pub type SPI2_DMA_CHAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI3_DMA_CHAN_SEL` reader - "]
pub type SPI3_DMA_CHAN_SEL_R = crate::FieldReader;
#[doc = "Field `SPI3_DMA_CHAN_SEL` writer - "]
pub type SPI3_DMA_CHAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi1_dma_chan_sel(&self) -> SPI1_DMA_CHAN_SEL_R {
        SPI1_DMA_CHAN_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi2_dma_chan_sel(&self) -> SPI2_DMA_CHAN_SEL_R {
        SPI2_DMA_CHAN_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi3_dma_chan_sel(&self) -> SPI3_DMA_CHAN_SEL_R {
        SPI3_DMA_CHAN_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DMA_CHAN_SEL")
            .field("spi1_dma_chan_sel", &self.spi1_dma_chan_sel())
            .field("spi2_dma_chan_sel", &self.spi2_dma_chan_sel())
            .field("spi3_dma_chan_sel", &self.spi3_dma_chan_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi1_dma_chan_sel(&mut self) -> SPI1_DMA_CHAN_SEL_W<SPI_DMA_CHAN_SEL_SPEC> {
        SPI1_DMA_CHAN_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi2_dma_chan_sel(&mut self) -> SPI2_DMA_CHAN_SEL_W<SPI_DMA_CHAN_SEL_SPEC> {
        SPI2_DMA_CHAN_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi3_dma_chan_sel(&mut self) -> SPI3_DMA_CHAN_SEL_W<SPI_DMA_CHAN_SEL_SPEC> {
        SPI3_DMA_CHAN_SEL_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dma_chan_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dma_chan_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DMA_CHAN_SEL_SPEC;
impl crate::RegisterSpec for SPI_DMA_CHAN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dma_chan_sel::R`](R) reader structure"]
impl crate::Readable for SPI_DMA_CHAN_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_dma_chan_sel::W`](W) writer structure"]
impl crate::Writable for SPI_DMA_CHAN_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DMA_CHAN_SEL to value 0"]
impl crate::Resettable for SPI_DMA_CHAN_SEL_SPEC {}
