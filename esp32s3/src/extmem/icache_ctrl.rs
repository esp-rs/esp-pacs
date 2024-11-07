#[doc = "Register `ICACHE_CTRL` reader"]
pub type R = crate::R<ICACHE_CTRL_SPEC>;
#[doc = "Register `ICACHE_CTRL` writer"]
pub type W = crate::W<ICACHE_CTRL_SPEC>;
#[doc = "Field `ICACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type ICACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `ICACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type ICACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_WAY_MODE` reader - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
pub type ICACHE_WAY_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_WAY_MODE` writer - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
pub type ICACHE_WAY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_SIZE_MODE` reader - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
pub type ICACHE_SIZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_SIZE_MODE` writer - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
pub type ICACHE_SIZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub type ICACHE_BLOCKSIZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub type ICACHE_BLOCKSIZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn icache_enable(&self) -> ICACHE_ENABLE_R {
        ICACHE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
    #[inline(always)]
    pub fn icache_way_mode(&self) -> ICACHE_WAY_MODE_R {
        ICACHE_WAY_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
    #[inline(always)]
    pub fn icache_size_mode(&self) -> ICACHE_SIZE_MODE_R {
        ICACHE_SIZE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    pub fn icache_blocksize_mode(&self) -> ICACHE_BLOCKSIZE_MODE_R {
        ICACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_CTRL")
            .field("icache_enable", &self.icache_enable())
            .field("icache_way_mode", &self.icache_way_mode())
            .field("icache_size_mode", &self.icache_size_mode())
            .field("icache_blocksize_mode", &self.icache_blocksize_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn icache_enable(&mut self) -> ICACHE_ENABLE_W<ICACHE_CTRL_SPEC> {
        ICACHE_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to configure cache way mode.0: 4-way, 1: 8-way"]
    #[inline(always)]
    pub fn icache_way_mode(&mut self) -> ICACHE_WAY_MODE_W<ICACHE_CTRL_SPEC> {
        ICACHE_WAY_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 16KB, 1: 32KB"]
    #[inline(always)]
    pub fn icache_size_mode(&mut self) -> ICACHE_SIZE_MODE_W<ICACHE_CTRL_SPEC> {
        ICACHE_SIZE_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    pub fn icache_blocksize_mode(&mut self) -> ICACHE_BLOCKSIZE_MODE_W<ICACHE_CTRL_SPEC> {
        ICACHE_BLOCKSIZE_MODE_W::new(self, 3)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_ctrl::R`](R) reader structure"]
impl crate::Readable for ICACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_ctrl::W`](W) writer structure"]
impl crate::Writable for ICACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICACHE_CTRL to value 0"]
impl crate::Resettable for ICACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
