#[doc = "Register `OUT_CONF1_CH%s` reader"]
pub type R = crate::R<OUT_CONF1_CH_SPEC>;
#[doc = "Register `OUT_CONF1_CH%s` writer"]
pub type W = crate::W<OUT_CONF1_CH_SPEC>;
#[doc = "Field `OUT_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&self) -> OUT_CHECK_OWNER_R {
        OUT_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF1_CH")
            .field(
                "out_check_owner",
                &format_args!("{}", self.out_check_owner().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF1_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn out_check_owner(&mut self) -> OUT_CHECK_OWNER_W<OUT_CONF1_CH_SPEC> {
        OUT_CHECK_OWNER_W::new(self, 12)
    }
}
#[doc = "DMA_OUT_CONF1_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
