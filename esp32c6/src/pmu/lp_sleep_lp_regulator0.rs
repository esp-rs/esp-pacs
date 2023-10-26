#[doc = "Register `LP_SLEEP_LP_REGULATOR0` reader"]
pub type R = crate::R<LP_SLEEP_LP_REGULATOR0_SPEC>;
#[doc = "Register `LP_SLEEP_LP_REGULATOR0` writer"]
pub type W = crate::W<LP_SLEEP_LP_REGULATOR0_SPEC>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_XPD` reader - need_des"]
pub type LP_SLEEP_LP_REGULATOR_SLP_XPD_R = crate::BitReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_XPD` writer - need_des"]
pub type LP_SLEEP_LP_REGULATOR_SLP_XPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_XPD` reader - need_des"]
pub type LP_SLEEP_LP_REGULATOR_XPD_R = crate::BitReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_XPD` writer - need_des"]
pub type LP_SLEEP_LP_REGULATOR_XPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_DBIAS` reader - need_des"]
pub type LP_SLEEP_LP_REGULATOR_SLP_DBIAS_R = crate::FieldReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_DBIAS` writer - need_des"]
pub type LP_SLEEP_LP_REGULATOR_SLP_DBIAS_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DBIAS` reader - need_des"]
pub type LP_SLEEP_LP_REGULATOR_DBIAS_R = crate::FieldReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DBIAS` writer - need_des"]
pub type LP_SLEEP_LP_REGULATOR_DBIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_slp_xpd(&self) -> LP_SLEEP_LP_REGULATOR_SLP_XPD_R {
        LP_SLEEP_LP_REGULATOR_SLP_XPD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_xpd(&self) -> LP_SLEEP_LP_REGULATOR_XPD_R {
        LP_SLEEP_LP_REGULATOR_XPD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_slp_dbias(&self) -> LP_SLEEP_LP_REGULATOR_SLP_DBIAS_R {
        LP_SLEEP_LP_REGULATOR_SLP_DBIAS_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_dbias(&self) -> LP_SLEEP_LP_REGULATOR_DBIAS_R {
        LP_SLEEP_LP_REGULATOR_DBIAS_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_SLEEP_LP_REGULATOR0")
            .field(
                "lp_sleep_lp_regulator_slp_xpd",
                &format_args!("{}", self.lp_sleep_lp_regulator_slp_xpd().bit()),
            )
            .field(
                "lp_sleep_lp_regulator_xpd",
                &format_args!("{}", self.lp_sleep_lp_regulator_xpd().bit()),
            )
            .field(
                "lp_sleep_lp_regulator_slp_dbias",
                &format_args!("{}", self.lp_sleep_lp_regulator_slp_dbias().bits()),
            )
            .field(
                "lp_sleep_lp_regulator_dbias",
                &format_args!("{}", self.lp_sleep_lp_regulator_dbias().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_SLEEP_LP_REGULATOR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_regulator_slp_xpd(
        &mut self,
    ) -> LP_SLEEP_LP_REGULATOR_SLP_XPD_W<LP_SLEEP_LP_REGULATOR0_SPEC, 21> {
        LP_SLEEP_LP_REGULATOR_SLP_XPD_W::new(self)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_regulator_xpd(
        &mut self,
    ) -> LP_SLEEP_LP_REGULATOR_XPD_W<LP_SLEEP_LP_REGULATOR0_SPEC, 22> {
        LP_SLEEP_LP_REGULATOR_XPD_W::new(self)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_regulator_slp_dbias(
        &mut self,
    ) -> LP_SLEEP_LP_REGULATOR_SLP_DBIAS_W<LP_SLEEP_LP_REGULATOR0_SPEC, 23> {
        LP_SLEEP_LP_REGULATOR_SLP_DBIAS_W::new(self)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_regulator_dbias(
        &mut self,
    ) -> LP_SLEEP_LP_REGULATOR_DBIAS_W<LP_SLEEP_LP_REGULATOR0_SPEC, 27> {
        LP_SLEEP_LP_REGULATOR_DBIAS_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_regulator0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_SLEEP_LP_REGULATOR0_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_REGULATOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_lp_regulator0::R`](R) reader structure"]
impl crate::Readable for LP_SLEEP_LP_REGULATOR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_regulator0::W`](W) writer structure"]
impl crate::Writable for LP_SLEEP_LP_REGULATOR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_REGULATOR0 to value 0xc660_0000"]
impl crate::Resettable for LP_SLEEP_LP_REGULATOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0xc660_0000;
}
