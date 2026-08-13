#[doc = "Register `ETM_CTRL` reader"]
pub type R = crate::R<ETM_CTRL_SPEC>;
#[doc = "Register `ETM_CTRL` writer"]
pub type W = crate::W<ETM_CTRL_SPEC>;
#[doc = "Field `READ_TEE_ETM` reader - Configures etm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_ETM_R = crate::BitReader;
#[doc = "Field `READ_TEE_ETM` writer - Configures etm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_ETM` reader - Configures etm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_ETM_R = crate::BitReader;
#[doc = "Field `READ_REE0_ETM` writer - Configures etm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_ETM` reader - Configures etm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_ETM_R = crate::BitReader;
#[doc = "Field `READ_REE1_ETM` writer - Configures etm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_ETM` reader - Configures etm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_ETM_R = crate::BitReader;
#[doc = "Field `READ_REE2_ETM` writer - Configures etm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_ETM` reader - Configures etm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_ETM_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_ETM` writer - Configures etm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_ETM` reader - Configures etm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_ETM_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_ETM` writer - Configures etm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_ETM` reader - Configures etm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_ETM_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_ETM` writer - Configures etm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_ETM` reader - Configures etm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_ETM_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_ETM` writer - Configures etm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_ETM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures etm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_etm(&self) -> READ_TEE_ETM_R {
        READ_TEE_ETM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures etm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_etm(&self) -> READ_REE0_ETM_R {
        READ_REE0_ETM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures etm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_etm(&self) -> READ_REE1_ETM_R {
        READ_REE1_ETM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures etm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_etm(&self) -> READ_REE2_ETM_R {
        READ_REE2_ETM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures etm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_etm(&self) -> WRITE_TEE_ETM_R {
        WRITE_TEE_ETM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures etm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_etm(&self) -> WRITE_REE0_ETM_R {
        WRITE_REE0_ETM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures etm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_etm(&self) -> WRITE_REE1_ETM_R {
        WRITE_REE1_ETM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures etm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_etm(&self) -> WRITE_REE2_ETM_R {
        WRITE_REE2_ETM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CTRL")
            .field("read_tee_etm", &self.read_tee_etm())
            .field("read_ree0_etm", &self.read_ree0_etm())
            .field("read_ree1_etm", &self.read_ree1_etm())
            .field("read_ree2_etm", &self.read_ree2_etm())
            .field("write_tee_etm", &self.write_tee_etm())
            .field("write_ree0_etm", &self.write_ree0_etm())
            .field("write_ree1_etm", &self.write_ree1_etm())
            .field("write_ree2_etm", &self.write_ree2_etm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures etm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_etm(&mut self) -> READ_TEE_ETM_W<'_, ETM_CTRL_SPEC> {
        READ_TEE_ETM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures etm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_etm(&mut self) -> READ_REE0_ETM_W<'_, ETM_CTRL_SPEC> {
        READ_REE0_ETM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures etm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_etm(&mut self) -> READ_REE1_ETM_W<'_, ETM_CTRL_SPEC> {
        READ_REE1_ETM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures etm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_etm(&mut self) -> READ_REE2_ETM_W<'_, ETM_CTRL_SPEC> {
        READ_REE2_ETM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures etm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_etm(&mut self) -> WRITE_TEE_ETM_W<'_, ETM_CTRL_SPEC> {
        WRITE_TEE_ETM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures etm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_etm(&mut self) -> WRITE_REE0_ETM_W<'_, ETM_CTRL_SPEC> {
        WRITE_REE0_ETM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures etm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_etm(&mut self) -> WRITE_REE1_ETM_W<'_, ETM_CTRL_SPEC> {
        WRITE_REE1_ETM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures etm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_etm(&mut self) -> WRITE_REE2_ETM_W<'_, ETM_CTRL_SPEC> {
        WRITE_REE2_ETM_W::new(self, 7)
    }
}
#[doc = "etm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_CTRL_SPEC;
impl crate::RegisterSpec for ETM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_ctrl::R`](R) reader structure"]
impl crate::Readable for ETM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_ctrl::W`](W) writer structure"]
impl crate::Writable for ETM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_CTRL to value 0x11"]
impl crate::Resettable for ETM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
