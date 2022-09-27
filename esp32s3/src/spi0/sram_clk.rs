#[doc = "Register `SRAM_CLK` reader"]
pub struct R(crate::R<SRAM_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CLK` writer"]
pub struct W(crate::W<SRAM_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CLK_SPEC>;
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
impl From<crate::W<SRAM_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLKCNT_L` reader - It must equal to the value of SPI_MEM_SCLKCNT_N."]
pub type SCLKCNT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLKCNT_L` writer - It must equal to the value of SPI_MEM_SCLKCNT_N."]
pub type SCLKCNT_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAM_CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLKCNT_H` reader - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub type SCLKCNT_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLKCNT_H` writer - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub type SCLKCNT_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAM_CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLKCNT_N` reader - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
pub type SCLKCNT_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLKCNT_N` writer - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
pub type SCLKCNT_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAM_CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLK_EQU_SYSCLK` reader - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub type SCLK_EQU_SYSCLK_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_EQU_SYSCLK` writer - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub type SCLK_EQU_SYSCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CLK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn sclkcnt_l(&self) -> SCLKCNT_L_R {
        SCLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn sclkcnt_h(&self) -> SCLKCNT_H_R {
        SCLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn sclkcnt_n(&self) -> SCLKCNT_N_R {
        SCLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&self) -> SCLK_EQU_SYSCLK_R {
        SCLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn sclkcnt_l(&mut self) -> SCLKCNT_L_W<0> {
        SCLKCNT_L_W::new(self)
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn sclkcnt_h(&mut self) -> SCLKCNT_H_W<8> {
        SCLKCNT_H_W::new(self)
    }
    #[doc = "Bits 16:23 - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn sclkcnt_n(&mut self) -> SCLKCNT_N_W<16> {
        SCLKCNT_N_W::new(self)
    }
    #[doc = "Bit 31 - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&mut self) -> SCLK_EQU_SYSCLK_W<31> {
        SCLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CLK clock division register when SPI0 accesses to Ext_RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_clk](index.html) module"]
pub struct SRAM_CLK_SPEC;
impl crate::RegisterSpec for SRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_clk::R](R) reader structure"]
impl crate::Readable for SRAM_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_clk::W](W) writer structure"]
impl crate::Writable for SRAM_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_CLK to value 0x0003_0103"]
impl crate::Resettable for SRAM_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0103
    }
}
