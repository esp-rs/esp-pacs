#[doc = "Register `CH%s_DUTY` reader"]
pub type R = crate::R<CH_DUTY_SPEC>;
#[doc = "Register `CH%s_DUTY` writer"]
pub type W = crate::W<CH_DUTY_SPEC>;
#[doc = "Field `DUTY` reader - reg_duty_lsch0."]
pub type DUTY_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY` writer - reg_duty_lsch0."]
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch0."]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x0007_ffff)
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18 - reg_duty_lsch0."]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CH_DUTY_SPEC> {
        DUTY_W::new(self, 0)
    }
}
#[doc = "LEDC_LSCH%s_DUTY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_DUTY_SPEC;
impl crate::RegisterSpec for CH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_duty::R`](R) reader structure"]
impl crate::Readable for CH_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_duty::W`](W) writer structure"]
impl crate::Writable for CH_DUTY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_DUTY to value 0"]
impl crate::Resettable for CH_DUTY_SPEC {
    const RESET_VALUE: u32 = 0;
}
