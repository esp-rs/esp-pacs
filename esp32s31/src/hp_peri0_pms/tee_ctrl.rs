#[doc = "Register `TEE_CTRL` reader"]
pub type R = crate::R<TEE_CTRL_SPEC>;
#[doc = "Register `TEE_CTRL` writer"]
pub type W = crate::W<TEE_CTRL_SPEC>;
#[doc = "Field `READ_TEE_TEE` reader - Configures tee registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_TEE_R = crate::BitReader;
#[doc = "Field `READ_TEE_TEE` writer - Configures tee registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_TEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_TEE` reader - Configures tee registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_TEE_R = crate::BitReader;
#[doc = "Field `READ_REE1_TEE` reader - Configures tee registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_TEE_R = crate::BitReader;
#[doc = "Field `READ_REE2_TEE` reader - Configures tee registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_TEE_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_TEE` reader - Configures tee registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_TEE_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_TEE` writer - Configures tee registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_TEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_TEE` reader - Configures tee registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_TEE_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_TEE` reader - Configures tee registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_TEE_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_TEE` reader - Configures tee registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_TEE_R = crate::BitReader;
#[doc = "Field `TEE_LOCK` reader - Set 1 to lock tee peri_apm configuration"]
pub type TEE_LOCK_R = crate::BitReader;
#[doc = "Field `TEE_LOCK` writer - Set 1 to lock tee peri_apm configuration"]
pub type TEE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures tee registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_tee(&self) -> READ_TEE_TEE_R {
        READ_TEE_TEE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures tee registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_tee(&self) -> READ_REE0_TEE_R {
        READ_REE0_TEE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures tee registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_tee(&self) -> READ_REE1_TEE_R {
        READ_REE1_TEE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures tee registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_tee(&self) -> READ_REE2_TEE_R {
        READ_REE2_TEE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures tee registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_tee(&self) -> WRITE_TEE_TEE_R {
        WRITE_TEE_TEE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures tee registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_tee(&self) -> WRITE_REE0_TEE_R {
        WRITE_REE0_TEE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures tee registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_tee(&self) -> WRITE_REE1_TEE_R {
        WRITE_REE1_TEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures tee registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_tee(&self) -> WRITE_REE2_TEE_R {
        WRITE_REE2_TEE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock tee peri_apm configuration"]
    #[inline(always)]
    pub fn tee_lock(&self) -> TEE_LOCK_R {
        TEE_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEE_CTRL")
            .field("read_tee_tee", &self.read_tee_tee())
            .field("read_ree0_tee", &self.read_ree0_tee())
            .field("read_ree1_tee", &self.read_ree1_tee())
            .field("read_ree2_tee", &self.read_ree2_tee())
            .field("write_tee_tee", &self.write_tee_tee())
            .field("write_ree0_tee", &self.write_ree0_tee())
            .field("write_ree1_tee", &self.write_ree1_tee())
            .field("write_ree2_tee", &self.write_ree2_tee())
            .field("tee_lock", &self.tee_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures tee registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_tee(&mut self) -> READ_TEE_TEE_W<'_, TEE_CTRL_SPEC> {
        READ_TEE_TEE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Configures tee registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_tee(&mut self) -> WRITE_TEE_TEE_W<'_, TEE_CTRL_SPEC> {
        WRITE_TEE_TEE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Set 1 to lock tee peri_apm configuration"]
    #[inline(always)]
    pub fn tee_lock(&mut self) -> TEE_LOCK_W<'_, TEE_CTRL_SPEC> {
        TEE_LOCK_W::new(self, 8)
    }
}
#[doc = "tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tee_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tee_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEE_CTRL_SPEC;
impl crate::RegisterSpec for TEE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tee_ctrl::R`](R) reader structure"]
impl crate::Readable for TEE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tee_ctrl::W`](W) writer structure"]
impl crate::Writable for TEE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEE_CTRL to value 0x11"]
impl crate::Resettable for TEE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
