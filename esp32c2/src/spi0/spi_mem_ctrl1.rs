#[doc = "Register `SPI_MEM_CTRL1` reader"]
pub struct R(crate::R<SPI_MEM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CTRL1` writer"]
pub struct W(crate::W<SPI_MEM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CTRL1_SPEC>;
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
impl From<crate::W<SPI_MEM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_CTRL1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SPI_MEM_RXFIFO_RST` writer - SPI0 RX FIFO reset signal."]
pub type SPI_MEM_RXFIFO_RST_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CTRL1_SPEC, bool, 30>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&self) -> SPI_MEM_CLK_MODE_R {
        SPI_MEM_CLK_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&mut self) -> SPI_MEM_CLK_MODE_W {
        SPI_MEM_CLK_MODE_W::new(self)
    }
    #[doc = "Bit 30 - SPI0 RX FIFO reset signal."]
    #[inline(always)]
    pub fn spi_mem_rxfifo_rst(&mut self) -> SPI_MEM_RXFIFO_RST_W {
        SPI_MEM_RXFIFO_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl1](index.html) module"]
pub struct SPI_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ctrl1::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl1::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL1 to value 0"]
impl crate::Resettable for SPI_MEM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
