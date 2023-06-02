#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_6` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_6` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT` reader - Core1 access bt permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT` writer - Core1 access bt permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0` reader - Core1 access i2c_ext0 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0` writer - Core1 access i2c_ext0 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0` reader - Core1 access uhci0 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0` writer - Core1 access uhci0 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST` reader - Core1 access slchost permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST` writer - Core1 access slchost permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT` reader - Core1 access rmt permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT` writer - Core1 access rmt permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT` reader - Core1 access pcnt permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT` writer - Core1 access pcnt permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC` reader - Core1 access slc permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC` writer - Core1 access slc permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC` reader - Core1 access ledc permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC` writer - Core1 access ledc permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP` reader - Core1 access backup permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP` writer - Core1 access backup permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB` reader - Core1 access bb permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB` writer - Core1 access bb permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0` reader - Core1 access pwm0 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0` writer - Core1 access pwm0 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP` reader - Core1 access timergroup permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP` writer - Core1 access timergroup permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1` reader - Core1 access timergroup1 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1` writer - Core1 access timergroup1 permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER` reader - Core1 access systimer permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER` writer - Core1 access systimer permission in world1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_6_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core1 access bt permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_bt(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core1 access i2c_ext0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_i2c_ext0(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core1 access uhci0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_uhci0(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core1 access slchost permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_slchost(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core1 access rmt permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_rmt(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core1 access pcnt permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_pcnt(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core1 access slc permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_slc(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core1 access ledc permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_ledc(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Core1 access backup permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_backup(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core1 access bb permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_bb(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core1 access pwm0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_pwm0(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core1 access timergroup permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_timergroup(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core1 access timergroup1 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_timergroup1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core1 access systimer permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_systimer(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_CONSTRAIN_6")
            .field(
                "core_1_pif_pms_constrain_world_1_bt",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_bt().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_i2c_ext0",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_1_i2c_ext0().bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_uhci0",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_uhci0().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_slchost",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_slchost().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_rmt",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_rmt().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_pcnt",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_pcnt().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_slc",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_slc().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_ledc",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_ledc().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_backup",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_backup().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_bb",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_bb().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_pwm0",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_1_pwm0().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_timergroup",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_1_timergroup().bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_timergroup1",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_1_timergroup1().bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_world_1_systimer",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_1_systimer().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_PIF_PMS_CONSTRAIN_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core1 access bt permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_bt(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<0> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BT_W::new(self)
    }
    #[doc = "Bits 4:5 - Core1 access i2c_ext0 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_i2c_ext0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<4> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W::new(self)
    }
    #[doc = "Bits 6:7 - Core1 access uhci0 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_uhci0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<6> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W::new(self)
    }
    #[doc = "Bits 8:9 - Core1 access slchost permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_slchost(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_W<8> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_W::new(self)
    }
    #[doc = "Bits 10:11 - Core1 access rmt permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_rmt(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<10> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W::new(self)
    }
    #[doc = "Bits 12:13 - Core1 access pcnt permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_pcnt(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_W<12> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_W::new(self)
    }
    #[doc = "Bits 14:15 - Core1 access slc permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_slc(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC_W<14> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SLC_W::new(self)
    }
    #[doc = "Bits 16:17 - Core1 access ledc permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_ledc(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<16> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W::new(self)
    }
    #[doc = "Bits 18:19 - Core1 access backup permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_backup(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_W<18> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_W::new(self)
    }
    #[doc = "Bits 22:23 - Core1 access bb permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_bb(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<22> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_BB_W::new(self)
    }
    #[doc = "Bits 24:25 - Core1 access pwm0 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_pwm0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_W<24> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_W::new(self)
    }
    #[doc = "Bits 26:27 - Core1 access timergroup permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_timergroup(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<26> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W::new(self)
    }
    #[doc = "Bits 28:29 - Core1 access timergroup1 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_timergroup1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<28> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W::new(self)
    }
    #[doc = "Bits 30:31 - Core1 access systimer permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_1_systimer(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<30> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 access peripherals permission configuration register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_6](index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_6::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_6::W](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_6 to value 0xffcf_fff3"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffcf_fff3;
}
