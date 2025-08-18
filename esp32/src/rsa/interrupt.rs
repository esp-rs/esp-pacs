#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<INTERRUPT_SPEC>;
#[doc = "Register `INTERRUPT` writer"]
pub type W = crate::W<INTERRUPT_SPEC>;
#[doc = "Field `INTERRUPT` reader - RSA interrupt status register. Will read 1 once an operation has completed."]
pub type INTERRUPT_R = crate::BitReader;
#[doc = "Field `INTERRUPT` writer - RSA interrupt status register. Will read 1 once an operation has completed."]
pub type INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RSA interrupt status register. Will read 1 once an operation has completed."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT")
            .field("interrupt", &self.interrupt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RSA interrupt status register. Will read 1 once an operation has completed."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<'_, INTERRUPT_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {}
