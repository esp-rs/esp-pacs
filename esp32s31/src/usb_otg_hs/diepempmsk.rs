#[doc = "Register `DIEPEMPMSK` reader"]
pub type R = crate::R<DIEPEMPMSK_SPEC>;
#[doc = "Register `DIEPEMPMSK` writer"]
pub type W = crate::W<DIEPEMPMSK_SPEC>;
#[doc = "Field `INEPTXFEMPMSK` reader - IN EP Tx FIFO Empty Interrupt Mask Bits (InEpTxfEmpMsk) These bits acts as mask bits for DIEPINTn.TxFEmp interrupt, one bit per IN Endpoint: Bit 0 for IN EP 0, bit 15 for IN EP 15"]
pub type INEPTXFEMPMSK_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFEMPMSK` writer - IN EP Tx FIFO Empty Interrupt Mask Bits (InEpTxfEmpMsk) These bits acts as mask bits for DIEPINTn.TxFEmp interrupt, one bit per IN Endpoint: Bit 0 for IN EP 0, bit 15 for IN EP 15"]
pub type INEPTXFEMPMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits (InEpTxfEmpMsk) These bits acts as mask bits for DIEPINTn.TxFEmp interrupt, one bit per IN Endpoint: Bit 0 for IN EP 0, bit 15 for IN EP 15"]
    #[inline(always)]
    pub fn ineptxfempmsk(&self) -> INEPTXFEMPMSK_R {
        INEPTXFEMPMSK_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPEMPMSK")
            .field("ineptxfempmsk", &self.ineptxfempmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits (InEpTxfEmpMsk) These bits acts as mask bits for DIEPINTn.TxFEmp interrupt, one bit per IN Endpoint: Bit 0 for IN EP 0, bit 15 for IN EP 15"]
    #[inline(always)]
    pub fn ineptxfempmsk(&mut self) -> INEPTXFEMPMSK_W<'_, DIEPEMPMSK_SPEC> {
        INEPTXFEMPMSK_W::new(self, 0)
    }
}
#[doc = "This register is valid only in Dedicated FIFO operation (OTG_EN_DED_TX_FIFO = 1). This register is used to control the IN endpoint FIFO empty interrupt generation (DIEPINTn.TxfEmp).\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for DIEPEMPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepempmsk::R`](R) reader structure"]
impl crate::Readable for DIEPEMPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepempmsk::W`](W) writer structure"]
impl crate::Writable for DIEPEMPMSK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DIEPEMPMSK_SPEC {}
