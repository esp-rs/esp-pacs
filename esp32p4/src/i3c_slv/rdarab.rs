#[doc = "Register `RDARAB` reader"]
pub type R = crate::R<RDARAB_SPEC>;
#[doc = "Field `DATA0` reader - This register allows reading a byte from the bus unless external FIFO is used. A byte should not be read unless there is data waiting, as indicated by the RXPEND bit being set in the STATUS register"]
pub type DATA0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This register allows reading a byte from the bus unless external FIFO is used. A byte should not be read unless there is data waiting, as indicated by the RXPEND bit being set in the STATUS register"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDARAB")
            .field("data0", &format_args!("{}", self.data0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RDARAB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Read Byte Data (from-bus) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdarab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDARAB_SPEC;
impl crate::RegisterSpec for RDARAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdarab::R`](R) reader structure"]
impl crate::Readable for RDARAB_SPEC {}
#[doc = "`reset()` method sets RDARAB to value 0"]
impl crate::Resettable for RDARAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
