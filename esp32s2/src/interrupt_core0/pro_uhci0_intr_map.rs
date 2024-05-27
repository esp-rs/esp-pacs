///Register `PRO_UHCI0_INTR_MAP` reader
pub type R = crate::R<PRO_UHCI0_INTR_MAP_SPEC>;
///Register `PRO_UHCI0_INTR_MAP` writer
pub type W = crate::W<PRO_UHCI0_INTR_MAP_SPEC>;
///Field `PRO_UHCI0_INTR_MAP` reader - This register is used to map UHCI0_INTR interrupt signal to one of the CPU interrupts.
pub type PRO_UHCI0_INTR_MAP_R = crate::FieldReader;
///Field `PRO_UHCI0_INTR_MAP` writer - This register is used to map UHCI0_INTR interrupt signal to one of the CPU interrupts.
pub type PRO_UHCI0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - This register is used to map UHCI0_INTR interrupt signal to one of the CPU interrupts.
    #[inline(always)]
    pub fn pro_uhci0_intr_map(&self) -> PRO_UHCI0_INTR_MAP_R {
        PRO_UHCI0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_UHCI0_INTR_MAP")
            .field("pro_uhci0_intr_map", &self.pro_uhci0_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - This register is used to map UHCI0_INTR interrupt signal to one of the CPU interrupts.
    #[inline(always)]
    #[must_use]
    pub fn pro_uhci0_intr_map(&mut self) -> PRO_UHCI0_INTR_MAP_W<PRO_UHCI0_INTR_MAP_SPEC> {
        PRO_UHCI0_INTR_MAP_W::new(self, 0)
    }
}
/**UHCI0_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_uhci0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uhci0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_UHCI0_INTR_MAP_SPEC;
impl crate::RegisterSpec for PRO_UHCI0_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_uhci0_intr_map::R`](R) reader structure
impl crate::Readable for PRO_UHCI0_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`pro_uhci0_intr_map::W`](W) writer structure
impl crate::Writable for PRO_UHCI0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_UHCI0_INTR_MAP to value 0x10
impl crate::Resettable for PRO_UHCI0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
