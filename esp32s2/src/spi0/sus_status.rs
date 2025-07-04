#[doc = "Register `SUS_STATUS` reader"]
pub type R = crate::R<SUS_STATUS_SPEC>;
#[doc = "Field `FLASH_SUS` reader - "]
pub type FLASH_SUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_sus(&self) -> FLASH_SUS_R {
        FLASH_SUS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUS_STATUS")
            .field("flash_sus", &self.flash_sus())
            .finish()
    }
}
#[doc = "SPI Memory Suspend Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUS_STATUS_SPEC;
impl crate::RegisterSpec for SUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sus_status::R`](R) reader structure"]
impl crate::Readable for SUS_STATUS_SPEC {}
#[doc = "`reset()` method sets SUS_STATUS to value 0"]
impl crate::Resettable for SUS_STATUS_SPEC {}
