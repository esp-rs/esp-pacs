#[doc = "Register `LPPERI` reader"]
pub struct R(crate::R<LPPERI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPPERI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPPERI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPPERI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPPERI` writer"]
pub struct W(crate::W<LPPERI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPPERI_SPEC>;
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
impl From<crate::W<LPPERI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPPERI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_CLK_SEL` reader - need_des"]
pub type LP_I2C_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LP_I2C_CLK_SEL` writer - need_des"]
pub type LP_I2C_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, LPPERI_SPEC, O>;
#[doc = "Field `LP_UART_CLK_SEL` reader - need_des"]
pub type LP_UART_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LP_UART_CLK_SEL` writer - need_des"]
pub type LP_UART_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, LPPERI_SPEC, O>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&self) -> LP_I2C_CLK_SEL_R {
        LP_I2C_CLK_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_uart_clk_sel(&self) -> LP_UART_CLK_SEL_R {
        LP_UART_CLK_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPPERI")
            .field(
                "lp_i2c_clk_sel",
                &format_args!("{}", self.lp_i2c_clk_sel().bit()),
            )
            .field(
                "lp_uart_clk_sel",
                &format_args!("{}", self.lp_uart_clk_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPPERI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_clk_sel(&mut self) -> LP_I2C_CLK_SEL_W<30> {
        LP_I2C_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_clk_sel(&mut self) -> LP_UART_CLK_SEL_W<31> {
        LP_UART_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpperi](index.html) module"]
pub struct LPPERI_SPEC;
impl crate::RegisterSpec for LPPERI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpperi::R](R) reader structure"]
impl crate::Readable for LPPERI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpperi::W](W) writer structure"]
impl crate::Writable for LPPERI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPPERI to value 0"]
impl crate::Resettable for LPPERI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
