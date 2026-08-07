#[doc = "Register `MEM_ACS_MONITOR_CTRL` reader"]
pub type R = crate::R<MEM_ACS_MONITOR_CTRL_SPEC>;
#[doc = "Register `MEM_ACS_MONITOR_CTRL` writer"]
pub type W = crate::W<MEM_ACS_MONITOR_CTRL_SPEC>;
#[doc = "Field `READ_TEE_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `READ_TEE_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `READ_REE0_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `READ_REE1_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `READ_REE2_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_MEM_ACS_MONITOR` reader - Configures mem_acs_monitor registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_MEM_ACS_MONITOR_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_MEM_ACS_MONITOR` writer - Configures mem_acs_monitor registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_MEM_ACS_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures mem_acs_monitor registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_mem_acs_monitor(&self) -> READ_TEE_MEM_ACS_MONITOR_R {
        READ_TEE_MEM_ACS_MONITOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures mem_acs_monitor registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_mem_acs_monitor(&self) -> READ_REE0_MEM_ACS_MONITOR_R {
        READ_REE0_MEM_ACS_MONITOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures mem_acs_monitor registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_mem_acs_monitor(&self) -> READ_REE1_MEM_ACS_MONITOR_R {
        READ_REE1_MEM_ACS_MONITOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures mem_acs_monitor registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_mem_acs_monitor(&self) -> READ_REE2_MEM_ACS_MONITOR_R {
        READ_REE2_MEM_ACS_MONITOR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures mem_acs_monitor registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_mem_acs_monitor(&self) -> WRITE_TEE_MEM_ACS_MONITOR_R {
        WRITE_TEE_MEM_ACS_MONITOR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures mem_acs_monitor registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_mem_acs_monitor(&self) -> WRITE_REE0_MEM_ACS_MONITOR_R {
        WRITE_REE0_MEM_ACS_MONITOR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures mem_acs_monitor registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_mem_acs_monitor(&self) -> WRITE_REE1_MEM_ACS_MONITOR_R {
        WRITE_REE1_MEM_ACS_MONITOR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures mem_acs_monitor registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_mem_acs_monitor(&self) -> WRITE_REE2_MEM_ACS_MONITOR_R {
        WRITE_REE2_MEM_ACS_MONITOR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_ACS_MONITOR_CTRL")
            .field("read_tee_mem_acs_monitor", &self.read_tee_mem_acs_monitor())
            .field(
                "read_ree0_mem_acs_monitor",
                &self.read_ree0_mem_acs_monitor(),
            )
            .field(
                "read_ree1_mem_acs_monitor",
                &self.read_ree1_mem_acs_monitor(),
            )
            .field(
                "read_ree2_mem_acs_monitor",
                &self.read_ree2_mem_acs_monitor(),
            )
            .field(
                "write_tee_mem_acs_monitor",
                &self.write_tee_mem_acs_monitor(),
            )
            .field(
                "write_ree0_mem_acs_monitor",
                &self.write_ree0_mem_acs_monitor(),
            )
            .field(
                "write_ree1_mem_acs_monitor",
                &self.write_ree1_mem_acs_monitor(),
            )
            .field(
                "write_ree2_mem_acs_monitor",
                &self.write_ree2_mem_acs_monitor(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures mem_acs_monitor registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_mem_acs_monitor(
        &mut self,
    ) -> READ_TEE_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        READ_TEE_MEM_ACS_MONITOR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures mem_acs_monitor registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_mem_acs_monitor(
        &mut self,
    ) -> READ_REE0_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        READ_REE0_MEM_ACS_MONITOR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures mem_acs_monitor registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_mem_acs_monitor(
        &mut self,
    ) -> READ_REE1_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        READ_REE1_MEM_ACS_MONITOR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures mem_acs_monitor registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_mem_acs_monitor(
        &mut self,
    ) -> READ_REE2_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        READ_REE2_MEM_ACS_MONITOR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures mem_acs_monitor registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_mem_acs_monitor(
        &mut self,
    ) -> WRITE_TEE_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        WRITE_TEE_MEM_ACS_MONITOR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures mem_acs_monitor registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_mem_acs_monitor(
        &mut self,
    ) -> WRITE_REE0_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        WRITE_REE0_MEM_ACS_MONITOR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures mem_acs_monitor registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_mem_acs_monitor(
        &mut self,
    ) -> WRITE_REE1_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        WRITE_REE1_MEM_ACS_MONITOR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures mem_acs_monitor registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_mem_acs_monitor(
        &mut self,
    ) -> WRITE_REE2_MEM_ACS_MONITOR_W<'_, MEM_ACS_MONITOR_CTRL_SPEC> {
        WRITE_REE2_MEM_ACS_MONITOR_W::new(self, 7)
    }
}
#[doc = "mem_acs_monitor read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_acs_monitor_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_acs_monitor_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ACS_MONITOR_CTRL_SPEC;
impl crate::RegisterSpec for MEM_ACS_MONITOR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_acs_monitor_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_ACS_MONITOR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_acs_monitor_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_ACS_MONITOR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_ACS_MONITOR_CTRL to value 0x11"]
impl crate::Resettable for MEM_ACS_MONITOR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
