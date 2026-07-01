#[doc = "Register `BLK3_W9` reader"]
pub type R = crate::R<BLK3_W9_SPEC>;
#[doc = "Register `BLK3_W9` writer"]
pub type W = crate::W<BLK3_W9_SPEC>;
#[doc = "Field `BLOCK3_W9` reader - OTP block3 word9 data."]
pub type BLOCK3_W9_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK3_W9` writer - OTP block3 word9 data."]
pub type BLOCK3_W9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block3 word9 data."]
    #[inline(always)]
    pub fn block3_w9(&self) -> BLOCK3_W9_R {
        BLOCK3_W9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_W9")
            .field("block3_w9", &self.block3_w9())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block3 word9 data."]
    #[inline(always)]
    pub fn block3_w9(&mut self) -> BLOCK3_W9_W<'_, BLK3_W9_SPEC> {
        BLOCK3_W9_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block3 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk3_w9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk3_w9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_W9_SPEC;
impl crate::RegisterSpec for BLK3_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_w9::R`](R) reader structure"]
impl crate::Readable for BLK3_W9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk3_w9::W`](W) writer structure"]
impl crate::Writable for BLK3_W9_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK3_W9 to value 0"]
impl crate::Resettable for BLK3_W9_SPEC {}
