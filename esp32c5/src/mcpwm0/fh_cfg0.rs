#[doc = "Register `FH%s_CFG0` reader"]
pub type R = crate::R<FH_CFG0_SPEC>;
#[doc = "Register `FH%s_CFG0` writer"]
pub type W = crate::W<FH_CFG0_SPEC>;
#[doc = "Field `TZ_SW_CBC` reader - Configures whether or not to enable software force cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_SW_CBC_R = crate::BitReader;
#[doc = "Field `TZ_SW_CBC` writer - Configures whether or not to enable software force cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_SW_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_F2_CBC` reader - Configures whether or not event_f2 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F2_CBC_R = crate::BitReader;
#[doc = "Field `TZ_F2_CBC` writer - Configures whether or not event_f2 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F2_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_F1_CBC` reader - Configures whether or not event_f1 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F1_CBC_R = crate::BitReader;
#[doc = "Field `TZ_F1_CBC` writer - Configures whether or not event_f1 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F1_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_F0_CBC` reader - Configures whether or not event_f0 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F0_CBC_R = crate::BitReader;
#[doc = "Field `TZ_F0_CBC` writer - Configures whether or not event_f0 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F0_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_SW_OST` reader - Configures whether or not to enable software force one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_SW_OST_R = crate::BitReader;
#[doc = "Field `TZ_SW_OST` writer - Configures whether or not to enable software force one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_SW_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_F2_OST` reader - Configures whether or not event_f2 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F2_OST_R = crate::BitReader;
#[doc = "Field `TZ_F2_OST` writer - Configures whether or not event_f2 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_F1_OST` reader - Configures whether or not event_f1 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F1_OST_R = crate::BitReader;
#[doc = "Field `TZ_F1_OST` writer - Configures whether or not event_f1 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_F0_OST` reader - Configures whether or not event_f0 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F0_OST_R = crate::BitReader;
#[doc = "Field `TZ_F0_OST` writer - Configures whether or not event_f0 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
pub type TZ_F0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_A_CBC_D` reader - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_CBC_D_R = crate::FieldReader;
#[doc = "Field `TZ_A_CBC_D` writer - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_CBC_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_A_CBC_U` reader - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_CBC_U_R = crate::FieldReader;
#[doc = "Field `TZ_A_CBC_U` writer - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_CBC_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_A_OST_D` reader - Configures one-shot mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_OST_D_R = crate::FieldReader;
#[doc = "Field `TZ_A_OST_D` writer - Configures one-shot mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_OST_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_A_OST_U` reader - Configures one-shot mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_OST_U_R = crate::FieldReader;
#[doc = "Field `TZ_A_OST_U` writer - Configures one-shot mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_A_OST_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_B_CBC_D` reader - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_CBC_D_R = crate::FieldReader;
#[doc = "Field `TZ_B_CBC_D` writer - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_CBC_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_B_CBC_U` reader - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_CBC_U_R = crate::FieldReader;
#[doc = "Field `TZ_B_CBC_U` writer - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_CBC_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_B_OST_D` reader - Configures one-shot mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_OST_D_R = crate::FieldReader;
#[doc = "Field `TZ_B_OST_D` writer - Configures one-shot mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_OST_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_B_OST_U` reader - Configures one-shot mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_OST_U_R = crate::FieldReader;
#[doc = "Field `TZ_B_OST_U` writer - Configures one-shot mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
pub type TZ_B_OST_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable software force cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_sw_cbc(&self) -> TZ_SW_CBC_R {
        TZ_SW_CBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not event_f2 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f2_cbc(&self) -> TZ_F2_CBC_R {
        TZ_F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not event_f1 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f1_cbc(&self) -> TZ_F1_CBC_R {
        TZ_F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not event_f0 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f0_cbc(&self) -> TZ_F0_CBC_R {
        TZ_F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable software force one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_sw_ost(&self) -> TZ_SW_OST_R {
        TZ_SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not event_f2 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f2_ost(&self) -> TZ_F2_OST_R {
        TZ_F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not event_f1 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f1_ost(&self) -> TZ_F1_OST_R {
        TZ_F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not event_f0 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f0_ost(&self) -> TZ_F0_OST_R {
        TZ_F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_cbc_d(&self) -> TZ_A_CBC_D_R {
        TZ_A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_cbc_u(&self) -> TZ_A_CBC_U_R {
        TZ_A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Configures one-shot mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_ost_d(&self) -> TZ_A_OST_D_R {
        TZ_A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Configures one-shot mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_ost_u(&self) -> TZ_A_OST_U_R {
        TZ_A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_cbc_d(&self) -> TZ_B_CBC_D_R {
        TZ_B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_cbc_u(&self) -> TZ_B_CBC_U_R {
        TZ_B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Configures one-shot mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_ost_d(&self) -> TZ_B_OST_D_R {
        TZ_B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Configures one-shot mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_ost_u(&self) -> TZ_B_OST_U_R {
        TZ_B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH_CFG0")
            .field("tz_sw_cbc", &self.tz_sw_cbc())
            .field("tz_f2_cbc", &self.tz_f2_cbc())
            .field("tz_f1_cbc", &self.tz_f1_cbc())
            .field("tz_f0_cbc", &self.tz_f0_cbc())
            .field("tz_sw_ost", &self.tz_sw_ost())
            .field("tz_f2_ost", &self.tz_f2_ost())
            .field("tz_f1_ost", &self.tz_f1_ost())
            .field("tz_f0_ost", &self.tz_f0_ost())
            .field("tz_a_cbc_d", &self.tz_a_cbc_d())
            .field("tz_a_cbc_u", &self.tz_a_cbc_u())
            .field("tz_a_ost_d", &self.tz_a_ost_d())
            .field("tz_a_ost_u", &self.tz_a_ost_u())
            .field("tz_b_cbc_d", &self.tz_b_cbc_d())
            .field("tz_b_cbc_u", &self.tz_b_cbc_u())
            .field("tz_b_ost_d", &self.tz_b_ost_d())
            .field("tz_b_ost_u", &self.tz_b_ost_u())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable software force cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_sw_cbc(&mut self) -> TZ_SW_CBC_W<'_, FH_CFG0_SPEC> {
        TZ_SW_CBC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not event_f2 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f2_cbc(&mut self) -> TZ_F2_CBC_W<'_, FH_CFG0_SPEC> {
        TZ_F2_CBC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not event_f1 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f1_cbc(&mut self) -> TZ_F1_CBC_W<'_, FH_CFG0_SPEC> {
        TZ_F1_CBC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not event_f0 will trigger cycle-by-cycle mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f0_cbc(&mut self) -> TZ_F0_CBC_W<'_, FH_CFG0_SPEC> {
        TZ_F0_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable software force one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_sw_ost(&mut self) -> TZ_SW_OST_W<'_, FH_CFG0_SPEC> {
        TZ_SW_OST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not event_f2 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f2_ost(&mut self) -> TZ_F2_OST_W<'_, FH_CFG0_SPEC> {
        TZ_F2_OST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not event_f1 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f1_ost(&mut self) -> TZ_F1_OST_W<'_, FH_CFG0_SPEC> {
        TZ_F1_OST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not event_f0 will trigger one-shot mode action.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz_f0_ost(&mut self) -> TZ_F0_OST_W<'_, FH_CFG0_SPEC> {
        TZ_F0_OST_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_cbc_d(&mut self) -> TZ_A_CBC_D_W<'_, FH_CFG0_SPEC> {
        TZ_A_CBC_D_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Configures cycle-by-cycle mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_cbc_u(&mut self) -> TZ_A_CBC_U_W<'_, FH_CFG0_SPEC> {
        TZ_A_CBC_U_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Configures one-shot mode action on PWM%s A when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_ost_d(&mut self) -> TZ_A_OST_D_W<'_, FH_CFG0_SPEC> {
        TZ_A_OST_D_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Configures one-shot mode action on PWM%s A when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_a_ost_u(&mut self) -> TZ_A_OST_U_W<'_, FH_CFG0_SPEC> {
        TZ_A_OST_U_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_cbc_d(&mut self) -> TZ_B_CBC_D_W<'_, FH_CFG0_SPEC> {
        TZ_B_CBC_D_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Configures cycle-by-cycle mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_cbc_u(&mut self) -> TZ_B_CBC_U_W<'_, FH_CFG0_SPEC> {
        TZ_B_CBC_U_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Configures one-shot mode action on PWM%s B when fault event occurs and timer is decreasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_ost_d(&mut self) -> TZ_B_OST_D_W<'_, FH_CFG0_SPEC> {
        TZ_B_OST_D_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Configures one-shot mode action on PWM%s B when fault event occurs and timer is increasing.\\\\0: Do nothing\\\\1: Force low\\\\2: Force high\\\\3: Toggle"]
    #[inline(always)]
    pub fn tz_b_ost_u(&mut self) -> TZ_B_OST_U_W<'_, FH_CFG0_SPEC> {
        TZ_B_OST_U_W::new(self, 22)
    }
}
#[doc = "PWM%s A and PWM%s B trip events actions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fh_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH_CFG0_SPEC;
impl crate::RegisterSpec for FH_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh_cfg0::R`](R) reader structure"]
impl crate::Readable for FH_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh_cfg0::W`](W) writer structure"]
impl crate::Writable for FH_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FH%s_CFG0 to value 0"]
impl crate::Resettable for FH_CFG0_SPEC {}
