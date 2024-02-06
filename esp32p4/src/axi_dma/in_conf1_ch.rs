#[doc = "Register `IN_CONF1_CH%s` reader"]
pub type R = crate::R<IN_CONF1_CH_SPEC>;
#[doc = "Register `IN_CONF1_CH%s` writer"]
pub type W = crate::W<IN_CONF1_CH_SPEC>;
#[doc = "Field `IN_CHECK_OWNER_CH` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_CH_R = crate::BitReader;
#[doc = "Field `IN_CHECK_OWNER_CH` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch(&self) -> IN_CHECK_OWNER_CH_R {
        IN_CHECK_OWNER_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF1_CH")
            .field(
                "in_check_owner_ch",
                &format_args!("{}", self.in_check_owner_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF1_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn in_check_owner_ch(&mut self) -> IN_CHECK_OWNER_CH_W<IN_CONF1_CH_SPEC> {
        IN_CHECK_OWNER_CH_W::new(self, 12)
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
#[doc = "Configure 1 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF1_CH_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf1_ch::R`](R) reader structure"]
impl crate::Readable for IN_CONF1_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf1_ch::W`](W) writer structure"]
impl crate::Writable for IN_CONF1_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF1_CH%s to value 0"]
impl crate::Resettable for IN_CONF1_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
