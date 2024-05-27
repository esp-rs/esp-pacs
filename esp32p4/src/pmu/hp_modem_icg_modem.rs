///Register `HP_MODEM_ICG_MODEM` writer
pub type W = crate::W<HP_MODEM_ICG_MODEM_SPEC>;
///Field `HP_MODEM_DIG_ICG_MODEM_CODE` writer - need_des
pub type HP_MODEM_DIG_ICG_MODEM_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_ICG_MODEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 30:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_icg_modem_code(
        &mut self,
    ) -> HP_MODEM_DIG_ICG_MODEM_CODE_W<HP_MODEM_ICG_MODEM_SPEC> {
        HP_MODEM_DIG_ICG_MODEM_CODE_W::new(self, 30)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_MODEM_ICG_MODEM_SPEC;
impl crate::RegisterSpec for HP_MODEM_ICG_MODEM_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hp_modem_icg_modem::W`](W) writer structure
impl crate::Writable for HP_MODEM_ICG_MODEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_MODEM_ICG_MODEM to value 0
impl crate::Resettable for HP_MODEM_ICG_MODEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
