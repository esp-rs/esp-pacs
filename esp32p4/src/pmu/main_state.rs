#[doc = "Register `MAIN_STATE` reader"]
pub type R = crate::R<MAIN_STATE_SPEC>;
#[doc = "Register `MAIN_STATE` writer"]
pub type W = crate::W<MAIN_STATE_SPEC>;
#[doc = "Field `ENABLE_CALI_PMU_CNTL` reader - need_des"]
pub type ENABLE_CALI_PMU_CNTL_R = crate::BitReader;
#[doc = "Field `ENABLE_CALI_PMU_CNTL` writer - need_des"]
pub type ENABLE_CALI_PMU_CNTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_MAIN_LAST_ST_STATE` reader - need_des"]
pub type PMU_MAIN_LAST_ST_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_MAIN_TAR_ST_STATE` reader - need_des"]
pub type PMU_MAIN_TAR_ST_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_MAIN_CUR_ST_STATE` reader - need_des"]
pub type PMU_MAIN_CUR_ST_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn enable_cali_pmu_cntl(&self) -> ENABLE_CALI_PMU_CNTL_R {
        ENABLE_CALI_PMU_CNTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 11:17 - need_des"]
    #[inline(always)]
    pub fn pmu_main_last_st_state(&self) -> PMU_MAIN_LAST_ST_STATE_R {
        PMU_MAIN_LAST_ST_STATE_R::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 18:24 - need_des"]
    #[inline(always)]
    pub fn pmu_main_tar_st_state(&self) -> PMU_MAIN_TAR_ST_STATE_R {
        PMU_MAIN_TAR_ST_STATE_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - need_des"]
    #[inline(always)]
    pub fn pmu_main_cur_st_state(&self) -> PMU_MAIN_CUR_ST_STATE_R {
        PMU_MAIN_CUR_ST_STATE_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAIN_STATE")
            .field("enable_cali_pmu_cntl", &self.enable_cali_pmu_cntl())
            .field("pmu_main_last_st_state", &self.pmu_main_last_st_state())
            .field("pmu_main_tar_st_state", &self.pmu_main_tar_st_state())
            .field("pmu_main_cur_st_state", &self.pmu_main_cur_st_state())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn enable_cali_pmu_cntl(&mut self) -> ENABLE_CALI_PMU_CNTL_W<'_, MAIN_STATE_SPEC> {
        ENABLE_CALI_PMU_CNTL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`main_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAIN_STATE_SPEC;
impl crate::RegisterSpec for MAIN_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`main_state::R`](R) reader structure"]
impl crate::Readable for MAIN_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`main_state::W`](W) writer structure"]
impl crate::Writable for MAIN_STATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAIN_STATE to value 0x0810_0801"]
impl crate::Resettable for MAIN_STATE_SPEC {
    const RESET_VALUE: u32 = 0x0810_0801;
}
