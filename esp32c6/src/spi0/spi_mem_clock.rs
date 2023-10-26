#[doc = "Register `SPI_MEM_CLOCK` reader"]
pub type R = crate::R<SPI_MEM_CLOCK_SPEC>;
#[doc = "Register `SPI_MEM_CLOCK` writer"]
pub type W = crate::W<SPI_MEM_CLOCK_SPEC>;
#[doc = "Field `SPI_MEM_CLKCNT_L` reader - In the master mode it must be equal to spi_mem_clkcnt_N."]
pub type SPI_MEM_CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CLKCNT_L` writer - In the master mode it must be equal to spi_mem_clkcnt_N."]
pub type SPI_MEM_CLKCNT_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPI_MEM_CLKCNT_H` reader - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
pub type SPI_MEM_CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CLKCNT_H` writer - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
pub type SPI_MEM_CLKCNT_H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPI_MEM_CLKCNT_N` reader - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
pub type SPI_MEM_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CLKCNT_N` writer - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
pub type SPI_MEM_CLKCNT_N_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPI_MEM_CLK_EQU_SYSCLK` reader - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
pub type SPI_MEM_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CLK_EQU_SYSCLK` writer - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
pub type SPI_MEM_CLK_EQU_SYSCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - In the master mode it must be equal to spi_mem_clkcnt_N."]
    #[inline(always)]
    pub fn spi_mem_clkcnt_l(&self) -> SPI_MEM_CLKCNT_L_R {
        SPI_MEM_CLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
    #[inline(always)]
    pub fn spi_mem_clkcnt_h(&self) -> SPI_MEM_CLKCNT_H_R {
        SPI_MEM_CLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
    #[inline(always)]
    pub fn spi_mem_clkcnt_n(&self) -> SPI_MEM_CLKCNT_N_R {
        SPI_MEM_CLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
    #[inline(always)]
    pub fn spi_mem_clk_equ_sysclk(&self) -> SPI_MEM_CLK_EQU_SYSCLK_R {
        SPI_MEM_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CLOCK")
            .field(
                "spi_mem_clkcnt_l",
                &format_args!("{}", self.spi_mem_clkcnt_l().bits()),
            )
            .field(
                "spi_mem_clkcnt_h",
                &format_args!("{}", self.spi_mem_clkcnt_h().bits()),
            )
            .field(
                "spi_mem_clkcnt_n",
                &format_args!("{}", self.spi_mem_clkcnt_n().bits()),
            )
            .field(
                "spi_mem_clk_equ_sysclk",
                &format_args!("{}", self.spi_mem_clk_equ_sysclk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the master mode it must be equal to spi_mem_clkcnt_N."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clkcnt_l(&mut self) -> SPI_MEM_CLKCNT_L_W<SPI_MEM_CLOCK_SPEC, 0> {
        SPI_MEM_CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clkcnt_h(&mut self) -> SPI_MEM_CLKCNT_H_W<SPI_MEM_CLOCK_SPEC, 8> {
        SPI_MEM_CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clkcnt_n(&mut self) -> SPI_MEM_CLKCNT_N_W<SPI_MEM_CLOCK_SPEC, 16> {
        SPI_MEM_CLKCNT_N_W::new(self)
    }
    #[doc = "Bit 31 - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clk_equ_sysclk(&mut self) -> SPI_MEM_CLK_EQU_SYSCLK_W<SPI_MEM_CLOCK_SPEC, 31> {
        SPI_MEM_CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI clock division control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_clock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_clock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CLOCK_SPEC;
impl crate::RegisterSpec for SPI_MEM_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_clock::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_clock::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CLOCK to value 0x0003_0103"]
impl crate::Resettable for SPI_MEM_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0103;
}
