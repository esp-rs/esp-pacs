///Register `CLOCK` reader
pub type R = crate::R<CLOCK_SPEC>;
///Register `CLOCK` writer
pub type W = crate::W<CLOCK_SPEC>;
///Field `CLKCNT_L` reader - It must equal to the value of SPI_MEM_CLKCNT_N.
pub type CLKCNT_L_R = crate::FieldReader;
///Field `CLKCNT_L` writer - It must equal to the value of SPI_MEM_CLKCNT_N.
pub type CLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLKCNT_H` reader - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1).
pub type CLKCNT_H_R = crate::FieldReader;
///Field `CLKCNT_H` writer - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1).
pub type CLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLKCNT_N` reader - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)
pub type CLKCNT_N_R = crate::FieldReader;
///Field `CLKCNT_N` writer - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)
pub type CLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLK_EQU_SYSCLK` reader - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
pub type CLK_EQU_SYSCLK_R = crate::BitReader;
///Field `CLK_EQU_SYSCLK` writer - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
pub type CLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - It must equal to the value of SPI_MEM_CLKCNT_N.
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1).
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK")
            .field("clkcnt_l", &self.clkcnt_l())
            .field("clkcnt_h", &self.clkcnt_h())
            .field("clkcnt_n", &self.clkcnt_n())
            .field("clk_equ_sysclk", &self.clk_equ_sysclk())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - It must equal to the value of SPI_MEM_CLKCNT_N.
    #[inline(always)]
    #[must_use]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W<CLOCK_SPEC> {
        CLKCNT_L_W::new(self, 0)
    }
    ///Bits 8:15 - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1).
    #[inline(always)]
    #[must_use]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W<CLOCK_SPEC> {
        CLKCNT_H_W::new(self, 8)
    }
    ///Bits 16:23 - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)
    #[inline(always)]
    #[must_use]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W<CLOCK_SPEC> {
        CLKCNT_N_W::new(self, 16)
    }
    ///Bit 31 - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
    #[inline(always)]
    #[must_use]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W<CLOCK_SPEC> {
        CLK_EQU_SYSCLK_W::new(self, 31)
    }
}
/**SPI_CLK clock division register when SPI1 accesses to flash or Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clock::R`](R) reader structure
impl crate::Readable for CLOCK_SPEC {}
///`write(|w| ..)` method takes [`clock::W`](W) writer structure
impl crate::Writable for CLOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLOCK to value 0x0003_0103
impl crate::Resettable for CLOCK_SPEC {
    const RESET_VALUE: u32 = 0x0003_0103;
}
