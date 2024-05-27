///Register `SRAM_CLK` reader
pub type R = crate::R<SRAM_CLK_SPEC>;
///Register `SRAM_CLK` writer
pub type W = crate::W<SRAM_CLK_SPEC>;
///Field `SCLKCNT_L` reader - It must equal to the value of SPI_MEM_SCLKCNT_N.
pub type SCLKCNT_L_R = crate::FieldReader;
///Field `SCLKCNT_L` writer - It must equal to the value of SPI_MEM_SCLKCNT_N.
pub type SCLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLKCNT_H` reader - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1).
pub type SCLKCNT_H_R = crate::FieldReader;
///Field `SCLKCNT_H` writer - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1).
pub type SCLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLKCNT_N` reader - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)
pub type SCLKCNT_N_R = crate::FieldReader;
///Field `SCLKCNT_N` writer - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)
pub type SCLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLK_EQU_SYSCLK` reader - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
pub type SCLK_EQU_SYSCLK_R = crate::BitReader;
///Field `SCLK_EQU_SYSCLK` writer - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
pub type SCLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - It must equal to the value of SPI_MEM_SCLKCNT_N.
    #[inline(always)]
    pub fn sclkcnt_l(&self) -> SCLKCNT_L_R {
        SCLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1).
    #[inline(always)]
    pub fn sclkcnt_h(&self) -> SCLKCNT_H_R {
        SCLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)
    #[inline(always)]
    pub fn sclkcnt_n(&self) -> SCLKCNT_N_R {
        SCLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
    #[inline(always)]
    pub fn sclk_equ_sysclk(&self) -> SCLK_EQU_SYSCLK_R {
        SCLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CLK")
            .field("sclkcnt_l", &self.sclkcnt_l())
            .field("sclkcnt_h", &self.sclkcnt_h())
            .field("sclkcnt_n", &self.sclkcnt_n())
            .field("sclk_equ_sysclk", &self.sclk_equ_sysclk())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - It must equal to the value of SPI_MEM_SCLKCNT_N.
    #[inline(always)]
    #[must_use]
    pub fn sclkcnt_l(&mut self) -> SCLKCNT_L_W<SRAM_CLK_SPEC> {
        SCLKCNT_L_W::new(self, 0)
    }
    ///Bits 8:15 - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1).
    #[inline(always)]
    #[must_use]
    pub fn sclkcnt_h(&mut self) -> SCLKCNT_H_W<SRAM_CLK_SPEC> {
        SCLKCNT_H_W::new(self, 8)
    }
    ///Bits 16:23 - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)
    #[inline(always)]
    #[must_use]
    pub fn sclkcnt_n(&mut self) -> SCLKCNT_N_W<SRAM_CLK_SPEC> {
        SCLKCNT_N_W::new(self, 16)
    }
    ///Bit 31 - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK.
    #[inline(always)]
    #[must_use]
    pub fn sclk_equ_sysclk(&mut self) -> SCLK_EQU_SYSCLK_W<SRAM_CLK_SPEC> {
        SCLK_EQU_SYSCLK_W::new(self, 31)
    }
}
/**SPI_CLK clock division register when SPI0 accesses to Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`sram_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRAM_CLK_SPEC;
impl crate::RegisterSpec for SRAM_CLK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sram_clk::R`](R) reader structure
impl crate::Readable for SRAM_CLK_SPEC {}
///`write(|w| ..)` method takes [`sram_clk::W`](W) writer structure
impl crate::Writable for SRAM_CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRAM_CLK to value 0x0003_0103
impl crate::Resettable for SRAM_CLK_SPEC {
    const RESET_VALUE: u32 = 0x0003_0103;
}
