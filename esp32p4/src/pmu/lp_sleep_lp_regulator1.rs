#[doc = "Register `LP_SLEEP_LP_REGULATOR1` reader"]
pub type R = crate::R<LP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "Register `LP_SLEEP_LP_REGULATOR1` writer"]
pub type W = crate::W<LP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DRV_B` reader - need_des"]
pub type LP_SLEEP_LP_REGULATOR_DRV_B_R = crate::FieldReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DRV_B` writer - need_des"]
pub type LP_SLEEP_LP_REGULATOR_DRV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_drv_b(&self) -> LP_SLEEP_LP_REGULATOR_DRV_B_R {
        LP_SLEEP_LP_REGULATOR_DRV_B_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_SLEEP_LP_REGULATOR1")
            .field(
                "lp_sleep_lp_regulator_drv_b",
                &self.lp_sleep_lp_regulator_drv_b(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_regulator_drv_b(
        &mut self,
    ) -> LP_SLEEP_LP_REGULATOR_DRV_B_W<LP_SLEEP_LP_REGULATOR1_SPEC> {
        LP_SLEEP_LP_REGULATOR_DRV_B_W::new(self, 26)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_regulator1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_SLEEP_LP_REGULATOR1_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_REGULATOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_lp_regulator1::R`](R) reader structure"]
impl crate::Readable for LP_SLEEP_LP_REGULATOR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_regulator1::W`](W) writer structure"]
impl crate::Writable for LP_SLEEP_LP_REGULATOR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_REGULATOR1 to value 0"]
impl crate::Resettable for LP_SLEEP_LP_REGULATOR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
