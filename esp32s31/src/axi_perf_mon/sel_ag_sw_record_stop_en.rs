#[doc = "Register `SEL_AG_SW_RECORD_STOP_EN` writer"]
pub type W = crate::W<SEL_AG_SW_RECORD_STOP_EN_SPEC>;
#[doc = "Field `SEL_AG0_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG0_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG1_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG1_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG2_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG2_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG3_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG3_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG4_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG4_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG5_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG5_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG6_SW_RECORD_STOP_EN` writer - SW use to stop event log function, record information will keep"]
pub type SEL_AG6_SW_RECORD_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEL_AG_SW_RECORD_STOP_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag0_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG0_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG0_SW_RECORD_STOP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag1_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG1_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG1_SW_RECORD_STOP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag2_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG2_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG2_SW_RECORD_STOP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag3_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG3_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG3_SW_RECORD_STOP_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag4_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG4_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG4_SW_RECORD_STOP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag5_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG5_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG5_SW_RECORD_STOP_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - SW use to stop event log function, record information will keep"]
    #[inline(always)]
    pub fn sel_ag6_sw_record_stop_en(
        &mut self,
    ) -> SEL_AG6_SW_RECORD_STOP_EN_W<'_, SEL_AG_SW_RECORD_STOP_EN_SPEC> {
        SEL_AG6_SW_RECORD_STOP_EN_W::new(self, 6)
    }
}
#[doc = "reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_sw_record_stop_en::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG_SW_RECORD_STOP_EN_SPEC;
impl crate::RegisterSpec for SEL_AG_SW_RECORD_STOP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sel_ag_sw_record_stop_en::W`](W) writer structure"]
impl crate::Writable for SEL_AG_SW_RECORD_STOP_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG_SW_RECORD_STOP_EN to value 0"]
impl crate::Resettable for SEL_AG_SW_RECORD_STOP_EN_SPEC {}
