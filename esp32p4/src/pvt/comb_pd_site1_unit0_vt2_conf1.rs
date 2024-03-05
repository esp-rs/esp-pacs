#[doc = "Register `COMB_PD_SITE1_UNIT0_VT2_CONF1` reader"]
pub type R = crate::R<COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC>;
#[doc = "Register `COMB_PD_SITE1_UNIT0_VT2_CONF1` writer"]
pub type W = crate::W<COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC>;
#[doc = "Field `MONITOR_EN_VT2_PD_SITE1_UNIT0` reader - needs field desc"]
pub type MONITOR_EN_VT2_PD_SITE1_UNIT0_R = crate::BitReader;
#[doc = "Field `MONITOR_EN_VT2_PD_SITE1_UNIT0` writer - needs field desc"]
pub type MONITOR_EN_VT2_PD_SITE1_UNIT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0` writer - needs field desc"]
pub type TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY_LIMIT_VT2_PD_SITE1_UNIT0` reader - needs field desc"]
pub type DELAY_LIMIT_VT2_PD_SITE1_UNIT0_R = crate::FieldReader;
#[doc = "Field `DELAY_LIMIT_VT2_PD_SITE1_UNIT0` writer - needs field desc"]
pub type DELAY_LIMIT_VT2_PD_SITE1_UNIT0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELAY_NUM_O_VT2_PD_SITE1_UNIT0` reader - needs field desc"]
pub type DELAY_NUM_O_VT2_PD_SITE1_UNIT0_R = crate::FieldReader;
#[doc = "Field `TIMING_ERR_VT2_PD_SITE1_UNIT0` reader - needs field desc"]
pub type TIMING_ERR_VT2_PD_SITE1_UNIT0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site1_unit0(&self) -> MONITOR_EN_VT2_PD_SITE1_UNIT0_R {
        MONITOR_EN_VT2_PD_SITE1_UNIT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site1_unit0(&self) -> DELAY_LIMIT_VT2_PD_SITE1_UNIT0_R {
        DELAY_LIMIT_VT2_PD_SITE1_UNIT0_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - needs field desc"]
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site1_unit0(&self) -> DELAY_NUM_O_VT2_PD_SITE1_UNIT0_R {
        DELAY_NUM_O_VT2_PD_SITE1_UNIT0_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_vt2_pd_site1_unit0(&self) -> TIMING_ERR_VT2_PD_SITE1_UNIT0_R {
        TIMING_ERR_VT2_PD_SITE1_UNIT0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE1_UNIT0_VT2_CONF1")
            .field(
                "monitor_en_vt2_pd_site1_unit0",
                &format_args!("{}", self.monitor_en_vt2_pd_site1_unit0().bit()),
            )
            .field(
                "delay_limit_vt2_pd_site1_unit0",
                &format_args!("{}", self.delay_limit_vt2_pd_site1_unit0().bits()),
            )
            .field(
                "delay_num_o_vt2_pd_site1_unit0",
                &format_args!("{}", self.delay_num_o_vt2_pd_site1_unit0().bits()),
            )
            .field(
                "timing_err_vt2_pd_site1_unit0",
                &format_args!("{}", self.timing_err_vt2_pd_site1_unit0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn monitor_en_vt2_pd_site1_unit0(
        &mut self,
    ) -> MONITOR_EN_VT2_PD_SITE1_UNIT0_W<COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC> {
        MONITOR_EN_VT2_PD_SITE1_UNIT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn timing_err_cnt_clr_vt2_pd_site1_unit0(
        &mut self,
    ) -> TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_W<COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC> {
        TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_W::new(self, 1)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn delay_limit_vt2_pd_site1_unit0(
        &mut self,
    ) -> DELAY_LIMIT_VT2_PD_SITE1_UNIT0_W<COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC> {
        DELAY_LIMIT_VT2_PD_SITE1_UNIT0_W::new(self, 2)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt2_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt2_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site1_unit0_vt2_conf1::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site1_unit0_vt2_conf1::W`](W) writer structure"]
impl crate::Writable for COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMB_PD_SITE1_UNIT0_VT2_CONF1 to value 0x50"]
impl crate::Resettable for COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x50;
}
