#[doc = "Register `BLK7_W4` reader"]
pub type R = crate::R<BLK7_W4_SPEC>;
#[doc = "Register `BLK7_W4` writer"]
pub type W = crate::W<BLK7_W4_SPEC>;
#[doc = "Field `BLOCK7_W4` reader - OTP block7 word4 data."]
pub type BLOCK7_W4_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK7_W4` writer - OTP block7 word4 data."]
pub type BLOCK7_W4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block7 word4 data."]
    #[inline(always)]
    pub fn block7_w4(&self) -> BLOCK7_W4_R {
        BLOCK7_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W4")
            .field("block7_w4", &self.block7_w4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block7 word4 data."]
    #[inline(always)]
    pub fn block7_w4(&mut self) -> BLOCK7_W4_W<'_, BLK7_W4_SPEC> {
        BLOCK7_W4_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block7 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk7_w4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk7_w4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK7_W4_SPEC;
impl crate::RegisterSpec for BLK7_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk7_w4::R`](R) reader structure"]
impl crate::Readable for BLK7_W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk7_w4::W`](W) writer structure"]
impl crate::Writable for BLK7_W4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK7_W4 to value 0"]
impl crate::Resettable for BLK7_W4_SPEC {}
