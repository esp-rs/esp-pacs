#[doc = "Register `HUK_CTRL` reader"]
pub type R = crate::R<HUK_CTRL_SPEC>;
#[doc = "Register `HUK_CTRL` writer"]
pub type W = crate::W<HUK_CTRL_SPEC>;
#[doc = "Field `READ_TEE_HUK` reader - Configures huk registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HUK_R = crate::BitReader;
#[doc = "Field `READ_TEE_HUK` writer - Configures huk registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_HUK` reader - Configures huk registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HUK_R = crate::BitReader;
#[doc = "Field `READ_REE0_HUK` writer - Configures huk registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_HUK` reader - Configures huk registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HUK_R = crate::BitReader;
#[doc = "Field `READ_REE1_HUK` writer - Configures huk registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_HUK` reader - Configures huk registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HUK_R = crate::BitReader;
#[doc = "Field `READ_REE2_HUK` writer - Configures huk registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_HUK` reader - Configures huk registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HUK_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_HUK` writer - Configures huk registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_HUK` reader - Configures huk registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HUK_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_HUK` writer - Configures huk registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_HUK` reader - Configures huk registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HUK_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_HUK` writer - Configures huk registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_HUK` reader - Configures huk registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HUK_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_HUK` writer - Configures huk registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures huk registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_huk(&self) -> READ_TEE_HUK_R {
        READ_TEE_HUK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures huk registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_huk(&self) -> READ_REE0_HUK_R {
        READ_REE0_HUK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures huk registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_huk(&self) -> READ_REE1_HUK_R {
        READ_REE1_HUK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures huk registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_huk(&self) -> READ_REE2_HUK_R {
        READ_REE2_HUK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures huk registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_huk(&self) -> WRITE_TEE_HUK_R {
        WRITE_TEE_HUK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures huk registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_huk(&self) -> WRITE_REE0_HUK_R {
        WRITE_REE0_HUK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures huk registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_huk(&self) -> WRITE_REE1_HUK_R {
        WRITE_REE1_HUK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures huk registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_huk(&self) -> WRITE_REE2_HUK_R {
        WRITE_REE2_HUK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUK_CTRL")
            .field("read_tee_huk", &self.read_tee_huk())
            .field("read_ree0_huk", &self.read_ree0_huk())
            .field("read_ree1_huk", &self.read_ree1_huk())
            .field("read_ree2_huk", &self.read_ree2_huk())
            .field("write_tee_huk", &self.write_tee_huk())
            .field("write_ree0_huk", &self.write_ree0_huk())
            .field("write_ree1_huk", &self.write_ree1_huk())
            .field("write_ree2_huk", &self.write_ree2_huk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures huk registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_huk(&mut self) -> READ_TEE_HUK_W<'_, HUK_CTRL_SPEC> {
        READ_TEE_HUK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures huk registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_huk(&mut self) -> READ_REE0_HUK_W<'_, HUK_CTRL_SPEC> {
        READ_REE0_HUK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures huk registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_huk(&mut self) -> READ_REE1_HUK_W<'_, HUK_CTRL_SPEC> {
        READ_REE1_HUK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures huk registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_huk(&mut self) -> READ_REE2_HUK_W<'_, HUK_CTRL_SPEC> {
        READ_REE2_HUK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures huk registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_huk(&mut self) -> WRITE_TEE_HUK_W<'_, HUK_CTRL_SPEC> {
        WRITE_TEE_HUK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures huk registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_huk(&mut self) -> WRITE_REE0_HUK_W<'_, HUK_CTRL_SPEC> {
        WRITE_REE0_HUK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures huk registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_huk(&mut self) -> WRITE_REE1_HUK_W<'_, HUK_CTRL_SPEC> {
        WRITE_REE1_HUK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures huk registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_huk(&mut self) -> WRITE_REE2_HUK_W<'_, HUK_CTRL_SPEC> {
        WRITE_REE2_HUK_W::new(self, 7)
    }
}
#[doc = "lp_tee read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`huk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUK_CTRL_SPEC;
impl crate::RegisterSpec for HUK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huk_ctrl::R`](R) reader structure"]
impl crate::Readable for HUK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`huk_ctrl::W`](W) writer structure"]
impl crate::Writable for HUK_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUK_CTRL to value 0x11"]
impl crate::Resettable for HUK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
