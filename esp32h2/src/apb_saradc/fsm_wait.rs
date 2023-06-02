#[doc = "Register `FSM_WAIT` reader"]
pub struct R(crate::R<FSM_WAIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_WAIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_WAIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_WAIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_WAIT` writer"]
pub struct W(crate::W<FSM_WAIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_WAIT_SPEC>;
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
impl From<crate::W<FSM_WAIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_WAIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_XPD_WAIT` reader - saradc_xpd_wait"]
pub type SARADC_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_XPD_WAIT` writer - saradc_xpd_wait"]
pub type SARADC_XPD_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_WAIT_SPEC, 8, O>;
#[doc = "Field `SARADC_RSTB_WAIT` reader - saradc_rstb_wait"]
pub type SARADC_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_RSTB_WAIT` writer - saradc_rstb_wait"]
pub type SARADC_RSTB_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_WAIT_SPEC, 8, O>;
#[doc = "Field `SARADC_STANDBY_WAIT` reader - saradc_standby_wait"]
pub type SARADC_STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_STANDBY_WAIT` writer - saradc_standby_wait"]
pub type SARADC_STANDBY_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_WAIT_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - saradc_xpd_wait"]
    #[inline(always)]
    pub fn saradc_xpd_wait(&self) -> SARADC_XPD_WAIT_R {
        SARADC_XPD_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - saradc_rstb_wait"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&self) -> SARADC_RSTB_WAIT_R {
        SARADC_RSTB_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - saradc_standby_wait"]
    #[inline(always)]
    pub fn saradc_standby_wait(&self) -> SARADC_STANDBY_WAIT_R {
        SARADC_STANDBY_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_WAIT")
            .field(
                "saradc_xpd_wait",
                &format_args!("{}", self.saradc_xpd_wait().bits()),
            )
            .field(
                "saradc_rstb_wait",
                &format_args!("{}", self.saradc_rstb_wait().bits()),
            )
            .field(
                "saradc_standby_wait",
                &format_args!("{}", self.saradc_standby_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FSM_WAIT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - saradc_xpd_wait"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_xpd_wait(&mut self) -> SARADC_XPD_WAIT_W<0> {
        SARADC_XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 8:15 - saradc_rstb_wait"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_rstb_wait(&mut self) -> SARADC_RSTB_WAIT_W<8> {
        SARADC_RSTB_WAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - saradc_standby_wait"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_standby_wait(&mut self) -> SARADC_STANDBY_WAIT_W<16> {
        SARADC_STANDBY_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_wait](index.html) module"]
pub struct FSM_WAIT_SPEC;
impl crate::RegisterSpec for FSM_WAIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_wait::R](R) reader structure"]
impl crate::Readable for FSM_WAIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_wait::W](W) writer structure"]
impl crate::Writable for FSM_WAIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_WAIT to value 0x00ff_0808"]
impl crate::Resettable for FSM_WAIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0808;
}
