#[doc = "Register `CH%s_DUTY` reader"]
pub struct R(crate::R<CH_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_DUTY` writer"]
pub struct W(crate::W<CH_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_DUTY_SPEC>;
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
impl From<crate::W<CH_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_CH0` reader - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
pub type DUTY_CH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DUTY_CH0` writer - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
pub type DUTY_CH0_W<'a> = crate::FieldWriter<'a, u32, CH_DUTY_SPEC, u32, u32, 19, 0>;
impl R {
    #[doc = "Bits 0:18 - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
    #[inline(always)]
    pub fn duty_ch0(&self) -> DUTY_CH0_R {
        DUTY_CH0_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
    #[inline(always)]
    pub fn duty_ch0(&mut self) -> DUTY_CH0_W {
        DUTY_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial duty cycle for channel %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_duty](index.html) module"]
pub struct CH_DUTY_SPEC;
impl crate::RegisterSpec for CH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_duty::R](R) reader structure"]
impl crate::Readable for CH_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_duty::W](W) writer structure"]
impl crate::Writable for CH_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_DUTY to value 0"]
impl crate::Resettable for CH_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
