#[doc = "Register `IO_MUX_CONF` reader"]
pub struct R(crate::R<IO_MUX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_MUX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_MUX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_MUX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_MUX_CONF` writer"]
pub struct W(crate::W<IO_MUX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_MUX_CONF_SPEC>;
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
impl From<crate::W<IO_MUX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_MUX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_CLK_EQU_SYS_CLK` reader - "]
pub type SPI0_CLK_EQU_SYS_CLK_R = crate::BitReader<bool>;
#[doc = "Field `SPI0_CLK_EQU_SYS_CLK` writer - "]
pub type SPI0_CLK_EQU_SYS_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IO_MUX_CONF_SPEC, bool, O>;
#[doc = "Field `SPI1_CLK_EQU_SYS_CLK` reader - "]
pub type SPI1_CLK_EQU_SYS_CLK_R = crate::BitReader<bool>;
#[doc = "Field `SPI1_CLK_EQU_SYS_CLK` writer - "]
pub type SPI1_CLK_EQU_SYS_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IO_MUX_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi0_clk_equ_sys_clk(&self) -> SPI0_CLK_EQU_SYS_CLK_R {
        SPI0_CLK_EQU_SYS_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi1_clk_equ_sys_clk(&self) -> SPI1_CLK_EQU_SYS_CLK_R {
        SPI1_CLK_EQU_SYS_CLK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi0_clk_equ_sys_clk(&mut self) -> SPI0_CLK_EQU_SYS_CLK_W<8> {
        SPI0_CLK_EQU_SYS_CLK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi1_clk_equ_sys_clk(&mut self) -> SPI1_CLK_EQU_SYS_CLK_W<9> {
        SPI1_CLK_EQU_SYS_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO_MUX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_mux_conf](index.html) module"]
pub struct IO_MUX_CONF_SPEC;
impl crate::RegisterSpec for IO_MUX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_mux_conf::R](R) reader structure"]
impl crate::Readable for IO_MUX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_mux_conf::W](W) writer structure"]
impl crate::Writable for IO_MUX_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IO_MUX_CONF to value 0"]
impl crate::Resettable for IO_MUX_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
