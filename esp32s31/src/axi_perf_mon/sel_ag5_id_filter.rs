#[doc = "Register `SEL_AG5_ID_FILTER` reader"]
pub type R = crate::R<SEL_AG5_ID_FILTER_SPEC>;
#[doc = "Register `SEL_AG5_ID_FILTER` writer"]
pub type W = crate::W<SEL_AG5_ID_FILTER_SPEC>;
#[doc = "Field `SEL_AG5_RD_ID_FILTER` reader - Use with mask, no mask bits must same as filter bits, the pass filter"]
pub type SEL_AG5_RD_ID_FILTER_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_RD_ID_FILTER` writer - Use with mask, no mask bits must same as filter bits, the pass filter"]
pub type SEL_AG5_RD_ID_FILTER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL_AG5_WR_ID_FILTER` reader - Use with mask, no mask bits must same as filter bits, the pass filter"]
pub type SEL_AG5_WR_ID_FILTER_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_WR_ID_FILTER` writer - Use with mask, no mask bits must same as filter bits, the pass filter"]
pub type SEL_AG5_WR_ID_FILTER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Use with mask, no mask bits must same as filter bits, the pass filter"]
    #[inline(always)]
    pub fn sel_ag5_rd_id_filter(&self) -> SEL_AG5_RD_ID_FILTER_R {
        SEL_AG5_RD_ID_FILTER_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Use with mask, no mask bits must same as filter bits, the pass filter"]
    #[inline(always)]
    pub fn sel_ag5_wr_id_filter(&self) -> SEL_AG5_WR_ID_FILTER_R {
        SEL_AG5_WR_ID_FILTER_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG5_ID_FILTER")
            .field("sel_ag5_rd_id_filter", &self.sel_ag5_rd_id_filter())
            .field("sel_ag5_wr_id_filter", &self.sel_ag5_wr_id_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Use with mask, no mask bits must same as filter bits, the pass filter"]
    #[inline(always)]
    pub fn sel_ag5_rd_id_filter(&mut self) -> SEL_AG5_RD_ID_FILTER_W<'_, SEL_AG5_ID_FILTER_SPEC> {
        SEL_AG5_RD_ID_FILTER_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Use with mask, no mask bits must same as filter bits, the pass filter"]
    #[inline(always)]
    pub fn sel_ag5_wr_id_filter(&mut self) -> SEL_AG5_WR_ID_FILTER_W<'_, SEL_AG5_ID_FILTER_SPEC> {
        SEL_AG5_WR_ID_FILTER_W::new(self, 7)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_id_filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_id_filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG5_ID_FILTER_SPEC;
impl crate::RegisterSpec for SEL_AG5_ID_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag5_id_filter::R`](R) reader structure"]
impl crate::Readable for SEL_AG5_ID_FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag5_id_filter::W`](W) writer structure"]
impl crate::Writable for SEL_AG5_ID_FILTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG5_ID_FILTER to value 0"]
impl crate::Resettable for SEL_AG5_ID_FILTER_SPEC {}
