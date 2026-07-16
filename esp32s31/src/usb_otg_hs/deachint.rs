#[doc = "Register `DEACHINT` reader"]
pub type R = crate::R<DEACHINT_SPEC>;
#[doc = "Field `ECHINEPINT` reader - Each IN Endpoint Interrupt Bits (EchInEpInt) One bit per IN Endpoint: Bit 0 for IN endpoint 0, bit 15 for endpoint 15"]
pub type ECHINEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `ECHOUTEPINT` reader - Each OUT Endpoint Interrupt Bits (EchOutEPInt) One bit per OUT endpoint: Bit 16 for OUT endpoint 0, bit 31 for OUT endpoint 15"]
pub type ECHOUTEPINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt Bits (EchInEpInt) One bit per IN Endpoint: Bit 0 for IN endpoint 0, bit 15 for endpoint 15"]
    #[inline(always)]
    pub fn echinepint(&self) -> ECHINEPINT_R {
        ECHINEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt Bits (EchOutEPInt) One bit per OUT endpoint: Bit 16 for OUT endpoint 0, bit 31 for OUT endpoint 15"]
    #[inline(always)]
    pub fn echoutepint(&self) -> ECHOUTEPINT_R {
        ECHOUTEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINT")
            .field("echinepint", &self.echinepint())
            .field("echoutepint", &self.echoutepint())
            .finish()
    }
}
#[doc = "This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn). The interrupt is automatically cleared once the DOEPINTn/DIEPINTn interrupt is cleared by the application.\n\nYou can [`read`](crate::Reg::read) this register and get [`deachint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINT_SPEC;
impl crate::RegisterSpec for DEACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachint::R`](R) reader structure"]
impl crate::Readable for DEACHINT_SPEC {}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DEACHINT_SPEC {}
