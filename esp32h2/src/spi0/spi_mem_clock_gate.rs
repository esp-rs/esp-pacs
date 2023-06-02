#[doc = "Register `SPI_MEM_CLOCK_GATE` reader"]
pub struct R(crate::R<SPI_MEM_CLOCK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CLOCK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CLOCK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CLOCK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CLOCK_GATE` writer"]
pub struct W(crate::W<SPI_MEM_CLOCK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CLOCK_GATE_SPEC>;
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
impl From<crate::W<SPI_MEM_CLOCK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CLOCK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_CLK_EN` reader - Register clock gate enable signal. 1: Enable. 0: Disable."]
pub type SPI_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI_CLK_EN` writer - Register clock gate enable signal. 1: Enable. 0: Disable."]
pub type SPI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CLOCK_GATE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Register clock gate enable signal. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CLOCK_GATE")
            .field("spi_clk_en", &format_args!("{}", self.spi_clk_en().bit()))
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
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W<0> {
        SPI_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 clock gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_clock_gate](index.html) module"]
pub struct SPI_MEM_CLOCK_GATE_SPEC;
impl crate::RegisterSpec for SPI_MEM_CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_clock_gate::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CLOCK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_clock_gate::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CLOCK_GATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CLOCK_GATE to value 0x01"]
impl crate::Resettable for SPI_MEM_CLOCK_GATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
