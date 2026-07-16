#[doc = "Register `HP_CORDIC_CTRL` reader"]
pub type R = crate::R<HP_CORDIC_CTRL_SPEC>;
#[doc = "Register `HP_CORDIC_CTRL` writer"]
pub type W = crate::W<HP_CORDIC_CTRL_SPEC>;
#[doc = "Field `READ_TEE_HP_CORDIC` reader - Configures hp_cordic registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `READ_TEE_HP_CORDIC` writer - Configures hp_cordic registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_HP_CORDIC` reader - Configures hp_cordic registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `READ_REE0_HP_CORDIC` writer - Configures hp_cordic registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_HP_CORDIC` reader - Configures hp_cordic registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `READ_REE1_HP_CORDIC` writer - Configures hp_cordic registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_HP_CORDIC` reader - Configures hp_cordic registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `READ_REE2_HP_CORDIC` writer - Configures hp_cordic registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_HP_CORDIC` reader - Configures hp_cordic registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_HP_CORDIC` writer - Configures hp_cordic registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_HP_CORDIC` reader - Configures hp_cordic registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_HP_CORDIC` writer - Configures hp_cordic registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_HP_CORDIC` reader - Configures hp_cordic registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_HP_CORDIC` writer - Configures hp_cordic registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_HP_CORDIC` reader - Configures hp_cordic registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HP_CORDIC_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_HP_CORDIC` writer - Configures hp_cordic registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HP_CORDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORDIC_LOCK` reader - Set 1 to lock hp_cordic peri_apm configuration"]
pub type HP_CORDIC_LOCK_R = crate::BitReader;
#[doc = "Field `HP_CORDIC_LOCK` writer - Set 1 to lock hp_cordic peri_apm configuration"]
pub type HP_CORDIC_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures hp_cordic registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_hp_cordic(&self) -> READ_TEE_HP_CORDIC_R {
        READ_TEE_HP_CORDIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures hp_cordic registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_hp_cordic(&self) -> READ_REE0_HP_CORDIC_R {
        READ_REE0_HP_CORDIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures hp_cordic registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_hp_cordic(&self) -> READ_REE1_HP_CORDIC_R {
        READ_REE1_HP_CORDIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures hp_cordic registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_hp_cordic(&self) -> READ_REE2_HP_CORDIC_R {
        READ_REE2_HP_CORDIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures hp_cordic registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_hp_cordic(&self) -> WRITE_TEE_HP_CORDIC_R {
        WRITE_TEE_HP_CORDIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures hp_cordic registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_hp_cordic(&self) -> WRITE_REE0_HP_CORDIC_R {
        WRITE_REE0_HP_CORDIC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures hp_cordic registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_hp_cordic(&self) -> WRITE_REE1_HP_CORDIC_R {
        WRITE_REE1_HP_CORDIC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures hp_cordic registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_hp_cordic(&self) -> WRITE_REE2_HP_CORDIC_R {
        WRITE_REE2_HP_CORDIC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock hp_cordic peri_apm configuration"]
    #[inline(always)]
    pub fn hp_cordic_lock(&self) -> HP_CORDIC_LOCK_R {
        HP_CORDIC_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORDIC_CTRL")
            .field("read_tee_hp_cordic", &self.read_tee_hp_cordic())
            .field("read_ree0_hp_cordic", &self.read_ree0_hp_cordic())
            .field("read_ree1_hp_cordic", &self.read_ree1_hp_cordic())
            .field("read_ree2_hp_cordic", &self.read_ree2_hp_cordic())
            .field("write_tee_hp_cordic", &self.write_tee_hp_cordic())
            .field("write_ree0_hp_cordic", &self.write_ree0_hp_cordic())
            .field("write_ree1_hp_cordic", &self.write_ree1_hp_cordic())
            .field("write_ree2_hp_cordic", &self.write_ree2_hp_cordic())
            .field("hp_cordic_lock", &self.hp_cordic_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures hp_cordic registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_hp_cordic(&mut self) -> READ_TEE_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        READ_TEE_HP_CORDIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures hp_cordic registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_hp_cordic(&mut self) -> READ_REE0_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        READ_REE0_HP_CORDIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures hp_cordic registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_hp_cordic(&mut self) -> READ_REE1_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        READ_REE1_HP_CORDIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures hp_cordic registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_hp_cordic(&mut self) -> READ_REE2_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        READ_REE2_HP_CORDIC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures hp_cordic registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_hp_cordic(&mut self) -> WRITE_TEE_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        WRITE_TEE_HP_CORDIC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures hp_cordic registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_hp_cordic(&mut self) -> WRITE_REE0_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        WRITE_REE0_HP_CORDIC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures hp_cordic registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_hp_cordic(&mut self) -> WRITE_REE1_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        WRITE_REE1_HP_CORDIC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures hp_cordic registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_hp_cordic(&mut self) -> WRITE_REE2_HP_CORDIC_W<'_, HP_CORDIC_CTRL_SPEC> {
        WRITE_REE2_HP_CORDIC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock hp_cordic peri_apm configuration"]
    #[inline(always)]
    pub fn hp_cordic_lock(&mut self) -> HP_CORDIC_LOCK_W<'_, HP_CORDIC_CTRL_SPEC> {
        HP_CORDIC_LOCK_W::new(self, 8)
    }
}
#[doc = "hp_cordic read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cordic_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cordic_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORDIC_CTRL_SPEC;
impl crate::RegisterSpec for HP_CORDIC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cordic_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_CORDIC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cordic_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_CORDIC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CORDIC_CTRL to value 0x11"]
impl crate::Resettable for HP_CORDIC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
