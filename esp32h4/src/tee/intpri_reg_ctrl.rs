#[doc = "Register `INTPRI_REG_CTRL` reader"]
pub type R = crate::R<INTPRI_REG_CTRL_SPEC>;
#[doc = "Register `INTPRI_REG_CTRL` writer"]
pub type W = crate::W<INTPRI_REG_CTRL_SPEC>;
#[doc = "Field `READ_TEE_INTPRI` reader - Configures intpri_reg registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_INTPRI_R = crate::BitReader;
#[doc = "Field `READ_TEE_INTPRI` writer - Configures intpri_reg registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_INTPRI` reader - Configures intpri_reg registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_INTPRI_R = crate::BitReader;
#[doc = "Field `READ_REE0_INTPRI` writer - Configures intpri_reg registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_INTPRI` reader - Configures intpri_reg registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_INTPRI_R = crate::BitReader;
#[doc = "Field `READ_REE1_INTPRI` writer - Configures intpri_reg registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_INTPRI` reader - Configures intpri_reg registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_INTPRI_R = crate::BitReader;
#[doc = "Field `READ_REE2_INTPRI` writer - Configures intpri_reg registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_INTPRI` reader - Configures intpri_reg registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_INTPRI_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_INTPRI` writer - Configures intpri_reg registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_INTPRI` reader - Configures intpri_reg registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_INTPRI_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_INTPRI` writer - Configures intpri_reg registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_INTPRI` reader - Configures intpri_reg registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_INTPRI_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_INTPRI` writer - Configures intpri_reg registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_INTPRI` reader - Configures intpri_reg registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_INTPRI_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_INTPRI` writer - Configures intpri_reg registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_INTPRI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures intpri_reg registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_intpri(&self) -> READ_TEE_INTPRI_R {
        READ_TEE_INTPRI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures intpri_reg registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_intpri(&self) -> READ_REE0_INTPRI_R {
        READ_REE0_INTPRI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures intpri_reg registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_intpri(&self) -> READ_REE1_INTPRI_R {
        READ_REE1_INTPRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures intpri_reg registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_intpri(&self) -> READ_REE2_INTPRI_R {
        READ_REE2_INTPRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures intpri_reg registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_intpri(&self) -> WRITE_TEE_INTPRI_R {
        WRITE_TEE_INTPRI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures intpri_reg registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_intpri(&self) -> WRITE_REE0_INTPRI_R {
        WRITE_REE0_INTPRI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures intpri_reg registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_intpri(&self) -> WRITE_REE1_INTPRI_R {
        WRITE_REE1_INTPRI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures intpri_reg registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_intpri(&self) -> WRITE_REE2_INTPRI_R {
        WRITE_REE2_INTPRI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPRI_REG_CTRL")
            .field("read_tee_intpri", &self.read_tee_intpri())
            .field("read_ree0_intpri", &self.read_ree0_intpri())
            .field("read_ree1_intpri", &self.read_ree1_intpri())
            .field("read_ree2_intpri", &self.read_ree2_intpri())
            .field("write_tee_intpri", &self.write_tee_intpri())
            .field("write_ree0_intpri", &self.write_ree0_intpri())
            .field("write_ree1_intpri", &self.write_ree1_intpri())
            .field("write_ree2_intpri", &self.write_ree2_intpri())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures intpri_reg registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_intpri(&mut self) -> READ_TEE_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        READ_TEE_INTPRI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures intpri_reg registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_intpri(&mut self) -> READ_REE0_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        READ_REE0_INTPRI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures intpri_reg registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_intpri(&mut self) -> READ_REE1_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        READ_REE1_INTPRI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures intpri_reg registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_intpri(&mut self) -> READ_REE2_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        READ_REE2_INTPRI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures intpri_reg registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_intpri(&mut self) -> WRITE_TEE_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        WRITE_TEE_INTPRI_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures intpri_reg registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_intpri(&mut self) -> WRITE_REE0_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        WRITE_REE0_INTPRI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures intpri_reg registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_intpri(&mut self) -> WRITE_REE1_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        WRITE_REE1_INTPRI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures intpri_reg registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_intpri(&mut self) -> WRITE_REE2_INTPRI_W<'_, INTPRI_REG_CTRL_SPEC> {
        WRITE_REE2_INTPRI_W::new(self, 7)
    }
}
#[doc = "intpri_reg read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpri_reg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpri_reg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTPRI_REG_CTRL_SPEC;
impl crate::RegisterSpec for INTPRI_REG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpri_reg_ctrl::R`](R) reader structure"]
impl crate::Readable for INTPRI_REG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intpri_reg_ctrl::W`](W) writer structure"]
impl crate::Writable for INTPRI_REG_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTPRI_REG_CTRL to value 0x11"]
impl crate::Resettable for INTPRI_REG_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
