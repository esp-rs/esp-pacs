#[doc = "Register `RTC_EXT_XTL_CONF` reader"]
pub struct R(crate::R<RTC_EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_EXT_XTL_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_EXT_XTL_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_EXT_XTL_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_EXT_XTL_CONF` writer"]
pub struct W(crate::W<RTC_EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_EXT_XTL_CONF_SPEC>;
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
impl From<crate::W<RTC_EXT_XTL_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_EXT_XTL_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTL_EXT_CTR_LV` reader - 0: power down XTAL at high level"]
pub type XTL_EXT_CTR_LV_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EXT_CTR_LV` writer - 0: power down XTAL at high level"]
pub type XTL_EXT_CTR_LV_W<'a> = crate::BitWriter<'a, u32, RTC_EXT_XTL_CONF_SPEC, bool, 30>;
#[doc = "Field `XTL_EXT_CTR_EN` reader - Need add desc"]
pub type XTL_EXT_CTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EXT_CTR_EN` writer - Need add desc"]
pub type XTL_EXT_CTR_EN_W<'a> = crate::BitWriter<'a, u32, RTC_EXT_XTL_CONF_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 30 - 0: power down XTAL at high level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - 0: power down XTAL at high level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W {
        XTL_EXT_CTR_LV_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W {
        XTL_EXT_CTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ext_xtl_conf](index.html) module"]
pub struct RTC_EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for RTC_EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ext_xtl_conf::R](R) reader structure"]
impl crate::Readable for RTC_EXT_XTL_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ext_xtl_conf::W](W) writer structure"]
impl crate::Writable for RTC_EXT_XTL_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_EXT_XTL_CONF to value 0"]
impl crate::Resettable for RTC_EXT_XTL_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
