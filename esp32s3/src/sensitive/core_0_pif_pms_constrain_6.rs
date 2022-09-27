#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_6` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_6` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT` reader - Core0 access bt permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT` writer - Core0 access bt permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0` reader - Core0 access i2c_ext0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0` writer - Core0 access i2c_ext0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0` reader - Core0 access uhci0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0` writer - Core0 access uhci0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST` reader - Core0 access slchost permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST` writer - Core0 access slchost permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT` reader - Core0 access rmt permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT` writer - Core0 access rmt permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT` reader - Core0 access pcnt permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT` writer - Core0 access pcnt permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC` reader - Core0 access slc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC` writer - Core0 access slc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC` reader - Core0 access ledc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC` writer - Core0 access ledc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP` reader - Core0 access backup permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP` writer - Core0 access backup permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB` reader - Core0 access bb permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB` writer - Core0 access bb permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0` reader - Core0 access pwm0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0` writer - Core0 access pwm0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP` reader - Core0 access timergroup permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP` writer - Core0 access timergroup permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1` reader - Core0 access timergroup1 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1` writer - Core0 access timergroup1 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER` reader - Core0 access systimer permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER` writer - Core0 access systimer permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_6_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core0 access bt permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core0 access i2c_ext0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2c_ext0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core0 access uhci0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_uhci0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core0 access slchost permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_slchost(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core0 access rmt permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_rmt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core0 access pcnt permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_pcnt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core0 access slc permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_slc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core0 access ledc permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ledc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Core0 access backup permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_backup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core0 access bb permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bb(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core0 access pwm0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_pwm0(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core0 access timergroup permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core0 access timergroup1 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core0 access systimer permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_systimer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core0 access bt permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W::new(self)
    }
    #[doc = "Bits 4:5 - Core0 access i2c_ext0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2c_ext0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W::new(self)
    }
    #[doc = "Bits 6:7 - Core0 access uhci0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_uhci0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W::new(self)
    }
    #[doc = "Bits 8:9 - Core0 access slchost permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_slchost(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLCHOST_W::new(self)
    }
    #[doc = "Bits 10:11 - Core0 access rmt permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_rmt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W::new(self)
    }
    #[doc = "Bits 12:13 - Core0 access pcnt permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_pcnt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_W<12> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PCNT_W::new(self)
    }
    #[doc = "Bits 14:15 - Core0 access slc permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_slc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SLC_W::new(self)
    }
    #[doc = "Bits 16:17 - Core0 access ledc permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ledc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W::new(self)
    }
    #[doc = "Bits 18:19 - Core0 access backup permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_backup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_W<18> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BACKUP_W::new(self)
    }
    #[doc = "Bits 22:23 - Core0 access bb permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bb(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<22> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W::new(self)
    }
    #[doc = "Bits 24:25 - Core0 access pwm0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_pwm0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_W<24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWM0_W::new(self)
    }
    #[doc = "Bits 26:27 - Core0 access timergroup permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W::new(self)
    }
    #[doc = "Bits 28:29 - Core0 access timergroup1 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W::new(self)
    }
    #[doc = "Bits 30:31 - Core0 access systimer permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_systimer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<30> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 access peripherals permission configuration register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_6](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_6::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_6::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_6 to value 0xffcf_fff3"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffcf_fff3
    }
}
