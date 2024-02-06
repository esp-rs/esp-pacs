#[doc = "Register `LP_ANA_TOUCH_SLP0` reader"]
pub type R = crate::R<LP_ANA_TOUCH_SLP0_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_SLP0` writer"]
pub type W = crate::W<LP_ANA_TOUCH_SLP0_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_SLP_TH0` reader - need_des"]
pub type LP_ANA_TOUCH_SLP_TH0_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_SLP_TH0` writer - need_des"]
pub type LP_ANA_TOUCH_SLP_TH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP_ANA_TOUCH_SLP_CHANNEL_CLR` writer - need_des"]
pub type LP_ANA_TOUCH_SLP_CHANNEL_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_SLP_PAD` reader - need_des"]
pub type LP_ANA_TOUCH_SLP_PAD_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_SLP_PAD` writer - need_des"]
pub type LP_ANA_TOUCH_SLP_PAD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_slp_th0(&self) -> LP_ANA_TOUCH_SLP_TH0_R {
        LP_ANA_TOUCH_SLP_TH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_slp_pad(&self) -> LP_ANA_TOUCH_SLP_PAD_R {
        LP_ANA_TOUCH_SLP_PAD_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_SLP0")
            .field(
                "lp_ana_touch_slp_th0",
                &format_args!("{}", self.lp_ana_touch_slp_th0().bits()),
            )
            .field(
                "lp_ana_touch_slp_pad",
                &format_args!("{}", self.lp_ana_touch_slp_pad().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_SLP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_slp_th0(&mut self) -> LP_ANA_TOUCH_SLP_TH0_W<LP_ANA_TOUCH_SLP0_SPEC> {
        LP_ANA_TOUCH_SLP_TH0_W::new(self, 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_slp_channel_clr(
        &mut self,
    ) -> LP_ANA_TOUCH_SLP_CHANNEL_CLR_W<LP_ANA_TOUCH_SLP0_SPEC> {
        LP_ANA_TOUCH_SLP_CHANNEL_CLR_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_slp_pad(&mut self) -> LP_ANA_TOUCH_SLP_PAD_W<LP_ANA_TOUCH_SLP0_SPEC> {
        LP_ANA_TOUCH_SLP_PAD_W::new(self, 17)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_slp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_slp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_SLP0_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_SLP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_slp0::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_SLP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_slp0::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_SLP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_SLP0 to value 0x001e_0000"]
impl crate::Resettable for LP_ANA_TOUCH_SLP0_SPEC {
    const RESET_VALUE: u32 = 0x001e_0000;
}
