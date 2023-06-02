#[doc = "Register `I2C_MST_CLK_CONF` reader"]
pub struct R(crate::R<I2C_MST_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_MST_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_MST_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_MST_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_MST_CLK_CONF` writer"]
pub struct W(crate::W<I2C_MST_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_MST_CLK_CONF_SPEC>;
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
impl From<crate::W<I2C_MST_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_MST_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_I2C_MST_SEL_160M` reader - "]
pub type CLK_I2C_MST_SEL_160M_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_SEL_160M` writer - "]
pub type CLK_I2C_MST_SEL_160M_W<'a, const O: u8> = crate::BitWriter<'a, I2C_MST_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_i2c_mst_sel_160m(&self) -> CLK_I2C_MST_SEL_160M_R {
        CLK_I2C_MST_SEL_160M_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_MST_CLK_CONF")
            .field(
                "clk_i2c_mst_sel_160m",
                &format_args!("{}", self.clk_i2c_mst_sel_160m().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_MST_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_sel_160m(&mut self) -> CLK_I2C_MST_SEL_160M_W<0> {
        CLK_I2C_MST_SEL_160M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_mst_clk_conf](index.html) module"]
pub struct I2C_MST_CLK_CONF_SPEC;
impl crate::RegisterSpec for I2C_MST_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_mst_clk_conf::R](R) reader structure"]
impl crate::Readable for I2C_MST_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_mst_clk_conf::W](W) writer structure"]
impl crate::Writable for I2C_MST_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_MST_CLK_CONF to value 0"]
impl crate::Resettable for I2C_MST_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
