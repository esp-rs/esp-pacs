#[doc = "Register `SPI_MEM_SRAM_CLK` reader"]
pub type R = crate::R<SPI_MEM_SRAM_CLK_SPEC>;
#[doc = "Register `SPI_MEM_SRAM_CLK` writer"]
pub type W = crate::W<SPI_MEM_SRAM_CLK_SPEC>;
#[doc = "Field `SPI_MEM_SCLKCNT_L` reader - For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N."]
pub type SPI_MEM_SCLKCNT_L_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SCLKCNT_L` writer - For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N."]
pub type SPI_MEM_SCLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_MEM_SCLKCNT_H` reader - For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
pub type SPI_MEM_SCLKCNT_H_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SCLKCNT_H` writer - For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
pub type SPI_MEM_SCLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_MEM_SCLKCNT_N` reader - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
pub type SPI_MEM_SCLKCNT_N_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SCLKCNT_N` writer - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
pub type SPI_MEM_SCLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_MEM_SCLK_EQU_SYSCLK` reader - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
pub type SPI_MEM_SCLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SCLK_EQU_SYSCLK` writer - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
pub type SPI_MEM_SCLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N."]
    #[inline(always)]
    pub fn spi_mem_sclkcnt_l(&self) -> SPI_MEM_SCLKCNT_L_R {
        SPI_MEM_SCLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
    #[inline(always)]
    pub fn spi_mem_sclkcnt_h(&self) -> SPI_MEM_SCLKCNT_H_R {
        SPI_MEM_SCLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
    #[inline(always)]
    pub fn spi_mem_sclkcnt_n(&self) -> SPI_MEM_SCLKCNT_N_R {
        SPI_MEM_SCLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
    #[inline(always)]
    pub fn spi_mem_sclk_equ_sysclk(&self) -> SPI_MEM_SCLK_EQU_SYSCLK_R {
        SPI_MEM_SCLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_SRAM_CLK")
            .field(
                "spi_mem_sclkcnt_l",
                &format_args!("{}", self.spi_mem_sclkcnt_l().bits()),
            )
            .field(
                "spi_mem_sclkcnt_h",
                &format_args!("{}", self.spi_mem_sclkcnt_h().bits()),
            )
            .field(
                "spi_mem_sclkcnt_n",
                &format_args!("{}", self.spi_mem_sclkcnt_n().bits()),
            )
            .field(
                "spi_mem_sclk_equ_sysclk",
                &format_args!("{}", self.spi_mem_sclk_equ_sysclk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_SRAM_CLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - For SPI0 external RAM interface, it must be equal to spi_mem_clkcnt_N."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sclkcnt_l(&mut self) -> SPI_MEM_SCLKCNT_L_W<SPI_MEM_SRAM_CLK_SPEC> {
        SPI_MEM_SCLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - For SPI0 external RAM interface, it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sclkcnt_h(&mut self) -> SPI_MEM_SCLKCNT_H_W<SPI_MEM_SRAM_CLK_SPEC> {
        SPI_MEM_SCLKCNT_H_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sclkcnt_n(&mut self) -> SPI_MEM_SCLKCNT_N_W<SPI_MEM_SRAM_CLK_SPEC> {
        SPI_MEM_SCLKCNT_N_W::new(self, 16)
    }
    #[doc = "Bit 31 - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sclk_equ_sysclk(&mut self) -> SPI_MEM_SCLK_EQU_SYSCLK_W<SPI_MEM_SRAM_CLK_SPEC> {
        SPI_MEM_SCLK_EQU_SYSCLK_W::new(self, 31)
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
#[doc = "SPI0 external RAM clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_sram_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_sram_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_SRAM_CLK_SPEC;
impl crate::RegisterSpec for SPI_MEM_SRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_sram_clk::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_SRAM_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_sram_clk::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_SRAM_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_SRAM_CLK to value 0x0003_0103"]
impl crate::Resettable for SPI_MEM_SRAM_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0103;
}
