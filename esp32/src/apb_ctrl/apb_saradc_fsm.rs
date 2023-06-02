#[doc = "Register `APB_SARADC_FSM` reader"]
pub struct R(crate::R<APB_SARADC_FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC_FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC_FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_SARADC_FSM` writer"]
pub struct W(crate::W<APB_SARADC_FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_SARADC_FSM_SPEC>;
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
impl From<crate::W<APB_SARADC_FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_SARADC_FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_RSTB_WAIT` reader - "]
pub type SARADC_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_RSTB_WAIT` writer - "]
pub type SARADC_RSTB_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, APB_SARADC_FSM_SPEC, 8, O>;
#[doc = "Field `SARADC_STANDBY_WAIT` reader - "]
pub type SARADC_STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_STANDBY_WAIT` writer - "]
pub type SARADC_STANDBY_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, APB_SARADC_FSM_SPEC, 8, O>;
#[doc = "Field `SARADC_START_WAIT` reader - "]
pub type SARADC_START_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_START_WAIT` writer - "]
pub type SARADC_START_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, APB_SARADC_FSM_SPEC, 8, O>;
#[doc = "Field `SARADC_SAMPLE_CYCLE` reader - sample cycles"]
pub type SARADC_SAMPLE_CYCLE_R = crate::FieldReader;
#[doc = "Field `SARADC_SAMPLE_CYCLE` writer - sample cycles"]
pub type SARADC_SAMPLE_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, APB_SARADC_FSM_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&self) -> SARADC_RSTB_WAIT_R {
        SARADC_RSTB_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn saradc_standby_wait(&self) -> SARADC_STANDBY_WAIT_R {
        SARADC_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn saradc_start_wait(&self) -> SARADC_START_WAIT_R {
        SARADC_START_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn saradc_sample_cycle(&self) -> SARADC_SAMPLE_CYCLE_R {
        SARADC_SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_FSM")
            .field(
                "saradc_rstb_wait",
                &format_args!("{}", self.saradc_rstb_wait().bits()),
            )
            .field(
                "saradc_standby_wait",
                &format_args!("{}", self.saradc_standby_wait().bits()),
            )
            .field(
                "saradc_start_wait",
                &format_args!("{}", self.saradc_start_wait().bits()),
            )
            .field(
                "saradc_sample_cycle",
                &format_args!("{}", self.saradc_sample_cycle().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_SARADC_FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_rstb_wait(&mut self) -> SARADC_RSTB_WAIT_W<0> {
        SARADC_RSTB_WAIT_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_standby_wait(&mut self) -> SARADC_STANDBY_WAIT_W<8> {
        SARADC_STANDBY_WAIT_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start_wait(&mut self) -> SARADC_START_WAIT_W<16> {
        SARADC_START_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sample_cycle(&mut self) -> SARADC_SAMPLE_CYCLE_W<24> {
        SARADC_SAMPLE_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_fsm](index.html) module"]
pub struct APB_SARADC_FSM_SPEC;
impl crate::RegisterSpec for APB_SARADC_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc_fsm::R](R) reader structure"]
impl crate::Readable for APB_SARADC_FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_saradc_fsm::W](W) writer structure"]
impl crate::Writable for APB_SARADC_FSM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_SARADC_FSM to value 0x0208_ff08"]
impl crate::Resettable for APB_SARADC_FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0208_ff08;
}
