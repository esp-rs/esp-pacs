#[doc = "Register `HFLBAddr` reader"]
pub type R = crate::R<HFLBADDR_SPEC>;
#[doc = "Register `HFLBAddr` writer"]
pub type W = crate::W<HFLBADDR_SPEC>;
#[doc = "Field `HFLBADDR_HFLBADDR` reader - The starting address of the Frame list. This register is used only for Isochronous and Interrupt Channels."]
pub type HFLBADDR_HFLBADDR_R = crate::FieldReader<u32>;
#[doc = "Field `HFLBADDR_HFLBADDR` writer - The starting address of the Frame list. This register is used only for Isochronous and Interrupt Channels."]
pub type HFLBADDR_HFLBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The starting address of the Frame list. This register is used only for Isochronous and Interrupt Channels."]
    #[inline(always)]
    pub fn hflbaddr_hflbaddr(&self) -> HFLBADDR_HFLBADDR_R {
        HFLBADDR_HFLBADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFLBAddr")
            .field("hflbaddr_hflbaddr", &self.hflbaddr_hflbaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The starting address of the Frame list. This register is used only for Isochronous and Interrupt Channels."]
    #[inline(always)]
    pub fn hflbaddr_hflbaddr(&mut self) -> HFLBADDR_HFLBADDR_W<'_, HFLBADDR_SPEC> {
        HFLBADDR_HFLBADDR_W::new(self, 0)
    }
}
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented as flops. This register holds the starting address of the Frame list information.\n\nYou can [`read`](crate::Reg::read) this register and get [`hflbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hflbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFLBADDR_SPEC;
impl crate::RegisterSpec for HFLBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hflbaddr::R`](R) reader structure"]
impl crate::Readable for HFLBADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hflbaddr::W`](W) writer structure"]
impl crate::Writable for HFLBADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFLBAddr to value 0"]
impl crate::Resettable for HFLBADDR_SPEC {}
