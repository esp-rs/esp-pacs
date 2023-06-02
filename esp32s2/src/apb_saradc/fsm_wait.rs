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
#[doc = "Field `XPD_WAIT` reader - xpd wait"]
pub type XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `XPD_WAIT` writer - xpd wait"]
pub type XPD_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_WAIT_SPEC, 8, O>;
#[doc = "Field `RSTB_WAIT` reader - reset time"]
pub type RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `RSTB_WAIT` writer - reset time"]
pub type RSTB_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_WAIT_SPEC, 8, O>;
#[doc = "Field `STANDBY_WAIT` reader - standby wait"]
pub type STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `STANDBY_WAIT` writer - standby wait"]
pub type STANDBY_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_WAIT_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - xpd wait"]
    #[inline(always)]
    pub fn xpd_wait(&self) -> XPD_WAIT_R {
        XPD_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reset time"]
    #[inline(always)]
    pub fn rstb_wait(&self) -> RSTB_WAIT_R {
        RSTB_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - standby wait"]
    #[inline(always)]
    pub fn standby_wait(&self) -> STANDBY_WAIT_R {
        STANDBY_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_WAIT")
            .field("xpd_wait", &format_args!("{}", self.xpd_wait().bits()))
            .field("rstb_wait", &format_args!("{}", self.rstb_wait().bits()))
            .field(
                "standby_wait",
                &format_args!("{}", self.standby_wait().bits()),
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
    #[doc = "Bits 0:7 - xpd wait"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_wait(&mut self) -> XPD_WAIT_W<0> {
        XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 8:15 - reset time"]
    #[inline(always)]
    #[must_use]
    pub fn rstb_wait(&mut self) -> RSTB_WAIT_W<8> {
        RSTB_WAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - standby wait"]
    #[inline(always)]
    #[must_use]
    pub fn standby_wait(&mut self) -> STANDBY_WAIT_W<16> {
        STANDBY_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc fsm internal parameter base on test\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_wait](index.html) module"]
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
