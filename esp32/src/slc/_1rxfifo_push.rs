#[doc = "Register `_1RXFIFO_PUSH` reader"]
pub type R = crate::R<_1RXFIFO_PUSH_SPEC>;
#[doc = "Register `_1RXFIFO_PUSH` writer"]
pub type W = crate::W<_1RXFIFO_PUSH_SPEC>;
#[doc = "Field `SLC1_RXFIFO_WDATA` reader - "]
pub type SLC1_RXFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_RXFIFO_WDATA` writer - "]
pub type SLC1_RXFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SLC1_RXFIFO_PUSH` reader - "]
pub type SLC1_RXFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `SLC1_RXFIFO_PUSH` writer - "]
pub type SLC1_RXFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc1_rxfifo_wdata(&self) -> SLC1_RXFIFO_WDATA_R {
        SLC1_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rxfifo_push(&self) -> SLC1_RXFIFO_PUSH_R {
        SLC1_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1RXFIFO_PUSH")
            .field("slc1_rxfifo_wdata", &self.slc1_rxfifo_wdata())
            .field("slc1_rxfifo_push", &self.slc1_rxfifo_push())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc1_rxfifo_wdata(&mut self) -> SLC1_RXFIFO_WDATA_W<_1RXFIFO_PUSH_SPEC> {
        SLC1_RXFIFO_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rxfifo_push(&mut self) -> SLC1_RXFIFO_PUSH_W<_1RXFIFO_PUSH_SPEC> {
        SLC1_RXFIFO_PUSH_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_1rxfifo_push::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1rxfifo_push::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for _1RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1rxfifo_push::R`](R) reader structure"]
impl crate::Readable for _1RXFIFO_PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_1rxfifo_push::W`](W) writer structure"]
impl crate::Writable for _1RXFIFO_PUSH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _1RXFIFO_PUSH to value 0"]
impl crate::Resettable for _1RXFIFO_PUSH_SPEC {}
