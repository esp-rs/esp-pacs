#[doc = "Register `ID` reader"]
pub type R = crate::R<ID_SPEC>;
#[doc = "Field `ROM_ID` reader - Represents the ROM ID of chip"]
pub type ROM_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 12:27 - Represents the ROM ID of chip"]
    #[inline(always)]
    pub fn rom_id(&self) -> ROM_ID_R {
        ROM_ID_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("rom_id", &self.rom_id())
            .finish()
    }
}
#[doc = "ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for ID_SPEC {}
#[doc = "`reset()` method sets ID to value 0"]
impl crate::Resettable for ID_SPEC {}
