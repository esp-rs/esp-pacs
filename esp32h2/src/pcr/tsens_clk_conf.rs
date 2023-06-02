#[doc = "Register `TSENS_CLK_CONF` reader"]
pub struct R(crate::R<TSENS_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSENS_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSENS_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSENS_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSENS_CLK_CONF` writer"]
pub struct W(crate::W<TSENS_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSENS_CLK_CONF_SPEC>;
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
impl From<crate::W<TSENS_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSENS_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_CLK_SEL` reader - set this field to select clock-source. 0(default): FOSC, 1: XTAL."]
pub type TSENS_CLK_SEL_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_SEL` writer - set this field to select clock-source. 0(default): FOSC, 1: XTAL."]
pub type TSENS_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TSENS_CLK_CONF_SPEC, O>;
#[doc = "Field `TSENS_CLK_EN` reader - Set 1 to enable tsens clock"]
pub type TSENS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_EN` writer - Set 1 to enable tsens clock"]
pub type TSENS_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TSENS_CLK_CONF_SPEC, O>;
#[doc = "Field `TSENS_RST_EN` reader - Set 0 to reset tsens module"]
pub type TSENS_RST_EN_R = crate::BitReader;
#[doc = "Field `TSENS_RST_EN` writer - Set 0 to reset tsens module"]
pub type TSENS_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, TSENS_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): FOSC, 1: XTAL."]
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TSENS_CLK_SEL_R {
        TSENS_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Set 1 to enable tsens clock"]
    #[inline(always)]
    pub fn tsens_clk_en(&self) -> TSENS_CLK_EN_R {
        TSENS_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set 0 to reset tsens module"]
    #[inline(always)]
    pub fn tsens_rst_en(&self) -> TSENS_RST_EN_R {
        TSENS_RST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSENS_CLK_CONF")
            .field(
                "tsens_clk_sel",
                &format_args!("{}", self.tsens_clk_sel().bit()),
            )
            .field(
                "tsens_clk_en",
                &format_args!("{}", self.tsens_clk_en().bit()),
            )
            .field(
                "tsens_rst_en",
                &format_args!("{}", self.tsens_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TSENS_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): FOSC, 1: XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W<20> {
        TSENS_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable tsens clock"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_en(&mut self) -> TSENS_CLK_EN_W<22> {
        TSENS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - Set 0 to reset tsens module"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_rst_en(&mut self) -> TSENS_RST_EN_W<23> {
        TSENS_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSENS_CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsens_clk_conf](index.html) module"]
pub struct TSENS_CLK_CONF_SPEC;
impl crate::RegisterSpec for TSENS_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsens_clk_conf::R](R) reader structure"]
impl crate::Readable for TSENS_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsens_clk_conf::W](W) writer structure"]
impl crate::Writable for TSENS_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSENS_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TSENS_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
