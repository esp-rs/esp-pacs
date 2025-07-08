#[doc = "Register `MEM_XTS_STATE` reader"]
pub type R = crate::R<MEM_XTS_STATE_SPEC>;
#[doc = "Field `XTS_STATE` reader - This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi."]
pub type XTS_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - This bits stores the status of manual encryption. 0: idle, 1: busy of encryption calculation, 2: encryption calculation is done but the encrypted result is invisible to mspi, 3: the encrypted result is visible to mspi."]
    #[inline(always)]
    pub fn xts_state(&self) -> XTS_STATE_R {
        XTS_STATE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_XTS_STATE")
            .field("xts_state", &self.xts_state())
            .finish()
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_XTS_STATE_SPEC;
impl crate::RegisterSpec for MEM_XTS_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_xts_state::R`](R) reader structure"]
impl crate::Readable for MEM_XTS_STATE_SPEC {}
#[doc = "`reset()` method sets MEM_XTS_STATE to value 0"]
impl crate::Resettable for MEM_XTS_STATE_SPEC {}
