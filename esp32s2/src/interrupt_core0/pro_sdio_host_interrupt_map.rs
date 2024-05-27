#[doc = "Register `PRO_SDIO_HOST_INTERRUPT_MAP` reader"]
pub type R = crate::R<PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "Register `PRO_SDIO_HOST_INTERRUPT_MAP` writer"]
pub type W = crate::W<PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "Field `PRO_SDIO_HOST_INTERRUPT_MAP` reader - This register is used to map SDIO_HOST_INTERRUPT signal to one of the CPU interrupts."]
pub type PRO_SDIO_HOST_INTERRUPT_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_SDIO_HOST_INTERRUPT_MAP` writer - This register is used to map SDIO_HOST_INTERRUPT signal to one of the CPU interrupts."]
pub type PRO_SDIO_HOST_INTERRUPT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map SDIO_HOST_INTERRUPT signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_sdio_host_interrupt_map(&self) -> PRO_SDIO_HOST_INTERRUPT_MAP_R {
        PRO_SDIO_HOST_INTERRUPT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_SDIO_HOST_INTERRUPT_MAP")
            .field(
                "pro_sdio_host_interrupt_map",
                &self.pro_sdio_host_interrupt_map(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map SDIO_HOST_INTERRUPT signal to one of the CPU interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pro_sdio_host_interrupt_map(
        &mut self,
    ) -> PRO_SDIO_HOST_INTERRUPT_MAP_W<PRO_SDIO_HOST_INTERRUPT_MAP_SPEC> {
        PRO_SDIO_HOST_INTERRUPT_MAP_W::new(self, 0)
    }
}
#[doc = "SDIO_HOST_INTERRUPT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_sdio_host_interrupt_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_sdio_host_interrupt_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_SDIO_HOST_INTERRUPT_MAP_SPEC;
impl crate::RegisterSpec for PRO_SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_sdio_host_interrupt_map::R`](R) reader structure"]
impl crate::Readable for PRO_SDIO_HOST_INTERRUPT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_sdio_host_interrupt_map::W`](W) writer structure"]
impl crate::Writable for PRO_SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_SDIO_HOST_INTERRUPT_MAP to value 0x10"]
impl crate::Resettable for PRO_SDIO_HOST_INTERRUPT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
