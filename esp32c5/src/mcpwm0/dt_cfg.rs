#[doc = "Register `DT%s_CFG` reader"]
pub type R = crate::R<DT_CFG_SPEC>;
#[doc = "Register `DT%s_CFG` writer"]
pub type W = crate::W<DT_CFG_SPEC>;
#[doc = "Field `DB_FED_UPMETHOD` reader - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type DB_FED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DB_FED_UPMETHOD` writer - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type DB_FED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DB_RED_UPMETHOD` reader - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type DB_RED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DB_RED_UPMETHOD` writer - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type DB_RED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DB_DEB_MODE` reader - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB_DEB_MODE_R = crate::BitReader;
#[doc = "Field `DB_DEB_MODE` writer - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB_DEB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_A_OUTSWAP` reader - Configures S6 in table."]
pub type DB_A_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DB_A_OUTSWAP` writer - Configures S6 in table."]
pub type DB_A_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_B_OUTSWAP` reader - Configures S7 in table."]
pub type DB_B_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DB_B_OUTSWAP` writer - Configures S7 in table."]
pub type DB_B_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_RED_INSEL` reader - Configures S4 in table."]
pub type DB_RED_INSEL_R = crate::BitReader;
#[doc = "Field `DB_RED_INSEL` writer - Configures S4 in table."]
pub type DB_RED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_FED_INSEL` reader - Configures S5 in table."]
pub type DB_FED_INSEL_R = crate::BitReader;
#[doc = "Field `DB_FED_INSEL` writer - Configures S5 in table."]
pub type DB_FED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_RED_OUTINVERT` reader - Configures S2 in table."]
pub type DB_RED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DB_RED_OUTINVERT` writer - Configures S2 in table."]
pub type DB_RED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_FED_OUTINVERT` reader - Configures S3 in table."]
pub type DB_FED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DB_FED_OUTINVERT` writer - Configures S3 in table."]
pub type DB_FED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_A_OUTBYPASS` reader - Configures S1 in table."]
pub type DB_A_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DB_A_OUTBYPASS` writer - Configures S1 in table."]
pub type DB_A_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_B_OUTBYPASS` reader - Configures S0 in table."]
pub type DB_B_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DB_B_OUTBYPASS` writer - Configures S0 in table."]
pub type DB_B_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_CLK_SEL` reader - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
pub type DB_CLK_SEL_R = crate::BitReader;
#[doc = "Field `DB_CLK_SEL` writer - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
pub type DB_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn db_fed_upmethod(&self) -> DB_FED_UPMETHOD_R {
        DB_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn db_red_upmethod(&self) -> DB_RED_UPMETHOD_R {
        DB_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db_deb_mode(&self) -> DB_DEB_MODE_R {
        DB_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures S6 in table."]
    #[inline(always)]
    pub fn db_a_outswap(&self) -> DB_A_OUTSWAP_R {
        DB_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures S7 in table."]
    #[inline(always)]
    pub fn db_b_outswap(&self) -> DB_B_OUTSWAP_R {
        DB_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures S4 in table."]
    #[inline(always)]
    pub fn db_red_insel(&self) -> DB_RED_INSEL_R {
        DB_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures S5 in table."]
    #[inline(always)]
    pub fn db_fed_insel(&self) -> DB_FED_INSEL_R {
        DB_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures S2 in table."]
    #[inline(always)]
    pub fn db_red_outinvert(&self) -> DB_RED_OUTINVERT_R {
        DB_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures S3 in table."]
    #[inline(always)]
    pub fn db_fed_outinvert(&self) -> DB_FED_OUTINVERT_R {
        DB_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures S1 in table."]
    #[inline(always)]
    pub fn db_a_outbypass(&self) -> DB_A_OUTBYPASS_R {
        DB_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures S0 in table."]
    #[inline(always)]
    pub fn db_b_outbypass(&self) -> DB_B_OUTBYPASS_R {
        DB_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
    #[inline(always)]
    pub fn db_clk_sel(&self) -> DB_CLK_SEL_R {
        DB_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT_CFG")
            .field("db_fed_upmethod", &self.db_fed_upmethod())
            .field("db_red_upmethod", &self.db_red_upmethod())
            .field("db_deb_mode", &self.db_deb_mode())
            .field("db_a_outswap", &self.db_a_outswap())
            .field("db_b_outswap", &self.db_b_outswap())
            .field("db_red_insel", &self.db_red_insel())
            .field("db_fed_insel", &self.db_fed_insel())
            .field("db_red_outinvert", &self.db_red_outinvert())
            .field("db_fed_outinvert", &self.db_fed_outinvert())
            .field("db_a_outbypass", &self.db_a_outbypass())
            .field("db_b_outbypass", &self.db_b_outbypass())
            .field("db_clk_sel", &self.db_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn db_fed_upmethod(&mut self) -> DB_FED_UPMETHOD_W<'_, DT_CFG_SPEC> {
        DB_FED_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn db_red_upmethod(&mut self) -> DB_RED_UPMETHOD_W<'_, DT_CFG_SPEC> {
        DB_RED_UPMETHOD_W::new(self, 4)
    }
    #[doc = "Bit 8 - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db_deb_mode(&mut self) -> DB_DEB_MODE_W<'_, DT_CFG_SPEC> {
        DB_DEB_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures S6 in table."]
    #[inline(always)]
    pub fn db_a_outswap(&mut self) -> DB_A_OUTSWAP_W<'_, DT_CFG_SPEC> {
        DB_A_OUTSWAP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures S7 in table."]
    #[inline(always)]
    pub fn db_b_outswap(&mut self) -> DB_B_OUTSWAP_W<'_, DT_CFG_SPEC> {
        DB_B_OUTSWAP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures S4 in table."]
    #[inline(always)]
    pub fn db_red_insel(&mut self) -> DB_RED_INSEL_W<'_, DT_CFG_SPEC> {
        DB_RED_INSEL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures S5 in table."]
    #[inline(always)]
    pub fn db_fed_insel(&mut self) -> DB_FED_INSEL_W<'_, DT_CFG_SPEC> {
        DB_FED_INSEL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures S2 in table."]
    #[inline(always)]
    pub fn db_red_outinvert(&mut self) -> DB_RED_OUTINVERT_W<'_, DT_CFG_SPEC> {
        DB_RED_OUTINVERT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures S3 in table."]
    #[inline(always)]
    pub fn db_fed_outinvert(&mut self) -> DB_FED_OUTINVERT_W<'_, DT_CFG_SPEC> {
        DB_FED_OUTINVERT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures S1 in table."]
    #[inline(always)]
    pub fn db_a_outbypass(&mut self) -> DB_A_OUTBYPASS_W<'_, DT_CFG_SPEC> {
        DB_A_OUTBYPASS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures S0 in table."]
    #[inline(always)]
    pub fn db_b_outbypass(&mut self) -> DB_B_OUTBYPASS_W<'_, DT_CFG_SPEC> {
        DB_B_OUTBYPASS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
    #[inline(always)]
    pub fn db_clk_sel(&mut self) -> DB_CLK_SEL_W<'_, DT_CFG_SPEC> {
        DB_CLK_SEL_W::new(self, 17)
    }
}
#[doc = "Dead time configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_CFG_SPEC;
impl crate::RegisterSpec for DT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_cfg::R`](R) reader structure"]
impl crate::Readable for DT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt_cfg::W`](W) writer structure"]
impl crate::Writable for DT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT%s_CFG to value 0x0001_8000"]
impl crate::Resettable for DT_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0001_8000;
}
