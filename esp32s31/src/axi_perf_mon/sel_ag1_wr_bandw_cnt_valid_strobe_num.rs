#[doc = "Register `SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM` reader"]
pub type R = crate::R<SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "Register `SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM` writer"]
pub type W = crate::W<SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "Field `SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM` reader - Set this register to configure the time valid scaling multiplier"]
pub type SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM_R = crate::FieldReader;
#[doc = "Field `SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM` writer - Set this register to configure the time valid scaling multiplier"]
pub type SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM` reader - Set this register to configure the data valid scaling multiplier"]
pub type SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM_R = crate::FieldReader;
#[doc = "Field `SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM` writer - Set this register to configure the data valid scaling multiplier"]
pub type SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set this register to configure the time valid scaling multiplier"]
    #[inline(always)]
    pub fn sel_ag1_wr_bandw_time_cnt_valid_strobe_num(
        &self,
    ) -> SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM_R {
        SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Set this register to configure the data valid scaling multiplier"]
    #[inline(always)]
    pub fn sel_ag1_wr_bandw_data_cnt_valid_strobe_num(
        &self,
    ) -> SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM_R {
        SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM")
            .field(
                "sel_ag1_wr_bandw_time_cnt_valid_strobe_num",
                &self.sel_ag1_wr_bandw_time_cnt_valid_strobe_num(),
            )
            .field(
                "sel_ag1_wr_bandw_data_cnt_valid_strobe_num",
                &self.sel_ag1_wr_bandw_data_cnt_valid_strobe_num(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Set this register to configure the time valid scaling multiplier"]
    #[inline(always)]
    pub fn sel_ag1_wr_bandw_time_cnt_valid_strobe_num(
        &mut self,
    ) -> SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM_W<'_, SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>
    {
        SEL_AG1_WR_BANDW_TIME_CNT_VALID_STROBE_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Set this register to configure the data valid scaling multiplier"]
    #[inline(always)]
    pub fn sel_ag1_wr_bandw_data_cnt_valid_strobe_num(
        &mut self,
    ) -> SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM_W<'_, SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>
    {
        SEL_AG1_WR_BANDW_DATA_CNT_VALID_STROBE_NUM_W::new(self, 8)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_bandw_cnt_valid_strobe_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_bandw_cnt_valid_strobe_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC;
impl crate::RegisterSpec for SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag1_wr_bandw_cnt_valid_strobe_num::R`](R) reader structure"]
impl crate::Readable for SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag1_wr_bandw_cnt_valid_strobe_num::W`](W) writer structure"]
impl crate::Writable for SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM to value 0"]
impl crate::Resettable for SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC {}
