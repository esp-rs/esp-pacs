#[doc = "Register `CACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Field `CACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of DCache as 32 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_32_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - The field is used to configureblocksize of DCache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_32(&self) -> CACHE_BLOCKSIZE_32_R {
        CACHE_BLOCKSIZE_32_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_BLOCKSIZE_CONF")
            .field("cache_blocksize_32", &self.cache_blocksize_32())
            .finish()
    }
}
#[doc = "Data Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_blocksize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_BLOCKSIZE_CONF_SPEC;
impl crate::RegisterSpec for CACHE_BLOCKSIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_BLOCKSIZE_CONF_SPEC {}
#[doc = "`reset()` method sets CACHE_BLOCKSIZE_CONF to value 0"]
impl crate::Resettable for CACHE_BLOCKSIZE_CONF_SPEC {}
