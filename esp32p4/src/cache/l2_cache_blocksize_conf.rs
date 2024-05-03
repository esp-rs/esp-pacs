#[doc = "Register `L2_CACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<L2_CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Register `L2_CACHE_BLOCKSIZE_CONF` writer"]
pub type W = crate::W<L2_CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Field `L2_CACHE_BLOCKSIZE_8` reader - The field is used to configureblocksize of L2-Cache as 8 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_8_R = crate::BitReader;
#[doc = "Field `L2_CACHE_BLOCKSIZE_16` reader - The field is used to configureblocksize of L2-Cache as 16 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_16_R = crate::BitReader;
#[doc = "Field `L2_CACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of L2-Cache as 32 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_32_R = crate::BitReader;
#[doc = "Field `L2_CACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of L2-Cache as 64 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_64_R = crate::BitReader;
#[doc = "Field `L2_CACHE_BLOCKSIZE_64` writer - The field is used to configureblocksize of L2-Cache as 64 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_BLOCKSIZE_128` reader - The field is used to configureblocksize of L2-Cache as 128 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_128_R = crate::BitReader;
#[doc = "Field `L2_CACHE_BLOCKSIZE_128` writer - The field is used to configureblocksize of L2-Cache as 128 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_BLOCKSIZE_256` reader - The field is used to configureblocksize of L2-Cache as 256 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_BLOCKSIZE_256_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configureblocksize of L2-Cache as 8 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_blocksize_8(&self) -> L2_CACHE_BLOCKSIZE_8_R {
        L2_CACHE_BLOCKSIZE_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configureblocksize of L2-Cache as 16 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_blocksize_16(&self) -> L2_CACHE_BLOCKSIZE_16_R {
        L2_CACHE_BLOCKSIZE_16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configureblocksize of L2-Cache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_blocksize_32(&self) -> L2_CACHE_BLOCKSIZE_32_R {
        L2_CACHE_BLOCKSIZE_32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configureblocksize of L2-Cache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_blocksize_64(&self) -> L2_CACHE_BLOCKSIZE_64_R {
        L2_CACHE_BLOCKSIZE_64_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L2-Cache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_blocksize_128(&self) -> L2_CACHE_BLOCKSIZE_128_R {
        L2_CACHE_BLOCKSIZE_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configureblocksize of L2-Cache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_blocksize_256(&self) -> L2_CACHE_BLOCKSIZE_256_R {
        L2_CACHE_BLOCKSIZE_256_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_BLOCKSIZE_CONF")
            .field("l2_cache_blocksize_8", &self.l2_cache_blocksize_8().bit())
            .field("l2_cache_blocksize_16", &self.l2_cache_blocksize_16().bit())
            .field("l2_cache_blocksize_32", &self.l2_cache_blocksize_32().bit())
            .field("l2_cache_blocksize_64", &self.l2_cache_blocksize_64().bit())
            .field(
                "l2_cache_blocksize_128",
                &self.l2_cache_blocksize_128().bit(),
            )
            .field(
                "l2_cache_blocksize_256",
                &self.l2_cache_blocksize_256().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_BLOCKSIZE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - The field is used to configureblocksize of L2-Cache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_blocksize_64(
        &mut self,
    ) -> L2_CACHE_BLOCKSIZE_64_W<L2_CACHE_BLOCKSIZE_CONF_SPEC> {
        L2_CACHE_BLOCKSIZE_64_W::new(self, 3)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L2-Cache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_blocksize_128(
        &mut self,
    ) -> L2_CACHE_BLOCKSIZE_128_W<L2_CACHE_BLOCKSIZE_CONF_SPEC> {
        L2_CACHE_BLOCKSIZE_128_W::new(self, 4)
    }
}
#[doc = "L2 Cache BlockSize mode configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_blocksize_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_blocksize_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_BLOCKSIZE_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_BLOCKSIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_BLOCKSIZE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_blocksize_conf::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_BLOCKSIZE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_BLOCKSIZE_CONF to value 0x08"]
impl crate::Resettable for L2_CACHE_BLOCKSIZE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
