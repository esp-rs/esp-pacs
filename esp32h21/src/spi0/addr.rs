#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Field `USR_ADDR_VALUE` reader - In SPI0 USR_CMD mode when SPI_MEM_USR is set, it is the memory address."]
pub type USR_ADDR_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - In SPI0 USR_CMD mode when SPI_MEM_USR is set, it is the memory address."]
    #[inline(always)]
    pub fn usr_addr_value(&self) -> USR_ADDR_VALUE_R {
        USR_ADDR_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR")
            .field("usr_addr_value", &self.usr_addr_value())
            .finish()
    }
}
#[doc = "SPI0 USR_CMD address register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {}
