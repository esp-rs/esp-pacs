#[doc = "Register `COMB_PD_SITE0_UNIT1_VT0_CONF2` reader"]
pub type R = crate::R<COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC>;
#[doc = "Register `COMB_PD_SITE0_UNIT1_VT0_CONF2` writer"]
pub type W = crate::W<COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC>;
#[doc = "Field `MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1` reader - needs field desc"]
pub type MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_R = crate::FieldReader;
#[doc = "Field `MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1` writer - needs field desc"]
pub type MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_OVF_VT0_PD_SITE0_UNIT1` reader - needs field desc"]
pub type DELAY_OVF_VT0_PD_SITE0_UNIT1_R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1` reader - needs field desc"]
pub type TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site0_unit1(&self) -> MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_R {
        MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - needs field desc"]
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site0_unit1(&self) -> DELAY_OVF_VT0_PD_SITE0_UNIT1_R {
        DELAY_OVF_VT0_PD_SITE0_UNIT1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site0_unit1(&self) -> TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_R {
        TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE0_UNIT1_VT0_CONF2")
            .field(
                "monitor_edg_mod_vt0_pd_site0_unit1",
                &self.monitor_edg_mod_vt0_pd_site0_unit1(),
            )
            .field(
                "delay_ovf_vt0_pd_site0_unit1",
                &self.delay_ovf_vt0_pd_site0_unit1(),
            )
            .field(
                "timing_err_cnt_o_vt0_pd_site0_unit1",
                &self.timing_err_cnt_o_vt0_pd_site0_unit1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site0_unit1(
        &mut self,
    ) -> MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_W<COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC> {
        MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site0_unit1_vt0_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt0_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site0_unit1_vt0_conf2::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site0_unit1_vt0_conf2::W`](W) writer structure"]
impl crate::Writable for COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMB_PD_SITE0_UNIT1_VT0_CONF2 to value 0"]
impl crate::Resettable for COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC {}
