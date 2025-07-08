#[doc = "Register `CACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Field `CACHE_BLOCKSIZE_8` reader - The field is used to configureblocksize of L1-DCache as 8 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_8_R = crate::BitReader;
#[doc = "Field `CACHE_BLOCKSIZE_16` reader - The field is used to configureblocksize of L1-DCache as 16 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_16_R = crate::BitReader;
#[doc = "Field `CACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of L1-DCache as 32 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_32_R = crate::BitReader;
#[doc = "Field `CACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of L1-DCache as 64 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_64_R = crate::BitReader;
#[doc = "Field `CACHE_BLOCKSIZE_128` reader - The field is used to configureblocksize of L1-DCache as 128 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_128_R = crate::BitReader;
#[doc = "Field `CACHE_BLOCKSIZE_256` reader - The field is used to configureblocksize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_BLOCKSIZE_256_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configureblocksize of L1-DCache as 8 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_8(&self) -> CACHE_BLOCKSIZE_8_R {
        CACHE_BLOCKSIZE_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configureblocksize of L1-DCache as 16 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_16(&self) -> CACHE_BLOCKSIZE_16_R {
        CACHE_BLOCKSIZE_16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configureblocksize of L1-DCache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_32(&self) -> CACHE_BLOCKSIZE_32_R {
        CACHE_BLOCKSIZE_32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configureblocksize of L1-DCache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_64(&self) -> CACHE_BLOCKSIZE_64_R {
        CACHE_BLOCKSIZE_64_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L1-DCache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_128(&self) -> CACHE_BLOCKSIZE_128_R {
        CACHE_BLOCKSIZE_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configureblocksize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_blocksize_256(&self) -> CACHE_BLOCKSIZE_256_R {
        CACHE_BLOCKSIZE_256_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_BLOCKSIZE_CONF")
            .field("cache_blocksize_8", &self.cache_blocksize_8())
            .field("cache_blocksize_16", &self.cache_blocksize_16())
            .field("cache_blocksize_32", &self.cache_blocksize_32())
            .field("cache_blocksize_64", &self.cache_blocksize_64())
            .field("cache_blocksize_128", &self.cache_blocksize_128())
            .field("cache_blocksize_256", &self.cache_blocksize_256())
            .finish()
    }
}
#[doc = "L1 data Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_blocksize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_BLOCKSIZE_CONF_SPEC;
impl crate::RegisterSpec for CACHE_BLOCKSIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_BLOCKSIZE_CONF_SPEC {}
#[doc = "`reset()` method sets CACHE_BLOCKSIZE_CONF to value 0x04"]
impl crate::Resettable for CACHE_BLOCKSIZE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
