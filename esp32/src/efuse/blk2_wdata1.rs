#[doc = "Register `BLK2_WDATA1` reader"]
pub type R = crate::R<BLK2_WDATA1_SPEC>;
#[doc = "Register `BLK2_WDATA1` writer"]
pub type W = crate::W<BLK2_WDATA1_SPEC>;
#[doc = "Field `BLK2_DIN1` reader - "]
pub type BLK2_DIN1_R = crate::FieldReader<u32>;
#[doc = "Field `BLK2_DIN1` writer - "]
pub type BLK2_DIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk2_din1(&self) -> BLK2_DIN1_R {
        BLK2_DIN1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_WDATA1")
            .field("blk2_din1", &self.blk2_din1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk2_din1(&mut self) -> BLK2_DIN1_W<BLK2_WDATA1_SPEC> {
        BLK2_DIN1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk2_wdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk2_wdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_WDATA1_SPEC;
impl crate::RegisterSpec for BLK2_WDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_wdata1::R`](R) reader structure"]
impl crate::Readable for BLK2_WDATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk2_wdata1::W`](W) writer structure"]
impl crate::Writable for BLK2_WDATA1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK2_WDATA1 to value 0"]
impl crate::Resettable for BLK2_WDATA1_SPEC {}
