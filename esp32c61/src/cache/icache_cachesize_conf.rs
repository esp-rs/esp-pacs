#[doc = "Register `ICACHE_CACHESIZE_CONF` reader"]
pub type R = crate::R<ICACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Field `ICACHE_CACHESIZE_256` reader - The field is used to configure cachesize of L1-ICache as 256 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_256_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_512` reader - The field is used to configure cachesize of L1-ICache as 512 bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_512_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_1K` reader - The field is used to configure cachesize of L1-ICache as 1k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_1K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_2K` reader - The field is used to configure cachesize of L1-ICache as 2k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_2K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_4K` reader - The field is used to configure cachesize of L1-ICache as 4k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_4K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_8K` reader - The field is used to configure cachesize of L1-ICache as 8k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_8K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_16K` reader - The field is used to configure cachesize of L1-ICache as 16k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_16K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_32K` reader - The field is used to configure cachesize of L1-ICache as 32k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_32K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_64K` reader - The field is used to configure cachesize of L1-ICache as 64k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_64K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_128K` reader - The field is used to configure cachesize of L1-ICache as 128k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_128K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_256K` reader - The field is used to configure cachesize of L1-ICache as 256k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_256K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_512K` reader - The field is used to configure cachesize of L1-ICache as 512k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_512K_R = crate::BitReader;
#[doc = "Field `ICACHE_CACHESIZE_1024K` reader - The field is used to configure cachesize of L1-ICache as 1024k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_1024K_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configure cachesize of L1-ICache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_256(&self) -> ICACHE_CACHESIZE_256_R {
        ICACHE_CACHESIZE_256_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configure cachesize of L1-ICache as 512 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_512(&self) -> ICACHE_CACHESIZE_512_R {
        ICACHE_CACHESIZE_512_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configure cachesize of L1-ICache as 1k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_1k(&self) -> ICACHE_CACHESIZE_1K_R {
        ICACHE_CACHESIZE_1K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configure cachesize of L1-ICache as 2k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_2k(&self) -> ICACHE_CACHESIZE_2K_R {
        ICACHE_CACHESIZE_2K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configure cachesize of L1-ICache as 4k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_4k(&self) -> ICACHE_CACHESIZE_4K_R {
        ICACHE_CACHESIZE_4K_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configure cachesize of L1-ICache as 8k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_8k(&self) -> ICACHE_CACHESIZE_8K_R {
        ICACHE_CACHESIZE_8K_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The field is used to configure cachesize of L1-ICache as 16k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_16k(&self) -> ICACHE_CACHESIZE_16K_R {
        ICACHE_CACHESIZE_16K_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The field is used to configure cachesize of L1-ICache as 32k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_32k(&self) -> ICACHE_CACHESIZE_32K_R {
        ICACHE_CACHESIZE_32K_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The field is used to configure cachesize of L1-ICache as 64k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_64k(&self) -> ICACHE_CACHESIZE_64K_R {
        ICACHE_CACHESIZE_64K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The field is used to configure cachesize of L1-ICache as 128k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_128k(&self) -> ICACHE_CACHESIZE_128K_R {
        ICACHE_CACHESIZE_128K_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The field is used to configure cachesize of L1-ICache as 256k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_256k(&self) -> ICACHE_CACHESIZE_256K_R {
        ICACHE_CACHESIZE_256K_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The field is used to configure cachesize of L1-ICache as 512k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_512k(&self) -> ICACHE_CACHESIZE_512K_R {
        ICACHE_CACHESIZE_512K_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The field is used to configure cachesize of L1-ICache as 1024k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_1024k(&self) -> ICACHE_CACHESIZE_1024K_R {
        ICACHE_CACHESIZE_1024K_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_CACHESIZE_CONF")
            .field("icache_cachesize_256", &self.icache_cachesize_256())
            .field("icache_cachesize_512", &self.icache_cachesize_512())
            .field("icache_cachesize_1k", &self.icache_cachesize_1k())
            .field("icache_cachesize_2k", &self.icache_cachesize_2k())
            .field("icache_cachesize_4k", &self.icache_cachesize_4k())
            .field("icache_cachesize_8k", &self.icache_cachesize_8k())
            .field("icache_cachesize_16k", &self.icache_cachesize_16k())
            .field("icache_cachesize_32k", &self.icache_cachesize_32k())
            .field("icache_cachesize_64k", &self.icache_cachesize_64k())
            .field("icache_cachesize_128k", &self.icache_cachesize_128k())
            .field("icache_cachesize_256k", &self.icache_cachesize_256k())
            .field("icache_cachesize_512k", &self.icache_cachesize_512k())
            .field("icache_cachesize_1024k", &self.icache_cachesize_1024k())
            .finish()
    }
}
#[doc = "L1 instruction Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_cachesize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_CACHESIZE_CONF_SPEC;
impl crate::RegisterSpec for ICACHE_CACHESIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_cachesize_conf::R`](R) reader structure"]
impl crate::Readable for ICACHE_CACHESIZE_CONF_SPEC {}
#[doc = "`reset()` method sets ICACHE_CACHESIZE_CONF to value 0"]
impl crate::Resettable for ICACHE_CACHESIZE_CONF_SPEC {}
