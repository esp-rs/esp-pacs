///Register `TZ_CFG0` reader
pub type R = crate::R<TZ_CFG0_SPEC>;
///Register `TZ_CFG0` writer
pub type W = crate::W<TZ_CFG0_SPEC>;
///Field `SW_CBC` reader - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable
pub type SW_CBC_R = crate::BitReader;
///Field `SW_CBC` writer - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable
pub type SW_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F2_CBC` reader - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
pub type F2_CBC_R = crate::BitReader;
///Field `F2_CBC` writer - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
pub type F2_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F1_CBC` reader - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
pub type F1_CBC_R = crate::BitReader;
///Field `F1_CBC` writer - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
pub type F1_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F0_CBC` reader - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
pub type F0_CBC_R = crate::BitReader;
///Field `F0_CBC` writer - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
pub type F0_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW_OST` reader - Enable register for software force one-shot mode action. 0: disable, 1: enable
pub type SW_OST_R = crate::BitReader;
///Field `SW_OST` writer - Enable register for software force one-shot mode action. 0: disable, 1: enable
pub type SW_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F2_OST` reader - event_f2 will trigger one-shot mode action. 0: disable, 1: enable
pub type F2_OST_R = crate::BitReader;
///Field `F2_OST` writer - event_f2 will trigger one-shot mode action. 0: disable, 1: enable
pub type F2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F1_OST` reader - event_f1 will trigger one-shot mode action. 0: disable, 1: enable
pub type F1_OST_R = crate::BitReader;
///Field `F1_OST` writer - event_f1 will trigger one-shot mode action. 0: disable, 1: enable
pub type F1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F0_OST` reader - event_f0 will trigger one-shot mode action. 0: disable, 1: enable
pub type F0_OST_R = crate::BitReader;
///Field `F0_OST` writer - event_f0 will trigger one-shot mode action. 0: disable, 1: enable
pub type F0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `A_CBC_D` reader - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_CBC_D_R = crate::FieldReader;
///Field `A_CBC_D` writer - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_CBC_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `A_CBC_U` reader - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_CBC_U_R = crate::FieldReader;
///Field `A_CBC_U` writer - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_CBC_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `A_OST_D` reader - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_OST_D_R = crate::FieldReader;
///Field `A_OST_D` writer - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_OST_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `A_OST_U` reader - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_OST_U_R = crate::FieldReader;
///Field `A_OST_U` writer - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type A_OST_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `B_CBC_D` reader - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_CBC_D_R = crate::FieldReader;
///Field `B_CBC_D` writer - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_CBC_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `B_CBC_U` reader - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_CBC_U_R = crate::FieldReader;
///Field `B_CBC_U` writer - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_CBC_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `B_OST_D` reader - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_OST_D_R = crate::FieldReader;
///Field `B_OST_D` writer - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_OST_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `B_OST_U` reader - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_OST_U_R = crate::FieldReader;
///Field `B_OST_U` writer - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
pub type B_OST_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn sw_cbc(&self) -> SW_CBC_R {
        SW_CBC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn f2_cbc(&self) -> F2_CBC_R {
        F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn f1_cbc(&self) -> F1_CBC_R {
        F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn f0_cbc(&self) -> F0_CBC_R {
        F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn sw_ost(&self) -> SW_OST_R {
        SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn f2_ost(&self) -> F2_OST_R {
        F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn f1_ost(&self) -> F1_OST_R {
        F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    pub fn f0_ost(&self) -> F0_OST_R {
        F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn a_cbc_d(&self) -> A_CBC_D_R {
        A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn a_cbc_u(&self) -> A_CBC_U_R {
        A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn a_ost_d(&self) -> A_OST_D_R {
        A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn a_ost_u(&self) -> A_OST_U_R {
        A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn b_cbc_d(&self) -> B_CBC_D_R {
        B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn b_cbc_u(&self) -> B_CBC_U_R {
        B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn b_ost_d(&self) -> B_OST_D_R {
        B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    pub fn b_ost_u(&self) -> B_OST_U_R {
        B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ_CFG0")
            .field("sw_cbc", &self.sw_cbc())
            .field("f2_cbc", &self.f2_cbc())
            .field("f1_cbc", &self.f1_cbc())
            .field("f0_cbc", &self.f0_cbc())
            .field("sw_ost", &self.sw_ost())
            .field("f2_ost", &self.f2_ost())
            .field("f1_ost", &self.f1_ost())
            .field("f0_ost", &self.f0_ost())
            .field("a_cbc_d", &self.a_cbc_d())
            .field("a_cbc_u", &self.a_cbc_u())
            .field("a_ost_d", &self.a_ost_d())
            .field("a_ost_u", &self.a_ost_u())
            .field("b_cbc_d", &self.b_cbc_d())
            .field("b_cbc_u", &self.b_cbc_u())
            .field("b_ost_d", &self.b_ost_d())
            .field("b_ost_u", &self.b_ost_u())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn sw_cbc(&mut self) -> SW_CBC_W<TZ_CFG0_SPEC> {
        SW_CBC_W::new(self, 0)
    }
    ///Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn f2_cbc(&mut self) -> F2_CBC_W<TZ_CFG0_SPEC> {
        F2_CBC_W::new(self, 1)
    }
    ///Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn f1_cbc(&mut self) -> F1_CBC_W<TZ_CFG0_SPEC> {
        F1_CBC_W::new(self, 2)
    }
    ///Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn f0_cbc(&mut self) -> F0_CBC_W<TZ_CFG0_SPEC> {
        F0_CBC_W::new(self, 3)
    }
    ///Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn sw_ost(&mut self) -> SW_OST_W<TZ_CFG0_SPEC> {
        SW_OST_W::new(self, 4)
    }
    ///Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn f2_ost(&mut self) -> F2_OST_W<TZ_CFG0_SPEC> {
        F2_OST_W::new(self, 5)
    }
    ///Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn f1_ost(&mut self) -> F1_OST_W<TZ_CFG0_SPEC> {
        F1_OST_W::new(self, 6)
    }
    ///Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn f0_ost(&mut self) -> F0_OST_W<TZ_CFG0_SPEC> {
        F0_OST_W::new(self, 7)
    }
    ///Bits 8:9 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn a_cbc_d(&mut self) -> A_CBC_D_W<TZ_CFG0_SPEC> {
        A_CBC_D_W::new(self, 8)
    }
    ///Bits 10:11 - Cycle-by-cycle mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn a_cbc_u(&mut self) -> A_CBC_U_W<TZ_CFG0_SPEC> {
        A_CBC_U_W::new(self, 10)
    }
    ///Bits 12:13 - One-shot mode action on PWM0A when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn a_ost_d(&mut self) -> A_OST_D_W<TZ_CFG0_SPEC> {
        A_OST_D_W::new(self, 12)
    }
    ///Bits 14:15 - One-shot mode action on PWM0A when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn a_ost_u(&mut self) -> A_OST_U_W<TZ_CFG0_SPEC> {
        A_OST_U_W::new(self, 14)
    }
    ///Bits 16:17 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn b_cbc_d(&mut self) -> B_CBC_D_W<TZ_CFG0_SPEC> {
        B_CBC_D_W::new(self, 16)
    }
    ///Bits 18:19 - Cycle-by-cycle mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn b_cbc_u(&mut self) -> B_CBC_U_W<TZ_CFG0_SPEC> {
        B_CBC_U_W::new(self, 18)
    }
    ///Bits 20:21 - One-shot mode action on PWM0B when fault event occurs and timer is decreasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn b_ost_d(&mut self) -> B_OST_D_W<TZ_CFG0_SPEC> {
        B_OST_D_W::new(self, 20)
    }
    ///Bits 22:23 - One-shot mode action on PWM0B when fault event occurs and timer is increasing. 0: do nothing, 1: force lo, 2: force hi, 3: toggle
    #[inline(always)]
    #[must_use]
    pub fn b_ost_u(&mut self) -> B_OST_U_W<TZ_CFG0_SPEC> {
        B_OST_U_W::new(self, 22)
    }
}
/**Actions on PWMxA and PWMxB trip events

You can [`read`](crate::generic::Reg::read) this register and get [`tz_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tz_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TZ_CFG0_SPEC;
impl crate::RegisterSpec for TZ_CFG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tz_cfg0::R`](R) reader structure
impl crate::Readable for TZ_CFG0_SPEC {}
///`write(|w| ..)` method takes [`tz_cfg0::W`](W) writer structure
impl crate::Writable for TZ_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TZ_CFG0 to value 0
impl crate::Resettable for TZ_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
