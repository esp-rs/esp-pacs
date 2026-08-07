#[doc = "Register `SYSTIMER_CTRL` reader"]
pub type R = crate::R<SYSTIMER_CTRL_SPEC>;
#[doc = "Register `SYSTIMER_CTRL` writer"]
pub type W = crate::W<SYSTIMER_CTRL_SPEC>;
#[doc = "Field `READ_TEE_SYSTIMER` reader - Configures systimer registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_SYSTIMER_R = crate::BitReader;
#[doc = "Field `READ_TEE_SYSTIMER` writer - Configures systimer registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_SYSTIMER` reader - Configures systimer registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_SYSTIMER_R = crate::BitReader;
#[doc = "Field `READ_REE0_SYSTIMER` writer - Configures systimer registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_SYSTIMER` reader - Configures systimer registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_SYSTIMER_R = crate::BitReader;
#[doc = "Field `READ_REE1_SYSTIMER` writer - Configures systimer registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_SYSTIMER` reader - Configures systimer registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_SYSTIMER_R = crate::BitReader;
#[doc = "Field `READ_REE2_SYSTIMER` writer - Configures systimer registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_SYSTIMER` reader - Configures systimer registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_SYSTIMER_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_SYSTIMER` writer - Configures systimer registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_SYSTIMER` reader - Configures systimer registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_SYSTIMER_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_SYSTIMER` writer - Configures systimer registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_SYSTIMER` reader - Configures systimer registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_SYSTIMER_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_SYSTIMER` writer - Configures systimer registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_SYSTIMER` reader - Configures systimer registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_SYSTIMER_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_SYSTIMER` writer - Configures systimer registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_SYSTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures systimer registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_systimer(&self) -> READ_TEE_SYSTIMER_R {
        READ_TEE_SYSTIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures systimer registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_systimer(&self) -> READ_REE0_SYSTIMER_R {
        READ_REE0_SYSTIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures systimer registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_systimer(&self) -> READ_REE1_SYSTIMER_R {
        READ_REE1_SYSTIMER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures systimer registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_systimer(&self) -> READ_REE2_SYSTIMER_R {
        READ_REE2_SYSTIMER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures systimer registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_systimer(&self) -> WRITE_TEE_SYSTIMER_R {
        WRITE_TEE_SYSTIMER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures systimer registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_systimer(&self) -> WRITE_REE0_SYSTIMER_R {
        WRITE_REE0_SYSTIMER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures systimer registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_systimer(&self) -> WRITE_REE1_SYSTIMER_R {
        WRITE_REE1_SYSTIMER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures systimer registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_systimer(&self) -> WRITE_REE2_SYSTIMER_R {
        WRITE_REE2_SYSTIMER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_CTRL")
            .field("read_tee_systimer", &self.read_tee_systimer())
            .field("read_ree0_systimer", &self.read_ree0_systimer())
            .field("read_ree1_systimer", &self.read_ree1_systimer())
            .field("read_ree2_systimer", &self.read_ree2_systimer())
            .field("write_tee_systimer", &self.write_tee_systimer())
            .field("write_ree0_systimer", &self.write_ree0_systimer())
            .field("write_ree1_systimer", &self.write_ree1_systimer())
            .field("write_ree2_systimer", &self.write_ree2_systimer())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures systimer registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_systimer(&mut self) -> READ_TEE_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        READ_TEE_SYSTIMER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures systimer registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_systimer(&mut self) -> READ_REE0_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        READ_REE0_SYSTIMER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures systimer registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_systimer(&mut self) -> READ_REE1_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        READ_REE1_SYSTIMER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures systimer registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_systimer(&mut self) -> READ_REE2_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        READ_REE2_SYSTIMER_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures systimer registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_systimer(&mut self) -> WRITE_TEE_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        WRITE_TEE_SYSTIMER_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures systimer registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_systimer(&mut self) -> WRITE_REE0_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        WRITE_REE0_SYSTIMER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures systimer registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_systimer(&mut self) -> WRITE_REE1_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        WRITE_REE1_SYSTIMER_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures systimer registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_systimer(&mut self) -> WRITE_REE2_SYSTIMER_W<'_, SYSTIMER_CTRL_SPEC> {
        WRITE_REE2_SYSTIMER_W::new(self, 7)
    }
}
#[doc = "systimer read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_CTRL_SPEC;
impl crate::RegisterSpec for SYSTIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_ctrl::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_ctrl::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTIMER_CTRL to value 0x11"]
impl crate::Resettable for SYSTIMER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
