///Register `BACKUP_BUS_PMS_CONSTRAIN_2` reader
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
///Register `BACKUP_BUS_PMS_CONSTRAIN_2` writer
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_BT` reader - BackUp access bt permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_BT` writer - BackUp access bt permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` reader - BackUp access i2c_ext0 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0` writer - BackUp access i2c_ext0 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` reader - BackUp access uhci0 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_UHCI0` writer - BackUp access uhci0 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` reader - BackUp access slchost permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_SLCHOST` writer - BackUp access slchost permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` reader - BackUp access rmt permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_RMT` writer - BackUp access rmt permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_RMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` reader - BackUp access pcnt permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_PCNT_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_PCNT` writer - BackUp access pcnt permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` reader - BackUp access slc permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_SLC_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_SLC` writer - BackUp access slc permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_SLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` reader - BackUp access ledc permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_LEDC` writer - BackUp access ledc permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` reader - BackUp access backup permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_BACKUP` writer - BackUp access backup permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_BB` reader - BackUp access bb permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_BB` writer - BackUp access bb permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_BB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` reader - BackUp access pwm0 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM0_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_PWM0` writer - BackUp access pwm0 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` reader - BackUp access timergroup permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP` writer - BackUp access timergroup permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` reader - BackUp access timergroup1 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1` writer - BackUp access timergroup1 permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` reader - BackUp access systimer permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_R = crate::FieldReader;
///Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER` writer - BackUp access systimer permission.
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - BackUp access bt permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - BackUp access i2c_ext0 permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - BackUp access uhci0 permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uhci0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - BackUp access slchost permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slchost(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - BackUp access rmt permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rmt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - BackUp access pcnt permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pcnt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_R {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - BackUp access slc permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_slc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_R {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - BackUp access ledc permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ledc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_R {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - BackUp access backup permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_backup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 22:23 - BackUp access bb permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BB_R {
        BACKUP_BUS_PMS_CONSTRAIN_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - BackUp access pwm0 permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - BackUp access timergroup permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - BackUp access timergroup1 permission.
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timergroup1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - BackUp access systimer permission.
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
                &self.backup_bus_pms_constrain_bt(),
            )
            .field(
                "backup_bus_pms_constrain_i2c_ext0",
                &self.backup_bus_pms_constrain_i2c_ext0(),
            )
            .field(
                "backup_bus_pms_constrain_uhci0",
                &self.backup_bus_pms_constrain_uhci0(),
            )
            .field(
                "backup_bus_pms_constrain_slchost",
                &self.backup_bus_pms_constrain_slchost(),
            )
            .field(
                "backup_bus_pms_constrain_rmt",
                &self.backup_bus_pms_constrain_rmt(),
            )
            .field(
                "backup_bus_pms_constrain_pcnt",
                &self.backup_bus_pms_constrain_pcnt(),
            )
            .field(
                "backup_bus_pms_constrain_slc",
                &self.backup_bus_pms_constrain_slc(),
            )
            .field(
                "backup_bus_pms_constrain_ledc",
                &self.backup_bus_pms_constrain_ledc(),
            )
            .field(
                "backup_bus_pms_constrain_backup",
                &self.backup_bus_pms_constrain_backup(),
            )
            .field(
                "backup_bus_pms_constrain_bb",
                &self.backup_bus_pms_constrain_bb(),
            )
            .field(
                "backup_bus_pms_constrain_pwm0",
                &self.backup_bus_pms_constrain_pwm0(),
            )
            .field(
                "backup_bus_pms_constrain_timergroup",
                &self.backup_bus_pms_constrain_timergroup(),
            )
            .field(
                "backup_bus_pms_constrain_timergroup1",
                &self.backup_bus_pms_constrain_timergroup1(),
            )
            .field(
                "backup_bus_pms_constrain_systimer",
                &self.backup_bus_pms_constrain_systimer(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:1 - BackUp access bt permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_BT_W::new(self, 0)
    }
    ///Bits 4:5 - BackUp access i2c_ext0 permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2c_ext0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0_W::new(self, 4)
    }
    ///Bits 6:7 - BackUp access uhci0 permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_uhci0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_UHCI0_W::new(self, 6)
    }
    ///Bits 8:9 - BackUp access slchost permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_slchost(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SLCHOST_W::new(self, 8)
    }
    ///Bits 10:11 - BackUp access rmt permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rmt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RMT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_RMT_W::new(self, 10)
    }
    ///Bits 12:13 - BackUp access pcnt permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pcnt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_PCNT_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_PCNT_W::new(self, 12)
    }
    ///Bits 14:15 - BackUp access slc permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_slc(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SLC_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SLC_W::new(self, 14)
    }
    ///Bits 16:17 - BackUp access ledc permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_ledc(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_LEDC_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_LEDC_W::new(self, 16)
    }
    ///Bits 18:19 - BackUp access backup permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_backup(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_BACKUP_W::new(self, 18)
    }
    ///Bits 22:23 - BackUp access bb permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bb(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_BB_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_BB_W::new(self, 22)
    }
    ///Bits 24:25 - BackUp access pwm0 permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pwm0(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_PWM0_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_PWM0_W::new(self, 24)
    }
    ///Bits 26:27 - BackUp access timergroup permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timergroup(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP_W::new(self, 26)
    }
    ///Bits 28:29 - BackUp access timergroup1 permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timergroup1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1_W::new(self, 28)
    }
    ///Bits 30:31 - BackUp access systimer permission.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_systimer(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER_W::new(self, 30)
    }
}
/**BackUp access peripherals permission configuration register 2.

You can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BACKUP_BUS_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`backup_bus_pms_constrain_2::R`](R) reader structure
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {}
///`write(|w| ..)` method takes [`backup_bus_pms_constrain_2::W`](W) writer structure
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_2 to value 0xffcf_fff3
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_2_SPEC {
    const RESET_VALUE: u32 = 0xffcf_fff3;
}
