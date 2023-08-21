#[doc = "Register `PAD_XPD_DCDC_CONF` reader"]
pub type R = crate::R<PAD_XPD_DCDC_CONF_SPEC>;
#[doc = "Register `PAD_XPD_DCDC_CONF` writer"]
pub type W = crate::W<PAD_XPD_DCDC_CONF_SPEC>;
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
        self.bits = bits;
        self
    }
}
#[doc = "PAD_XPD_DCDC_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_xpd_dcdc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_xpd_dcdc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_XPD_DCDC_CONF_SPEC;
impl crate::RegisterSpec for PAD_XPD_DCDC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_xpd_dcdc_conf::R`](R) reader structure"]
impl crate::Readable for PAD_XPD_DCDC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_xpd_dcdc_conf::W`](W) writer structure"]
impl crate::Writable for PAD_XPD_DCDC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_XPD_DCDC_CONF to value 0"]
impl crate::Resettable for PAD_XPD_DCDC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
