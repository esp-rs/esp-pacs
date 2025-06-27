#[doc = "Register `BLKSIZ` reader"]
pub type R = crate::R<BLKSIZ_SPEC>;
#[doc = "Register `BLKSIZ` writer"]
pub type W = crate::W<BLKSIZ_SPEC>;
#[doc = "Field `BLOCK_SIZE` reader - Block size."]
pub type BLOCK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_SIZE` writer - Block size."]
pub type BLOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Block size."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLKSIZ")
            .field("block_size", &self.block_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Block size."]
    #[inline(always)]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<BLKSIZ_SPEC> {
        BLOCK_SIZE_W::new(self, 0)
    }
}
#[doc = "Card data block size configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`blksiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLKSIZ_SPEC;
impl crate::RegisterSpec for BLKSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksiz::R`](R) reader structure"]
impl crate::Readable for BLKSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blksiz::W`](W) writer structure"]
impl crate::Writable for BLKSIZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLKSIZ to value 0x0200"]
impl crate::Resettable for BLKSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
