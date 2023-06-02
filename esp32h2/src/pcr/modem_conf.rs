#[doc = "Register `MODEM_CONF` reader"]
pub struct R(crate::R<MODEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEM_CONF` writer"]
pub struct W(crate::W<MODEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEM_CONF_SPEC>;
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
impl From<crate::W<MODEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEM_CLK_SEL` reader - xxxx"]
pub type MODEM_CLK_SEL_R = crate::BitReader;
#[doc = "Field `MODEM_CLK_SEL` writer - xxxx"]
pub type MODEM_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_CONF_SPEC, O>;
#[doc = "Field `MODEM_CLK_EN` reader - xxxx"]
pub type MODEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MODEM_CLK_EN` writer - xxxx"]
pub type MODEM_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_CONF_SPEC, O>;
#[doc = "Field `MODEM_RST_EN` reader - Set this file as 1 to reset modem-subsystem."]
pub type MODEM_RST_EN_R = crate::BitReader;
#[doc = "Field `MODEM_RST_EN` writer - Set this file as 1 to reset modem-subsystem."]
pub type MODEM_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn modem_clk_sel(&self) -> MODEM_CLK_SEL_R {
        MODEM_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xxxx"]
    #[inline(always)]
    pub fn modem_clk_en(&self) -> MODEM_CLK_EN_R {
        MODEM_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this file as 1 to reset modem-subsystem."]
    #[inline(always)]
    pub fn modem_rst_en(&self) -> MODEM_RST_EN_R {
        MODEM_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_CONF")
            .field(
                "modem_clk_sel",
                &format_args!("{}", self.modem_clk_sel().bit()),
            )
            .field(
                "modem_clk_en",
                &format_args!("{}", self.modem_clk_en().bit()),
            )
            .field(
                "modem_rst_en",
                &format_args!("{}", self.modem_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn modem_clk_sel(&mut self) -> MODEM_CLK_SEL_W<0> {
        MODEM_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 1 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn modem_clk_en(&mut self) -> MODEM_CLK_EN_W<1> {
        MODEM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this file as 1 to reset modem-subsystem."]
    #[inline(always)]
    #[must_use]
    pub fn modem_rst_en(&mut self) -> MODEM_RST_EN_W<2> {
        MODEM_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODEM_APB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_conf](index.html) module"]
pub struct MODEM_CONF_SPEC;
impl crate::RegisterSpec for MODEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modem_conf::R](R) reader structure"]
impl crate::Readable for MODEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modem_conf::W](W) writer structure"]
impl crate::Writable for MODEM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_CONF to value 0x02"]
impl crate::Resettable for MODEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
