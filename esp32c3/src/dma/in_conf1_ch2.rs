#[doc = "Register `IN_CONF1_CH2` reader"]
pub type R = crate::R<IN_CONF1_CH2_SPEC>;
#[doc = "Register `IN_CONF1_CH2` writer"]
pub type W = crate::W<IN_CONF1_CH2_SPEC>;
#[doc = "Field `IN_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `IN_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner(&self) -> IN_CHECK_OWNER_R {
        IN_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF1_CH2")
            .field(
                "in_check_owner",
                &format_args!("{}", self.in_check_owner().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF1_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn in_check_owner(&mut self) -> IN_CHECK_OWNER_W<IN_CONF1_CH2_SPEC> {
        IN_CHECK_OWNER_W::new(self, 12)
    }
}
#[doc = "DMA_IN_CONF1_CH2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF1_CH2_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf1_ch2::R`](R) reader structure"]
impl crate::Readable for IN_CONF1_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf1_ch2::W`](W) writer structure"]
impl crate::Writable for IN_CONF1_CH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF1_CH2 to value 0"]
impl crate::Resettable for IN_CONF1_CH2_SPEC {
    const RESET_VALUE: u32 = 0;
}
