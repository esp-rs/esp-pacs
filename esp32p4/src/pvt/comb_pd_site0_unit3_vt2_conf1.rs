#[doc = "Register `COMB_PD_SITE0_UNIT3_VT2_CONF1` reader"]
pub type R = crate::R<COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC>;
#[doc = "Register `COMB_PD_SITE0_UNIT3_VT2_CONF1` writer"]
pub type W = crate::W<COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC>;
#[doc = "Field `MONITOR_EN_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type MONITOR_EN_VT2_PD_SITE0_UNIT3_R = crate::BitReader;
#[doc = "Field `MONITOR_EN_VT2_PD_SITE0_UNIT3` writer - needs field desc"]
pub type MONITOR_EN_VT2_PD_SITE0_UNIT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3` writer - needs field desc"]
pub type TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY_LIMIT_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type DELAY_LIMIT_VT2_PD_SITE0_UNIT3_R = crate::FieldReader;
#[doc = "Field `DELAY_LIMIT_VT2_PD_SITE0_UNIT3` writer - needs field desc"]
pub type DELAY_LIMIT_VT2_PD_SITE0_UNIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELAY_NUM_O_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type DELAY_NUM_O_VT2_PD_SITE0_UNIT3_R = crate::FieldReader;
#[doc = "Field `TIMING_ERR_VT2_PD_SITE0_UNIT3` reader - needs field desc"]
pub type TIMING_ERR_VT2_PD_SITE0_UNIT3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site0_unit3(&self) -> MONITOR_EN_VT2_PD_SITE0_UNIT3_R {
        MONITOR_EN_VT2_PD_SITE0_UNIT3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site0_unit3(&self) -> DELAY_LIMIT_VT2_PD_SITE0_UNIT3_R {
        DELAY_LIMIT_VT2_PD_SITE0_UNIT3_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - needs field desc"]
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site0_unit3(&self) -> DELAY_NUM_O_VT2_PD_SITE0_UNIT3_R {
        DELAY_NUM_O_VT2_PD_SITE0_UNIT3_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_vt2_pd_site0_unit3(&self) -> TIMING_ERR_VT2_PD_SITE0_UNIT3_R {
        TIMING_ERR_VT2_PD_SITE0_UNIT3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE0_UNIT3_VT2_CONF1")
            .field(
                "monitor_en_vt2_pd_site0_unit3",
                &self.monitor_en_vt2_pd_site0_unit3(),
            )
            .field(
                "delay_limit_vt2_pd_site0_unit3",
                &self.delay_limit_vt2_pd_site0_unit3(),
            )
            .field(
                "delay_num_o_vt2_pd_site0_unit3",
                &self.delay_num_o_vt2_pd_site0_unit3(),
            )
            .field(
                "timing_err_vt2_pd_site0_unit3",
                &self.timing_err_vt2_pd_site0_unit3(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site0_unit3(
        &mut self,
    ) -> MONITOR_EN_VT2_PD_SITE0_UNIT3_W<COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC> {
        MONITOR_EN_VT2_PD_SITE0_UNIT3_W::new(self, 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_clr_vt2_pd_site0_unit3(
        &mut self,
    ) -> TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_W<COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC> {
        TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_W::new(self, 1)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site0_unit3(
        &mut self,
    ) -> DELAY_LIMIT_VT2_PD_SITE0_UNIT3_W<COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC> {
        DELAY_LIMIT_VT2_PD_SITE0_UNIT3_W::new(self, 2)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site0_unit3_vt2_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt2_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site0_unit3_vt2_conf1::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site0_unit3_vt2_conf1::W`](W) writer structure"]
impl crate::Writable for COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMB_PD_SITE0_UNIT3_VT2_CONF1 to value 0x50"]
impl crate::Resettable for COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x50;
}
