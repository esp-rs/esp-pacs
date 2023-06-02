#[doc = "Register `MODEM_32K_CLK_CONF` reader"]
pub struct R(crate::R<MODEM_32K_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_32K_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_32K_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_32K_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEM_32K_CLK_CONF` writer"]
pub struct W(crate::W<MODEM_32K_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEM_32K_CLK_CONF_SPEC>;
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
impl From<crate::W<MODEM_32K_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEM_32K_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_MODEM_32K_SEL` reader - "]
pub type CLK_MODEM_32K_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_32K_SEL` writer - "]
pub type CLK_MODEM_32K_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, MODEM_32K_CLK_CONF_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_modem_32k_sel(&self) -> CLK_MODEM_32K_SEL_R {
        CLK_MODEM_32K_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_32K_CLK_CONF")
            .field(
                "clk_modem_32k_sel",
                &format_args!("{}", self.clk_modem_32k_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_32K_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_32k_sel(&mut self) -> CLK_MODEM_32K_SEL_W<0> {
        CLK_MODEM_32K_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_32k_clk_conf](index.html) module"]
pub struct MODEM_32K_CLK_CONF_SPEC;
impl crate::RegisterSpec for MODEM_32K_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modem_32k_clk_conf::R](R) reader structure"]
impl crate::Readable for MODEM_32K_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modem_32k_clk_conf::W](W) writer structure"]
impl crate::Writable for MODEM_32K_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_32K_CLK_CONF to value 0"]
impl crate::Resettable for MODEM_32K_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
