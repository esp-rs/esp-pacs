#[doc = "Register `DUTY` reader"]
pub type R = crate::R<DUTY_SPEC>;
#[doc = "Register `DUTY` writer"]
pub type W = crate::W<DUTY_SPEC>;
#[doc = "Field `DUTY_CH` reader - Configures the duty of signal output on channel %s."]
pub type DUTY_CH_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY_CH` writer - Configures the duty of signal output on channel %s."]
pub type DUTY_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Configures the duty of signal output on channel %s."]
    #[inline(always)]
    pub fn duty_ch(&self) -> DUTY_CH_R {
        DUTY_CH_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DUTY")
            .field("duty_ch", &self.duty_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - Configures the duty of signal output on channel %s."]
    #[inline(always)]
    pub fn duty_ch(&mut self) -> DUTY_CH_W<DUTY_SPEC> {
        DUTY_CH_W::new(self, 0)
    }
}
#[doc = "Initial duty cycle register for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUTY_SPEC;
impl crate::RegisterSpec for DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty::R`](R) reader structure"]
impl crate::Readable for DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`duty::W`](W) writer structure"]
impl crate::Writable for DUTY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUTY to value 0"]
impl crate::Resettable for DUTY_SPEC {}
