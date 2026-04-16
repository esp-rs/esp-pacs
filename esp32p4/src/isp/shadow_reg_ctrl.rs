#[doc = "Register `SHADOW_REG_CTRL` reader"]
pub type R = crate::R<SHADOW_REG_CTRL_SPEC>;
#[doc = "Register `SHADOW_REG_CTRL` writer"]
pub type W = crate::W<SHADOW_REG_CTRL_SPEC>;
#[doc = "Field `BLC_UPDATE` reader - Write 1 to update blc configuration register"]
pub type BLC_UPDATE_R = crate::BitReader;
#[doc = "Field `BLC_UPDATE` writer - Write 1 to update blc configuration register"]
pub type BLC_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_UPDATE` reader - Write 1 to update dpc configuration register"]
pub type DPC_UPDATE_R = crate::BitReader;
#[doc = "Field `DPC_UPDATE` writer - Write 1 to update dpc configuration register"]
pub type DPC_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_UPDATE` reader - Write 1 to update bf configuration register"]
pub type BF_UPDATE_R = crate::BitReader;
#[doc = "Field `BF_UPDATE` writer - Write 1 to update bf configuration register"]
pub type BF_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBG_UPDATE` reader - Write 1 to update wbg configuration register"]
pub type WBG_UPDATE_R = crate::BitReader;
#[doc = "Field `WBG_UPDATE` writer - Write 1 to update wbg configuration register"]
pub type WBG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_UPDATE` reader - Write 1 to update ccm configuration register"]
pub type CCM_UPDATE_R = crate::BitReader;
#[doc = "Field `CCM_UPDATE` writer - Write 1 to update ccm configuration register"]
pub type CCM_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARP_UPDATE` reader - Write 1 to update sharp configuration register"]
pub type SHARP_UPDATE_R = crate::BitReader;
#[doc = "Field `SHARP_UPDATE` writer - Write 1 to update sharp configuration register"]
pub type SHARP_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOR_UPDATE` reader - Write 1 to update color configuration register"]
pub type COLOR_UPDATE_R = crate::BitReader;
#[doc = "Field `COLOR_UPDATE` writer - Write 1 to update color configuration register"]
pub type COLOR_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHADOW_UPDATE_SEL` reader - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
pub type SHADOW_UPDATE_SEL_R = crate::FieldReader;
#[doc = "Field `SHADOW_UPDATE_SEL` writer - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
pub type SHADOW_UPDATE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Write 1 to update blc configuration register"]
    #[inline(always)]
    pub fn blc_update(&self) -> BLC_UPDATE_R {
        BLC_UPDATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to update dpc configuration register"]
    #[inline(always)]
    pub fn dpc_update(&self) -> DPC_UPDATE_R {
        DPC_UPDATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to update bf configuration register"]
    #[inline(always)]
    pub fn bf_update(&self) -> BF_UPDATE_R {
        BF_UPDATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to update wbg configuration register"]
    #[inline(always)]
    pub fn wbg_update(&self) -> WBG_UPDATE_R {
        WBG_UPDATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to update ccm configuration register"]
    #[inline(always)]
    pub fn ccm_update(&self) -> CCM_UPDATE_R {
        CCM_UPDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to update sharp configuration register"]
    #[inline(always)]
    pub fn sharp_update(&self) -> SHARP_UPDATE_R {
        SHARP_UPDATE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to update color configuration register"]
    #[inline(always)]
    pub fn color_update(&self) -> COLOR_UPDATE_R {
        COLOR_UPDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
    #[inline(always)]
    pub fn shadow_update_sel(&self) -> SHADOW_UPDATE_SEL_R {
        SHADOW_UPDATE_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHADOW_REG_CTRL")
            .field("blc_update", &self.blc_update())
            .field("dpc_update", &self.dpc_update())
            .field("bf_update", &self.bf_update())
            .field("wbg_update", &self.wbg_update())
            .field("ccm_update", &self.ccm_update())
            .field("sharp_update", &self.sharp_update())
            .field("color_update", &self.color_update())
            .field("shadow_update_sel", &self.shadow_update_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to update blc configuration register"]
    #[inline(always)]
    pub fn blc_update(&mut self) -> BLC_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        BLC_UPDATE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to update dpc configuration register"]
    #[inline(always)]
    pub fn dpc_update(&mut self) -> DPC_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        DPC_UPDATE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to update bf configuration register"]
    #[inline(always)]
    pub fn bf_update(&mut self) -> BF_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        BF_UPDATE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to update wbg configuration register"]
    #[inline(always)]
    pub fn wbg_update(&mut self) -> WBG_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        WBG_UPDATE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to update ccm configuration register"]
    #[inline(always)]
    pub fn ccm_update(&mut self) -> CCM_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        CCM_UPDATE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Write 1 to update sharp configuration register"]
    #[inline(always)]
    pub fn sharp_update(&mut self) -> SHARP_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        SHARP_UPDATE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to update color configuration register"]
    #[inline(always)]
    pub fn color_update(&mut self) -> COLOR_UPDATE_W<'_, SHADOW_REG_CTRL_SPEC> {
        COLOR_UPDATE_W::new(self, 7)
    }
    #[doc = "Bits 30:31 - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
    #[inline(always)]
    pub fn shadow_update_sel(&mut self) -> SHADOW_UPDATE_SEL_W<'_, SHADOW_REG_CTRL_SPEC> {
        SHADOW_UPDATE_SEL_W::new(self, 30)
    }
}
#[doc = "shadow register ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`shadow_reg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadow_reg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHADOW_REG_CTRL_SPEC;
impl crate::RegisterSpec for SHADOW_REG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow_reg_ctrl::R`](R) reader structure"]
impl crate::Readable for SHADOW_REG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shadow_reg_ctrl::W`](W) writer structure"]
impl crate::Writable for SHADOW_REG_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHADOW_REG_CTRL to value 0x4000_0000"]
impl crate::Resettable for SHADOW_REG_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
