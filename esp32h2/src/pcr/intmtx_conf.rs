#[doc = "Register `INTMTX_CONF` reader"]
pub struct R(crate::R<INTMTX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMTX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMTX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMTX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTMTX_CONF` writer"]
pub struct W(crate::W<INTMTX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMTX_CONF_SPEC>;
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
impl From<crate::W<INTMTX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMTX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTMTX_CLK_EN` reader - Set 1 to enable intmtx clock"]
pub type INTMTX_CLK_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_CLK_EN` writer - Set 1 to enable intmtx clock"]
pub type INTMTX_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, INTMTX_CONF_SPEC, O>;
#[doc = "Field `INTMTX_RST_EN` reader - Set 0 to reset intmtx module"]
pub type INTMTX_RST_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_RST_EN` writer - Set 0 to reset intmtx module"]
pub type INTMTX_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, INTMTX_CONF_SPEC, O>;
#[doc = "Field `INTMTX_READY` reader - Query this field after reset intmtx module"]
pub type INTMTX_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable intmtx clock"]
    #[inline(always)]
    pub fn intmtx_clk_en(&self) -> INTMTX_CLK_EN_R {
        INTMTX_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset intmtx module"]
    #[inline(always)]
    pub fn intmtx_rst_en(&self) -> INTMTX_RST_EN_R {
        INTMTX_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset intmtx module"]
    #[inline(always)]
    pub fn intmtx_ready(&self) -> INTMTX_READY_R {
        INTMTX_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMTX_CONF")
            .field(
                "intmtx_clk_en",
                &format_args!("{}", self.intmtx_clk_en().bit()),
            )
            .field(
                "intmtx_rst_en",
                &format_args!("{}", self.intmtx_rst_en().bit()),
            )
            .field(
                "intmtx_ready",
                &format_args!("{}", self.intmtx_ready().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTMTX_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable intmtx clock"]
    #[inline(always)]
    #[must_use]
    pub fn intmtx_clk_en(&mut self) -> INTMTX_CLK_EN_W<0> {
        INTMTX_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset intmtx module"]
    #[inline(always)]
    #[must_use]
    pub fn intmtx_rst_en(&mut self) -> INTMTX_RST_EN_W<1> {
        INTMTX_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTMTX configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmtx_conf](index.html) module"]
pub struct INTMTX_CONF_SPEC;
impl crate::RegisterSpec for INTMTX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intmtx_conf::R](R) reader structure"]
impl crate::Readable for INTMTX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intmtx_conf::W](W) writer structure"]
impl crate::Writable for INTMTX_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTMTX_CONF to value 0x05"]
impl crate::Resettable for INTMTX_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
