#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `DUTY_START_CH` reader - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
pub type DUTY_START_CH_R = crate::BitReader;
#[doc = "Field `DUTY_START_CH` writer - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
pub type DUTY_START_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    pub fn duty_start_ch(&self) -> DUTY_START_CH_R {
        DUTY_START_CH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("duty_start_ch", &self.duty_start_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    pub fn duty_start_ch(&mut self) -> DUTY_START_CH_W<'_, CONF1_SPEC> {
        DUTY_START_CH_W::new(self, 31)
    }
}
#[doc = "Configuration register 1 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF1 to value 0"]
impl crate::Resettable for CONF1_SPEC {}
