#[doc = "Register `SEL_AG1_SW_RECORD_STOP_DATA_LIMIT` reader"]
pub type R = crate::R<SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "Register `SEL_AG1_SW_RECORD_STOP_DATA_LIMIT` writer"]
pub type W = crate::W<SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "Field `SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT` reader - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
pub type SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT` writer - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
pub type SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT` reader - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
pub type SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT` writer - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
pub type SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
    #[inline(always)]
    pub fn sel_ag1_sw_record_stop_rd_data_limit(&self) -> SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT_R {
        SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
    #[inline(always)]
    pub fn sel_ag1_sw_record_stop_wr_data_limit(&self) -> SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT_R {
        SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG1_SW_RECORD_STOP_DATA_LIMIT")
            .field(
                "sel_ag1_sw_record_stop_rd_data_limit",
                &self.sel_ag1_sw_record_stop_rd_data_limit(),
            )
            .field(
                "sel_ag1_sw_record_stop_wr_data_limit",
                &self.sel_ag1_sw_record_stop_wr_data_limit(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
    #[inline(always)]
    pub fn sel_ag1_sw_record_stop_rd_data_limit(
        &mut self,
    ) -> SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT_W<'_, SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC> {
        SEL_AG1_SW_RECORD_STOP_RD_DATA_LIMIT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read instantaneous bandwidth test data lower limit, when touch this limit, intr enable"]
    #[inline(always)]
    pub fn sel_ag1_sw_record_stop_wr_data_limit(
        &mut self,
    ) -> SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT_W<'_, SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC> {
        SEL_AG1_SW_RECORD_STOP_WR_DATA_LIMIT_W::new(self, 16)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_sw_record_stop_data_limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_sw_record_stop_data_limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC;
impl crate::RegisterSpec for SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag1_sw_record_stop_data_limit::R`](R) reader structure"]
impl crate::Readable for SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag1_sw_record_stop_data_limit::W`](W) writer structure"]
impl crate::Writable for SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG1_SW_RECORD_STOP_DATA_LIMIT to value 0"]
impl crate::Resettable for SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC {}
