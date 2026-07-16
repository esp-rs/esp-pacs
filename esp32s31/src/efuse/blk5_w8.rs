#[doc = "Register `BLK5_W8` reader"]
pub type R = crate::R<BLK5_W8_SPEC>;
#[doc = "Register `BLK5_W8` writer"]
pub type W = crate::W<BLK5_W8_SPEC>;
#[doc = "Field `BLOCK5_W8` reader - OTP block5 word8 data."]
pub type BLOCK5_W8_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK5_W8` writer - OTP block5 word8 data."]
pub type BLOCK5_W8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block5 word8 data."]
    #[inline(always)]
    pub fn block5_w8(&self) -> BLOCK5_W8_R {
        BLOCK5_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK5_W8")
            .field("block5_w8", &self.block5_w8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block5 word8 data."]
    #[inline(always)]
    pub fn block5_w8(&mut self) -> BLOCK5_W8_W<'_, BLK5_W8_SPEC> {
        BLOCK5_W8_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block5 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk5_w8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk5_w8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK5_W8_SPEC;
impl crate::RegisterSpec for BLK5_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk5_w8::R`](R) reader structure"]
impl crate::Readable for BLK5_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk5_w8::W`](W) writer structure"]
impl crate::Writable for BLK5_W8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK5_W8 to value 0"]
impl crate::Resettable for BLK5_W8_SPEC {}
