#[doc = "Register `MEM_CLOCK` reader"]
pub type R = crate::R<MEM_CLOCK_SPEC>;
#[doc = "Register `MEM_CLOCK` writer"]
pub type W = crate::W<MEM_CLOCK_SPEC>;
#[doc = "Field `MEM_CLKCNT_L` reader - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
pub type MEM_CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `MEM_CLKCNT_L` writer - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
pub type MEM_CLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_CLKCNT_H` reader - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub type MEM_CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `MEM_CLKCNT_H` writer - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub type MEM_CLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_CLKCNT_N` reader - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
pub type MEM_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `MEM_CLKCNT_N` writer - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
pub type MEM_CLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_CLK_EQU_SYSCLK` reader - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
pub type MEM_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `MEM_CLK_EQU_SYSCLK` writer - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
pub type MEM_CLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn mem_clkcnt_l(&self) -> MEM_CLKCNT_L_R {
        MEM_CLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn mem_clkcnt_h(&self) -> MEM_CLKCNT_H_R {
        MEM_CLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn mem_clkcnt_n(&self) -> MEM_CLKCNT_N_R {
        MEM_CLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
    #[inline(always)]
    pub fn mem_clk_equ_sysclk(&self) -> MEM_CLK_EQU_SYSCLK_R {
        MEM_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CLOCK")
            .field("mem_clkcnt_l", &self.mem_clkcnt_l())
            .field("mem_clkcnt_h", &self.mem_clkcnt_h())
            .field("mem_clkcnt_n", &self.mem_clkcnt_n())
            .field("mem_clk_equ_sysclk", &self.mem_clk_equ_sysclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn mem_clkcnt_l(&mut self) -> MEM_CLKCNT_L_W<MEM_CLOCK_SPEC> {
        MEM_CLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn mem_clkcnt_h(&mut self) -> MEM_CLKCNT_H_W<MEM_CLOCK_SPEC> {
        MEM_CLKCNT_H_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn mem_clkcnt_n(&mut self) -> MEM_CLKCNT_N_W<MEM_CLOCK_SPEC> {
        MEM_CLKCNT_N_W::new(self, 16)
    }
    #[doc = "Bit 31 - 1: 1-division mode, the frequency of SPI bus clock equals to that of MSPI module clock."]
    #[inline(always)]
    pub fn mem_clk_equ_sysclk(&mut self) -> MEM_CLK_EQU_SYSCLK_W<MEM_CLOCK_SPEC> {
        MEM_CLK_EQU_SYSCLK_W::new(self, 31)
    }
}
#[doc = "SPI clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CLOCK_SPEC;
impl crate::RegisterSpec for MEM_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clock::R`](R) reader structure"]
impl crate::Readable for MEM_CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_clock::W`](W) writer structure"]
impl crate::Writable for MEM_CLOCK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CLOCK to value 0x0003_0103"]
impl crate::Resettable for MEM_CLOCK_SPEC {
    const RESET_VALUE: u32 = 0x0003_0103;
}
