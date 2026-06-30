#[doc = "Register `CNNT_PAD_CTRL_CTRL` reader"]
pub type R = crate::R<CNNT_PAD_CTRL_CTRL_SPEC>;
#[doc = "Register `CNNT_PAD_CTRL_CTRL` writer"]
pub type W = crate::W<CNNT_PAD_CTRL_CTRL_SPEC>;
#[doc = "Field `READ_TEE_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `READ_TEE_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `READ_REE0_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `READ_REE1_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `READ_REE2_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_CNNT_PAD_CTRL` reader - Configures cnnt_pad_ctrl registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CNNT_PAD_CTRL_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_CNNT_PAD_CTRL` writer - Configures cnnt_pad_ctrl registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CNNT_PAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNT_PAD_CTRL_LOCK` reader - Set 1 to lock cnnt_pad_ctrl peri_apm configuration"]
pub type CNNT_PAD_CTRL_LOCK_R = crate::BitReader;
#[doc = "Field `CNNT_PAD_CTRL_LOCK` writer - Set 1 to lock cnnt_pad_ctrl peri_apm configuration"]
pub type CNNT_PAD_CTRL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures cnnt_pad_ctrl registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_cnnt_pad_ctrl(&self) -> READ_TEE_CNNT_PAD_CTRL_R {
        READ_TEE_CNNT_PAD_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures cnnt_pad_ctrl registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_cnnt_pad_ctrl(&self) -> READ_REE0_CNNT_PAD_CTRL_R {
        READ_REE0_CNNT_PAD_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures cnnt_pad_ctrl registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_cnnt_pad_ctrl(&self) -> READ_REE1_CNNT_PAD_CTRL_R {
        READ_REE1_CNNT_PAD_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures cnnt_pad_ctrl registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_cnnt_pad_ctrl(&self) -> READ_REE2_CNNT_PAD_CTRL_R {
        READ_REE2_CNNT_PAD_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures cnnt_pad_ctrl registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_cnnt_pad_ctrl(&self) -> WRITE_TEE_CNNT_PAD_CTRL_R {
        WRITE_TEE_CNNT_PAD_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures cnnt_pad_ctrl registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_cnnt_pad_ctrl(&self) -> WRITE_REE0_CNNT_PAD_CTRL_R {
        WRITE_REE0_CNNT_PAD_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures cnnt_pad_ctrl registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_cnnt_pad_ctrl(&self) -> WRITE_REE1_CNNT_PAD_CTRL_R {
        WRITE_REE1_CNNT_PAD_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures cnnt_pad_ctrl registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_cnnt_pad_ctrl(&self) -> WRITE_REE2_CNNT_PAD_CTRL_R {
        WRITE_REE2_CNNT_PAD_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock cnnt_pad_ctrl peri_apm configuration"]
    #[inline(always)]
    pub fn cnnt_pad_ctrl_lock(&self) -> CNNT_PAD_CTRL_LOCK_R {
        CNNT_PAD_CTRL_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNNT_PAD_CTRL_CTRL")
            .field("read_tee_cnnt_pad_ctrl", &self.read_tee_cnnt_pad_ctrl())
            .field("read_ree0_cnnt_pad_ctrl", &self.read_ree0_cnnt_pad_ctrl())
            .field("read_ree1_cnnt_pad_ctrl", &self.read_ree1_cnnt_pad_ctrl())
            .field("read_ree2_cnnt_pad_ctrl", &self.read_ree2_cnnt_pad_ctrl())
            .field("write_tee_cnnt_pad_ctrl", &self.write_tee_cnnt_pad_ctrl())
            .field("write_ree0_cnnt_pad_ctrl", &self.write_ree0_cnnt_pad_ctrl())
            .field("write_ree1_cnnt_pad_ctrl", &self.write_ree1_cnnt_pad_ctrl())
            .field("write_ree2_cnnt_pad_ctrl", &self.write_ree2_cnnt_pad_ctrl())
            .field("cnnt_pad_ctrl_lock", &self.cnnt_pad_ctrl_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures cnnt_pad_ctrl registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_cnnt_pad_ctrl(
        &mut self,
    ) -> READ_TEE_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        READ_TEE_CNNT_PAD_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures cnnt_pad_ctrl registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_cnnt_pad_ctrl(
        &mut self,
    ) -> READ_REE0_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        READ_REE0_CNNT_PAD_CTRL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures cnnt_pad_ctrl registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_cnnt_pad_ctrl(
        &mut self,
    ) -> READ_REE1_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        READ_REE1_CNNT_PAD_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures cnnt_pad_ctrl registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_cnnt_pad_ctrl(
        &mut self,
    ) -> READ_REE2_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        READ_REE2_CNNT_PAD_CTRL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures cnnt_pad_ctrl registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_cnnt_pad_ctrl(
        &mut self,
    ) -> WRITE_TEE_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        WRITE_TEE_CNNT_PAD_CTRL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures cnnt_pad_ctrl registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_cnnt_pad_ctrl(
        &mut self,
    ) -> WRITE_REE0_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        WRITE_REE0_CNNT_PAD_CTRL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures cnnt_pad_ctrl registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_cnnt_pad_ctrl(
        &mut self,
    ) -> WRITE_REE1_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        WRITE_REE1_CNNT_PAD_CTRL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures cnnt_pad_ctrl registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_cnnt_pad_ctrl(
        &mut self,
    ) -> WRITE_REE2_CNNT_PAD_CTRL_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        WRITE_REE2_CNNT_PAD_CTRL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock cnnt_pad_ctrl peri_apm configuration"]
    #[inline(always)]
    pub fn cnnt_pad_ctrl_lock(&mut self) -> CNNT_PAD_CTRL_LOCK_W<'_, CNNT_PAD_CTRL_CTRL_SPEC> {
        CNNT_PAD_CTRL_LOCK_W::new(self, 8)
    }
}
#[doc = "cnnt_pad_ctrl read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_pad_ctrl_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_pad_ctrl_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNNT_PAD_CTRL_CTRL_SPEC;
impl crate::RegisterSpec for CNNT_PAD_CTRL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnnt_pad_ctrl_ctrl::R`](R) reader structure"]
impl crate::Readable for CNNT_PAD_CTRL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnnt_pad_ctrl_ctrl::W`](W) writer structure"]
impl crate::Writable for CNNT_PAD_CTRL_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNNT_PAD_CTRL_CTRL to value 0x11"]
impl crate::Resettable for CNNT_PAD_CTRL_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
