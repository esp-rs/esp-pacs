#[doc = "Register `TIMERGROUP1_CONF` reader"]
pub struct R(crate::R<TIMERGROUP1_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERGROUP1_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERGROUP1_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERGROUP1_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERGROUP1_CONF` writer"]
pub struct W(crate::W<TIMERGROUP1_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERGROUP1_CONF_SPEC>;
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
impl From<crate::W<TIMERGROUP1_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERGROUP1_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TG1_CLK_EN` reader - Set 1 to enable timer_group1 apb clock"]
pub type TG1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG1_CLK_EN` writer - Set 1 to enable timer_group1 apb clock"]
pub type TG1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TIMERGROUP1_CONF_SPEC, O>;
#[doc = "Field `TG1_RST_EN` reader - Set 0 to reset timer_group1 module"]
pub type TG1_RST_EN_R = crate::BitReader;
#[doc = "Field `TG1_RST_EN` writer - Set 0 to reset timer_group1 module"]
pub type TG1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, TIMERGROUP1_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable timer_group1 apb clock"]
    #[inline(always)]
    pub fn tg1_clk_en(&self) -> TG1_CLK_EN_R {
        TG1_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group1 module"]
    #[inline(always)]
    pub fn tg1_rst_en(&self) -> TG1_RST_EN_R {
        TG1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP1_CONF")
            .field("tg1_clk_en", &format_args!("{}", self.tg1_clk_en().bit()))
            .field("tg1_rst_en", &format_args!("{}", self.tg1_rst_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMERGROUP1_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable timer_group1 apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn tg1_clk_en(&mut self) -> TG1_CLK_EN_W<0> {
        TG1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group1 module"]
    #[inline(always)]
    #[must_use]
    pub fn tg1_rst_en(&mut self) -> TG1_RST_EN_W<1> {
        TG1_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMERGROUP1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timergroup1_conf](index.html) module"]
pub struct TIMERGROUP1_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timergroup1_conf::R](R) reader structure"]
impl crate::Readable for TIMERGROUP1_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timergroup1_conf::W](W) writer structure"]
impl crate::Writable for TIMERGROUP1_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERGROUP1_CONF to value 0x01"]
impl crate::Resettable for TIMERGROUP1_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
