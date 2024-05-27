///Register `IMM_HP_APB_ICG` writer
pub type W = crate::W<IMM_HP_APB_ICG_SPEC>;
///Field `UPDATE_DIG_ICG_APB_EN` writer - need_des
pub type UPDATE_DIG_ICG_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_HP_APB_ICG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn update_dig_icg_apb_en(&mut self) -> UPDATE_DIG_ICG_APB_EN_W<IMM_HP_APB_ICG_SPEC> {
        UPDATE_DIG_ICG_APB_EN_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_hp_apb_icg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMM_HP_APB_ICG_SPEC;
impl crate::RegisterSpec for IMM_HP_APB_ICG_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`imm_hp_apb_icg::W`](W) writer structure
impl crate::Writable for IMM_HP_APB_ICG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMM_HP_APB_ICG to value 0
impl crate::Resettable for IMM_HP_APB_ICG_SPEC {
    const RESET_VALUE: u32 = 0;
}
