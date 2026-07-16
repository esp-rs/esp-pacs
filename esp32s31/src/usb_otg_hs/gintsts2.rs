#[doc = "Register `GINTSTS2` reader"]
pub type R = crate::R<GINTSTS2_SPEC>;
#[doc = "Register `GINTSTS2` writer"]
pub type W = crate::W<GINTSTS2_SPEC>;
#[doc = "Field `GINTSTS2` reader - Resvered"]
pub type GINTSTS2_R = crate::FieldReader<u32>;
#[doc = "Field `GINTSTS2` writer - Resvered"]
pub type GINTSTS2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Resvered"]
    #[inline(always)]
    pub fn gintsts2(&self) -> GINTSTS2_R {
        GINTSTS2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS2")
            .field("gintsts2", &self.gintsts2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Resvered"]
    #[inline(always)]
    pub fn gintsts2(&mut self) -> GINTSTS2_W<'_, GINTSTS2_SPEC> {
        GINTSTS2_W::new(self, 0)
    }
}
#[doc = "This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The application must clear the GINTSTS2 register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS2_SPEC;
impl crate::RegisterSpec for GINTSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts2::R`](R) reader structure"]
impl crate::Readable for GINTSTS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts2::W`](W) writer structure"]
impl crate::Writable for GINTSTS2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTSTS2 to value 0"]
impl crate::Resettable for GINTSTS2_SPEC {}
