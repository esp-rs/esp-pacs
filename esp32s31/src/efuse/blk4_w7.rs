#[doc = "Register `BLK4_W7` reader"]
pub type R = crate::R<BLK4_W7_SPEC>;
#[doc = "Register `BLK4_W7` writer"]
pub type W = crate::W<BLK4_W7_SPEC>;
#[doc = "Field `BLOCK4_W7` reader - OTP block4 word7 data."]
pub type BLOCK4_W7_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK4_W7` writer - OTP block4 word7 data."]
pub type BLOCK4_W7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block4 word7 data."]
    #[inline(always)]
    pub fn block4_w7(&self) -> BLOCK4_W7_R {
        BLOCK4_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK4_W7")
            .field("block4_w7", &self.block4_w7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block4 word7 data."]
    #[inline(always)]
    pub fn block4_w7(&mut self) -> BLOCK4_W7_W<'_, BLK4_W7_SPEC> {
        BLOCK4_W7_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block4 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk4_w7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk4_w7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK4_W7_SPEC;
impl crate::RegisterSpec for BLK4_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk4_w7::R`](R) reader structure"]
impl crate::Readable for BLK4_W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk4_w7::W`](W) writer structure"]
impl crate::Writable for BLK4_W7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK4_W7 to value 0"]
impl crate::Resettable for BLK4_W7_SPEC {}
