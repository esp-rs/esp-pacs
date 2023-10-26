#[doc = "Register `CH%sCARRIER_DUTY` reader"]
pub type R = crate::R<CHCARRIER_DUTY_SPEC>;
#[doc = "Register `CH%sCARRIER_DUTY` writer"]
pub type W = crate::W<CHCARRIER_DUTY_SPEC>;
#[doc = "Field `CARRIER_LOW` reader - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
pub type CARRIER_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_LOW` writer - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
pub type CARRIER_LOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CARRIER_HIGH` reader - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
pub type CARRIER_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_HIGH` writer - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
pub type CARRIER_HIGH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave 's low level clock period for CHANNEL%s."]
    #[inline(always)]
    pub fn carrier_low(&self) -> CARRIER_LOW_R {
        CARRIER_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
    #[inline(always)]
    pub fn carrier_high(&self) -> CARRIER_HIGH_R {
        CARRIER_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCARRIER_DUTY")
            .field(
                "carrier_low",
                &format_args!("{}", self.carrier_low().bits()),
            )
            .field(
                "carrier_high",
                &format_args!("{}", self.carrier_high().bits()),
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
    pub fn carrier_low(&mut self) -> CARRIER_LOW_W<CHCARRIER_DUTY_SPEC, 0> {
        CARRIER_LOW_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave 's high level clock period for CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_high(&mut self) -> CARRIER_HIGH_W<CHCARRIER_DUTY_SPEC, 16> {
        CARRIER_HIGH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel %s duty cycle configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCARRIER_DUTY_SPEC;
impl crate::RegisterSpec for CHCARRIER_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chcarrier_duty::R`](R) reader structure"]
impl crate::Readable for CHCARRIER_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chcarrier_duty::W`](W) writer structure"]
impl crate::Writable for CHCARRIER_DUTY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sCARRIER_DUTY to value 0x0040_0040"]
impl crate::Resettable for CHCARRIER_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0040;
}
