#[doc = "Register `CACHE_CTRL` reader"]
pub type R = crate::R<CACHE_CTRL_SPEC>;
#[doc = "Register `CACHE_CTRL` writer"]
pub type W = crate::W<CACHE_CTRL_SPEC>;
#[doc = "Field `READ_TEE_CACHE` reader - Configures cache registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CACHE_R = crate::BitReader;
#[doc = "Field `READ_TEE_CACHE` writer - Configures cache registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_CACHE` reader - Configures cache registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CACHE_R = crate::BitReader;
#[doc = "Field `READ_REE0_CACHE` writer - Configures cache registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_CACHE` reader - Configures cache registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CACHE_R = crate::BitReader;
#[doc = "Field `READ_REE1_CACHE` writer - Configures cache registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_CACHE` reader - Configures cache registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CACHE_R = crate::BitReader;
#[doc = "Field `READ_REE2_CACHE` writer - Configures cache registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_CACHE` reader - Configures cache registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CACHE_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_CACHE` writer - Configures cache registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_CACHE` reader - Configures cache registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CACHE_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_CACHE` writer - Configures cache registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_CACHE` reader - Configures cache registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CACHE_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_CACHE` writer - Configures cache registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_CACHE` reader - Configures cache registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CACHE_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_CACHE` writer - Configures cache registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_LOCK` reader - Set 1 to lock cache peri_apm configuration"]
pub type CACHE_LOCK_R = crate::BitReader;
#[doc = "Field `CACHE_LOCK` writer - Set 1 to lock cache peri_apm configuration"]
pub type CACHE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures cache registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_cache(&self) -> READ_TEE_CACHE_R {
        READ_TEE_CACHE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures cache registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_cache(&self) -> READ_REE0_CACHE_R {
        READ_REE0_CACHE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures cache registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_cache(&self) -> READ_REE1_CACHE_R {
        READ_REE1_CACHE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures cache registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_cache(&self) -> READ_REE2_CACHE_R {
        READ_REE2_CACHE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures cache registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_cache(&self) -> WRITE_TEE_CACHE_R {
        WRITE_TEE_CACHE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures cache registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_cache(&self) -> WRITE_REE0_CACHE_R {
        WRITE_REE0_CACHE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures cache registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_cache(&self) -> WRITE_REE1_CACHE_R {
        WRITE_REE1_CACHE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures cache registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_cache(&self) -> WRITE_REE2_CACHE_R {
        WRITE_REE2_CACHE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock cache peri_apm configuration"]
    #[inline(always)]
    pub fn cache_lock(&self) -> CACHE_LOCK_R {
        CACHE_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CTRL")
            .field("read_tee_cache", &self.read_tee_cache())
            .field("read_ree0_cache", &self.read_ree0_cache())
            .field("read_ree1_cache", &self.read_ree1_cache())
            .field("read_ree2_cache", &self.read_ree2_cache())
            .field("write_tee_cache", &self.write_tee_cache())
            .field("write_ree0_cache", &self.write_ree0_cache())
            .field("write_ree1_cache", &self.write_ree1_cache())
            .field("write_ree2_cache", &self.write_ree2_cache())
            .field("cache_lock", &self.cache_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures cache registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_cache(&mut self) -> READ_TEE_CACHE_W<'_, CACHE_CTRL_SPEC> {
        READ_TEE_CACHE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures cache registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_cache(&mut self) -> READ_REE0_CACHE_W<'_, CACHE_CTRL_SPEC> {
        READ_REE0_CACHE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures cache registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_cache(&mut self) -> READ_REE1_CACHE_W<'_, CACHE_CTRL_SPEC> {
        READ_REE1_CACHE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures cache registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_cache(&mut self) -> READ_REE2_CACHE_W<'_, CACHE_CTRL_SPEC> {
        READ_REE2_CACHE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures cache registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_cache(&mut self) -> WRITE_TEE_CACHE_W<'_, CACHE_CTRL_SPEC> {
        WRITE_TEE_CACHE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures cache registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_cache(&mut self) -> WRITE_REE0_CACHE_W<'_, CACHE_CTRL_SPEC> {
        WRITE_REE0_CACHE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures cache registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_cache(&mut self) -> WRITE_REE1_CACHE_W<'_, CACHE_CTRL_SPEC> {
        WRITE_REE1_CACHE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures cache registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_cache(&mut self) -> WRITE_REE2_CACHE_W<'_, CACHE_CTRL_SPEC> {
        WRITE_REE2_CACHE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock cache peri_apm configuration"]
    #[inline(always)]
    pub fn cache_lock(&mut self) -> CACHE_LOCK_W<'_, CACHE_CTRL_SPEC> {
        CACHE_LOCK_W::new(self, 8)
    }
}
#[doc = "cache read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_CTRL to value 0x11"]
impl crate::Resettable for CACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
