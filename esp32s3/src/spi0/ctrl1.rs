#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLK_MODE` reader - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXFIFO_RST` reader - SPI0 RX FIFO reset signal. Set this bit and clear it before SPI0 transfer starts."]
pub type RXFIFO_RST_R = crate::BitReader;
#[doc = "Field `RXFIFO_RST` writer - SPI0 RX FIFO reset signal. Set this bit and clear it before SPI0 transfer starts."]
pub type RXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 30 - SPI0 RX FIFO reset signal. Set this bit and clear it before SPI0 transfer starts."]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("clk_mode", &self.clk_mode())
            .field("rxfifo_rst", &self.rxfifo_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<CTRL1_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 30 - SPI0 RX FIFO reset signal. Set this bit and clear it before SPI0 transfer starts."]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<CTRL1_SPEC> {
        RXFIFO_RST_W::new(self, 30)
    }
}
#[doc = "SPI0 control 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
