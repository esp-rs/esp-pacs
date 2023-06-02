#[doc = "Register `ASSIST_CONF` reader"]
pub struct R(crate::R<ASSIST_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASSIST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASSIST_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASSIST_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASSIST_CONF` writer"]
pub struct W(crate::W<ASSIST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASSIST_CONF_SPEC>;
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
impl From<crate::W<ASSIST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASSIST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASSIST_CLK_EN` reader - Set 1 to enable assist clock"]
pub type ASSIST_CLK_EN_R = crate::BitReader;
#[doc = "Field `ASSIST_CLK_EN` writer - Set 1 to enable assist clock"]
pub type ASSIST_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, ASSIST_CONF_SPEC, O>;
#[doc = "Field `ASSIST_RST_EN` reader - Set 0 to reset assist module"]
pub type ASSIST_RST_EN_R = crate::BitReader;
#[doc = "Field `ASSIST_RST_EN` writer - Set 0 to reset assist module"]
pub type ASSIST_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, ASSIST_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable assist clock"]
    #[inline(always)]
    pub fn assist_clk_en(&self) -> ASSIST_CLK_EN_R {
        ASSIST_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset assist module"]
    #[inline(always)]
    pub fn assist_rst_en(&self) -> ASSIST_RST_EN_R {
        ASSIST_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_CONF")
            .field(
                "assist_clk_en",
                &format_args!("{}", self.assist_clk_en().bit()),
            )
            .field(
                "assist_rst_en",
                &format_args!("{}", self.assist_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ASSIST_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable assist clock"]
    #[inline(always)]
    #[must_use]
    pub fn assist_clk_en(&mut self) -> ASSIST_CLK_EN_W<0> {
        ASSIST_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset assist module"]
    #[inline(always)]
    #[must_use]
    pub fn assist_rst_en(&mut self) -> ASSIST_RST_EN_W<1> {
        ASSIST_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_conf](index.html) module"]
pub struct ASSIST_CONF_SPEC;
impl crate::RegisterSpec for ASSIST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [assist_conf::R](R) reader structure"]
impl crate::Readable for ASSIST_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [assist_conf::W](W) writer structure"]
impl crate::Writable for ASSIST_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASSIST_CONF to value 0x01"]
impl crate::Resettable for ASSIST_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
