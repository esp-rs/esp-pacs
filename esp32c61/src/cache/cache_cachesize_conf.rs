#[doc = "Register `CACHE_CACHESIZE_CONF` reader"]
pub type R = crate::R<CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Field `CACHE_CACHESIZE_256` reader - The field is used to configure cachesize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_256_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_512` reader - The field is used to configure cachesize of L1-DCache as 512 bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_512_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_1K` reader - The field is used to configure cachesize of L1-DCache as 1k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_1K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_2K` reader - The field is used to configure cachesize of L1-DCache as 2k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_2K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_4K` reader - The field is used to configure cachesize of L1-DCache as 4k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_4K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_8K` reader - The field is used to configure cachesize of L1-DCache as 8k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_8K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_16K` reader - The field is used to configure cachesize of L1-DCache as 16k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_16K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_32K` reader - The field is used to configure cachesize of L1-DCache as 32k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_32K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_64K` reader - The field is used to configure cachesize of L1-DCache as 64k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_64K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_128K` reader - The field is used to configure cachesize of L1-DCache as 128k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_128K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_256K` reader - The field is used to configure cachesize of L1-DCache as 256k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_256K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_512K` reader - The field is used to configure cachesize of L1-DCache as 512k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_512K_R = crate::BitReader;
#[doc = "Field `CACHE_CACHESIZE_1024K` reader - The field is used to configure cachesize of L1-DCache as 1024k bytes. This field and all other fields within this register is onehot."]
pub type CACHE_CACHESIZE_1024K_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configure cachesize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_256(&self) -> CACHE_CACHESIZE_256_R {
        CACHE_CACHESIZE_256_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configure cachesize of L1-DCache as 512 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_512(&self) -> CACHE_CACHESIZE_512_R {
        CACHE_CACHESIZE_512_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configure cachesize of L1-DCache as 1k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_1k(&self) -> CACHE_CACHESIZE_1K_R {
        CACHE_CACHESIZE_1K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configure cachesize of L1-DCache as 2k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_2k(&self) -> CACHE_CACHESIZE_2K_R {
        CACHE_CACHESIZE_2K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configure cachesize of L1-DCache as 4k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_4k(&self) -> CACHE_CACHESIZE_4K_R {
        CACHE_CACHESIZE_4K_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configure cachesize of L1-DCache as 8k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_8k(&self) -> CACHE_CACHESIZE_8K_R {
        CACHE_CACHESIZE_8K_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The field is used to configure cachesize of L1-DCache as 16k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_16k(&self) -> CACHE_CACHESIZE_16K_R {
        CACHE_CACHESIZE_16K_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The field is used to configure cachesize of L1-DCache as 32k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_32k(&self) -> CACHE_CACHESIZE_32K_R {
        CACHE_CACHESIZE_32K_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The field is used to configure cachesize of L1-DCache as 64k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_64k(&self) -> CACHE_CACHESIZE_64K_R {
        CACHE_CACHESIZE_64K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The field is used to configure cachesize of L1-DCache as 128k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_128k(&self) -> CACHE_CACHESIZE_128K_R {
        CACHE_CACHESIZE_128K_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The field is used to configure cachesize of L1-DCache as 256k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_256k(&self) -> CACHE_CACHESIZE_256K_R {
        CACHE_CACHESIZE_256K_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The field is used to configure cachesize of L1-DCache as 512k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_512k(&self) -> CACHE_CACHESIZE_512K_R {
        CACHE_CACHESIZE_512K_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The field is used to configure cachesize of L1-DCache as 1024k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn cache_cachesize_1024k(&self) -> CACHE_CACHESIZE_1024K_R {
        CACHE_CACHESIZE_1024K_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CACHESIZE_CONF")
            .field("cache_cachesize_256", &self.cache_cachesize_256())
            .field("cache_cachesize_512", &self.cache_cachesize_512())
            .field("cache_cachesize_1k", &self.cache_cachesize_1k())
            .field("cache_cachesize_2k", &self.cache_cachesize_2k())
            .field("cache_cachesize_4k", &self.cache_cachesize_4k())
            .field("cache_cachesize_8k", &self.cache_cachesize_8k())
            .field("cache_cachesize_16k", &self.cache_cachesize_16k())
            .field("cache_cachesize_32k", &self.cache_cachesize_32k())
            .field("cache_cachesize_64k", &self.cache_cachesize_64k())
            .field("cache_cachesize_128k", &self.cache_cachesize_128k())
            .field("cache_cachesize_256k", &self.cache_cachesize_256k())
            .field("cache_cachesize_512k", &self.cache_cachesize_512k())
            .field("cache_cachesize_1024k", &self.cache_cachesize_1024k())
            .finish()
    }
}
#[doc = "L1 data Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_cachesize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CACHESIZE_CONF_SPEC;
impl crate::RegisterSpec for CACHE_CACHESIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_cachesize_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_CACHESIZE_CONF_SPEC {}
#[doc = "`reset()` method sets CACHE_CACHESIZE_CONF to value 0x80"]
impl crate::Resettable for CACHE_CACHESIZE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
