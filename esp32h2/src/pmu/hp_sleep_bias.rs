#[doc = "Register `HP_SLEEP_BIAS` reader"]
pub struct R(crate::R<HP_SLEEP_BIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_SLEEP_BIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_SLEEP_BIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_SLEEP_BIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_SLEEP_BIAS` writer"]
pub struct W(crate::W<HP_SLEEP_BIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_SLEEP_BIAS_SPEC>;
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
impl From<crate::W<HP_SLEEP_BIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_SLEEP_BIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_SLEEP_XPD_TRX` reader - need_des"]
pub type HP_SLEEP_XPD_TRX_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_TRX` writer - need_des"]
pub type HP_SLEEP_XPD_TRX_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_BIAS_SPEC, O>;
#[doc = "Field `HP_SLEEP_XPD_BIAS` reader - need_des"]
pub type HP_SLEEP_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_BIAS` writer - need_des"]
pub type HP_SLEEP_XPD_BIAS_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_BIAS_SPEC, O>;
#[doc = "Field `HP_SLEEP_PD_CUR` reader - need_des"]
pub type HP_SLEEP_PD_CUR_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_CUR` writer - need_des"]
pub type HP_SLEEP_PD_CUR_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_BIAS_SPEC, O>;
#[doc = "Field `SLEEP` reader - need_des"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_BIAS_SPEC, O>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_trx(&self) -> HP_SLEEP_XPD_TRX_R {
        HP_SLEEP_XPD_TRX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&self) -> HP_SLEEP_XPD_BIAS_R {
        HP_SLEEP_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_cur(&self) -> HP_SLEEP_PD_CUR_R {
        HP_SLEEP_PD_CUR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_BIAS")
            .field(
                "hp_sleep_xpd_trx",
                &format_args!("{}", self.hp_sleep_xpd_trx().bit()),
            )
            .field(
                "hp_sleep_xpd_bias",
                &format_args!("{}", self.hp_sleep_xpd_bias().bit()),
            )
            .field(
                "hp_sleep_pd_cur",
                &format_args!("{}", self.hp_sleep_pd_cur().bit()),
            )
            .field("sleep", &format_args!("{}", self.sleep().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_BIAS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_trx(&mut self) -> HP_SLEEP_XPD_TRX_W<24> {
        HP_SLEEP_XPD_TRX_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_bias(&mut self) -> HP_SLEEP_XPD_BIAS_W<25> {
        HP_SLEEP_XPD_BIAS_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_cur(&mut self) -> HP_SLEEP_PD_CUR_W<30> {
        HP_SLEEP_PD_CUR_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<31> {
        SLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_sleep_bias](index.html) module"]
pub struct HP_SLEEP_BIAS_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_sleep_bias::R](R) reader structure"]
impl crate::Readable for HP_SLEEP_BIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_sleep_bias::W](W) writer structure"]
impl crate::Writable for HP_SLEEP_BIAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_BIAS to value 0x0100_0000"]
impl crate::Resettable for HP_SLEEP_BIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
