///Register `COMB_PD_SITE1_UNIT1_VT2_CONF1` reader
pub type R = crate::R<COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC>;
///Register `COMB_PD_SITE1_UNIT1_VT2_CONF1` writer
pub type W = crate::W<COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC>;
///Field `MONITOR_EN_VT2_PD_SITE1_UNIT1` reader - needs field desc
pub type MONITOR_EN_VT2_PD_SITE1_UNIT1_R = crate::BitReader;
///Field `MONITOR_EN_VT2_PD_SITE1_UNIT1` writer - needs field desc
pub type MONITOR_EN_VT2_PD_SITE1_UNIT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1` writer - needs field desc
pub type TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DELAY_LIMIT_VT2_PD_SITE1_UNIT1` reader - needs field desc
pub type DELAY_LIMIT_VT2_PD_SITE1_UNIT1_R = crate::FieldReader;
///Field `DELAY_LIMIT_VT2_PD_SITE1_UNIT1` writer - needs field desc
pub type DELAY_LIMIT_VT2_PD_SITE1_UNIT1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DELAY_NUM_O_VT2_PD_SITE1_UNIT1` reader - needs field desc
pub type DELAY_NUM_O_VT2_PD_SITE1_UNIT1_R = crate::FieldReader;
///Field `TIMING_ERR_VT2_PD_SITE1_UNIT1` reader - needs field desc
pub type TIMING_ERR_VT2_PD_SITE1_UNIT1_R = crate::BitReader;
impl R {
    ///Bit 0 - needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site1_unit1(&self) -> MONITOR_EN_VT2_PD_SITE1_UNIT1_R {
        MONITOR_EN_VT2_PD_SITE1_UNIT1_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:9 - needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site1_unit1(&self) -> DELAY_LIMIT_VT2_PD_SITE1_UNIT1_R {
        DELAY_LIMIT_VT2_PD_SITE1_UNIT1_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    ///Bits 23:30 - needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site1_unit1(&self) -> DELAY_NUM_O_VT2_PD_SITE1_UNIT1_R {
        DELAY_NUM_O_VT2_PD_SITE1_UNIT1_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    ///Bit 31 - needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site1_unit1(&self) -> TIMING_ERR_VT2_PD_SITE1_UNIT1_R {
        TIMING_ERR_VT2_PD_SITE1_UNIT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE1_UNIT1_VT2_CONF1")
            .field(
                "monitor_en_vt2_pd_site1_unit1",
                &self.monitor_en_vt2_pd_site1_unit1(),
            )
            .field(
                "delay_limit_vt2_pd_site1_unit1",
                &self.delay_limit_vt2_pd_site1_unit1(),
            )
            .field(
                "delay_num_o_vt2_pd_site1_unit1",
                &self.delay_num_o_vt2_pd_site1_unit1(),
            )
            .field(
                "timing_err_vt2_pd_site1_unit1",
                &self.timing_err_vt2_pd_site1_unit1(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - needs field desc
    #[inline(always)]
    #[must_use]
    pub fn monitor_en_vt2_pd_site1_unit1(
        &mut self,
    ) -> MONITOR_EN_VT2_PD_SITE1_UNIT1_W<COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC> {
        MONITOR_EN_VT2_PD_SITE1_UNIT1_W::new(self, 0)
    }
    ///Bit 1 - needs field desc
    #[inline(always)]
    #[must_use]
    pub fn timing_err_cnt_clr_vt2_pd_site1_unit1(
        &mut self,
    ) -> TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_W<COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC> {
        TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_W::new(self, 1)
    }
    ///Bits 2:9 - needs field desc
    #[inline(always)]
    #[must_use]
    pub fn delay_limit_vt2_pd_site1_unit1(
        &mut self,
    ) -> DELAY_LIMIT_VT2_PD_SITE1_UNIT1_W<COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC> {
        DELAY_LIMIT_VT2_PD_SITE1_UNIT1_W::new(self, 2)
    }
}
/**needs desc

You can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt2_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt2_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`comb_pd_site1_unit1_vt2_conf1::R`](R) reader structure
impl crate::Readable for COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC {}
///`write(|w| ..)` method takes [`comb_pd_site1_unit1_vt2_conf1::W`](W) writer structure
impl crate::Writable for COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMB_PD_SITE1_UNIT1_VT2_CONF1 to value 0x50
impl crate::Resettable for COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x50;
}
