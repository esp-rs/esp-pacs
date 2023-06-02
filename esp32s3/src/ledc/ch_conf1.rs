#[doc = "Register `CH%s_CONF1` reader"]
pub struct R(crate::R<CH_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_CONF1` writer"]
pub struct W(crate::W<CH_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CONF1_SPEC>;
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
impl From<crate::W<CH_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE` reader - This register is used to configure the changing step scale of duty on channel %s."]
pub type DUTY_SCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_SCALE` writer - This register is used to configure the changing step scale of duty on channel %s."]
pub type DUTY_SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, CH_CONF1_SPEC, 10, O, u16, u16>;
#[doc = "Field `DUTY_CYCLE` reader - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub type DUTY_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_CYCLE` writer - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub type DUTY_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, CH_CONF1_SPEC, 10, O, u16, u16>;
#[doc = "Field `DUTY_NUM` reader - This register is used to control the number of times the duty cycle will be changed."]
pub type DUTY_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_NUM` writer - This register is used to control the number of times the duty cycle will be changed."]
pub type DUTY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CH_CONF1_SPEC, 10, O, u16, u16>;
#[doc = "Field `DUTY_INC` reader - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
pub type DUTY_INC_R = crate::BitReader;
#[doc = "Field `DUTY_INC` writer - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
pub type DUTY_INC_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF1_SPEC, O>;
#[doc = "Field `DUTY_START` reader - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_R = crate::BitReader;
#[doc = "Field `DUTY_START` writer - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_W<'a, const O: u8> = crate::BitWriter<'a, CH_CONF1_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale(&self) -> DUTY_SCALE_R {
        DUTY_SCALE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle(&self) -> DUTY_CYCLE_R {
        DUTY_CYCLE_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num(&self) -> DUTY_NUM_R {
        DUTY_NUM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc(&self) -> DUTY_INC_R {
        DUTY_INC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start(&self) -> DUTY_START_R {
        DUTY_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_CONF1")
            .field("duty_scale", &format_args!("{}", self.duty_scale().bits()))
            .field("duty_cycle", &format_args!("{}", self.duty_cycle().bits()))
            .field("duty_num", &format_args!("{}", self.duty_num().bits()))
            .field("duty_inc", &format_args!("{}", self.duty_inc().bit()))
            .field("duty_start", &format_args!("{}", self.duty_start().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn duty_scale(&mut self) -> DUTY_SCALE_W<0> {
        DUTY_SCALE_W::new(self)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn duty_cycle(&mut self) -> DUTY_CYCLE_W<10> {
        DUTY_CYCLE_W::new(self)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    #[must_use]
    pub fn duty_num(&mut self) -> DUTY_NUM_W<20> {
        DUTY_NUM_W::new(self)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
    #[inline(always)]
    #[must_use]
    pub fn duty_inc(&mut self) -> DUTY_INC_W<30> {
        DUTY_INC_W::new(self)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn duty_start(&mut self) -> DUTY_START_W<31> {
        DUTY_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for channel %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_conf1](index.html) module"]
pub struct CH_CONF1_SPEC;
impl crate::RegisterSpec for CH_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_conf1::R](R) reader structure"]
impl crate::Readable for CH_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_conf1::W](W) writer structure"]
impl crate::Writable for CH_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_CONF1 to value 0x4000_0000"]
impl crate::Resettable for CH_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
