#[doc = "Register `SPI_MEM_CLOCK` reader"]
pub struct R(crate::R<SPI_MEM_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CLOCK` writer"]
pub struct W(crate::W<SPI_MEM_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CLOCK_SPEC>;
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
impl From<crate::W<SPI_MEM_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CLKCNT_L` reader - In the master mode it must be equal to spi_mem_clkcnt_N."]
pub type SPI_MEM_CLKCNT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CLKCNT_L` writer - In the master mode it must be equal to spi_mem_clkcnt_N."]
pub type SPI_MEM_CLKCNT_L_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_CLOCK_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SPI_MEM_CLKCNT_H` reader - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
pub type SPI_MEM_CLKCNT_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CLKCNT_H` writer - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
pub type SPI_MEM_CLKCNT_H_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_CLOCK_SPEC, u8, u8, 8, 8>;
#[doc = "Field `SPI_MEM_CLKCNT_N` reader - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
pub type SPI_MEM_CLKCNT_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CLKCNT_N` writer - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
pub type SPI_MEM_CLKCNT_N_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_CLOCK_SPEC, u8, u8, 8, 16>;
#[doc = "Field `SPI_MEM_CLK_EQU_SYSCLK` reader - Set this bit in 1-division mode."]
pub type SPI_MEM_CLK_EQU_SYSCLK_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CLK_EQU_SYSCLK` writer - Set this bit in 1-division mode."]
pub type SPI_MEM_CLK_EQU_SYSCLK_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CLOCK_SPEC, bool, 31>;
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
    #[doc = "Bit 31 - Set this bit in 1-division mode."]
    #[inline(always)]
    pub fn spi_mem_clk_equ_sysclk(&self) -> SPI_MEM_CLK_EQU_SYSCLK_R {
        SPI_MEM_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the master mode it must be equal to spi_mem_clkcnt_N."]
    #[inline(always)]
    pub fn spi_mem_clkcnt_l(&mut self) -> SPI_MEM_CLKCNT_L_W {
        SPI_MEM_CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
    #[inline(always)]
    pub fn spi_mem_clkcnt_h(&mut self) -> SPI_MEM_CLKCNT_H_W {
        SPI_MEM_CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
    #[inline(always)]
    pub fn spi_mem_clkcnt_n(&mut self) -> SPI_MEM_CLKCNT_N_W {
        SPI_MEM_CLKCNT_N_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit in 1-division mode."]
    #[inline(always)]
    pub fn spi_mem_clk_equ_sysclk(&mut self) -> SPI_MEM_CLK_EQU_SYSCLK_W {
        SPI_MEM_CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI clock division control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_clock](index.html) module"]
pub struct SPI_MEM_CLOCK_SPEC;
impl crate::RegisterSpec for SPI_MEM_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_clock::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_clock::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_CLOCK to value 0x0003_0103"]
impl crate::Resettable for SPI_MEM_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0103
    }
}
