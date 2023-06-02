#[doc = "Register `CH%s_GAMMA_WR` reader"]
pub struct R(crate::R<CH_GAMMA_WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_GAMMA_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_GAMMA_WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_GAMMA_WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_GAMMA_WR` writer"]
pub struct W(crate::W<CH_GAMMA_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_GAMMA_WR_SPEC>;
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
impl From<crate::W<CH_GAMMA_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_GAMMA_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_GAMMA_DUTY_INC` reader - Ledc ch%s gamma duty inc of current ram write address.This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase 0: Decrease."]
pub type CH_GAMMA_DUTY_INC_R = crate::BitReader;
#[doc = "Field `CH_GAMMA_DUTY_INC` writer - Ledc ch%s gamma duty inc of current ram write address.This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase 0: Decrease."]
pub type CH_GAMMA_DUTY_INC_W<'a, const O: u8> = crate::BitWriter<'a, CH_GAMMA_WR_SPEC, O>;
#[doc = "Field `CH_GAMMA_DUTY_CYCLE` reader - Ledc ch%s gamma duty cycle of current ram write address.The duty will change every LEDC_CH%s_GAMMA_DUTY_CYCLE on channel %s."]
pub type CH_GAMMA_DUTY_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH_GAMMA_DUTY_CYCLE` writer - Ledc ch%s gamma duty cycle of current ram write address.The duty will change every LEDC_CH%s_GAMMA_DUTY_CYCLE on channel %s."]
pub type CH_GAMMA_DUTY_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CH_GAMMA_WR_SPEC, 10, O, u16, u16>;
#[doc = "Field `CH_GAMMA_SCALE` reader - Ledc ch%s gamma scale of current ram write address.This register is used to configure the changing step scale of duty on channel %s."]
pub type CH_GAMMA_SCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH_GAMMA_SCALE` writer - Ledc ch%s gamma scale of current ram write address.This register is used to configure the changing step scale of duty on channel %s."]
pub type CH_GAMMA_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CH_GAMMA_WR_SPEC, 10, O, u16, u16>;
#[doc = "Field `CH_GAMMA_DUTY_NUM` reader - Ledc ch%s gamma duty num of current ram write address.This register is used to control the number of times the duty cycle will be changed."]
pub type CH_GAMMA_DUTY_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH_GAMMA_DUTY_NUM` writer - Ledc ch%s gamma duty num of current ram write address.This register is used to control the number of times the duty cycle will be changed."]
pub type CH_GAMMA_DUTY_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, CH_GAMMA_WR_SPEC, 10, O, u16, u16>;
impl R {
    #[doc = "Bit 0 - Ledc ch%s gamma duty inc of current ram write address.This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase 0: Decrease."]
    #[inline(always)]
    pub fn ch_gamma_duty_inc(&self) -> CH_GAMMA_DUTY_INC_R {
        CH_GAMMA_DUTY_INC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Ledc ch%s gamma duty cycle of current ram write address.The duty will change every LEDC_CH%s_GAMMA_DUTY_CYCLE on channel %s."]
    #[inline(always)]
    pub fn ch_gamma_duty_cycle(&self) -> CH_GAMMA_DUTY_CYCLE_R {
        CH_GAMMA_DUTY_CYCLE_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - Ledc ch%s gamma scale of current ram write address.This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn ch_gamma_scale(&self) -> CH_GAMMA_SCALE_R {
        CH_GAMMA_SCALE_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - Ledc ch%s gamma duty num of current ram write address.This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn ch_gamma_duty_num(&self) -> CH_GAMMA_DUTY_NUM_R {
        CH_GAMMA_DUTY_NUM_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_GAMMA_WR")
            .field(
                "ch_gamma_duty_inc",
                &format_args!("{}", self.ch_gamma_duty_inc().bit()),
            )
            .field(
                "ch_gamma_duty_cycle",
                &format_args!("{}", self.ch_gamma_duty_cycle().bits()),
            )
            .field(
                "ch_gamma_scale",
                &format_args!("{}", self.ch_gamma_scale().bits()),
            )
            .field(
                "ch_gamma_duty_num",
                &format_args!("{}", self.ch_gamma_duty_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_GAMMA_WR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Ledc ch%s gamma duty inc of current ram write address.This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase 0: Decrease."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_duty_inc(&mut self) -> CH_GAMMA_DUTY_INC_W<0> {
        CH_GAMMA_DUTY_INC_W::new(self)
    }
    #[doc = "Bits 1:10 - Ledc ch%s gamma duty cycle of current ram write address.The duty will change every LEDC_CH%s_GAMMA_DUTY_CYCLE on channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_duty_cycle(&mut self) -> CH_GAMMA_DUTY_CYCLE_W<1> {
        CH_GAMMA_DUTY_CYCLE_W::new(self)
    }
    #[doc = "Bits 11:20 - Ledc ch%s gamma scale of current ram write address.This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_scale(&mut self) -> CH_GAMMA_SCALE_W<11> {
        CH_GAMMA_SCALE_W::new(self)
    }
    #[doc = "Bits 21:30 - Ledc ch%s gamma duty num of current ram write address.This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_duty_num(&mut self) -> CH_GAMMA_DUTY_NUM_W<21> {
        CH_GAMMA_DUTY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc ch%s gamma ram write register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_gamma_wr](index.html) module"]
pub struct CH_GAMMA_WR_SPEC;
impl crate::RegisterSpec for CH_GAMMA_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_gamma_wr::R](R) reader structure"]
impl crate::Readable for CH_GAMMA_WR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_gamma_wr::W](W) writer structure"]
impl crate::Writable for CH_GAMMA_WR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_GAMMA_WR to value 0"]
impl crate::Resettable for CH_GAMMA_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
