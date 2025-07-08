#[doc = "Register `ICACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<ICACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Field `ICACHE_BLOCKSIZE_8` reader - The field is used to configureblocksize of L1-ICache as 8 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_8_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_16` reader - The field is used to configureblocksize of L1-ICache as 16 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_16_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of L1-ICache as 32 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_32_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of L1-ICache as 64 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_64_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_128` reader - The field is used to configureblocksize of L1-ICache as 128 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_128_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_256` reader - The field is used to configureblocksize of L1-ICache as 256 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_BLOCKSIZE_256_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configureblocksize of L1-ICache as 8 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_8(&self) -> ICACHE_BLOCKSIZE_8_R {
        ICACHE_BLOCKSIZE_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configureblocksize of L1-ICache as 16 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_16(&self) -> ICACHE_BLOCKSIZE_16_R {
        ICACHE_BLOCKSIZE_16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configureblocksize of L1-ICache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_32(&self) -> ICACHE_BLOCKSIZE_32_R {
        ICACHE_BLOCKSIZE_32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configureblocksize of L1-ICache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_64(&self) -> ICACHE_BLOCKSIZE_64_R {
        ICACHE_BLOCKSIZE_64_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L1-ICache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_128(&self) -> ICACHE_BLOCKSIZE_128_R {
        ICACHE_BLOCKSIZE_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configureblocksize of L1-ICache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_blocksize_256(&self) -> ICACHE_BLOCKSIZE_256_R {
        ICACHE_BLOCKSIZE_256_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_BLOCKSIZE_CONF")
            .field("icache_blocksize_8", &self.icache_blocksize_8())
            .field("icache_blocksize_16", &self.icache_blocksize_16())
            .field("icache_blocksize_32", &self.icache_blocksize_32())
            .field("icache_blocksize_64", &self.icache_blocksize_64())
            .field("icache_blocksize_128", &self.icache_blocksize_128())
            .field("icache_blocksize_256", &self.icache_blocksize_256())
            .finish()
    }
}
#[doc = "L1 instruction Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_blocksize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_BLOCKSIZE_CONF_SPEC;
impl crate::RegisterSpec for ICACHE_BLOCKSIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for ICACHE_BLOCKSIZE_CONF_SPEC {}
#[doc = "`reset()` method sets ICACHE_BLOCKSIZE_CONF to value 0"]
impl crate::Resettable for ICACHE_BLOCKSIZE_CONF_SPEC {}
