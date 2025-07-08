#[doc = "Register `OUT_CONF1_CH0` reader"]
pub type R = crate::R<OUT_CONF1_CH0_SPEC>;
#[doc = "Register `OUT_CONF1_CH0` writer"]
pub type W = crate::W<OUT_CONF1_CH0_SPEC>;
#[doc = "Field `OUT_CHECK_OWNER_CH0` reader - Configures whether to enable owner bit check for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OUT_CHECK_OWNER_CH0_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER_CH0` writer - Configures whether to enable owner bit check for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OUT_CHECK_OWNER_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Configures whether to enable owner bit check for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_check_owner_ch0(&self) -> OUT_CHECK_OWNER_CH0_R {
        OUT_CHECK_OWNER_CH0_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF1_CH0")
            .field("out_check_owner_ch0", &self.out_check_owner_ch0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Configures whether to enable owner bit check for TX channel 0.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn out_check_owner_ch0(&mut self) -> OUT_CHECK_OWNER_CH0_W<OUT_CONF1_CH0_SPEC> {
        OUT_CHECK_OWNER_CH0_W::new(self, 12)
    }
}
#[doc = "Configuration register 1 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF1_CH0_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf1_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_CONF1_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf1_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_CONF1_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CONF1_CH0 to value 0"]
impl crate::Resettable for OUT_CONF1_CH0_SPEC {}
