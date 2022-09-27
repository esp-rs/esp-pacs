#[doc = "Register `CLOCK` reader"]
pub struct R(crate::R<CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK` writer"]
pub struct W(crate::W<CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_SPEC>;
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
impl From<crate::W<CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKCNT_L` reader - It must equal to the value of SPI_MEM_CLKCNT_N."]
pub type CLKCNT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCNT_L` writer - It must equal to the value of SPI_MEM_CLKCNT_N."]
pub type CLKCNT_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKCNT_H` reader - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub type CLKCNT_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCNT_H` writer - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub type CLKCNT_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKCNT_N` reader - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
pub type CLKCNT_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCNT_N` writer - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
pub type CLKCNT_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLK_EQU_SYSCLK` reader - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub type CLK_EQU_SYSCLK_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EQU_SYSCLK` writer - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub type CLK_EQU_SYSCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W<0> {
        CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W<8> {
        CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 16:23 - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W<16> {
        CLKCNT_N_W::new(self)
    }
    #[doc = "Bit 31 - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W<31> {
        CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CLK clock division register when SPI1 accesses to flash or Ext_RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](index.html) module"]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock::R](R) reader structure"]
impl crate::Readable for CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock::W](W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK to value 0x0003_0103"]
impl crate::Resettable for CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0103
    }
}
