#[doc = "Register `SPI_MEM_CLOCK_GATE` reader"]
pub type R = crate::R<SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "Register `SPI_MEM_CLOCK_GATE` writer"]
pub type W = crate::W<SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "Field `SPI_MEM_CLK_EN` reader - Register clock gate enable signal. 1: Enable. 0: Disable."]
pub type SPI_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CLK_EN` writer - Register clock gate enable signal. 1: Enable. 0: Disable."]
pub type SPI_MEM_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Register clock gate enable signal. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn spi_mem_clk_en(&self) -> SPI_MEM_CLK_EN_R {
        SPI_MEM_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CLOCK_GATE")
            .field(
                "spi_mem_clk_en",
                &format_args!("{}", self.spi_mem_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CLOCK_GATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Register clock gate enable signal. 1: Enable. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clk_en(&mut self) -> SPI_MEM_CLK_EN_W<SPI_MEM_CLOCK_GATE_SPEC, 0> {
        SPI_MEM_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI1 clk_gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_clock_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_clock_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CLOCK_GATE_SPEC;
impl crate::RegisterSpec for SPI_MEM_CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_clock_gate::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CLOCK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_clock_gate::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CLOCK_GATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CLOCK_GATE to value 0x01"]
impl crate::Resettable for SPI_MEM_CLOCK_GATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
