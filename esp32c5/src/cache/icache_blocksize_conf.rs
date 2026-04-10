#[doc = "Register `ICACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<ICACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Field `ICACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of ICache as 64 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_64_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The field is used to configureblocksize of ICache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_64(&self) -> ICACHE_BLOCKSIZE_64_R {
        ICACHE_BLOCKSIZE_64_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_BLOCKSIZE_CONF")
            .field("icache_blocksize_64", &self.icache_blocksize_64())
            .finish()
    }
}
#[doc = "Instruction Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_blocksize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_BLOCKSIZE_CONF_SPEC;
impl crate::RegisterSpec for ICACHE_BLOCKSIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for ICACHE_BLOCKSIZE_CONF_SPEC {}
#[doc = "`reset()` method sets ICACHE_BLOCKSIZE_CONF to value 0"]
impl crate::Resettable for ICACHE_BLOCKSIZE_CONF_SPEC {}
