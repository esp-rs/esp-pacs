#[doc = "Register `BLK1_W5` reader"]
pub type R = crate::R<BLK1_W5_SPEC>;
#[doc = "Register `BLK1_W5` writer"]
pub type W = crate::W<BLK1_W5_SPEC>;
#[doc = "Field `BLOCK1_W5` reader - OTP block1 word5 data."]
pub type BLOCK1_W5_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK1_W5` writer - OTP block1 word5 data."]
pub type BLOCK1_W5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block1 word5 data."]
    #[inline(always)]
    pub fn block1_w5(&self) -> BLOCK1_W5_R {
        BLOCK1_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_W5")
            .field("block1_w5", &self.block1_w5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block1 word5 data."]
    #[inline(always)]
    pub fn block1_w5(&mut self) -> BLOCK1_W5_W<'_, BLK1_W5_SPEC> {
        BLOCK1_W5_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block1 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk1_w5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk1_w5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_W5_SPEC;
impl crate::RegisterSpec for BLK1_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_w5::R`](R) reader structure"]
impl crate::Readable for BLK1_W5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk1_w5::W`](W) writer structure"]
impl crate::Writable for BLK1_W5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK1_W5 to value 0"]
impl crate::Resettable for BLK1_W5_SPEC {}
