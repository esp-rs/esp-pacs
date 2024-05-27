#[doc = "Register `COMB_PD_SITE1_UNIT2_VT2_CONF2` reader"]
pub type R = crate::R<COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC>;
#[doc = "Register `COMB_PD_SITE1_UNIT2_VT2_CONF2` writer"]
pub type W = crate::W<COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC>;
#[doc = "Field `MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2` reader - needs field desc"]
pub type MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_R = crate::FieldReader;
#[doc = "Field `MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2` writer - needs field desc"]
pub type MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_OVF_VT2_PD_SITE1_UNIT2` reader - needs field desc"]
pub type DELAY_OVF_VT2_PD_SITE1_UNIT2_R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2` reader - needs field desc"]
pub type TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site1_unit2(&self) -> MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_R {
        MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - needs field desc"]
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site1_unit2(&self) -> DELAY_OVF_VT2_PD_SITE1_UNIT2_R {
        DELAY_OVF_VT2_PD_SITE1_UNIT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site1_unit2(&self) -> TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_R {
        TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE1_UNIT2_VT2_CONF2")
            .field(
                "monitor_edg_mod_vt2_pd_site1_unit2",
                &self.monitor_edg_mod_vt2_pd_site1_unit2(),
            )
            .field(
                "delay_ovf_vt2_pd_site1_unit2",
                &self.delay_ovf_vt2_pd_site1_unit2(),
            )
            .field(
                "timing_err_cnt_o_vt2_pd_site1_unit2",
                &self.timing_err_cnt_o_vt2_pd_site1_unit2(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn monitor_edg_mod_vt2_pd_site1_unit2(
        &mut self,
    ) -> MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_W<COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC> {
        MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt2_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt2_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site1_unit2_vt2_conf2::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site1_unit2_vt2_conf2::W`](W) writer structure"]
impl crate::Writable for COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMB_PD_SITE1_UNIT2_VT2_CONF2 to value 0"]
impl crate::Resettable for COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC {
    const RESET_VALUE: u32 = 0;
}
