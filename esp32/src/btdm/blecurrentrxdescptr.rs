#[doc = "Register `BLECURRENTRXDESCPTR` reader"]
pub type R = crate::R<BLECURRENTRXDESCPTR_SPEC>;
#[doc = "Register `BLECURRENTRXDESCPTR` writer"]
pub type W = crate::W<BLECURRENTRXDESCPTR_SPEC>;
#[doc = "Field `CURRENTRXDESCPTR` reader - Pointer that determines the starting point of the RX Buffer Chained List"]
pub type CURRENTRXDESCPTR_R = crate::FieldReader<u16>;
#[doc = "Field `CURRENTRXDESCPTR` writer - Pointer that determines the starting point of the RX Buffer Chained List"]
pub type CURRENTRXDESCPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ETPTR` reader - Pointer that determines the starting point of the Exchange Table"]
pub type ETPTR_R = crate::FieldReader<u16>;
#[doc = "Field `ETPTR` writer - Pointer that determines the starting point of the Exchange Table"]
pub type ETPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Pointer that determines the starting point of the RX Buffer Chained List"]
    #[inline(always)]
    pub fn currentrxdescptr(&self) -> CURRENTRXDESCPTR_R {
        CURRENTRXDESCPTR_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Pointer that determines the starting point of the Exchange Table"]
    #[inline(always)]
    pub fn etptr(&self) -> ETPTR_R {
        ETPTR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLECURRENTRXDESCPTR")
            .field("currentrxdescptr", &self.currentrxdescptr())
            .field("etptr", &self.etptr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Pointer that determines the starting point of the RX Buffer Chained List"]
    #[inline(always)]
    pub fn currentrxdescptr(&mut self) -> CURRENTRXDESCPTR_W<'_, BLECURRENTRXDESCPTR_SPEC> {
        CURRENTRXDESCPTR_W::new(self, 0)
    }
    #[doc = "Bits 16:30 - Pointer that determines the starting point of the Exchange Table"]
    #[inline(always)]
    pub fn etptr(&mut self) -> ETPTR_W<'_, BLECURRENTRXDESCPTR_SPEC> {
        ETPTR_W::new(self, 16)
    }
}
#[doc = "BLE RX descriptor pointer for the receive buffer chained list\n\nYou can [`read`](crate::Reg::read) this register and get [`blecurrentrxdescptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blecurrentrxdescptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLECURRENTRXDESCPTR_SPEC;
impl crate::RegisterSpec for BLECURRENTRXDESCPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blecurrentrxdescptr::R`](R) reader structure"]
impl crate::Readable for BLECURRENTRXDESCPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blecurrentrxdescptr::W`](W) writer structure"]
impl crate::Writable for BLECURRENTRXDESCPTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLECURRENTRXDESCPTR to value 0"]
impl crate::Resettable for BLECURRENTRXDESCPTR_SPEC {}
