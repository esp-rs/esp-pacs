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
#[doc = "Field `spi_clkcnt_L` reader - In the master mode, it must be eqaul to spi_clkcnt_N. In the slave mode, it must be 0."]
pub type SPI_CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `spi_clkcnt_L` writer - In the master mode, it must be eqaul to spi_clkcnt_N. In the slave mode, it must be 0."]
pub type SPI_CLKCNT_L_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CLOCK_SPEC, 6, O>;
#[doc = "Field `spi_clkcnt_H` reader - In the master mode, it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode, it must be 0."]
pub type SPI_CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `spi_clkcnt_H` writer - In the master mode, it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode, it must be 0."]
pub type SPI_CLKCNT_H_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CLOCK_SPEC, 6, O>;
#[doc = "Field `spi_clkcnt_N` reader - In the master mode, it is the divider of spi_clk. So spi_clk frequency is 80MHz/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
pub type SPI_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `spi_clkcnt_N` writer - In the master mode, it is the divider of spi_clk. So spi_clk frequency is 80MHz/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
pub type SPI_CLKCNT_N_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CLOCK_SPEC, 6, O>;
#[doc = "Field `spi_clkdiv_pre` reader - In the master mode, it is pre-divider of spi_clk."]
pub type SPI_CLKDIV_PRE_R = crate::FieldReader<u16>;
#[doc = "Field `spi_clkdiv_pre` writer - In the master mode, it is pre-divider of spi_clk."]
pub type SPI_CLKDIV_PRE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CLOCK_SPEC, 13, O, u16>;
#[doc = "Field `spi_clk_equ_sysclk` reader - In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
pub type SPI_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `spi_clk_equ_sysclk` writer - In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
pub type SPI_CLK_EQU_SYSCLK_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CLOCK_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - In the master mode, it must be eqaul to spi_clkcnt_N. In the slave mode, it must be 0."]
    #[inline(always)]
    pub fn spi_clkcnt_l(&self) -> SPI_CLKCNT_L_R {
        SPI_CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - In the master mode, it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode, it must be 0."]
    #[inline(always)]
    pub fn spi_clkcnt_h(&self) -> SPI_CLKCNT_H_R {
        SPI_CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - In the master mode, it is the divider of spi_clk. So spi_clk frequency is 80MHz/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
    #[inline(always)]
    pub fn spi_clkcnt_n(&self) -> SPI_CLKCNT_N_R {
        SPI_CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:30 - In the master mode, it is pre-divider of spi_clk."]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&self) -> SPI_CLKDIV_PRE_R {
        SPI_CLKDIV_PRE_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&self) -> SPI_CLK_EQU_SYSCLK_R {
        SPI_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CLOCK")
            .field(
                "spi_clk_equ_sysclk",
                &format_args!("{}", self.spi_clk_equ_sysclk().bit()),
            )
            .field(
                "spi_clkdiv_pre",
                &format_args!("{}", self.spi_clkdiv_pre().bits()),
            )
            .field(
                "spi_clkcnt_n",
                &format_args!("{}", self.spi_clkcnt_n().bits()),
            )
            .field(
                "spi_clkcnt_h",
                &format_args!("{}", self.spi_clkcnt_h().bits()),
            )
            .field(
                "spi_clkcnt_l",
                &format_args!("{}", self.spi_clkcnt_l().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - In the master mode, it must be eqaul to spi_clkcnt_N. In the slave mode, it must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkcnt_l(&mut self) -> SPI_CLKCNT_L_W<0> {
        SPI_CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 6:11 - In the master mode, it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode, it must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkcnt_h(&mut self) -> SPI_CLKCNT_H_W<6> {
        SPI_CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 12:17 - In the master mode, it is the divider of spi_clk. So spi_clk frequency is 80MHz/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1)"]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkcnt_n(&mut self) -> SPI_CLKCNT_N_W<12> {
        SPI_CLKCNT_N_W::new(self)
    }
    #[doc = "Bits 18:30 - In the master mode, it is pre-divider of spi_clk."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkdiv_pre(&mut self) -> SPI_CLKDIV_PRE_W<18> {
        SPI_CLKDIV_PRE_W::new(self)
    }
    #[doc = "Bit 31 - In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_equ_sysclk(&mut self) -> SPI_CLK_EQU_SYSCLK_W<31> {
        SPI_CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In the master mode, 1: spi_clk is eqaul to 80MHz, 0: spi_clk is divided from 80 MHz clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_clock](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CLOCK to value 0"]
impl crate::Resettable for SPI_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
