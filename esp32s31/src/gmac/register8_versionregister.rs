#[doc = "Register `REGISTER8_VERSIONREGISTER` reader"]
pub type R = crate::R<REGISTER8_VERSIONREGISTER_SPEC>;
#[doc = "Field `SNPSVER` reader - Synopsysdefined Version _37_"]
pub type SNPSVER_R = crate::FieldReader;
#[doc = "Field `USERVER` reader - Userdefined Version _configured with coreConsultant_"]
pub type USERVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Synopsysdefined Version _37_"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Userdefined Version _configured with coreConsultant_"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER8_VERSIONREGISTER")
            .field("snpsver", &self.snpsver())
            .field("userver", &self.userver())
            .finish()
    }
}
#[doc = "Identifies the version of the Core\n\nYou can [`read`](crate::Reg::read) this register and get [`register8_versionregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER8_VERSIONREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER8_VERSIONREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register8_versionregister::R`](R) reader structure"]
impl crate::Readable for REGISTER8_VERSIONREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER8_VERSIONREGISTER to value 0x37"]
impl crate::Resettable for REGISTER8_VERSIONREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x37;
}
