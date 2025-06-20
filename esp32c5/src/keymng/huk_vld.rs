#[doc = "Register `HUK_VLD` reader"]
pub type R = crate::R<HUK_VLD_SPEC>;
#[doc = "Field `HUK_VALID` reader - The HUK status. 0: HUK is not valid. 1: HUK is valid."]
pub type HUK_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The HUK status. 0: HUK is not valid. 1: HUK is valid."]
    #[inline(always)]
    pub fn huk_valid(&self) -> HUK_VALID_R {
        HUK_VALID_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUK_VLD")
            .field("huk_valid", &self.huk_valid())
            .finish()
    }
}
#[doc = "Key Manager HUK status register\n\nYou can [`read`](crate::Reg::read) this register and get [`huk_vld::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUK_VLD_SPEC;
impl crate::RegisterSpec for HUK_VLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huk_vld::R`](R) reader structure"]
impl crate::Readable for HUK_VLD_SPEC {}
#[doc = "`reset()` method sets HUK_VLD to value 0"]
impl crate::Resettable for HUK_VLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
