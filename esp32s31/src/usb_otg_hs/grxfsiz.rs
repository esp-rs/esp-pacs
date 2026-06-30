#[doc = "Register `GRXFSIZ` reader"]
pub type R = crate::R<GRXFSIZ_SPEC>;
#[doc = "Register `GRXFSIZ` writer"]
pub type W = crate::W<GRXFSIZ_SPEC>;
#[doc = "Field `RXFDEP` reader - Mode: Host and Device RxFIFO Depth (RxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Rx Data FIFO Depth during configuration. If Enable Dynamic FIFO Sizing is selected in coreConsultant, these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing is selected in coreConsultant, you can write a new value in this field. Programmed values must not exceed the power-on value."]
pub type RXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `RXFDEP` writer - Mode: Host and Device RxFIFO Depth (RxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Rx Data FIFO Depth during configuration. If Enable Dynamic FIFO Sizing is selected in coreConsultant, these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing is selected in coreConsultant, you can write a new value in this field. Programmed values must not exceed the power-on value."]
pub type RXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Mode: Host and Device RxFIFO Depth (RxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Rx Data FIFO Depth during configuration. If Enable Dynamic FIFO Sizing is selected in coreConsultant, these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing is selected in coreConsultant, you can write a new value in this field. Programmed values must not exceed the power-on value."]
    #[inline(always)]
    pub fn rxfdep(&self) -> RXFDEP_R {
        RXFDEP_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXFSIZ")
            .field("rxfdep", &self.rxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Mode: Host and Device RxFIFO Depth (RxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Rx Data FIFO Depth during configuration. If Enable Dynamic FIFO Sizing is selected in coreConsultant, these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing is selected in coreConsultant, you can write a new value in this field. Programmed values must not exceed the power-on value."]
    #[inline(always)]
    pub fn rxfdep(&mut self) -> RXFDEP_W<'_, GRXFSIZ_SPEC> {
        RXFDEP_W::new(self, 0)
    }
}
#[doc = "The application can program the RAM size that must be allocated to the RxFIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXFSIZ_SPEC;
impl crate::RegisterSpec for GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfsiz::R`](R) reader structure"]
impl crate::Readable for GRXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure"]
impl crate::Writable for GRXFSIZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0400"]
impl crate::Resettable for GRXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
