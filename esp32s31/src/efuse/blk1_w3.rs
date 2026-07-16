#[doc = "Register `BLK1_W3` reader"]
pub type R = crate::R<BLK1_W3_SPEC>;
#[doc = "Register `BLK1_W3` writer"]
pub type W = crate::W<BLK1_W3_SPEC>;
#[doc = "Field `BLOCK1_W3` reader - OTP block1 word3 data."]
pub type BLOCK1_W3_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK1_W3` writer - OTP block1 word3 data."]
pub type BLOCK1_W3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block1 word3 data."]
    #[inline(always)]
    pub fn block1_w3(&self) -> BLOCK1_W3_R {
        BLOCK1_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_W3")
            .field("block1_w3", &self.block1_w3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block1 word3 data."]
    #[inline(always)]
    pub fn block1_w3(&mut self) -> BLOCK1_W3_W<'_, BLK1_W3_SPEC> {
        BLOCK1_W3_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block1 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk1_w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk1_w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_W3_SPEC;
impl crate::RegisterSpec for BLK1_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_w3::R`](R) reader structure"]
impl crate::Readable for BLK1_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk1_w3::W`](W) writer structure"]
impl crate::Writable for BLK1_W3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK1_W3 to value 0"]
impl crate::Resettable for BLK1_W3_SPEC {}
