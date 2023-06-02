#[doc = "Register `DS_CONF` reader"]
pub struct R(crate::R<DS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DS_CONF` writer"]
pub struct W(crate::W<DS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DS_CONF_SPEC>;
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
impl From<crate::W<DS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS_CLK_EN` reader - Set 1 to enable ds clock"]
pub type DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `DS_CLK_EN` writer - Set 1 to enable ds clock"]
pub type DS_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, DS_CONF_SPEC, O>;
#[doc = "Field `DS_RST_EN` reader - Set 0 to reset ds module"]
pub type DS_RST_EN_R = crate::BitReader;
#[doc = "Field `DS_RST_EN` writer - Set 0 to reset ds module"]
pub type DS_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, DS_CONF_SPEC, O>;
#[doc = "Field `DS_READY` reader - Query this field after reset ds module"]
pub type DS_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ds clock"]
    #[inline(always)]
    pub fn ds_clk_en(&self) -> DS_CLK_EN_R {
        DS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ds module"]
    #[inline(always)]
    pub fn ds_rst_en(&self) -> DS_RST_EN_R {
        DS_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset ds module"]
    #[inline(always)]
    pub fn ds_ready(&self) -> DS_READY_R {
        DS_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DS_CONF")
            .field("ds_clk_en", &format_args!("{}", self.ds_clk_en().bit()))
            .field("ds_rst_en", &format_args!("{}", self.ds_rst_en().bit()))
            .field("ds_ready", &format_args!("{}", self.ds_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ds clock"]
    #[inline(always)]
    #[must_use]
    pub fn ds_clk_en(&mut self) -> DS_CLK_EN_W<0> {
        DS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset ds module"]
    #[inline(always)]
    #[must_use]
    pub fn ds_rst_en(&mut self) -> DS_RST_EN_W<1> {
        DS_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds_conf](index.html) module"]
pub struct DS_CONF_SPEC;
impl crate::RegisterSpec for DS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ds_conf::R](R) reader structure"]
impl crate::Readable for DS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ds_conf::W](W) writer structure"]
impl crate::Writable for DS_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DS_CONF to value 0x05"]
impl crate::Resettable for DS_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
