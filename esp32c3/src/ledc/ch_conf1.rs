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
#[doc = "Field `DUTY_SCALE` reader - reg_duty_scale_lsch0."]
pub type DUTY_SCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_SCALE` writer - reg_duty_scale_lsch0."]
pub type DUTY_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH_CONF1_SPEC, u16, u16, 10, O>;
#[doc = "Field `DUTY_CYCLE` reader - reg_duty_cycle_lsch0."]
pub type DUTY_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_CYCLE` writer - reg_duty_cycle_lsch0."]
pub type DUTY_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH_CONF1_SPEC, u16, u16, 10, O>;
#[doc = "Field `DUTY_NUM` reader - reg_duty_num_lsch0."]
pub type DUTY_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTY_NUM` writer - reg_duty_num_lsch0."]
pub type DUTY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CONF1_SPEC, u16, u16, 10, O>;
#[doc = "Field `DUTY_INC` reader - reg_duty_inc_lsch0."]
pub type DUTY_INC_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_INC` writer - reg_duty_inc_lsch0."]
pub type DUTY_INC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF1_SPEC, bool, O>;
#[doc = "Field `DUTY_START` reader - reg_duty_start_lsch0."]
pub type DUTY_START_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_START` writer - reg_duty_start_lsch0."]
pub type DUTY_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - reg_duty_scale_lsch0."]
    #[inline(always)]
    pub fn duty_scale(&self) -> DUTY_SCALE_R {
        DUTY_SCALE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - reg_duty_cycle_lsch0."]
    #[inline(always)]
    pub fn duty_cycle(&self) -> DUTY_CYCLE_R {
        DUTY_CYCLE_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - reg_duty_num_lsch0."]
    #[inline(always)]
    pub fn duty_num(&self) -> DUTY_NUM_R {
        DUTY_NUM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - reg_duty_inc_lsch0."]
    #[inline(always)]
    pub fn duty_inc(&self) -> DUTY_INC_R {
        DUTY_INC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_duty_start_lsch0."]
    #[inline(always)]
    pub fn duty_start(&self) -> DUTY_START_R {
        DUTY_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - reg_duty_scale_lsch0."]
    #[inline(always)]
    pub fn duty_scale(&mut self) -> DUTY_SCALE_W<0> {
        DUTY_SCALE_W::new(self)
    }
    #[doc = "Bits 10:19 - reg_duty_cycle_lsch0."]
    #[inline(always)]
    pub fn duty_cycle(&mut self) -> DUTY_CYCLE_W<10> {
        DUTY_CYCLE_W::new(self)
    }
    #[doc = "Bits 20:29 - reg_duty_num_lsch0."]
    #[inline(always)]
    pub fn duty_num(&mut self) -> DUTY_NUM_W<20> {
        DUTY_NUM_W::new(self)
    }
    #[doc = "Bit 30 - reg_duty_inc_lsch0."]
    #[inline(always)]
    pub fn duty_inc(&mut self) -> DUTY_INC_W<30> {
        DUTY_INC_W::new(self)
    }
    #[doc = "Bit 31 - reg_duty_start_lsch0."]
    #[inline(always)]
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
#[doc = "LEDC_LSCH%s_CONF1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_conf1](index.html) module"]
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
