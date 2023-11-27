#[doc = "Register `LP_ANA_TOUCH_SCAN_CTRL1` reader"]
pub type R = crate::R<LP_ANA_TOUCH_SCAN_CTRL1_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_SCAN_CTRL1` writer"]
pub type W = crate::W<LP_ANA_TOUCH_SCAN_CTRL1_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_SHIELD_PAD_EN` reader - need_des"]
pub type LP_ANA_TOUCH_SHIELD_PAD_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_SHIELD_PAD_EN` writer - need_des"]
pub type LP_ANA_TOUCH_SHIELD_PAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_INACTIVE_CONNECTION` reader - need_des"]
pub type LP_ANA_TOUCH_INACTIVE_CONNECTION_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_INACTIVE_CONNECTION` writer - need_des"]
pub type LP_ANA_TOUCH_INACTIVE_CONNECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_SCAN_PAD_MAP` reader - need_des"]
pub type LP_ANA_TOUCH_SCAN_PAD_MAP_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_SCAN_PAD_MAP` writer - need_des"]
pub type LP_ANA_TOUCH_SCAN_PAD_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LP_ANA_TOUCH_XPD_WAIT` reader - need_des"]
pub type LP_ANA_TOUCH_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_XPD_WAIT` writer - need_des"]
pub type LP_ANA_TOUCH_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_shield_pad_en(&self) -> LP_ANA_TOUCH_SHIELD_PAD_EN_R {
        LP_ANA_TOUCH_SHIELD_PAD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_inactive_connection(&self) -> LP_ANA_TOUCH_INACTIVE_CONNECTION_R {
        LP_ANA_TOUCH_INACTIVE_CONNECTION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:16 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_scan_pad_map(&self) -> LP_ANA_TOUCH_SCAN_PAD_MAP_R {
        LP_ANA_TOUCH_SCAN_PAD_MAP_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_xpd_wait(&self) -> LP_ANA_TOUCH_XPD_WAIT_R {
        LP_ANA_TOUCH_XPD_WAIT_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_SCAN_CTRL1")
            .field(
                "lp_ana_touch_shield_pad_en",
                &format_args!("{}", self.lp_ana_touch_shield_pad_en().bit()),
            )
            .field(
                "lp_ana_touch_inactive_connection",
                &format_args!("{}", self.lp_ana_touch_inactive_connection().bit()),
            )
            .field(
                "lp_ana_touch_scan_pad_map",
                &format_args!("{}", self.lp_ana_touch_scan_pad_map().bits()),
            )
            .field(
                "lp_ana_touch_xpd_wait",
                &format_args!("{}", self.lp_ana_touch_xpd_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_SCAN_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_shield_pad_en(
        &mut self,
    ) -> LP_ANA_TOUCH_SHIELD_PAD_EN_W<LP_ANA_TOUCH_SCAN_CTRL1_SPEC> {
        LP_ANA_TOUCH_SHIELD_PAD_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_inactive_connection(
        &mut self,
    ) -> LP_ANA_TOUCH_INACTIVE_CONNECTION_W<LP_ANA_TOUCH_SCAN_CTRL1_SPEC> {
        LP_ANA_TOUCH_INACTIVE_CONNECTION_W::new(self, 1)
    }
    #[doc = "Bits 2:16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_scan_pad_map(
        &mut self,
    ) -> LP_ANA_TOUCH_SCAN_PAD_MAP_W<LP_ANA_TOUCH_SCAN_CTRL1_SPEC> {
        LP_ANA_TOUCH_SCAN_PAD_MAP_W::new(self, 2)
    }
    #[doc = "Bits 17:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_xpd_wait(
        &mut self,
    ) -> LP_ANA_TOUCH_XPD_WAIT_W<LP_ANA_TOUCH_SCAN_CTRL1_SPEC> {
        LP_ANA_TOUCH_XPD_WAIT_W::new(self, 17)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_scan_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_scan_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_SCAN_CTRL1_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_SCAN_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_scan_ctrl1::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_SCAN_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_scan_ctrl1::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_SCAN_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_SCAN_CTRL1 to value 0x0008_0000"]
impl crate::Resettable for LP_ANA_TOUCH_SCAN_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
