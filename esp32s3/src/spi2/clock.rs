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
#[doc = "Field `CLKCNT_L` reader - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
pub type CLKCNT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCNT_L` writer - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
pub type CLKCNT_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLKCNT_H` reader - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
pub type CLKCNT_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCNT_H` writer - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
pub type CLKCNT_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLKCNT_N` reader - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
pub type CLKCNT_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCNT_N` writer - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
pub type CLKCNT_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLKDIV_PRE` reader - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
pub type CLKDIV_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV_PRE` writer - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
pub type CLKDIV_PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u8, u8, 4, O>;
#[doc = "Field `CLK_EQU_SYSCLK` reader - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
pub type CLK_EQU_SYSCLK_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EQU_SYSCLK` writer - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
pub type CLK_EQU_SYSCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21 - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkdiv_pre(&self) -> CLKDIV_PRE_R {
        CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W<0> {
        CLKCNT_L_W::new(self)
    }
    #[doc = "Bits 6:11 - In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W<6> {
        CLKCNT_H_W::new(self)
    }
    #[doc = "Bits 12:17 - In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W<12> {
        CLKCNT_N_W::new(self)
    }
    #[doc = "Bits 18:21 - In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkdiv_pre(&mut self) -> CLKDIV_PRE_W<18> {
        CLKDIV_PRE_W::new(self)
    }
    #[doc = "Bit 31 - In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
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
#[doc = "SPI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](index.html) module"]
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
#[doc = "`reset()` method sets CLOCK to value 0x8000_3043"]
impl crate::Resettable for CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_3043
    }
}