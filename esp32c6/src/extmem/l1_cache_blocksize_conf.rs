#[doc = "Register `L1_CACHE_BLOCKSIZE_CONF` reader"]
pub struct R(crate::R<L1_CACHE_BLOCKSIZE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_BLOCKSIZE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_BLOCKSIZE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_BLOCKSIZE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_CACHE_BLOCKSIZE_8` reader - The field is used to configureblocksize of L1-DCache as 8 bytes. This field and all other fields within this register is onehot."]
pub type L1_CACHE_BLOCKSIZE_8_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_BLOCKSIZE_16` reader - The field is used to configureblocksize of L1-DCache as 16 bytes. This field and all other fields within this register is onehot."]
pub type L1_CACHE_BLOCKSIZE_16_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of L1-DCache as 32 bytes. This field and all other fields within this register is onehot."]
pub type L1_CACHE_BLOCKSIZE_32_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of L1-DCache as 64 bytes. This field and all other fields within this register is onehot."]
pub type L1_CACHE_BLOCKSIZE_64_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_BLOCKSIZE_128` reader - The field is used to configureblocksize of L1-DCache as 128 bytes. This field and all other fields within this register is onehot."]
pub type L1_CACHE_BLOCKSIZE_128_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_BLOCKSIZE_256` reader - The field is used to configureblocksize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
pub type L1_CACHE_BLOCKSIZE_256_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The field is used to configureblocksize of L1-DCache as 8 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_cache_blocksize_8(&self) -> L1_CACHE_BLOCKSIZE_8_R {
        L1_CACHE_BLOCKSIZE_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configureblocksize of L1-DCache as 16 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_cache_blocksize_16(&self) -> L1_CACHE_BLOCKSIZE_16_R {
        L1_CACHE_BLOCKSIZE_16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configureblocksize of L1-DCache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_cache_blocksize_32(&self) -> L1_CACHE_BLOCKSIZE_32_R {
        L1_CACHE_BLOCKSIZE_32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configureblocksize of L1-DCache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_cache_blocksize_64(&self) -> L1_CACHE_BLOCKSIZE_64_R {
        L1_CACHE_BLOCKSIZE_64_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L1-DCache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_cache_blocksize_128(&self) -> L1_CACHE_BLOCKSIZE_128_R {
        L1_CACHE_BLOCKSIZE_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configureblocksize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_cache_blocksize_256(&self) -> L1_CACHE_BLOCKSIZE_256_R {
        L1_CACHE_BLOCKSIZE_256_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "L1 data Cache BlockSize mode configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_blocksize_conf](index.html) module"]
pub struct L1_CACHE_BLOCKSIZE_CONF_SPEC;
impl crate::RegisterSpec for L1_CACHE_BLOCKSIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_blocksize_conf::R](R) reader structure"]
impl crate::Readable for L1_CACHE_BLOCKSIZE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_CACHE_BLOCKSIZE_CONF to value 0x04"]
impl crate::Resettable for L1_CACHE_BLOCKSIZE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
