#[doc = "Register `CH%s_DUTY` reader"]
pub type R = crate::R<CH_DUTY_SPEC>;
#[doc = "Register `CH%s_DUTY` writer"]
pub type W = crate::W<CH_DUTY_SPEC>;
#[doc = "Field `DUTY` reader - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
pub type DUTY_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY` writer - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
pub type DUTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 25, O, u32>;
impl R {
    #[doc = "Bits 0:24 - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_DUTY")
            .field("duty", &format_args!("{}", self.duty().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:24 - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint."]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CH_DUTY_SPEC, 0> {
        DUTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Initial duty cycle for channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_DUTY_SPEC;
impl crate::RegisterSpec for CH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_duty::R`](R) reader structure"]
impl crate::Readable for CH_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_duty::W`](W) writer structure"]
impl crate::Writable for CH_DUTY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_DUTY to value 0"]
impl crate::Resettable for CH_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
