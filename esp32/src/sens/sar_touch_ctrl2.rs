#[doc = "Register `SAR_TOUCH_CTRL2` reader"]
pub type R = crate::R<SAR_TOUCH_CTRL2_SPEC>;
#[doc = "Register `SAR_TOUCH_CTRL2` writer"]
pub type W = crate::W<SAR_TOUCH_CTRL2_SPEC>;
#[doc = "Field `TOUCH_MEAS_EN` reader - 10-bit register to indicate which pads are \"touched\""]
pub type TOUCH_MEAS_EN_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_DONE` reader - fsm set 1 to indicate touch touch meas is done"]
pub type TOUCH_MEAS_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FSM_EN` reader - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
pub type TOUCH_START_FSM_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FSM_EN` writer - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
pub type TOUCH_START_FSM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_EN` reader - 1: start touch fsm valid when reg_touch_start_force is set"]
pub type TOUCH_START_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_EN` writer - 1: start touch fsm valid when reg_touch_start_force is set"]
pub type TOUCH_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_FORCE` reader - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
pub type TOUCH_START_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FORCE` writer - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
pub type TOUCH_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_MEAS_EN_CLR` writer - to clear reg_touch_meas_en"]
pub type TOUCH_MEAS_EN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 10-bit register to indicate which pads are \"touched\""]
    #[inline(always)]
    pub fn touch_meas_en(&self) -> TOUCH_MEAS_EN_R {
        TOUCH_MEAS_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - fsm set 1 to indicate touch touch meas is done"]
    #[inline(always)]
    pub fn touch_meas_done(&self) -> TOUCH_MEAS_DONE_R {
        TOUCH_MEAS_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&self) -> TOUCH_START_FSM_EN_R {
        TOUCH_START_FSM_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn touch_start_en(&self) -> TOUCH_START_EN_R {
        TOUCH_START_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn touch_start_force(&self) -> TOUCH_START_FORCE_R {
        TOUCH_START_FORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CTRL2")
            .field("touch_meas_en", &self.touch_meas_en())
            .field("touch_meas_done", &self.touch_meas_done())
            .field("touch_start_fsm_en", &self.touch_start_fsm_en())
            .field("touch_start_en", &self.touch_start_en())
            .field("touch_start_force", &self.touch_start_force())
            .field("touch_sleep_cycles", &self.touch_sleep_cycles())
            .finish()
    }
}
impl W {
    #[doc = "Bit 11 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&mut self) -> TOUCH_START_FSM_EN_W<SAR_TOUCH_CTRL2_SPEC> {
        TOUCH_START_FSM_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn touch_start_en(&mut self) -> TOUCH_START_EN_W<SAR_TOUCH_CTRL2_SPEC> {
        TOUCH_START_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn touch_start_force(&mut self) -> TOUCH_START_FORCE_W<SAR_TOUCH_CTRL2_SPEC> {
        TOUCH_START_FORCE_W::new(self, 13)
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W<SAR_TOUCH_CTRL2_SPEC> {
        TOUCH_SLEEP_CYCLES_W::new(self, 14)
    }
    #[doc = "Bit 30 - to clear reg_touch_meas_en"]
    #[inline(always)]
    pub fn touch_meas_en_clr(&mut self) -> TOUCH_MEAS_EN_CLR_W<SAR_TOUCH_CTRL2_SPEC> {
        TOUCH_MEAS_EN_CLR_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_CTRL2 to value 0x0040_0800"]
impl crate::Resettable for SAR_TOUCH_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0040_0800;
}
