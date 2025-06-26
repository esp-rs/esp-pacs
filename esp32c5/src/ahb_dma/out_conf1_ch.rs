#[doc = "Register `OUT_CONF1_CH%s` reader"]
pub type R = crate::R<OUT_CONF1_CH_SPEC>;
#[doc = "Register `OUT_CONF1_CH%s` writer"]
pub type W = crate::W<OUT_CONF1_CH_SPEC>;
#[doc = "Field `OUT_CHECK_OWNER` reader - Configures whether or not to enable owner bit check for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OUT_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER` writer - Configures whether or not to enable owner bit check for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OUT_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Configures whether or not to enable owner bit check for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_check_owner(&self) -> OUT_CHECK_OWNER_R {
        OUT_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF1_CH")
            .field("out_check_owner", &self.out_check_owner())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Configures whether or not to enable owner bit check for TX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_check_owner(&mut self) -> OUT_CHECK_OWNER_W<OUT_CONF1_CH_SPEC> {
        OUT_CHECK_OWNER_W::new(self, 12)
    }
}
#[doc = "Configuration register 1 of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF1_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf1_ch::R`](R) reader structure"]
impl crate::Readable for OUT_CONF1_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf1_ch::W`](W) writer structure"]
impl crate::Writable for OUT_CONF1_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CONF1_CH%s to value 0"]
impl crate::Resettable for OUT_CONF1_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
