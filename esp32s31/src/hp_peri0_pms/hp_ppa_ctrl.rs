#[doc = "Register `HP_PPA_CTRL` reader"]
pub type R = crate::R<HP_PPA_CTRL_SPEC>;
#[doc = "Register `HP_PPA_CTRL` writer"]
pub type W = crate::W<HP_PPA_CTRL_SPEC>;
#[doc = "Field `READ_TEE_HP_PPA` reader - Configures hp_ppa registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HP_PPA_R = crate::BitReader;
#[doc = "Field `READ_TEE_HP_PPA` writer - Configures hp_ppa registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_HP_PPA` reader - Configures hp_ppa registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HP_PPA_R = crate::BitReader;
#[doc = "Field `READ_REE0_HP_PPA` writer - Configures hp_ppa registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_HP_PPA` reader - Configures hp_ppa registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HP_PPA_R = crate::BitReader;
#[doc = "Field `READ_REE1_HP_PPA` writer - Configures hp_ppa registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_HP_PPA` reader - Configures hp_ppa registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HP_PPA_R = crate::BitReader;
#[doc = "Field `READ_REE2_HP_PPA` writer - Configures hp_ppa registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_HP_PPA` reader - Configures hp_ppa registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HP_PPA_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_HP_PPA` writer - Configures hp_ppa registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_HP_PPA` reader - Configures hp_ppa registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HP_PPA_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_HP_PPA` writer - Configures hp_ppa registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_HP_PPA` reader - Configures hp_ppa registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HP_PPA_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_HP_PPA` writer - Configures hp_ppa registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_HP_PPA` reader - Configures hp_ppa registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HP_PPA_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_HP_PPA` writer - Configures hp_ppa registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HP_PPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_PPA_LOCK` reader - Set 1 to lock hp_ppa peri_apm configuration"]
pub type HP_PPA_LOCK_R = crate::BitReader;
#[doc = "Field `HP_PPA_LOCK` writer - Set 1 to lock hp_ppa peri_apm configuration"]
pub type HP_PPA_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures hp_ppa registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_hp_ppa(&self) -> READ_TEE_HP_PPA_R {
        READ_TEE_HP_PPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures hp_ppa registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_hp_ppa(&self) -> READ_REE0_HP_PPA_R {
        READ_REE0_HP_PPA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures hp_ppa registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_hp_ppa(&self) -> READ_REE1_HP_PPA_R {
        READ_REE1_HP_PPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures hp_ppa registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_hp_ppa(&self) -> READ_REE2_HP_PPA_R {
        READ_REE2_HP_PPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures hp_ppa registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_hp_ppa(&self) -> WRITE_TEE_HP_PPA_R {
        WRITE_TEE_HP_PPA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures hp_ppa registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_hp_ppa(&self) -> WRITE_REE0_HP_PPA_R {
        WRITE_REE0_HP_PPA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures hp_ppa registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_hp_ppa(&self) -> WRITE_REE1_HP_PPA_R {
        WRITE_REE1_HP_PPA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures hp_ppa registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_hp_ppa(&self) -> WRITE_REE2_HP_PPA_R {
        WRITE_REE2_HP_PPA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock hp_ppa peri_apm configuration"]
    #[inline(always)]
    pub fn hp_ppa_lock(&self) -> HP_PPA_LOCK_R {
        HP_PPA_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PPA_CTRL")
            .field("read_tee_hp_ppa", &self.read_tee_hp_ppa())
            .field("read_ree0_hp_ppa", &self.read_ree0_hp_ppa())
            .field("read_ree1_hp_ppa", &self.read_ree1_hp_ppa())
            .field("read_ree2_hp_ppa", &self.read_ree2_hp_ppa())
            .field("write_tee_hp_ppa", &self.write_tee_hp_ppa())
            .field("write_ree0_hp_ppa", &self.write_ree0_hp_ppa())
            .field("write_ree1_hp_ppa", &self.write_ree1_hp_ppa())
            .field("write_ree2_hp_ppa", &self.write_ree2_hp_ppa())
            .field("hp_ppa_lock", &self.hp_ppa_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures hp_ppa registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_hp_ppa(&mut self) -> READ_TEE_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        READ_TEE_HP_PPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures hp_ppa registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_hp_ppa(&mut self) -> READ_REE0_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        READ_REE0_HP_PPA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures hp_ppa registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_hp_ppa(&mut self) -> READ_REE1_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        READ_REE1_HP_PPA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures hp_ppa registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_hp_ppa(&mut self) -> READ_REE2_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        READ_REE2_HP_PPA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures hp_ppa registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_hp_ppa(&mut self) -> WRITE_TEE_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        WRITE_TEE_HP_PPA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures hp_ppa registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_hp_ppa(&mut self) -> WRITE_REE0_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        WRITE_REE0_HP_PPA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures hp_ppa registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_hp_ppa(&mut self) -> WRITE_REE1_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        WRITE_REE1_HP_PPA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures hp_ppa registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_hp_ppa(&mut self) -> WRITE_REE2_HP_PPA_W<'_, HP_PPA_CTRL_SPEC> {
        WRITE_REE2_HP_PPA_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock hp_ppa peri_apm configuration"]
    #[inline(always)]
    pub fn hp_ppa_lock(&mut self) -> HP_PPA_LOCK_W<'_, HP_PPA_CTRL_SPEC> {
        HP_PPA_LOCK_W::new(self, 8)
    }
}
#[doc = "hp_ppa read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ppa_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ppa_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PPA_CTRL_SPEC;
impl crate::RegisterSpec for HP_PPA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_ppa_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_PPA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_ppa_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_PPA_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_PPA_CTRL to value 0x11"]
impl crate::Resettable for HP_PPA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
