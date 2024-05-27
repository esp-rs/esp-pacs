#[doc = "Register `BLK3_WDATA5` reader"]
pub type R = crate::R<BLK3_WDATA5_SPEC>;
#[doc = "Register `BLK3_WDATA5` writer"]
pub type W = crate::W<BLK3_WDATA5_SPEC>;
#[doc = "Field `BLK3_DIN5` reader - "]
pub type BLK3_DIN5_R = crate::FieldReader<u32>;
#[doc = "Field `BLK3_DIN5` writer - "]
pub type BLK3_DIN5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk3_din5(&self) -> BLK3_DIN5_R {
        BLK3_DIN5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_WDATA5")
            .field("blk3_din5", &self.blk3_din5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn blk3_din5(&mut self) -> BLK3_DIN5_W<BLK3_WDATA5_SPEC> {
        BLK3_DIN5_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_WDATA5_SPEC;
impl crate::RegisterSpec for BLK3_WDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_wdata5::R`](R) reader structure"]
impl crate::Readable for BLK3_WDATA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk3_wdata5::W`](W) writer structure"]
impl crate::Writable for BLK3_WDATA5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK3_WDATA5 to value 0"]
impl crate::Resettable for BLK3_WDATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
