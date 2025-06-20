#[doc = "Register `COMB_PD_SITE3_UNIT1_VT1_CONF1` reader"]
pub type R = crate::R<COMB_PD_SITE3_UNIT1_VT1_CONF1_SPEC>;
#[doc = "Field `MONITOR_EN_VT1_PD_SITE3_UNIT1` reader - needs field desc"]
pub type MONITOR_EN_VT1_PD_SITE3_UNIT1_R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1` reader - needs field desc"]
pub type TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_R = crate::BitReader;
#[doc = "Field `DELAY_LIMIT_VT1_PD_SITE3_UNIT1` reader - needs field desc"]
pub type DELAY_LIMIT_VT1_PD_SITE3_UNIT1_R = crate::FieldReader;
#[doc = "Field `DELAY_NUM_O_VT1_PD_SITE3_UNIT1` reader - needs field desc"]
pub type DELAY_NUM_O_VT1_PD_SITE3_UNIT1_R = crate::FieldReader;
#[doc = "Field `TIMING_ERR_VT1_PD_SITE3_UNIT1` reader - needs field desc"]
pub type TIMING_ERR_VT1_PD_SITE3_UNIT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site3_unit1(&self) -> MONITOR_EN_VT1_PD_SITE3_UNIT1_R {
        MONITOR_EN_VT1_PD_SITE3_UNIT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_clr_vt1_pd_site3_unit1(&self) -> TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_R {
        TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site3_unit1(&self) -> DELAY_LIMIT_VT1_PD_SITE3_UNIT1_R {
        DELAY_LIMIT_VT1_PD_SITE3_UNIT1_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - needs field desc"]
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site3_unit1(&self) -> DELAY_NUM_O_VT1_PD_SITE3_UNIT1_R {
        DELAY_NUM_O_VT1_PD_SITE3_UNIT1_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_vt1_pd_site3_unit1(&self) -> TIMING_ERR_VT1_PD_SITE3_UNIT1_R {
        TIMING_ERR_VT1_PD_SITE3_UNIT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE3_UNIT1_VT1_CONF1")
            .field(
                "monitor_en_vt1_pd_site3_unit1",
                &self.monitor_en_vt1_pd_site3_unit1(),
            )
            .field(
                "timing_err_cnt_clr_vt1_pd_site3_unit1",
                &self.timing_err_cnt_clr_vt1_pd_site3_unit1(),
            )
            .field(
                "delay_limit_vt1_pd_site3_unit1",
                &self.delay_limit_vt1_pd_site3_unit1(),
            )
            .field(
                "delay_num_o_vt1_pd_site3_unit1",
                &self.delay_num_o_vt1_pd_site3_unit1(),
            )
            .field(
                "timing_err_vt1_pd_site3_unit1",
                &self.timing_err_vt1_pd_site3_unit1(),
            )
            .finish()
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site3_unit1_vt1_conf1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE3_UNIT1_VT1_CONF1_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE3_UNIT1_VT1_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site3_unit1_vt1_conf1::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE3_UNIT1_VT1_CONF1_SPEC {}
#[doc = "`reset()` method sets COMB_PD_SITE3_UNIT1_VT1_CONF1 to value 0x50"]
impl crate::Resettable for COMB_PD_SITE3_UNIT1_VT1_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x50;
}
