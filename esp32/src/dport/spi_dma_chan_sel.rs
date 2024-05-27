///Register `SPI_DMA_CHAN_SEL` reader
pub type R = crate::R<SPI_DMA_CHAN_SEL_SPEC>;
///Register `SPI_DMA_CHAN_SEL` writer
pub type W = crate::W<SPI_DMA_CHAN_SEL_SPEC>;
///Field `SPI1_DMA_CHAN_SEL` reader -
pub type SPI1_DMA_CHAN_SEL_R = crate::FieldReader;
///Field `SPI1_DMA_CHAN_SEL` writer -
pub type SPI1_DMA_CHAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI2_DMA_CHAN_SEL` reader -
pub type SPI2_DMA_CHAN_SEL_R = crate::FieldReader;
///Field `SPI2_DMA_CHAN_SEL` writer -
pub type SPI2_DMA_CHAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI3_DMA_CHAN_SEL` reader -
pub type SPI3_DMA_CHAN_SEL_R = crate::FieldReader;
///Field `SPI3_DMA_CHAN_SEL` writer -
pub type SPI3_DMA_CHAN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn spi1_dma_chan_sel(&self) -> SPI1_DMA_CHAN_SEL_R {
        SPI1_DMA_CHAN_SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn spi2_dma_chan_sel(&self) -> SPI2_DMA_CHAN_SEL_R {
        SPI2_DMA_CHAN_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5
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
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn spi1_dma_chan_sel(&mut self) -> SPI1_DMA_CHAN_SEL_W<SPI_DMA_CHAN_SEL_SPEC> {
        SPI1_DMA_CHAN_SEL_W::new(self, 0)
    }
    ///Bits 2:3
    #[inline(always)]
    #[must_use]
    pub fn spi2_dma_chan_sel(&mut self) -> SPI2_DMA_CHAN_SEL_W<SPI_DMA_CHAN_SEL_SPEC> {
        SPI2_DMA_CHAN_SEL_W::new(self, 2)
    }
    ///Bits 4:5
    #[inline(always)]
    #[must_use]
    pub fn spi3_dma_chan_sel(&mut self) -> SPI3_DMA_CHAN_SEL_W<SPI_DMA_CHAN_SEL_SPEC> {
        SPI3_DMA_CHAN_SEL_W::new(self, 4)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_chan_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_chan_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPI_DMA_CHAN_SEL_SPEC;
impl crate::RegisterSpec for SPI_DMA_CHAN_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spi_dma_chan_sel::R`](R) reader structure
impl crate::Readable for SPI_DMA_CHAN_SEL_SPEC {}
///`write(|w| ..)` method takes [`spi_dma_chan_sel::W`](W) writer structure
impl crate::Writable for SPI_DMA_CHAN_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_DMA_CHAN_SEL to value 0
impl crate::Resettable for SPI_DMA_CHAN_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
