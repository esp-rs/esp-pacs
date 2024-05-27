///Register `L1_DCACHE_CACHESIZE_CONF` reader
pub type R = crate::R<L1_DCACHE_CACHESIZE_CONF_SPEC>;
///Field `L1_DCACHE_CACHESIZE_256` reader - The field is used to configure cachesize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_256_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_512` reader - The field is used to configure cachesize of L1-DCache as 512 bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_512_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_1K` reader - The field is used to configure cachesize of L1-DCache as 1k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_1K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_2K` reader - The field is used to configure cachesize of L1-DCache as 2k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_2K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_4K` reader - The field is used to configure cachesize of L1-DCache as 4k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_4K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_8K` reader - The field is used to configure cachesize of L1-DCache as 8k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_8K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_16K` reader - The field is used to configure cachesize of L1-DCache as 16k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_16K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_32K` reader - The field is used to configure cachesize of L1-DCache as 32k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_32K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_64K` reader - The field is used to configure cachesize of L1-DCache as 64k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_64K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_128K` reader - The field is used to configure cachesize of L1-DCache as 128k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_128K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_256K` reader - The field is used to configure cachesize of L1-DCache as 256k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_256K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_512K` reader - The field is used to configure cachesize of L1-DCache as 512k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_512K_R = crate::BitReader;
///Field `L1_DCACHE_CACHESIZE_1024K` reader - The field is used to configure cachesize of L1-DCache as 1024k bytes. This field and all other fields within this register is onehot.
pub type L1_DCACHE_CACHESIZE_1024K_R = crate::BitReader;
impl R {
    ///Bit 0 - The field is used to configure cachesize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_256(&self) -> L1_DCACHE_CACHESIZE_256_R {
        L1_DCACHE_CACHESIZE_256_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The field is used to configure cachesize of L1-DCache as 512 bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_512(&self) -> L1_DCACHE_CACHESIZE_512_R {
        L1_DCACHE_CACHESIZE_512_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The field is used to configure cachesize of L1-DCache as 1k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_1k(&self) -> L1_DCACHE_CACHESIZE_1K_R {
        L1_DCACHE_CACHESIZE_1K_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The field is used to configure cachesize of L1-DCache as 2k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_2k(&self) -> L1_DCACHE_CACHESIZE_2K_R {
        L1_DCACHE_CACHESIZE_2K_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The field is used to configure cachesize of L1-DCache as 4k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_4k(&self) -> L1_DCACHE_CACHESIZE_4K_R {
        L1_DCACHE_CACHESIZE_4K_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The field is used to configure cachesize of L1-DCache as 8k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_8k(&self) -> L1_DCACHE_CACHESIZE_8K_R {
        L1_DCACHE_CACHESIZE_8K_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The field is used to configure cachesize of L1-DCache as 16k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_16k(&self) -> L1_DCACHE_CACHESIZE_16K_R {
        L1_DCACHE_CACHESIZE_16K_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The field is used to configure cachesize of L1-DCache as 32k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_32k(&self) -> L1_DCACHE_CACHESIZE_32K_R {
        L1_DCACHE_CACHESIZE_32K_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The field is used to configure cachesize of L1-DCache as 64k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_64k(&self) -> L1_DCACHE_CACHESIZE_64K_R {
        L1_DCACHE_CACHESIZE_64K_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The field is used to configure cachesize of L1-DCache as 128k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_128k(&self) -> L1_DCACHE_CACHESIZE_128K_R {
        L1_DCACHE_CACHESIZE_128K_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - The field is used to configure cachesize of L1-DCache as 256k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_256k(&self) -> L1_DCACHE_CACHESIZE_256K_R {
        L1_DCACHE_CACHESIZE_256K_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - The field is used to configure cachesize of L1-DCache as 512k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_512k(&self) -> L1_DCACHE_CACHESIZE_512K_R {
        L1_DCACHE_CACHESIZE_512K_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - The field is used to configure cachesize of L1-DCache as 1024k bytes. This field and all other fields within this register is onehot.
    #[inline(always)]
    pub fn l1_dcache_cachesize_1024k(&self) -> L1_DCACHE_CACHESIZE_1024K_R {
        L1_DCACHE_CACHESIZE_1024K_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DCACHE_CACHESIZE_CONF")
            .field("l1_dcache_cachesize_256", &self.l1_dcache_cachesize_256())
            .field("l1_dcache_cachesize_512", &self.l1_dcache_cachesize_512())
            .field("l1_dcache_cachesize_1k", &self.l1_dcache_cachesize_1k())
            .field("l1_dcache_cachesize_2k", &self.l1_dcache_cachesize_2k())
            .field("l1_dcache_cachesize_4k", &self.l1_dcache_cachesize_4k())
            .field("l1_dcache_cachesize_8k", &self.l1_dcache_cachesize_8k())
            .field("l1_dcache_cachesize_16k", &self.l1_dcache_cachesize_16k())
            .field("l1_dcache_cachesize_32k", &self.l1_dcache_cachesize_32k())
            .field("l1_dcache_cachesize_64k", &self.l1_dcache_cachesize_64k())
            .field("l1_dcache_cachesize_128k", &self.l1_dcache_cachesize_128k())
            .field("l1_dcache_cachesize_256k", &self.l1_dcache_cachesize_256k())
            .field("l1_dcache_cachesize_512k", &self.l1_dcache_cachesize_512k())
            .field(
                "l1_dcache_cachesize_1024k",
                &self.l1_dcache_cachesize_1024k(),
            )
            .finish()
    }
}
/**L1 data Cache CacheSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_cachesize_conf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_DCACHE_CACHESIZE_CONF_SPEC;
impl crate::RegisterSpec for L1_DCACHE_CACHESIZE_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_dcache_cachesize_conf::R`](R) reader structure
impl crate::Readable for L1_DCACHE_CACHESIZE_CONF_SPEC {}
///`reset()` method sets L1_DCACHE_CACHESIZE_CONF to value 0x0100
impl crate::Resettable for L1_DCACHE_CACHESIZE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
