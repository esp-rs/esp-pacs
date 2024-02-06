#[doc = "Register `LP_ANA_TOUCH_PAD13_TH0` reader"]
pub type R = crate::R<LP_ANA_TOUCH_PAD13_TH0_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_PAD13_TH0` writer"]
pub type W = crate::W<LP_ANA_TOUCH_PAD13_TH0_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_PAD13_TH0` reader - Reserved"]
pub type LP_ANA_TOUCH_PAD13_TH0_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_PAD13_TH0` writer - Reserved"]
pub type LP_ANA_TOUCH_PAD13_TH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn lp_ana_touch_pad13_th0(&self) -> LP_ANA_TOUCH_PAD13_TH0_R {
        LP_ANA_TOUCH_PAD13_TH0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_PAD13_TH0")
            .field(
                "lp_ana_touch_pad13_th0",
                &format_args!("{}", self.lp_ana_touch_pad13_th0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_PAD13_TH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_pad13_th0(
        &mut self,
    ) -> LP_ANA_TOUCH_PAD13_TH0_W<LP_ANA_TOUCH_PAD13_TH0_SPEC> {
        LP_ANA_TOUCH_PAD13_TH0_W::new(self, 16)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad13_th0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad13_th0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_PAD13_TH0_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_PAD13_TH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_pad13_th0::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_PAD13_TH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_pad13_th0::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_PAD13_TH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_PAD13_TH0 to value 0"]
impl crate::Resettable for LP_ANA_TOUCH_PAD13_TH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
