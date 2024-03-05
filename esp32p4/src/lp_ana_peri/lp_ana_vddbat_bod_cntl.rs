#[doc = "Register `LP_ANA_VDDBAT_BOD_CNTL` reader"]
pub type R = crate::R<LP_ANA_VDDBAT_BOD_CNTL_SPEC>;
#[doc = "Register `LP_ANA_VDDBAT_BOD_CNTL` writer"]
pub type W = crate::W<LP_ANA_VDDBAT_BOD_CNTL_SPEC>;
#[doc = "Field `LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG` reader - need_des"]
pub type LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_R = crate::BitReader;
#[doc = "Field `LP_ANA_VDDBAT_CHARGER` reader - need_des"]
pub type LP_ANA_VDDBAT_CHARGER_R = crate::BitReader;
#[doc = "Field `LP_ANA_VDDBAT_CHARGER` writer - need_des"]
pub type LP_ANA_VDDBAT_CHARGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_VDDBAT_CNT_CLR` reader - need_des"]
pub type LP_ANA_VDDBAT_CNT_CLR_R = crate::BitReader;
#[doc = "Field `LP_ANA_VDDBAT_CNT_CLR` writer - need_des"]
pub type LP_ANA_VDDBAT_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_VDDBAT_UPVOLTAGE_TARGET` reader - need_des"]
pub type LP_ANA_VDDBAT_UPVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_VDDBAT_UPVOLTAGE_TARGET` writer - need_des"]
pub type LP_ANA_VDDBAT_UPVOLTAGE_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET` reader - need_des"]
pub type LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET` writer - need_des"]
pub type LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_flag(&self) -> LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_R {
        LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn lp_ana_vddbat_charger(&self) -> LP_ANA_VDDBAT_CHARGER_R {
        LP_ANA_VDDBAT_CHARGER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn lp_ana_vddbat_cnt_clr(&self) -> LP_ANA_VDDBAT_CNT_CLR_R {
        LP_ANA_VDDBAT_CNT_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn lp_ana_vddbat_upvoltage_target(&self) -> LP_ANA_VDDBAT_UPVOLTAGE_TARGET_R {
        LP_ANA_VDDBAT_UPVOLTAGE_TARGET_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_target(&self) -> LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_R {
        LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_VDDBAT_BOD_CNTL")
            .field(
                "lp_ana_vddbat_undervoltage_flag",
                &format_args!("{}", self.lp_ana_vddbat_undervoltage_flag().bit()),
            )
            .field(
                "lp_ana_vddbat_charger",
                &format_args!("{}", self.lp_ana_vddbat_charger().bit()),
            )
            .field(
                "lp_ana_vddbat_cnt_clr",
                &format_args!("{}", self.lp_ana_vddbat_cnt_clr().bit()),
            )
            .field(
                "lp_ana_vddbat_upvoltage_target",
                &format_args!("{}", self.lp_ana_vddbat_upvoltage_target().bits()),
            )
            .field(
                "lp_ana_vddbat_undervoltage_target",
                &format_args!("{}", self.lp_ana_vddbat_undervoltage_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_VDDBAT_BOD_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_vddbat_charger(
        &mut self,
    ) -> LP_ANA_VDDBAT_CHARGER_W<LP_ANA_VDDBAT_BOD_CNTL_SPEC> {
        LP_ANA_VDDBAT_CHARGER_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_vddbat_cnt_clr(
        &mut self,
    ) -> LP_ANA_VDDBAT_CNT_CLR_W<LP_ANA_VDDBAT_BOD_CNTL_SPEC> {
        LP_ANA_VDDBAT_CNT_CLR_W::new(self, 11)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_vddbat_upvoltage_target(
        &mut self,
    ) -> LP_ANA_VDDBAT_UPVOLTAGE_TARGET_W<LP_ANA_VDDBAT_BOD_CNTL_SPEC> {
        LP_ANA_VDDBAT_UPVOLTAGE_TARGET_W::new(self, 12)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_vddbat_undervoltage_target(
        &mut self,
    ) -> LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_W<LP_ANA_VDDBAT_BOD_CNTL_SPEC> {
        LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_vddbat_bod_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_vddbat_bod_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_VDDBAT_BOD_CNTL_SPEC;
impl crate::RegisterSpec for LP_ANA_VDDBAT_BOD_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_vddbat_bod_cntl::R`](R) reader structure"]
impl crate::Readable for LP_ANA_VDDBAT_BOD_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_vddbat_bod_cntl::W`](W) writer structure"]
impl crate::Writable for LP_ANA_VDDBAT_BOD_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_VDDBAT_BOD_CNTL to value 0xffc0_0000"]
impl crate::Resettable for LP_ANA_VDDBAT_BOD_CNTL_SPEC {
    const RESET_VALUE: u32 = 0xffc0_0000;
}
