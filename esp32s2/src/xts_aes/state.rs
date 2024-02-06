#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `STATE` reader - Indicates the status of the Manual Encryption block. 0x0 (XTS_AES_IDLE): idle. 0x1 (XTS_AES_BUSY): busy with encryption. 0x2 (XTS_AES_DONE): encryption is completed, but the encrypted result is not accessible to SPI. 0X3 (XTS_AES_RELEASE): encrypted result is accessible to SPI."]
pub type STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Indicates the status of the Manual Encryption block. 0x0 (XTS_AES_IDLE): idle. 0x1 (XTS_AES_BUSY): busy with encryption. 0x2 (XTS_AES_DONE): encryption is completed, but the encrypted result is not accessible to SPI. 0X3 (XTS_AES_RELEASE): encrypted result is accessible to SPI."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("state", &format_args!("{}", self.state().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
