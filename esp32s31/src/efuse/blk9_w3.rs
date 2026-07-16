#[doc = "Register `BLK9_W3` reader"]
pub type R = crate::R<BLK9_W3_SPEC>;
#[doc = "Register `BLK9_W3` writer"]
pub type W = crate::W<BLK9_W3_SPEC>;
#[doc = "Field `BLOCK9_W3` reader - OTP block9 word3 data."]
pub type BLOCK9_W3_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK9_W3` writer - OTP block9 word3 data."]
pub type BLOCK9_W3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block9 word3 data."]
    #[inline(always)]
    pub fn block9_w3(&self) -> BLOCK9_W3_R {
        BLOCK9_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK9_W3")
            .field("block9_w3", &self.block9_w3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block9 word3 data."]
    #[inline(always)]
    pub fn block9_w3(&mut self) -> BLOCK9_W3_W<'_, BLK9_W3_SPEC> {
        BLOCK9_W3_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block9 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk9_w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk9_w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK9_W3_SPEC;
impl crate::RegisterSpec for BLK9_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk9_w3::R`](R) reader structure"]
impl crate::Readable for BLK9_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk9_w3::W`](W) writer structure"]
impl crate::Writable for BLK9_W3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK9_W3 to value 0"]
impl crate::Resettable for BLK9_W3_SPEC {}
