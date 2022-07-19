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
#[doc = "Field `DUTY_SCALE_CH0` reader - This register is used to configure the changing step scale of duty on channel %s."]
pub type DUTY_SCALE_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_SCALE_CH0` writer - This register is used to configure the changing step scale of duty on channel %s."]
pub type DUTY_SCALE_CH0_W<'a> = crate::FieldWriter<'a, u32, CH_CONF1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DUTY_CYCLE_CH0` reader - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub type DUTY_CYCLE_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_CYCLE_CH0` writer - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub type DUTY_CYCLE_CH0_W<'a> = crate::FieldWriter<'a, u32, CH_CONF1_SPEC, u16, u16, 10, 10>;
#[doc = "Field `DUTY_NUM_CH0` reader - This register is used to control the number of times the duty cycle will be changed."]
pub type DUTY_NUM_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_NUM_CH0` writer - This register is used to control the number of times the duty cycle will be changed."]
pub type DUTY_NUM_CH0_W<'a> = crate::FieldWriter<'a, u32, CH_CONF1_SPEC, u16, u16, 10, 20>;
#[doc = "Field `DUTY_INC_CH0` reader - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
pub type DUTY_INC_CH0_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_INC_CH0` writer - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
pub type DUTY_INC_CH0_W<'a> = crate::BitWriter<'a, u32, CH_CONF1_SPEC, bool, 30>;
#[doc = "Field `DUTY_START_CH0` reader - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_CH0_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_START_CH0` writer - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_CH0_W<'a> = crate::BitWriter<'a, u32, CH_CONF1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale_ch0(&self) -> DUTY_SCALE_CH0_R {
        DUTY_SCALE_CH0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle_ch0(&self) -> DUTY_CYCLE_CH0_R {
        DUTY_CYCLE_CH0_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num_ch0(&self) -> DUTY_NUM_CH0_R {
        DUTY_NUM_CH0_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc_ch0(&self) -> DUTY_INC_CH0_R {
        DUTY_INC_CH0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start_ch0(&self) -> DUTY_START_CH0_R {
        DUTY_START_CH0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale_ch0(&mut self) -> DUTY_SCALE_CH0_W {
        DUTY_SCALE_CH0_W::new(self)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle_ch0(&mut self) -> DUTY_CYCLE_CH0_W {
        DUTY_CYCLE_CH0_W::new(self)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num_ch0(&mut self) -> DUTY_NUM_CH0_W {
        DUTY_NUM_CH0_W::new(self)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc_ch0(&mut self) -> DUTY_INC_CH0_W {
        DUTY_INC_CH0_W::new(self)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start_ch0(&mut self) -> DUTY_START_CH0_W {
        DUTY_START_CH0_W::new(self)
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
}
#[doc = "`reset()` method sets CH%s_CONF1 to value 0x4000_0000"]
impl crate::Resettable for CH_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
