///Register `PWR_STATE` reader
pub type R = crate::R<PWR_STATE_SPEC>;
///Field `PMU_BACKUP_ST_STATE` reader - need_des
pub type PMU_BACKUP_ST_STATE_R = crate::FieldReader;
///Field `PMU_LP_PWR_ST_STATE` reader - need_des
pub type PMU_LP_PWR_ST_STATE_R = crate::FieldReader;
///Field `PMU_HP_PWR_ST_STATE` reader - need_des
pub type PMU_HP_PWR_ST_STATE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 13:17 - need_des
    #[inline(always)]
    pub fn pmu_backup_st_state(&self) -> PMU_BACKUP_ST_STATE_R {
        PMU_BACKUP_ST_STATE_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    ///Bits 18:22 - need_des
    #[inline(always)]
    pub fn pmu_lp_pwr_st_state(&self) -> PMU_LP_PWR_ST_STATE_R {
        PMU_LP_PWR_ST_STATE_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 23:31 - need_des
    #[inline(always)]
    pub fn pmu_hp_pwr_st_state(&self) -> PMU_HP_PWR_ST_STATE_R {
        PMU_HP_PWR_ST_STATE_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_STATE")
            .field("pmu_backup_st_state", &self.pmu_backup_st_state())
            .field("pmu_lp_pwr_st_state", &self.pmu_lp_pwr_st_state())
            .field("pmu_hp_pwr_st_state", &self.pmu_hp_pwr_st_state())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`pwr_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PWR_STATE_SPEC;
impl crate::RegisterSpec for PWR_STATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pwr_state::R`](R) reader structure
impl crate::Readable for PWR_STATE_SPEC {}
///`reset()` method sets PWR_STATE to value 0x0080_2000
impl crate::Resettable for PWR_STATE_SPEC {
    const RESET_VALUE: u32 = 0x0080_2000;
}
