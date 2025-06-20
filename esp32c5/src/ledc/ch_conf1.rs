#[doc = "Register `CH%s_CONF1` reader"]
pub type R = crate::R<CH_CONF1_SPEC>;
#[doc = "Register `CH%s_CONF1` writer"]
pub type W = crate::W<CH_CONF1_SPEC>;
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
        f.debug_struct("CH_CONF1")
            .field("duty_start_ch", &self.duty_start_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    pub fn duty_start_ch(&mut self) -> DUTY_START_CH_W<CH_CONF1_SPEC> {
        DUTY_START_CH_W::new(self, 31)
    }
}
#[doc = "Configuration register 1 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_CONF1_SPEC;
impl crate::RegisterSpec for CH_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_conf1::R`](R) reader structure"]
impl crate::Readable for CH_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_conf1::W`](W) writer structure"]
impl crate::Writable for CH_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_CONF1 to value 0"]
impl crate::Resettable for CH_CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
