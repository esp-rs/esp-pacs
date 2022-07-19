#[doc = "Register `SPI_CLOCK` reader"]
pub struct R(crate::R<SPI_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CLOCK` writer"]
pub struct W(crate::W<SPI_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CLOCK_SPEC>;
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
impl From<crate::W<SPI_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_CLKCNT_L` reader - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CLKCNT_L` writer - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_L_W<'a> = crate::FieldWriter<'a, u32, SPI_CLOCK_SPEC, u8, u8, 6, 0>;
#[doc = "Field `SPI_CLKCNT_H` reader - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CLKCNT_H` writer - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_H_W<'a> = crate::FieldWriter<'a, u32, SPI_CLOCK_SPEC, u8, u8, 6, 6>;
#[doc = "Field `SPI_CLKCNT_N` reader - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
pub type SPI_CLKCNT_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CLKCNT_N` writer - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
pub type SPI_CLKCNT_N_W<'a> = crate::FieldWriter<'a, u32, SPI_CLOCK_SPEC, u8, u8, 6, 12>;
#[doc = "Field `SPI_CLKDIV_PRE` reader - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
pub type SPI_CLKDIV_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CLKDIV_PRE` writer - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
pub type SPI_CLKDIV_PRE_W<'a> = crate::FieldWriter<'a, u32, SPI_CLOCK_SPEC, u8, u8, 4, 18>;
#[doc = "Field `SPI_CLK_EQU_SYSCLK` reader - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
pub type SPI_CLK_EQU_SYSCLK_R = crate::BitReader<bool>;
#[doc = "Field `SPI_CLK_EQU_SYSCLK` writer - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
pub type SPI_CLK_EQU_SYSCLK_W<'a> = crate::BitWriter<'a, u32, SPI_CLOCK_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_l(&self) -> SPI_CLKCNT_L_R {
        SPI_CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_h(&self) -> SPI_CLKCNT_H_R {
        SPI_CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_n(&self) -> SPI_CLKCNT_N_R {
        SPI_CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21 - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&self) -> SPI_CLKDIV_PRE_R {
        SPI_CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&self) -> SPI_CLK_EQU_SYSCLK_R {
        SPI_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_l(&mut self) -> SPI_CLKCNT_L_W {
        SPI_CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_h(&mut self) -> SPI_CLKCNT_H_W {
        SPI_CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_n(&mut self) -> SPI_CLKCNT_N_W {
        SPI_CLKCNT_N_W::new(self)
    }
    #[doc = "Bits 18:21 - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&mut self) -> SPI_CLKDIV_PRE_W {
        SPI_CLKDIV_PRE_W::new(self)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&mut self) -> SPI_CLK_EQU_SYSCLK_W {
        SPI_CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_clock](index.html) module"]
pub struct SPI_CLOCK_SPEC;
impl crate::RegisterSpec for SPI_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_clock::R](R) reader structure"]
impl crate::Readable for SPI_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_clock::W](W) writer structure"]
impl crate::Writable for SPI_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CLOCK to value 0x8000_3043"]
impl crate::Resettable for SPI_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_3043
    }
}
