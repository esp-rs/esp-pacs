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
    #[must_use]
    pub fn interrupt(&mut self) -> INTERRUPT_W<INTERRUPT_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {
    const RESET_VALUE: u32 = 0;
}
