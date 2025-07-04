#[doc = "Register `SRAM_CLK` reader"]
pub type R = crate::R<SRAM_CLK_SPEC>;
#[doc = "Register `SRAM_CLK` writer"]
pub type W = crate::W<SRAM_CLK_SPEC>;
#[doc = "Field `SCLKCNT_L` reader - "]
pub type SCLKCNT_L_R = crate::FieldReader;
#[doc = "Field `SCLKCNT_L` writer - "]
pub type SCLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLKCNT_H` reader - "]
pub type SCLKCNT_H_R = crate::FieldReader;
#[doc = "Field `SCLKCNT_H` writer - "]
pub type SCLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLKCNT_N` reader - "]
pub type SCLKCNT_N_R = crate::FieldReader;
#[doc = "Field `SCLKCNT_N` writer - "]
pub type SCLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_EQU_SYSCLK` reader - "]
pub type SCLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `SCLK_EQU_SYSCLK` writer - "]
pub type SCLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sclkcnt_l(&self) -> SCLKCNT_L_R {
        SCLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sclkcnt_h(&self) -> SCLKCNT_H_R {
        SCLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sclkcnt_n(&self) -> SCLKCNT_N_R {
        SCLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&self) -> SCLK_EQU_SYSCLK_R {
        SCLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CLK")
            .field("sclk_equ_sysclk", &self.sclk_equ_sysclk())
            .field("sclkcnt_n", &self.sclkcnt_n())
            .field("sclkcnt_h", &self.sclkcnt_h())
            .field("sclkcnt_l", &self.sclkcnt_l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sclkcnt_l(&mut self) -> SCLKCNT_L_W<SRAM_CLK_SPEC> {
        SCLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sclkcnt_h(&mut self) -> SCLKCNT_H_W<SRAM_CLK_SPEC> {
        SCLKCNT_H_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sclkcnt_n(&mut self) -> SCLKCNT_N_W<SRAM_CLK_SPEC> {
        SCLKCNT_N_W::new(self, 16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&mut self) -> SCLK_EQU_SYSCLK_W<SRAM_CLK_SPEC> {
        SCLK_EQU_SYSCLK_W::new(self, 31)
    }
}
#[doc = "SPI Memory SRAM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CLK_SPEC;
impl crate::RegisterSpec for SRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_clk::R`](R) reader structure"]
impl crate::Readable for SRAM_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_clk::W`](W) writer structure"]
impl crate::Writable for SRAM_CLK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CLK to value 0"]
impl crate::Resettable for SRAM_CLK_SPEC {}
