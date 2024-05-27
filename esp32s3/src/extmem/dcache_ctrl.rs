///Register `DCACHE_CTRL` reader
pub type R = crate::R<DCACHE_CTRL_SPEC>;
///Register `DCACHE_CTRL` writer
pub type W = crate::W<DCACHE_CTRL_SPEC>;
///Field `DCACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable
pub type DCACHE_ENABLE_R = crate::BitReader;
///Field `DCACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable
pub type DCACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_SIZE_MODE` reader - The bit is used to configure cache memory size.0: 32KB, 1: 64KB
pub type DCACHE_SIZE_MODE_R = crate::BitReader;
///Field `DCACHE_SIZE_MODE` writer - The bit is used to configure cache memory size.0: 32KB, 1: 64KB
pub type DCACHE_SIZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes
pub type DCACHE_BLOCKSIZE_MODE_R = crate::FieldReader;
///Field `DCACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes
pub type DCACHE_BLOCKSIZE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable
    #[inline(always)]
    pub fn dcache_enable(&self) -> DCACHE_ENABLE_R {
        DCACHE_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - The bit is used to configure cache memory size.0: 32KB, 1: 64KB
    #[inline(always)]
    pub fn dcache_size_mode(&self) -> DCACHE_SIZE_MODE_R {
        DCACHE_SIZE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes
    #[inline(always)]
    pub fn dcache_blocksize_mode(&self) -> DCACHE_BLOCKSIZE_MODE_R {
        DCACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_CTRL")
            .field("dcache_enable", &self.dcache_enable())
            .field("dcache_size_mode", &self.dcache_size_mode())
            .field("dcache_blocksize_mode", &self.dcache_blocksize_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn dcache_enable(&mut self) -> DCACHE_ENABLE_W<DCACHE_CTRL_SPEC> {
        DCACHE_ENABLE_W::new(self, 0)
    }
    ///Bit 2 - The bit is used to configure cache memory size.0: 32KB, 1: 64KB
    #[inline(always)]
    #[must_use]
    pub fn dcache_size_mode(&mut self) -> DCACHE_SIZE_MODE_W<DCACHE_CTRL_SPEC> {
        DCACHE_SIZE_MODE_W::new(self, 2)
    }
    ///Bits 3:4 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes,2: 64 bytes
    #[inline(always)]
    #[must_use]
    pub fn dcache_blocksize_mode(&mut self) -> DCACHE_BLOCKSIZE_MODE_W<DCACHE_CTRL_SPEC> {
        DCACHE_BLOCKSIZE_MODE_W::new(self, 3)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dcache_ctrl::R`](R) reader structure
impl crate::Readable for DCACHE_CTRL_SPEC {}
///`write(|w| ..)` method takes [`dcache_ctrl::W`](W) writer structure
impl crate::Writable for DCACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_CTRL to value 0
impl crate::Resettable for DCACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
