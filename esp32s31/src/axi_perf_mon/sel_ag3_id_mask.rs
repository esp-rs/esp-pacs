#[doc = "Register `SEL_AG3_ID_MASK` reader"]
pub type R = crate::R<SEL_AG3_ID_MASK_SPEC>;
#[doc = "Register `SEL_AG3_ID_MASK` writer"]
pub type W = crate::W<SEL_AG3_ID_MASK_SPEC>;
#[doc = "Field `SEL_AG3_RD_ID_MASK` reader - Read id mask, ignore mask id bits"]
pub type SEL_AG3_RD_ID_MASK_R = crate::FieldReader;
#[doc = "Field `SEL_AG3_RD_ID_MASK` writer - Read id mask, ignore mask id bits"]
pub type SEL_AG3_RD_ID_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL_AG3_WR_ID_MASK` reader - Write id mask, ignore mask id bits"]
pub type SEL_AG3_WR_ID_MASK_R = crate::FieldReader;
#[doc = "Field `SEL_AG3_WR_ID_MASK` writer - Write id mask, ignore mask id bits"]
pub type SEL_AG3_WR_ID_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Read id mask, ignore mask id bits"]
    #[inline(always)]
    pub fn sel_ag3_rd_id_mask(&self) -> SEL_AG3_RD_ID_MASK_R {
        SEL_AG3_RD_ID_MASK_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Write id mask, ignore mask id bits"]
    #[inline(always)]
    pub fn sel_ag3_wr_id_mask(&self) -> SEL_AG3_WR_ID_MASK_R {
        SEL_AG3_WR_ID_MASK_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG3_ID_MASK")
            .field("sel_ag3_rd_id_mask", &self.sel_ag3_rd_id_mask())
            .field("sel_ag3_wr_id_mask", &self.sel_ag3_wr_id_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Read id mask, ignore mask id bits"]
    #[inline(always)]
    pub fn sel_ag3_rd_id_mask(&mut self) -> SEL_AG3_RD_ID_MASK_W<'_, SEL_AG3_ID_MASK_SPEC> {
        SEL_AG3_RD_ID_MASK_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Write id mask, ignore mask id bits"]
    #[inline(always)]
    pub fn sel_ag3_wr_id_mask(&mut self) -> SEL_AG3_WR_ID_MASK_W<'_, SEL_AG3_ID_MASK_SPEC> {
        SEL_AG3_WR_ID_MASK_W::new(self, 7)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_id_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_id_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG3_ID_MASK_SPEC;
impl crate::RegisterSpec for SEL_AG3_ID_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag3_id_mask::R`](R) reader structure"]
impl crate::Readable for SEL_AG3_ID_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag3_id_mask::W`](W) writer structure"]
impl crate::Writable for SEL_AG3_ID_MASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG3_ID_MASK to value 0"]
impl crate::Resettable for SEL_AG3_ID_MASK_SPEC {}
