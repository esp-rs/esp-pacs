#[doc = "Register `BLK0_BACKUP1_W8` reader"]
pub type R = crate::R<BLK0_BACKUP1_W8_SPEC>;
#[doc = "Register `BLK0_BACKUP1_W8` writer"]
pub type W = crate::W<BLK0_BACKUP1_W8_SPEC>;
#[doc = "Field `BLOCK0_BACKUP1_W8` reader - OTP block0 backup1 word8 data."]
pub type BLOCK0_BACKUP1_W8_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK0_BACKUP1_W8` writer - OTP block0 backup1 word8 data."]
pub type BLOCK0_BACKUP1_W8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block0 backup1 word8 data."]
    #[inline(always)]
    pub fn block0_backup1_w8(&self) -> BLOCK0_BACKUP1_W8_R {
        BLOCK0_BACKUP1_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_BACKUP1_W8")
            .field("block0_backup1_w8", &self.block0_backup1_w8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block0 backup1 word8 data."]
    #[inline(always)]
    pub fn block0_backup1_w8(&mut self) -> BLOCK0_BACKUP1_W8_W<'_, BLK0_BACKUP1_W8_SPEC> {
        BLOCK0_BACKUP1_W8_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block0 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk0_backup1_w8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk0_backup1_w8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_BACKUP1_W8_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP1_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_backup1_w8::R`](R) reader structure"]
impl crate::Readable for BLK0_BACKUP1_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_backup1_w8::W`](W) writer structure"]
impl crate::Writable for BLK0_BACKUP1_W8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK0_BACKUP1_W8 to value 0"]
impl crate::Resettable for BLK0_BACKUP1_W8_SPEC {}
