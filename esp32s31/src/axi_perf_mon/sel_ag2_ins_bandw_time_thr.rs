#[doc = "Register `SEL_AG2_INS_BANDW_TIME_THR` reader"]
pub type R = crate::R<SEL_AG2_INS_BANDW_TIME_THR_SPEC>;
#[doc = "Register `SEL_AG2_INS_BANDW_TIME_THR` writer"]
pub type W = crate::W<SEL_AG2_INS_BANDW_TIME_THR_SPEC>;
#[doc = "Field `SEL_AG2_RD_INS_BANDW_TIME_THR` reader - Read instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
pub type SEL_AG2_RD_INS_BANDW_TIME_THR_R = crate::FieldReader<u16>;
#[doc = "Field `SEL_AG2_RD_INS_BANDW_TIME_THR` writer - Read instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
pub type SEL_AG2_RD_INS_BANDW_TIME_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SEL_AG2_WR_INS_BANDW_TIME_THR` reader - Wirte instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
pub type SEL_AG2_WR_INS_BANDW_TIME_THR_R = crate::FieldReader<u16>;
#[doc = "Field `SEL_AG2_WR_INS_BANDW_TIME_THR` writer - Wirte instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
pub type SEL_AG2_WR_INS_BANDW_TIME_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
    #[inline(always)]
    pub fn sel_ag2_rd_ins_bandw_time_thr(&self) -> SEL_AG2_RD_INS_BANDW_TIME_THR_R {
        SEL_AG2_RD_INS_BANDW_TIME_THR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Wirte instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
    #[inline(always)]
    pub fn sel_ag2_wr_ins_bandw_time_thr(&self) -> SEL_AG2_WR_INS_BANDW_TIME_THR_R {
        SEL_AG2_WR_INS_BANDW_TIME_THR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG2_INS_BANDW_TIME_THR")
            .field(
                "sel_ag2_rd_ins_bandw_time_thr",
                &self.sel_ag2_rd_ins_bandw_time_thr(),
            )
            .field(
                "sel_ag2_wr_ins_bandw_time_thr",
                &self.sel_ag2_wr_ins_bandw_time_thr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Read instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
    #[inline(always)]
    pub fn sel_ag2_rd_ins_bandw_time_thr(
        &mut self,
    ) -> SEL_AG2_RD_INS_BANDW_TIME_THR_W<'_, SEL_AG2_INS_BANDW_TIME_THR_SPEC> {
        SEL_AG2_RD_INS_BANDW_TIME_THR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Wirte instantaneous bandwidth test time limit, counter data num in this time unit, and will fresh counter to count again"]
    #[inline(always)]
    pub fn sel_ag2_wr_ins_bandw_time_thr(
        &mut self,
    ) -> SEL_AG2_WR_INS_BANDW_TIME_THR_W<'_, SEL_AG2_INS_BANDW_TIME_THR_SPEC> {
        SEL_AG2_WR_INS_BANDW_TIME_THR_W::new(self, 16)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_ins_bandw_time_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_ins_bandw_time_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG2_INS_BANDW_TIME_THR_SPEC;
impl crate::RegisterSpec for SEL_AG2_INS_BANDW_TIME_THR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag2_ins_bandw_time_thr::R`](R) reader structure"]
impl crate::Readable for SEL_AG2_INS_BANDW_TIME_THR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag2_ins_bandw_time_thr::W`](W) writer structure"]
impl crate::Writable for SEL_AG2_INS_BANDW_TIME_THR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG2_INS_BANDW_TIME_THR to value 0"]
impl crate::Resettable for SEL_AG2_INS_BANDW_TIME_THR_SPEC {}
