#[doc = "Register `DATA` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Field `RND_DATA` reader - get rng data"]
pub type RND_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - get rng data"]
    #[inline(always)]
    pub fn rnd_data(&self) -> RND_DATA_R {
        RND_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA")
            .field("rnd_data", &self.rnd_data())
            .finish()
    }
}
#[doc = "RNG result register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {}
