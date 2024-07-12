#[doc = "Register `APB_PERIPHERAL_STATUS` reader"]
pub type R = crate::R<APB_PERIPHERAL_STATUS_SPEC>;
#[doc = "Field `APB_PERI_BYTE_ERROR_ADDR` reader - Record the illegitimate address of APB peripheral."]
pub type APB_PERI_BYTE_ERROR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Record the illegitimate address of APB peripheral."]
    #[inline(always)]
    pub fn apb_peri_byte_error_addr(&self) -> APB_PERI_BYTE_ERROR_ADDR_R {
        APB_PERI_BYTE_ERROR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_STATUS")
            .field("apb_peri_byte_error_addr", &self.apb_peri_byte_error_addr())
            .finish()
    }
}
#[doc = "PeribBus2 peripheral access status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_peripheral_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_PERIPHERAL_STATUS_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_peripheral_status::R`](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_STATUS_SPEC {}
#[doc = "`reset()` method sets APB_PERIPHERAL_STATUS to value 0"]
impl crate::Resettable for APB_PERIPHERAL_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
