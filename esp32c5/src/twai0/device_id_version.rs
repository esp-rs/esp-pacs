#[doc = "Register `DEVICE_ID_VERSION` reader"]
pub type R = crate::R<DEVICE_ID_VERSION_SPEC>;
#[doc = "Field `DEVICE_ID` reader - Represents whether CAN IP function is mapped correctly on its base address."]
pub type DEVICE_ID_R = crate::FieldReader<u16>;
#[doc = "Field `VER_MINOR` reader - TWAI FD IP version"]
pub type VER_MINOR_R = crate::FieldReader;
#[doc = "Field `VER_MAJOR` reader - TWAI FD IP version"]
pub type VER_MAJOR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Represents whether CAN IP function is mapped correctly on its base address."]
    #[inline(always)]
    pub fn device_id(&self) -> DEVICE_ID_R {
        DEVICE_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - TWAI FD IP version"]
    #[inline(always)]
    pub fn ver_minor(&self) -> VER_MINOR_R {
        VER_MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TWAI FD IP version"]
    #[inline(always)]
    pub fn ver_major(&self) -> VER_MAJOR_R {
        VER_MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_ID_VERSION")
            .field("device_id", &self.device_id())
            .field("ver_minor", &self.ver_minor())
            .field("ver_major", &self.ver_major())
            .finish()
    }
}
#[doc = "TWAI FD device id status register\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICE_ID_VERSION_SPEC;
impl crate::RegisterSpec for DEVICE_ID_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_id_version::R`](R) reader structure"]
impl crate::Readable for DEVICE_ID_VERSION_SPEC {}
#[doc = "`reset()` method sets DEVICE_ID_VERSION to value 0x0204_cafd"]
impl crate::Resettable for DEVICE_ID_VERSION_SPEC {
    const RESET_VALUE: u32 = 0x0204_cafd;
}
