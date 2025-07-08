#[doc = "Register `COMB_PD_SITE0_UNIT3_VT2_CONF2` reader"]
pub type R = crate::R<COMB_PD_SITE0_UNIT3_VT2_CONF2_SPEC>;
#[doc = "Field `MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_R = crate::FieldReader;
#[doc = "Field `DELAY_OVF_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type DELAY_OVF_VT2_PD_SITE0_UNIT3_R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit3(&self) -> MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_R {
        MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - needs field desc"]
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site0_unit3(&self) -> DELAY_OVF_VT2_PD_SITE0_UNIT3_R {
        DELAY_OVF_VT2_PD_SITE0_UNIT3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site0_unit3(&self) -> TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_R {
        TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE0_UNIT3_VT2_CONF2")
            .field(
                "monitor_edg_mod_vt2_pd_site0_unit3",
                &self.monitor_edg_mod_vt2_pd_site0_unit3(),
            )
            .field(
                "delay_ovf_vt2_pd_site0_unit3",
                &self.delay_ovf_vt2_pd_site0_unit3(),
            )
            .field(
                "timing_err_cnt_o_vt2_pd_site0_unit3",
                &self.timing_err_cnt_o_vt2_pd_site0_unit3(),
            )
            .finish()
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site0_unit3_vt2_conf2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE0_UNIT3_VT2_CONF2_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE0_UNIT3_VT2_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site0_unit3_vt2_conf2::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE0_UNIT3_VT2_CONF2_SPEC {}
#[doc = "`reset()` method sets COMB_PD_SITE0_UNIT3_VT2_CONF2 to value 0"]
impl crate::Resettable for COMB_PD_SITE0_UNIT3_VT2_CONF2_SPEC {}
