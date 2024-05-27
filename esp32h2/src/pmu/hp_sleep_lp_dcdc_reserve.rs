///Register `HP_SLEEP_LP_DCDC_RESERVE` writer
pub type W = crate::W<HP_SLEEP_LP_DCDC_RESERVE_SPEC>;
///Field `HP_SLEEP_LP_DCDC_RESERVE` writer - need_des
pub type HP_SLEEP_LP_DCDC_RESERVE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_LP_DCDC_RESERVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_lp_dcdc_reserve(
        &mut self,
    ) -> HP_SLEEP_LP_DCDC_RESERVE_W<HP_SLEEP_LP_DCDC_RESERVE_SPEC> {
        HP_SLEEP_LP_DCDC_RESERVE_W::new(self, 0)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_dcdc_reserve::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_SLEEP_LP_DCDC_RESERVE_SPEC;
impl crate::RegisterSpec for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hp_sleep_lp_dcdc_reserve::W`](W) writer structure
impl crate::Writable for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_SLEEP_LP_DCDC_RESERVE to value 0
impl crate::Resettable for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    const RESET_VALUE: u32 = 0;
}
