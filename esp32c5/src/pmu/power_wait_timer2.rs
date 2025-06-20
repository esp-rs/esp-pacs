#[doc = "Register `POWER_WAIT_TIMER2` reader"]
pub type R = crate::R<POWER_WAIT_TIMER2_SPEC>;
#[doc = "Register `POWER_WAIT_TIMER2` writer"]
pub type W = crate::W<POWER_WAIT_TIMER2_SPEC>;
#[doc = "Field `DG_LP_ISO_WAIT_TIMER` reader - need_des"]
pub type DG_LP_ISO_WAIT_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_LP_ISO_WAIT_TIMER` writer - need_des"]
pub type DG_LP_ISO_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DG_LP_RST_WAIT_TIMER` reader - need_des"]
pub type DG_LP_RST_WAIT_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_LP_RST_WAIT_TIMER` writer - need_des"]
pub type DG_LP_RST_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DG_HP_ISO_WAIT_TIMER` reader - need_des"]
pub type DG_HP_ISO_WAIT_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_HP_ISO_WAIT_TIMER` writer - need_des"]
pub type DG_HP_ISO_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DG_HP_RST_WAIT_TIMER` reader - need_des"]
pub type DG_HP_RST_WAIT_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_HP_RST_WAIT_TIMER` writer - need_des"]
pub type DG_HP_RST_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn dg_lp_iso_wait_timer(&self) -> DG_LP_ISO_WAIT_TIMER_R {
        DG_LP_ISO_WAIT_TIMER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn dg_lp_rst_wait_timer(&self) -> DG_LP_RST_WAIT_TIMER_R {
        DG_LP_RST_WAIT_TIMER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn dg_hp_iso_wait_timer(&self) -> DG_HP_ISO_WAIT_TIMER_R {
        DG_HP_ISO_WAIT_TIMER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn dg_hp_rst_wait_timer(&self) -> DG_HP_RST_WAIT_TIMER_R {
        DG_HP_RST_WAIT_TIMER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_WAIT_TIMER2")
            .field("dg_lp_iso_wait_timer", &self.dg_lp_iso_wait_timer())
            .field("dg_lp_rst_wait_timer", &self.dg_lp_rst_wait_timer())
            .field("dg_hp_iso_wait_timer", &self.dg_hp_iso_wait_timer())
            .field("dg_hp_rst_wait_timer", &self.dg_hp_rst_wait_timer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn dg_lp_iso_wait_timer(&mut self) -> DG_LP_ISO_WAIT_TIMER_W<POWER_WAIT_TIMER2_SPEC> {
        DG_LP_ISO_WAIT_TIMER_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn dg_lp_rst_wait_timer(&mut self) -> DG_LP_RST_WAIT_TIMER_W<POWER_WAIT_TIMER2_SPEC> {
        DG_LP_RST_WAIT_TIMER_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn dg_hp_iso_wait_timer(&mut self) -> DG_HP_ISO_WAIT_TIMER_W<POWER_WAIT_TIMER2_SPEC> {
        DG_HP_ISO_WAIT_TIMER_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn dg_hp_rst_wait_timer(&mut self) -> DG_HP_RST_WAIT_TIMER_W<POWER_WAIT_TIMER2_SPEC> {
        DG_HP_RST_WAIT_TIMER_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_wait_timer2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_wait_timer2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_WAIT_TIMER2_SPEC;
impl crate::RegisterSpec for POWER_WAIT_TIMER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_wait_timer2::R`](R) reader structure"]
impl crate::Readable for POWER_WAIT_TIMER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_wait_timer2::W`](W) writer structure"]
impl crate::Writable for POWER_WAIT_TIMER2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_WAIT_TIMER2 to value 0xffff_ffff"]
impl crate::Resettable for POWER_WAIT_TIMER2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
