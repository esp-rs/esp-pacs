#[doc = "Register `PAD_XPD_DCDC_CONF` reader"]
pub struct R(crate::R<PAD_XPD_DCDC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_XPD_DCDC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_XPD_DCDC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_XPD_DCDC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_XPD_DCDC_CONF` writer"]
pub struct W(crate::W<PAD_XPD_DCDC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_XPD_DCDC_CONF_SPEC>;
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
impl From<crate::W<PAD_XPD_DCDC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_XPD_DCDC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_XPD_DCDC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD_XPD_DCDC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_xpd_dcdc_conf](index.html) module"]
pub struct PAD_XPD_DCDC_CONF_SPEC;
impl crate::RegisterSpec for PAD_XPD_DCDC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_xpd_dcdc_conf::R](R) reader structure"]
impl crate::Readable for PAD_XPD_DCDC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_xpd_dcdc_conf::W](W) writer structure"]
impl crate::Writable for PAD_XPD_DCDC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_XPD_DCDC_CONF to value 0"]
impl crate::Resettable for PAD_XPD_DCDC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
