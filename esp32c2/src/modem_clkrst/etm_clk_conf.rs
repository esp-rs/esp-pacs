#[doc = "Register `ETM_CLK_CONF` reader"]
pub struct R(crate::R<ETM_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETM_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETM_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETM_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETM_CLK_CONF` writer"]
pub struct W(crate::W<ETM_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETM_CLK_CONF_SPEC>;
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
impl From<crate::W<ETM_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETM_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_CLK_SEL` reader - ."]
pub type ETM_CLK_SEL_R = crate::BitReader;
#[doc = "Field `ETM_CLK_SEL` writer - ."]
pub type ETM_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, ETM_CLK_CONF_SPEC, O>;
#[doc = "Field `ETM_CLK_ACTIVE` reader - ."]
pub type ETM_CLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `ETM_CLK_ACTIVE` writer - ."]
pub type ETM_CLK_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, ETM_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn etm_clk_sel(&self) -> ETM_CLK_SEL_R {
        ETM_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn etm_clk_active(&self) -> ETM_CLK_ACTIVE_R {
        ETM_CLK_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CLK_CONF")
            .field("etm_clk_sel", &format_args!("{}", self.etm_clk_sel().bit()))
            .field(
                "etm_clk_active",
                &format_args!("{}", self.etm_clk_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    #[must_use]
    pub fn etm_clk_sel(&mut self) -> ETM_CLK_SEL_W<0> {
        ETM_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    #[must_use]
    pub fn etm_clk_active(&mut self) -> ETM_CLK_ACTIVE_W<1> {
        ETM_CLK_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm_clk_conf](index.html) module"]
pub struct ETM_CLK_CONF_SPEC;
impl crate::RegisterSpec for ETM_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etm_clk_conf::R](R) reader structure"]
impl crate::Readable for ETM_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etm_clk_conf::W](W) writer structure"]
impl crate::Writable for ETM_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_CLK_CONF to value 0"]
impl crate::Resettable for ETM_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
