#[doc = "Register `SEL_AG5_RD_ADDR_MASK2` reader"]
pub type R = crate::R<SEL_AG5_RD_ADDR_MASK2_SPEC>;
#[doc = "Register `SEL_AG5_RD_ADDR_MASK2` writer"]
pub type W = crate::W<SEL_AG5_RD_ADDR_MASK2_SPEC>;
#[doc = "Field `SEL_AG5_RD_ADDR_MASK2` reader - Read addr mask of addr filter function for sel agent, mask bit will not compare with addr"]
pub type SEL_AG5_RD_ADDR_MASK2_R = crate::FieldReader<u32>;
#[doc = "Field `SEL_AG5_RD_ADDR_MASK2` writer - Read addr mask of addr filter function for sel agent, mask bit will not compare with addr"]
pub type SEL_AG5_RD_ADDR_MASK2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read addr mask of addr filter function for sel agent, mask bit will not compare with addr"]
    #[inline(always)]
    pub fn sel_ag5_rd_addr_mask2(&self) -> SEL_AG5_RD_ADDR_MASK2_R {
        SEL_AG5_RD_ADDR_MASK2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG5_RD_ADDR_MASK2")
            .field("sel_ag5_rd_addr_mask2", &self.sel_ag5_rd_addr_mask2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Read addr mask of addr filter function for sel agent, mask bit will not compare with addr"]
    #[inline(always)]
    pub fn sel_ag5_rd_addr_mask2(
        &mut self,
    ) -> SEL_AG5_RD_ADDR_MASK2_W<'_, SEL_AG5_RD_ADDR_MASK2_SPEC> {
        SEL_AG5_RD_ADDR_MASK2_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_mask2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_mask2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG5_RD_ADDR_MASK2_SPEC;
impl crate::RegisterSpec for SEL_AG5_RD_ADDR_MASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag5_rd_addr_mask2::R`](R) reader structure"]
impl crate::Readable for SEL_AG5_RD_ADDR_MASK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag5_rd_addr_mask2::W`](W) writer structure"]
impl crate::Writable for SEL_AG5_RD_ADDR_MASK2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG5_RD_ADDR_MASK2 to value 0"]
impl crate::Resettable for SEL_AG5_RD_ADDR_MASK2_SPEC {}
