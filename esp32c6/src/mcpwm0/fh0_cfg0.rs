#[doc = "Register `FH0_CFG0` reader"]
pub struct R(crate::R<FH0_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH0_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH0_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH0_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FH0_CFG0` writer"]
pub struct W(crate::W<FH0_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FH0_CFG0_SPEC>;
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
impl From<crate::W<FH0_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FH0_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZ0_SW_CBC` reader - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_SW_CBC_R = crate::BitReader;
#[doc = "Field `TZ0_SW_CBC` writer - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_SW_CBC_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_F2_CBC` reader - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_F2_CBC_R = crate::BitReader;
#[doc = "Field `TZ0_F2_CBC` writer - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_F2_CBC_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_F1_CBC` reader - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_F1_CBC_R = crate::BitReader;
#[doc = "Field `TZ0_F1_CBC` writer - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_F1_CBC_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_F0_CBC` reader - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_F0_CBC_R = crate::BitReader;
#[doc = "Field `TZ0_F0_CBC` writer - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ0_F0_CBC_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_SW_OST` reader - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_SW_OST_R = crate::BitReader;
#[doc = "Field `TZ0_SW_OST` writer - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_SW_OST_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_F2_OST` reader - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_F2_OST_R = crate::BitReader;
#[doc = "Field `TZ0_F2_OST` writer - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_F2_OST_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_F1_OST` reader - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_F1_OST_R = crate::BitReader;
#[doc = "Field `TZ0_F1_OST` writer - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_F1_OST_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_F0_OST` reader - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_F0_OST_R = crate::BitReader;
#[doc = "Field `TZ0_F0_OST` writer - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ0_F0_OST_W<'a, const O: u8> = crate::BitWriter<'a, FH0_CFG0_SPEC, O>;
#[doc = "Field `TZ0_A_CBC_D` reader - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_CBC_D_R = crate::FieldReader;
#[doc = "Field `TZ0_A_CBC_D` writer - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_CBC_D_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_A_CBC_U` reader - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_CBC_U_R = crate::FieldReader;
#[doc = "Field `TZ0_A_CBC_U` writer - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_CBC_U_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_A_OST_D` reader - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_OST_D_R = crate::FieldReader;
#[doc = "Field `TZ0_A_OST_D` writer - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_OST_D_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_A_OST_U` reader - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_OST_U_R = crate::FieldReader;
#[doc = "Field `TZ0_A_OST_U` writer - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_A_OST_U_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_B_CBC_D` reader - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_CBC_D_R = crate::FieldReader;
#[doc = "Field `TZ0_B_CBC_D` writer - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_CBC_D_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_B_CBC_U` reader - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing,1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_CBC_U_R = crate::FieldReader;
#[doc = "Field `TZ0_B_CBC_U` writer - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing,1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_CBC_U_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_B_OST_D` reader - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_OST_D_R = crate::FieldReader;
#[doc = "Field `TZ0_B_OST_D` writer - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_OST_D_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
#[doc = "Field `TZ0_B_OST_U` reader - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_OST_U_R = crate::FieldReader;
#[doc = "Field `TZ0_B_OST_U` writer - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ0_B_OST_U_W<'a, const O: u8> = crate::FieldWriter<'a, FH0_CFG0_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_sw_cbc(&self) -> TZ0_SW_CBC_R {
        TZ0_SW_CBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_f2_cbc(&self) -> TZ0_F2_CBC_R {
        TZ0_F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_f1_cbc(&self) -> TZ0_F1_CBC_R {
        TZ0_F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_f0_cbc(&self) -> TZ0_F0_CBC_R {
        TZ0_F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_sw_ost(&self) -> TZ0_SW_OST_R {
        TZ0_SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_f2_ost(&self) -> TZ0_F2_OST_R {
        TZ0_F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_f1_ost(&self) -> TZ0_F1_OST_R {
        TZ0_F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz0_f0_ost(&self) -> TZ0_F0_OST_R {
        TZ0_F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_a_cbc_d(&self) -> TZ0_A_CBC_D_R {
        TZ0_A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_a_cbc_u(&self) -> TZ0_A_CBC_U_R {
        TZ0_A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_a_ost_d(&self) -> TZ0_A_OST_D_R {
        TZ0_A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_a_ost_u(&self) -> TZ0_A_OST_U_R {
        TZ0_A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_b_cbc_d(&self) -> TZ0_B_CBC_D_R {
        TZ0_B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing,1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_b_cbc_u(&self) -> TZ0_B_CBC_U_R {
        TZ0_B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_b_ost_d(&self) -> TZ0_B_OST_D_R {
        TZ0_B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz0_b_ost_u(&self) -> TZ0_B_OST_U_R {
        TZ0_B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH0_CFG0")
            .field("tz0_sw_cbc", &format_args!("{}", self.tz0_sw_cbc().bit()))
            .field("tz0_f2_cbc", &format_args!("{}", self.tz0_f2_cbc().bit()))
            .field("tz0_f1_cbc", &format_args!("{}", self.tz0_f1_cbc().bit()))
            .field("tz0_f0_cbc", &format_args!("{}", self.tz0_f0_cbc().bit()))
            .field("tz0_sw_ost", &format_args!("{}", self.tz0_sw_ost().bit()))
            .field("tz0_f2_ost", &format_args!("{}", self.tz0_f2_ost().bit()))
            .field("tz0_f1_ost", &format_args!("{}", self.tz0_f1_ost().bit()))
            .field("tz0_f0_ost", &format_args!("{}", self.tz0_f0_ost().bit()))
            .field(
                "tz0_a_cbc_d",
                &format_args!("{}", self.tz0_a_cbc_d().bits()),
            )
            .field(
                "tz0_a_cbc_u",
                &format_args!("{}", self.tz0_a_cbc_u().bits()),
            )
            .field(
                "tz0_a_ost_d",
                &format_args!("{}", self.tz0_a_ost_d().bits()),
            )
            .field(
                "tz0_a_ost_u",
                &format_args!("{}", self.tz0_a_ost_u().bits()),
            )
            .field(
                "tz0_b_cbc_d",
                &format_args!("{}", self.tz0_b_cbc_d().bits()),
            )
            .field(
                "tz0_b_cbc_u",
                &format_args!("{}", self.tz0_b_cbc_u().bits()),
            )
            .field(
                "tz0_b_ost_d",
                &format_args!("{}", self.tz0_b_ost_d().bits()),
            )
            .field(
                "tz0_b_ost_u",
                &format_args!("{}", self.tz0_b_ost_u().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH0_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_sw_cbc(&mut self) -> TZ0_SW_CBC_W<0> {
        TZ0_SW_CBC_W::new(self)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_f2_cbc(&mut self) -> TZ0_F2_CBC_W<1> {
        TZ0_F2_CBC_W::new(self)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_f1_cbc(&mut self) -> TZ0_F1_CBC_W<2> {
        TZ0_F1_CBC_W::new(self)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_f0_cbc(&mut self) -> TZ0_F0_CBC_W<3> {
        TZ0_F0_CBC_W::new(self)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_sw_ost(&mut self) -> TZ0_SW_OST_W<4> {
        TZ0_SW_OST_W::new(self)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_f2_ost(&mut self) -> TZ0_F2_OST_W<5> {
        TZ0_F2_OST_W::new(self)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_f1_ost(&mut self) -> TZ0_F1_OST_W<6> {
        TZ0_F1_OST_W::new(self)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_f0_ost(&mut self) -> TZ0_F0_OST_W<7> {
        TZ0_F0_OST_W::new(self)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_a_cbc_d(&mut self) -> TZ0_A_CBC_D_W<8> {
        TZ0_A_CBC_D_W::new(self)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_a_cbc_u(&mut self) -> TZ0_A_CBC_U_W<10> {
        TZ0_A_CBC_U_W::new(self)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_a_ost_d(&mut self) -> TZ0_A_OST_D_W<12> {
        TZ0_A_OST_D_W::new(self)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_a_ost_u(&mut self) -> TZ0_A_OST_U_W<14> {
        TZ0_A_OST_U_W::new(self)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_b_cbc_d(&mut self) -> TZ0_B_CBC_D_W<16> {
        TZ0_B_CBC_D_W::new(self)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing,1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_b_cbc_u(&mut self) -> TZ0_B_CBC_U_W<18> {
        TZ0_B_CBC_U_W::new(self)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_b_ost_d(&mut self) -> TZ0_B_OST_D_W<20> {
        TZ0_B_OST_D_W::new(self)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_b_ost_u(&mut self) -> TZ0_B_OST_U_W<22> {
        TZ0_B_OST_U_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions on PWM0A and PWM0B trip events\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh0_cfg0](index.html) module"]
pub struct FH0_CFG0_SPEC;
impl crate::RegisterSpec for FH0_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh0_cfg0::R](R) reader structure"]
impl crate::Readable for FH0_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fh0_cfg0::W](W) writer structure"]
impl crate::Writable for FH0_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FH0_CFG0 to value 0"]
impl crate::Resettable for FH0_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
