#[doc = "Register `SPI_DMA_CHAN_SEL` reader"]
pub struct R(crate::R<SPI_DMA_CHAN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_DMA_CHAN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_DMA_CHAN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_DMA_CHAN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_DMA_CHAN_SEL` writer"]
pub struct W(crate::W<SPI_DMA_CHAN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_DMA_CHAN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_DMA_CHAN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_DMA_CHAN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_DMA_CHAN_SEL` reader - "]
pub type SPI1_DMA_CHAN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI1_DMA_CHAN_SEL` writer - "]
pub type SPI1_DMA_CHAN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_DMA_CHAN_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPI2_DMA_CHAN_SEL` reader - "]
pub type SPI2_DMA_CHAN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI2_DMA_CHAN_SEL` writer - "]
pub type SPI2_DMA_CHAN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_DMA_CHAN_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPI3_DMA_CHAN_SEL` reader - "]
pub type SPI3_DMA_CHAN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI3_DMA_CHAN_SEL` writer - "]
pub type SPI3_DMA_CHAN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_DMA_CHAN_SEL_SPEC, u8, u8, 2, O>;
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
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi1_dma_chan_sel(&mut self) -> SPI1_DMA_CHAN_SEL_W<0> {
        SPI1_DMA_CHAN_SEL_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi2_dma_chan_sel(&mut self) -> SPI2_DMA_CHAN_SEL_W<2> {
        SPI2_DMA_CHAN_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi3_dma_chan_sel(&mut self) -> SPI3_DMA_CHAN_SEL_W<4> {
        SPI3_DMA_CHAN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_chan_sel](index.html) module"]
pub struct SPI_DMA_CHAN_SEL_SPEC;
impl crate::RegisterSpec for SPI_DMA_CHAN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_dma_chan_sel::R](R) reader structure"]
impl crate::Readable for SPI_DMA_CHAN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_dma_chan_sel::W](W) writer structure"]
impl crate::Writable for SPI_DMA_CHAN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_DMA_CHAN_SEL to value 0"]
impl crate::Resettable for SPI_DMA_CHAN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
