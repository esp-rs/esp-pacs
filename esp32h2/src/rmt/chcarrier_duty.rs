#[doc = "Register `CH%sCARRIER_DUTY` reader"]
pub struct R(crate::R<CHCARRIER_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCARRIER_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCARRIER_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCARRIER_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sCARRIER_DUTY` writer"]
pub struct W(crate::W<CHCARRIER_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCARRIER_DUTY_SPEC>;
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
impl From<crate::W<CHCARRIER_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCARRIER_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER_LOW_CH0` reader - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
pub type CARRIER_LOW_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_LOW_CH0` writer - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
pub type CARRIER_LOW_CH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CHCARRIER_DUTY_SPEC, 16, O, u16>;
#[doc = "Field `CARRIER_HIGH_CH0` reader - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
pub type CARRIER_HIGH_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_HIGH_CH0` writer - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
pub type CARRIER_HIGH_CH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CHCARRIER_DUTY_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
    #[inline(always)]
    pub fn carrier_low_ch0(&self) -> CARRIER_LOW_CH0_R {
        CARRIER_LOW_CH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
    #[inline(always)]
    pub fn carrier_high_ch0(&self) -> CARRIER_HIGH_CH0_R {
        CARRIER_HIGH_CH0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCARRIER_DUTY")
            .field(
                "carrier_low_ch0",
                &format_args!("{}", self.carrier_low_ch0().bits()),
            )
            .field(
                "carrier_high_ch0",
                &format_args!("{}", self.carrier_high_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHCARRIER_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_low_ch0(&mut self) -> CARRIER_LOW_CH0_W<0> {
        CARRIER_LOW_CH0_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_high_ch0(&mut self) -> CARRIER_HIGH_CH0_W<16> {
        CARRIER_HIGH_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s duty cycle configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcarrier_duty](index.html) module"]
pub struct CHCARRIER_DUTY_SPEC;
impl crate::RegisterSpec for CHCARRIER_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chcarrier_duty::R](R) reader structure"]
impl crate::Readable for CHCARRIER_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chcarrier_duty::W](W) writer structure"]
impl crate::Writable for CHCARRIER_DUTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sCARRIER_DUTY to value 0x0040_0040"]
impl crate::Resettable for CHCARRIER_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0040;
}
