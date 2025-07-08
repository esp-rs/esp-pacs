#[doc = "Register `MEM_SRAM_CLK` reader"]
pub type R = crate::R<MEM_SRAM_CLK_SPEC>;
#[doc = "Register `MEM_SRAM_CLK` writer"]
pub type W = crate::W<MEM_SRAM_CLK_SPEC>;
#[doc = "Field `MEM_SCLKCNT_L` reader - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
pub type MEM_SCLKCNT_L_R = crate::FieldReader;
#[doc = "Field `MEM_SCLKCNT_L` writer - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
pub type MEM_SCLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SCLKCNT_H` reader - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub type MEM_SCLKCNT_H_R = crate::FieldReader;
#[doc = "Field `MEM_SCLKCNT_H` writer - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub type MEM_SCLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SCLKCNT_N` reader - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
pub type MEM_SCLKCNT_N_R = crate::FieldReader;
#[doc = "Field `MEM_SCLKCNT_N` writer - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
pub type MEM_SCLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SCLK_EQU_SYSCLK` reader - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
pub type MEM_SCLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `MEM_SCLK_EQU_SYSCLK` writer - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
pub type MEM_SCLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn mem_sclkcnt_l(&self) -> MEM_SCLKCNT_L_R {
        MEM_SCLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn mem_sclkcnt_h(&self) -> MEM_SCLKCNT_H_R {
        MEM_SCLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn mem_sclkcnt_n(&self) -> MEM_SCLKCNT_N_R {
        MEM_SCLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
    #[inline(always)]
    pub fn mem_sclk_equ_sysclk(&self) -> MEM_SCLK_EQU_SYSCLK_R {
        MEM_SCLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_SRAM_CLK")
            .field("mem_sclkcnt_l", &self.mem_sclkcnt_l())
            .field("mem_sclkcnt_h", &self.mem_sclkcnt_h())
            .field("mem_sclkcnt_n", &self.mem_sclkcnt_n())
            .field("mem_sclk_equ_sysclk", &self.mem_sclk_equ_sysclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn mem_sclkcnt_l(&mut self) -> MEM_SCLKCNT_L_W<MEM_SRAM_CLK_SPEC> {
        MEM_SCLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn mem_sclkcnt_h(&mut self) -> MEM_SCLKCNT_H_W<MEM_SRAM_CLK_SPEC> {
        MEM_SCLKCNT_H_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn mem_sclkcnt_n(&mut self) -> MEM_SCLKCNT_N_W<MEM_SRAM_CLK_SPEC> {
        MEM_SCLKCNT_N_W::new(self, 16)
    }
    #[doc = "Bit 31 - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
    #[inline(always)]
    pub fn mem_sclk_equ_sysclk(&mut self) -> MEM_SCLK_EQU_SYSCLK_W<MEM_SRAM_CLK_SPEC> {
        MEM_SCLK_EQU_SYSCLK_W::new(self, 31)
    }
}
#[doc = "SPI0 external RAM clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_sram_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_sram_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_SRAM_CLK_SPEC;
impl crate::RegisterSpec for MEM_SRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sram_clk::R`](R) reader structure"]
impl crate::Readable for MEM_SRAM_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_sram_clk::W`](W) writer structure"]
impl crate::Writable for MEM_SRAM_CLK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_SRAM_CLK to value 0x0003_0103"]
impl crate::Resettable for MEM_SRAM_CLK_SPEC {
    const RESET_VALUE: u32 = 0x0003_0103;
}
