#[doc = "Register `BLK2_WDATA7` reader"]
pub type R = crate::R<BLK2_WDATA7_SPEC>;
#[doc = "Register `BLK2_WDATA7` writer"]
pub type W = crate::W<BLK2_WDATA7_SPEC>;
#[doc = "Field `BLK2_DIN7` reader - "]
pub type BLK2_DIN7_R = crate::FieldReader<u32>;
#[doc = "Field `BLK2_DIN7` writer - "]
pub type BLK2_DIN7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk2_din7(&self) -> BLK2_DIN7_R {
        BLK2_DIN7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_WDATA7")
            .field("blk2_din7", &self.blk2_din7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn blk2_din7(&mut self) -> BLK2_DIN7_W<BLK2_WDATA7_SPEC> {
        BLK2_DIN7_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_WDATA7_SPEC;
impl crate::RegisterSpec for BLK2_WDATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_wdata7::R`](R) reader structure"]
impl crate::Readable for BLK2_WDATA7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk2_wdata7::W`](W) writer structure"]
impl crate::Writable for BLK2_WDATA7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK2_WDATA7 to value 0"]
impl crate::Resettable for BLK2_WDATA7_SPEC {
    const RESET_VALUE: u32 = 0;
}
