#[doc = "Register `SLP_WAKEUP_CNTL3` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL3_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL3` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL3_SPEC>;
#[doc = "Field `LP_MIN_SLP_VAL` reader - need_des"]
pub type LP_MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `LP_MIN_SLP_VAL` writer - need_des"]
pub type LP_MIN_SLP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_MIN_SLP_VAL` reader - need_des"]
pub type HP_MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `HP_MIN_SLP_VAL` writer - need_des"]
pub type HP_MIN_SLP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLEEP_PRT_SEL` reader - need_des"]
pub type SLEEP_PRT_SEL_R = crate::FieldReader;
#[doc = "Field `SLEEP_PRT_SEL` writer - need_des"]
pub type SLEEP_PRT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_min_slp_val(&self) -> LP_MIN_SLP_VAL_R {
        LP_MIN_SLP_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn hp_min_slp_val(&self) -> HP_MIN_SLP_VAL_R {
        HP_MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn sleep_prt_sel(&self) -> SLEEP_PRT_SEL_R {
        SLEEP_PRT_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL3")
            .field(
                "lp_min_slp_val",
                &format_args!("{}", self.lp_min_slp_val().bits()),
            )
            .field(
                "hp_min_slp_val",
                &format_args!("{}", self.hp_min_slp_val().bits()),
            )
            .field(
                "sleep_prt_sel",
                &format_args!("{}", self.sleep_prt_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_min_slp_val(&mut self) -> LP_MIN_SLP_VAL_W<SLP_WAKEUP_CNTL3_SPEC> {
        LP_MIN_SLP_VAL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_min_slp_val(&mut self) -> HP_MIN_SLP_VAL_W<SLP_WAKEUP_CNTL3_SPEC> {
        HP_MIN_SLP_VAL_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_prt_sel(&mut self) -> SLEEP_PRT_SEL_W<SLP_WAKEUP_CNTL3_SPEC> {
        SLEEP_PRT_SEL_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL3_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl3::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl3::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL3 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL3_SPEC {
    const RESET_VALUE: u32 = 0;
}
