#[doc = "Register `L2_CACHE_CACHESIZE_CONF` reader"]
pub type R = crate::R<L2_CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Register `L2_CACHE_CACHESIZE_CONF` writer"]
pub type W = crate::W<L2_CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Field `L2_CACHE_CACHESIZE_256` reader - The field is used to configure cachesize of L2-Cache as 256 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_256_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_512` reader - The field is used to configure cachesize of L2-Cache as 512 bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_512_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_1K` reader - The field is used to configure cachesize of L2-Cache as 1k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_1K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_2K` reader - The field is used to configure cachesize of L2-Cache as 2k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_2K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_4K` reader - The field is used to configure cachesize of L2-Cache as 4k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_4K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_8K` reader - The field is used to configure cachesize of L2-Cache as 8k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_8K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_16K` reader - The field is used to configure cachesize of L2-Cache as 16k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_16K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_32K` reader - The field is used to configure cachesize of L2-Cache as 32k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_32K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_64K` reader - The field is used to configure cachesize of L2-Cache as 64k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_64K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_128K` reader - The field is used to configure cachesize of L2-Cache as 128k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_128K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_128K` writer - The field is used to configure cachesize of L2-Cache as 128k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_128K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_CACHESIZE_256K` reader - The field is used to configure cachesize of L2-Cache as 256k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_256K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_256K` writer - The field is used to configure cachesize of L2-Cache as 256k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_256K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_CACHESIZE_512K` reader - The field is used to configure cachesize of L2-Cache as 512k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_512K_R = crate::BitReader;
#[doc = "Field `L2_CACHE_CACHESIZE_512K` writer - The field is used to configure cachesize of L2-Cache as 512k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_512K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_CACHESIZE_1024K` reader - The field is used to configure cachesize of L2-Cache as 1024k bytes. This field and all other fields within this register is onehot."]
pub type L2_CACHE_CACHESIZE_1024K_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configure cachesize of L2-Cache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_256(&self) -> L2_CACHE_CACHESIZE_256_R {
        L2_CACHE_CACHESIZE_256_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configure cachesize of L2-Cache as 512 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_512(&self) -> L2_CACHE_CACHESIZE_512_R {
        L2_CACHE_CACHESIZE_512_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configure cachesize of L2-Cache as 1k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_1k(&self) -> L2_CACHE_CACHESIZE_1K_R {
        L2_CACHE_CACHESIZE_1K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configure cachesize of L2-Cache as 2k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_2k(&self) -> L2_CACHE_CACHESIZE_2K_R {
        L2_CACHE_CACHESIZE_2K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configure cachesize of L2-Cache as 4k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_4k(&self) -> L2_CACHE_CACHESIZE_4K_R {
        L2_CACHE_CACHESIZE_4K_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configure cachesize of L2-Cache as 8k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_8k(&self) -> L2_CACHE_CACHESIZE_8K_R {
        L2_CACHE_CACHESIZE_8K_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The field is used to configure cachesize of L2-Cache as 16k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_16k(&self) -> L2_CACHE_CACHESIZE_16K_R {
        L2_CACHE_CACHESIZE_16K_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The field is used to configure cachesize of L2-Cache as 32k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_32k(&self) -> L2_CACHE_CACHESIZE_32K_R {
        L2_CACHE_CACHESIZE_32K_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The field is used to configure cachesize of L2-Cache as 64k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_64k(&self) -> L2_CACHE_CACHESIZE_64K_R {
        L2_CACHE_CACHESIZE_64K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The field is used to configure cachesize of L2-Cache as 128k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_128k(&self) -> L2_CACHE_CACHESIZE_128K_R {
        L2_CACHE_CACHESIZE_128K_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The field is used to configure cachesize of L2-Cache as 256k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_256k(&self) -> L2_CACHE_CACHESIZE_256K_R {
        L2_CACHE_CACHESIZE_256K_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The field is used to configure cachesize of L2-Cache as 512k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_512k(&self) -> L2_CACHE_CACHESIZE_512K_R {
        L2_CACHE_CACHESIZE_512K_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The field is used to configure cachesize of L2-Cache as 1024k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l2_cache_cachesize_1024k(&self) -> L2_CACHE_CACHESIZE_1024K_R {
        L2_CACHE_CACHESIZE_1024K_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_CACHESIZE_CONF")
            .field(
                "l2_cache_cachesize_256",
                &format_args!("{}", self.l2_cache_cachesize_256().bit()),
            )
            .field(
                "l2_cache_cachesize_512",
                &format_args!("{}", self.l2_cache_cachesize_512().bit()),
            )
            .field(
                "l2_cache_cachesize_1k",
                &format_args!("{}", self.l2_cache_cachesize_1k().bit()),
            )
            .field(
                "l2_cache_cachesize_2k",
                &format_args!("{}", self.l2_cache_cachesize_2k().bit()),
            )
            .field(
                "l2_cache_cachesize_4k",
                &format_args!("{}", self.l2_cache_cachesize_4k().bit()),
            )
            .field(
                "l2_cache_cachesize_8k",
                &format_args!("{}", self.l2_cache_cachesize_8k().bit()),
            )
            .field(
                "l2_cache_cachesize_16k",
                &format_args!("{}", self.l2_cache_cachesize_16k().bit()),
            )
            .field(
                "l2_cache_cachesize_32k",
                &format_args!("{}", self.l2_cache_cachesize_32k().bit()),
            )
            .field(
                "l2_cache_cachesize_64k",
                &format_args!("{}", self.l2_cache_cachesize_64k().bit()),
            )
            .field(
                "l2_cache_cachesize_128k",
                &format_args!("{}", self.l2_cache_cachesize_128k().bit()),
            )
            .field(
                "l2_cache_cachesize_256k",
                &format_args!("{}", self.l2_cache_cachesize_256k().bit()),
            )
            .field(
                "l2_cache_cachesize_512k",
                &format_args!("{}", self.l2_cache_cachesize_512k().bit()),
            )
            .field(
                "l2_cache_cachesize_1024k",
                &format_args!("{}", self.l2_cache_cachesize_1024k().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_CACHESIZE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 9 - The field is used to configure cachesize of L2-Cache as 128k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_cachesize_128k(
        &mut self,
    ) -> L2_CACHE_CACHESIZE_128K_W<L2_CACHE_CACHESIZE_CONF_SPEC> {
        L2_CACHE_CACHESIZE_128K_W::new(self, 9)
    }
    #[doc = "Bit 10 - The field is used to configure cachesize of L2-Cache as 256k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_cachesize_256k(
        &mut self,
    ) -> L2_CACHE_CACHESIZE_256K_W<L2_CACHE_CACHESIZE_CONF_SPEC> {
        L2_CACHE_CACHESIZE_256K_W::new(self, 10)
    }
    #[doc = "Bit 11 - The field is used to configure cachesize of L2-Cache as 512k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_cachesize_512k(
        &mut self,
    ) -> L2_CACHE_CACHESIZE_512K_W<L2_CACHE_CACHESIZE_CONF_SPEC> {
        L2_CACHE_CACHESIZE_512K_W::new(self, 11)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "L2 Cache CacheSize mode configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_cachesize_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_cachesize_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_CACHESIZE_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_CACHESIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_cachesize_conf::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_CACHESIZE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_cachesize_conf::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_CACHESIZE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2_CACHE_CACHESIZE_CONF to value 0x0400"]
impl crate::Resettable for L2_CACHE_CACHESIZE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
