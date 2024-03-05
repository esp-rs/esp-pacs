#[doc = "Register `LP_ANA_TOUCH_SLP1` reader"]
pub type R = crate::R<LP_ANA_TOUCH_SLP1_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_SLP1` writer"]
pub type W = crate::W<LP_ANA_TOUCH_SLP1_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_SLP_TH2` reader - need_des"]
pub type LP_ANA_TOUCH_SLP_TH2_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_SLP_TH2` writer - need_des"]
pub type LP_ANA_TOUCH_SLP_TH2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP_ANA_TOUCH_SLP_TH1` reader - need_des"]
pub type LP_ANA_TOUCH_SLP_TH1_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_SLP_TH1` writer - need_des"]
pub type LP_ANA_TOUCH_SLP_TH1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_slp_th2(&self) -> LP_ANA_TOUCH_SLP_TH2_R {
        LP_ANA_TOUCH_SLP_TH2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_slp_th1(&self) -> LP_ANA_TOUCH_SLP_TH1_R {
        LP_ANA_TOUCH_SLP_TH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_SLP1")
            .field(
                "lp_ana_touch_slp_th2",
                &format_args!("{}", self.lp_ana_touch_slp_th2().bits()),
            )
            .field(
                "lp_ana_touch_slp_th1",
                &format_args!("{}", self.lp_ana_touch_slp_th1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_SLP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_slp_th2(&mut self) -> LP_ANA_TOUCH_SLP_TH2_W<LP_ANA_TOUCH_SLP1_SPEC> {
        LP_ANA_TOUCH_SLP_TH2_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_slp_th1(&mut self) -> LP_ANA_TOUCH_SLP_TH1_W<LP_ANA_TOUCH_SLP1_SPEC> {
        LP_ANA_TOUCH_SLP_TH1_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_slp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_slp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_SLP1_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_SLP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_slp1::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_SLP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_slp1::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_SLP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_SLP1 to value 0"]
impl crate::Resettable for LP_ANA_TOUCH_SLP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
