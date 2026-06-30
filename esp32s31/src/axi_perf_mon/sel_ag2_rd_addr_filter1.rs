#[doc = "Register `SEL_AG2_RD_ADDR_FILTER1` reader"]
pub type R = crate::R<SEL_AG2_RD_ADDR_FILTER1_SPEC>;
#[doc = "Register `SEL_AG2_RD_ADDR_FILTER1` writer"]
pub type W = crate::W<SEL_AG2_RD_ADDR_FILTER1_SPEC>;
#[doc = "Field `SEL_AG2_RD_ADDR_FILTER1` reader - Read addr filter of addr filter function for sel agent, no mask bit in addr will compare with addr filter, if compare result same will pass filter"]
pub type SEL_AG2_RD_ADDR_FILTER1_R = crate::FieldReader<u32>;
#[doc = "Field `SEL_AG2_RD_ADDR_FILTER1` writer - Read addr filter of addr filter function for sel agent, no mask bit in addr will compare with addr filter, if compare result same will pass filter"]
pub type SEL_AG2_RD_ADDR_FILTER1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read addr filter of addr filter function for sel agent, no mask bit in addr will compare with addr filter, if compare result same will pass filter"]
    #[inline(always)]
    pub fn sel_ag2_rd_addr_filter1(&self) -> SEL_AG2_RD_ADDR_FILTER1_R {
        SEL_AG2_RD_ADDR_FILTER1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG2_RD_ADDR_FILTER1")
            .field("sel_ag2_rd_addr_filter1", &self.sel_ag2_rd_addr_filter1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Read addr filter of addr filter function for sel agent, no mask bit in addr will compare with addr filter, if compare result same will pass filter"]
    #[inline(always)]
    pub fn sel_ag2_rd_addr_filter1(
        &mut self,
    ) -> SEL_AG2_RD_ADDR_FILTER1_W<'_, SEL_AG2_RD_ADDR_FILTER1_SPEC> {
        SEL_AG2_RD_ADDR_FILTER1_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_filter1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_filter1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG2_RD_ADDR_FILTER1_SPEC;
impl crate::RegisterSpec for SEL_AG2_RD_ADDR_FILTER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag2_rd_addr_filter1::R`](R) reader structure"]
impl crate::Readable for SEL_AG2_RD_ADDR_FILTER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag2_rd_addr_filter1::W`](W) writer structure"]
impl crate::Writable for SEL_AG2_RD_ADDR_FILTER1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG2_RD_ADDR_FILTER1 to value 0"]
impl crate::Resettable for SEL_AG2_RD_ADDR_FILTER1_SPEC {}
