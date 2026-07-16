#[doc = "Register `STATUS2` reader"]
pub type R = crate::R<STATUS2_SPEC>;
#[doc = "Register `STATUS2` writer"]
pub type W = crate::W<STATUS2_SPEC>;
#[doc = "Field `INTERRUPT` reader - The interrupt status of GPIO64 ~ GPIO66, can be configured by the software. - Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
pub type INTERRUPT_R = crate::FieldReader;
#[doc = "Field `INTERRUPT` writer - The interrupt status of GPIO64 ~ GPIO66, can be configured by the software. - Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
pub type INTERRUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The interrupt status of GPIO64 ~ GPIO66, can be configured by the software. - Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS2")
            .field("interrupt", &self.interrupt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - The interrupt status of GPIO64 ~ GPIO66, can be configured by the software. - Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<'_, STATUS2_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS2_SPEC;
impl crate::RegisterSpec for STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status2::R`](R) reader structure"]
impl crate::Readable for STATUS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status2::W`](W) writer structure"]
impl crate::Writable for STATUS2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS2 to value 0"]
impl crate::Resettable for STATUS2_SPEC {}
