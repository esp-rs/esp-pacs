#[doc = "Register `ANA_XPD_PAD_GROUP` reader"]
pub type R = crate::R<ANA_XPD_PAD_GROUP_SPEC>;
#[doc = "Register `ANA_XPD_PAD_GROUP` writer"]
pub type W = crate::W<ANA_XPD_PAD_GROUP_SPEC>;
#[doc = "Field `ANA_REG_XPD_PAD_GROUP` reader - Set 1 to power up pad group"]
pub type ANA_REG_XPD_PAD_GROUP_R = crate::FieldReader;
#[doc = "Field `ANA_REG_XPD_PAD_GROUP` writer - Set 1 to power up pad group"]
pub type ANA_REG_XPD_PAD_GROUP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set 1 to power up pad group"]
    #[inline(always)]
    pub fn ana_reg_xpd_pad_group(&self) -> ANA_REG_XPD_PAD_GROUP_R {
        ANA_REG_XPD_PAD_GROUP_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_XPD_PAD_GROUP")
            .field(
                "ana_reg_xpd_pad_group",
                &format_args!("{}", self.ana_reg_xpd_pad_group().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ANA_XPD_PAD_GROUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set 1 to power up pad group"]
    #[inline(always)]
    #[must_use]
    pub fn ana_reg_xpd_pad_group(&mut self) -> ANA_REG_XPD_PAD_GROUP_W<ANA_XPD_PAD_GROUP_SPEC> {
        ANA_REG_XPD_PAD_GROUP_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_xpd_pad_group::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_xpd_pad_group::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_XPD_PAD_GROUP_SPEC;
impl crate::RegisterSpec for ANA_XPD_PAD_GROUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_xpd_pad_group::R`](R) reader structure"]
impl crate::Readable for ANA_XPD_PAD_GROUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_xpd_pad_group::W`](W) writer structure"]
impl crate::Writable for ANA_XPD_PAD_GROUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_XPD_PAD_GROUP to value 0xff"]
impl crate::Resettable for ANA_XPD_PAD_GROUP_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
