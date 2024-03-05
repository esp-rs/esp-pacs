#[doc = "Register `SPI_CLOCK` reader"]
pub type R = crate::R<SPI_CLOCK_SPEC>;
#[doc = "Register `SPI_CLOCK` writer"]
pub type W = crate::W<SPI_CLOCK_SPEC>;
#[doc = "Field `SPI_CLKCNT_L` reader - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `SPI_CLKCNT_L` writer - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLKCNT_H` reader - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `SPI_CLKCNT_H` writer - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLKCNT_N` reader - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
pub type SPI_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `SPI_CLKCNT_N` writer - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
pub type SPI_CLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLKDIV_PRE` reader - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
pub type SPI_CLKDIV_PRE_R = crate::FieldReader;
#[doc = "Field `SPI_CLKDIV_PRE` writer - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
pub type SPI_CLKDIV_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI_CLK_EQU_SYSCLK` reader - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
pub type SPI_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `SPI_CLK_EQU_SYSCLK` writer - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
pub type SPI_CLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CLOCK")
            .field(
                "spi_clkcnt_l",
                &format_args!("{}", self.spi_clkcnt_l().bits()),
            )
            .field(
                "spi_clkcnt_h",
                &format_args!("{}", self.spi_clkcnt_h().bits()),
            )
            .field(
                "spi_clkcnt_n",
                &format_args!("{}", self.spi_clkcnt_n().bits()),
            )
            .field(
                "spi_clkdiv_pre",
                &format_args!("{}", self.spi_clkdiv_pre().bits()),
            )
            .field(
                "spi_clk_equ_sysclk",
                &format_args!("{}", self.spi_clk_equ_sysclk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkcnt_l(&mut self) -> SPI_CLKCNT_L_W<SPI_CLOCK_SPEC> {
        SPI_CLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkcnt_h(&mut self) -> SPI_CLKCNT_H_W<SPI_CLOCK_SPEC> {
        SPI_CLKCNT_H_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkcnt_n(&mut self) -> SPI_CLKCNT_N_W<SPI_CLOCK_SPEC> {
        SPI_CLKCNT_N_W::new(self, 12)
    }
    #[doc = "Bits 18:21 - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkdiv_pre(&mut self) -> SPI_CLKDIV_PRE_W<SPI_CLOCK_SPEC> {
        SPI_CLKDIV_PRE_W::new(self, 18)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_equ_sysclk(&mut self) -> SPI_CLK_EQU_SYSCLK_W<SPI_CLOCK_SPEC> {
        SPI_CLK_EQU_SYSCLK_W::new(self, 31)
    }
}
#[doc = "SPI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_clock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_clock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CLOCK_SPEC;
impl crate::RegisterSpec for SPI_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_clock::R`](R) reader structure"]
impl crate::Readable for SPI_CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_clock::W`](W) writer structure"]
impl crate::Writable for SPI_CLOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CLOCK to value 0x8000_3043"]
impl crate::Resettable for SPI_CLOCK_SPEC {
    const RESET_VALUE: u32 = 0x8000_3043;
}
