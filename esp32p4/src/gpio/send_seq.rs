#[doc = "Register `SEND_SEQ` reader"]
pub type R = crate::R<SEND_SEQ_SPEC>;
#[doc = "Register `SEND_SEQ` writer"]
pub type W = crate::W<SEND_SEQ_SPEC>;
#[doc = "Field `SEND_SEQ` reader - High speed sdio pad bist send sequence"]
pub type SEND_SEQ_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_SEQ` writer - High speed sdio pad bist send sequence"]
pub type SEND_SEQ_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - High speed sdio pad bist send sequence"]
    #[inline(always)]
    pub fn send_seq(&self) -> SEND_SEQ_R {
        SEND_SEQ_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEND_SEQ")
            .field("send_seq", &self.send_seq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - High speed sdio pad bist send sequence"]
    #[inline(always)]
    pub fn send_seq(&mut self) -> SEND_SEQ_W<SEND_SEQ_SPEC> {
        SEND_SEQ_W::new(self, 0)
    }
}
#[doc = "High speed sdio pad bist send sequence\n\nYou can [`read`](crate::Reg::read) this register and get [`send_seq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`send_seq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEND_SEQ_SPEC;
impl crate::RegisterSpec for SEND_SEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`send_seq::R`](R) reader structure"]
impl crate::Readable for SEND_SEQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`send_seq::W`](W) writer structure"]
impl crate::Writable for SEND_SEQ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEND_SEQ to value 0x1234_5678"]
impl crate::Resettable for SEND_SEQ_SPEC {
    const RESET_VALUE: u32 = 0x1234_5678;
}
