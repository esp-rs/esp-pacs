#[doc = "Register `OUTFIFO_PUSH` reader"]
pub type R = crate::R<OUTFIFO_PUSH_SPEC>;
#[doc = "Register `OUTFIFO_PUSH` writer"]
pub type W = crate::W<OUTFIFO_PUSH_SPEC>;
#[doc = "Field `OUTFIFO_WDATA` reader - APB out FIFO write data."]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - APB out FIFO write data."]
pub type OUTFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OUTFIFO_PUSH` reader - APB out FIFO push."]
pub type OUTFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH` writer - APB out FIFO push."]
pub type OUTFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - APB out FIFO write data."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - APB out FIFO push."]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_PUSH")
            .field("outfifo_wdata", &self.outfifo_wdata())
            .field("outfifo_push", &self.outfifo_push())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - APB out FIFO write data."]
    #[inline(always)]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<'_, OUTFIFO_PUSH_SPEC> {
        OUTFIFO_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 16 - APB out FIFO push."]
    #[inline(always)]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<'_, OUTFIFO_PUSH_SPEC> {
        OUTFIFO_PUSH_W::new(self, 16)
    }
}
#[doc = "APB out FIFO mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_push::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outfifo_push::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_PUSH_SPEC;
impl crate::RegisterSpec for OUTFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_push::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outfifo_push::W`](W) writer structure"]
impl crate::Writable for OUTFIFO_PUSH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTFIFO_PUSH to value 0"]
impl crate::Resettable for OUTFIFO_PUSH_SPEC {}
