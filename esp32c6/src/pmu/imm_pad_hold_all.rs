///Register `IMM_PAD_HOLD_ALL` writer
pub type W = crate::W<IMM_PAD_HOLD_ALL_SPEC>;
///Field `TIE_HIGH_LP_PAD_HOLD_ALL` writer - need_des
pub type TIE_HIGH_LP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE_LOW_LP_PAD_HOLD_ALL` writer - need_des
pub type TIE_LOW_LP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE_HIGH_HP_PAD_HOLD_ALL` writer - need_des
pub type TIE_HIGH_HP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE_LOW_HP_PAD_HOLD_ALL` writer - need_des
pub type TIE_LOW_HP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_PAD_HOLD_ALL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tie_high_lp_pad_hold_all(
        &mut self,
    ) -> TIE_HIGH_LP_PAD_HOLD_ALL_W<IMM_PAD_HOLD_ALL_SPEC> {
        TIE_HIGH_LP_PAD_HOLD_ALL_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tie_low_lp_pad_hold_all(&mut self) -> TIE_LOW_LP_PAD_HOLD_ALL_W<IMM_PAD_HOLD_ALL_SPEC> {
        TIE_LOW_LP_PAD_HOLD_ALL_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tie_high_hp_pad_hold_all(
        &mut self,
    ) -> TIE_HIGH_HP_PAD_HOLD_ALL_W<IMM_PAD_HOLD_ALL_SPEC> {
        TIE_HIGH_HP_PAD_HOLD_ALL_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tie_low_hp_pad_hold_all(&mut self) -> TIE_LOW_HP_PAD_HOLD_ALL_W<IMM_PAD_HOLD_ALL_SPEC> {
        TIE_LOW_HP_PAD_HOLD_ALL_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_pad_hold_all::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMM_PAD_HOLD_ALL_SPEC;
impl crate::RegisterSpec for IMM_PAD_HOLD_ALL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`imm_pad_hold_all::W`](W) writer structure
impl crate::Writable for IMM_PAD_HOLD_ALL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMM_PAD_HOLD_ALL to value 0
impl crate::Resettable for IMM_PAD_HOLD_ALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
