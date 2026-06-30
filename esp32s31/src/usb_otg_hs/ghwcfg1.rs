#[doc = "Register `GHWCFG1` reader"]
pub type R = crate::R<GHWCFG1_SPEC>;
#[doc = "Field `EPDIR` reader - This 32-bit field uses two bits per endpoint to determine the endpoint direction. Endpoint - Bits \\[31:30\\]: Endpoint 15 direction - Bits \\[29:28\\]: Endpoint 14 direction ... - Bits \\[3:2\\]: Endpoint 1 direction - Bits\\[1:0\\]: Endpoint 0 direction (always BIDIR) Direction - 2'b00: BIDIR (IN and OUT) endpoint - 2'b01: IN endpoint - 2'b10: OUT endpoint - 2'b11: Reserved Note: This field is configured using the OTG_EP_DIR_1(n) parameter."]
pub type EPDIR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This 32-bit field uses two bits per endpoint to determine the endpoint direction. Endpoint - Bits \\[31:30\\]: Endpoint 15 direction - Bits \\[29:28\\]: Endpoint 14 direction ... - Bits \\[3:2\\]: Endpoint 1 direction - Bits\\[1:0\\]: Endpoint 0 direction (always BIDIR) Direction - 2'b00: BIDIR (IN and OUT) endpoint - 2'b01: IN endpoint - 2'b10: OUT endpoint - 2'b11: Reserved Note: This field is configured using the OTG_EP_DIR_1(n) parameter."]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG1")
            .field("epdir", &self.epdir())
            .finish()
    }
}
#[doc = "This register contains the logical endpoint direction(s) selected using coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHWCFG1_SPEC;
impl crate::RegisterSpec for GHWCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg1::R`](R) reader structure"]
impl crate::Readable for GHWCFG1_SPEC {}
#[doc = "`reset()` method sets GHWCFG1 to value 0"]
impl crate::Resettable for GHWCFG1_SPEC {}
