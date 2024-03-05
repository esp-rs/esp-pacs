#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_2` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` reader - backup_bus_pms_constrain_bt"]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT` writer - backup_bus_pms_constrain_bt"]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` reader - backup_bus_pms_constrain_i2c_ext0"]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` writer - backup_bus_pms_constrain_i2c_ext0"]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` reader - backup_bus_pms_constrain_uhci0"]
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` writer - backup_bus_pms_constrain_uhci0"]
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` reader - backup_bus_pms_constrain_rmt"]
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` writer - backup_bus_pms_constrain_rmt"]
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` reader - backup_bus_pms_constrain_ledc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` writer - backup_bus_pms_constrain_ledc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` reader - backup_bus_pms_constrain_bb"]
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BB` writer - backup_bus_pms_constrain_bb"]
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` reader - backup_bus_pms_constrain_timergroup"]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` writer - backup_bus_pms_constrain_timergroup"]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` reader - backup_bus_pms_constrain_timergroup1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` writer - backup_bus_pms_constrain_timergroup1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` reader - backup_bus_pms_constrain_systimer"]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` writer - backup_bus_pms_constrain_systimer"]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_bt"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_i2c_ext0"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_uhci0"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_rmt"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - backup_bus_pms_constrain_ledc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 22:23 - backup_bus_pms_constrain_bb"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_R {
        BACKUP_BUS_PMS_CONSTRAIN_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_timergroup"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - backup_bus_pms_constrain_timergroup1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - backup_bus_pms_constrain_systimer"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_systimer(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_2")
            .field(
                "backup_bus_pms_constrain_bt",
                &format_args!("{}", self.backup_bus_pms_constrain_bt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_i2c_ext0",
                &format_args!("{}", self.backup_bus_pms_constrain_i2c_ext0().bits()),
            )
            .field(
                "backup_bus_pms_constrain_uhci0",
                &format_args!("{}", self.backup_bus_pms_constrain_uhci0().bits()),
            )
            .field(
                "backup_bus_pms_constrain_rmt",
                &format_args!("{}", self.backup_bus_pms_constrain_rmt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_ledc",
                &format_args!("{}", self.backup_bus_pms_constrain_ledc().bits()),
            )
            .field(
                "backup_bus_pms_constrain_bb",
                &format_args!("{}", self.backup_bus_pms_constrain_bb().bits()),
            )
            .field(
                "backup_bus_pms_constrain_timergroup",
                &format_args!("{}", self.backup_bus_pms_constrain_timergroup().bits()),
            )
            .field(
                "backup_bus_pms_constrain_timergroup1",
                &format_args!("{}", self.backup_bus_pms_constrain_timergroup1().bits()),
            )
            .field(
                "backup_bus_pms_constrain_systimer",
                &format_args!("{}", self.backup_bus_pms_constrain_systimer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_bt"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_BT_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_i2c_ext0"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2c_ext0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_uhci0"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_uhci0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_rmt"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rmt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_W::new(self, 10)
    }
    #[doc = "Bits 16:17 - backup_bus_pms_constrain_ledc"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_ledc(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_W::new(self, 16)
    }
    #[doc = "Bits 22:23 - backup_bus_pms_constrain_bb"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bb(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BB_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_BB_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_timergroup"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timergroup(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - backup_bus_pms_constrain_timergroup1"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timergroup1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - backup_bus_pms_constrain_systimer"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_systimer(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W::new(self, 30)
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_2::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_2::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_2 to value 0xfcc3_0cf3"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    const RESET_VALUE: u32 = 0xfcc3_0cf3;
}
