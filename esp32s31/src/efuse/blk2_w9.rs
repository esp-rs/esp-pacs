#[doc = "Register `BLK2_W9` reader"]
pub type R = crate::R<BLK2_W9_SPEC>;
#[doc = "Register `BLK2_W9` writer"]
pub type W = crate::W<BLK2_W9_SPEC>;
#[doc = "Field `BLOCK2_W9` reader - OTP block2 word9 data."]
pub type BLOCK2_W9_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK2_W9` writer - OTP block2 word9 data."]
pub type BLOCK2_W9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block2 word9 data."]
    #[inline(always)]
    pub fn block2_w9(&self) -> BLOCK2_W9_R {
        BLOCK2_W9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_W9")
            .field("block2_w9", &self.block2_w9())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block2 word9 data."]
    #[inline(always)]
    pub fn block2_w9(&mut self) -> BLOCK2_W9_W<'_, BLK2_W9_SPEC> {
        BLOCK2_W9_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block2 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk2_w9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk2_w9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_W9_SPEC;
impl crate::RegisterSpec for BLK2_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_w9::R`](R) reader structure"]
impl crate::Readable for BLK2_W9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk2_w9::W`](W) writer structure"]
impl crate::Writable for BLK2_W9_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK2_W9 to value 0"]
impl crate::Resettable for BLK2_W9_SPEC {}
