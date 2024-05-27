///Register `DT_CFG` reader
pub type R = crate::R<DT_CFG_SPEC>;
///Register `DT_CFG` writer
pub type W = crate::W<DT_CFG_SPEC>;
///Field `FED_UPMETHOD` reader - Configures update method for FED (Falling edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
pub type FED_UPMETHOD_R = crate::FieldReader;
///Field `FED_UPMETHOD` writer - Configures update method for FED (Falling edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
pub type FED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RED_UPMETHOD` reader - Configures update method for RED (rising edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
pub type RED_UPMETHOD_R = crate::FieldReader;
///Field `RED_UPMETHOD` writer - Configures update method for RED (rising edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
pub type RED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DEB_MODE` reader - Configures S8 in table, dual-edge B mode.\\0: fed/red take effect on different path separately\\1: fed/red take effect on B path, A out is in bypass or dulpB mode
pub type DEB_MODE_R = crate::BitReader;
///Field `DEB_MODE` writer - Configures S8 in table, dual-edge B mode.\\0: fed/red take effect on different path separately\\1: fed/red take effect on B path, A out is in bypass or dulpB mode
pub type DEB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `A_OUTSWAP` reader - Configures S6 in table.
pub type A_OUTSWAP_R = crate::BitReader;
///Field `A_OUTSWAP` writer - Configures S6 in table.
pub type A_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B_OUTSWAP` reader - Configures S7 in table.
pub type B_OUTSWAP_R = crate::BitReader;
///Field `B_OUTSWAP` writer - Configures S7 in table.
pub type B_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RED_INSEL` reader - Configures S4 in table.
pub type RED_INSEL_R = crate::BitReader;
///Field `RED_INSEL` writer - Configures S4 in table.
pub type RED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FED_INSEL` reader - Configures S5 in table.
pub type FED_INSEL_R = crate::BitReader;
///Field `FED_INSEL` writer - Configures S5 in table.
pub type FED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RED_OUTINVERT` reader - Configures S2 in table.
pub type RED_OUTINVERT_R = crate::BitReader;
///Field `RED_OUTINVERT` writer - Configures S2 in table.
pub type RED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FED_OUTINVERT` reader - Configures S3 in table.
pub type FED_OUTINVERT_R = crate::BitReader;
///Field `FED_OUTINVERT` writer - Configures S3 in table.
pub type FED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `A_OUTBYPASS` reader - Configures S1 in table.
pub type A_OUTBYPASS_R = crate::BitReader;
///Field `A_OUTBYPASS` writer - Configures S1 in table.
pub type A_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B_OUTBYPASS` reader - Configures S0 in table.
pub type B_OUTBYPASS_R = crate::BitReader;
///Field `B_OUTBYPASS` writer - Configures S0 in table.
pub type B_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_SEL` reader - Configures dead time generator %s clock selection.\\0: PWM_clk\\1: PT_clk
pub type CLK_SEL_R = crate::BitReader;
///Field `CLK_SEL` writer - Configures dead time generator %s clock selection.\\0: PWM_clk\\1: PT_clk
pub type CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Configures update method for FED (Falling edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
    #[inline(always)]
    pub fn fed_upmethod(&self) -> FED_UPMETHOD_R {
        FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Configures update method for RED (rising edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
    #[inline(always)]
    pub fn red_upmethod(&self) -> RED_UPMETHOD_R {
        RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Configures S8 in table, dual-edge B mode.\\0: fed/red take effect on different path separately\\1: fed/red take effect on B path, A out is in bypass or dulpB mode
    #[inline(always)]
    pub fn deb_mode(&self) -> DEB_MODE_R {
        DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configures S6 in table.
    #[inline(always)]
    pub fn a_outswap(&self) -> A_OUTSWAP_R {
        A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Configures S7 in table.
    #[inline(always)]
    pub fn b_outswap(&self) -> B_OUTSWAP_R {
        B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Configures S4 in table.
    #[inline(always)]
    pub fn red_insel(&self) -> RED_INSEL_R {
        RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Configures S5 in table.
    #[inline(always)]
    pub fn fed_insel(&self) -> FED_INSEL_R {
        FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configures S2 in table.
    #[inline(always)]
    pub fn red_outinvert(&self) -> RED_OUTINVERT_R {
        RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Configures S3 in table.
    #[inline(always)]
    pub fn fed_outinvert(&self) -> FED_OUTINVERT_R {
        FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Configures S1 in table.
    #[inline(always)]
    pub fn a_outbypass(&self) -> A_OUTBYPASS_R {
        A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Configures S0 in table.
    #[inline(always)]
    pub fn b_outbypass(&self) -> B_OUTBYPASS_R {
        B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Configures dead time generator %s clock selection.\\0: PWM_clk\\1: PT_clk
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT_CFG")
            .field("fed_upmethod", &self.fed_upmethod())
            .field("red_upmethod", &self.red_upmethod())
            .field("deb_mode", &self.deb_mode())
            .field("a_outswap", &self.a_outswap())
            .field("b_outswap", &self.b_outswap())
            .field("red_insel", &self.red_insel())
            .field("fed_insel", &self.fed_insel())
            .field("red_outinvert", &self.red_outinvert())
            .field("fed_outinvert", &self.fed_outinvert())
            .field("a_outbypass", &self.a_outbypass())
            .field("b_outbypass", &self.b_outbypass())
            .field("clk_sel", &self.clk_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Configures update method for FED (Falling edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
    #[inline(always)]
    #[must_use]
    pub fn fed_upmethod(&mut self) -> FED_UPMETHOD_W<DT_CFG_SPEC> {
        FED_UPMETHOD_W::new(self, 0)
    }
    ///Bits 4:7 - Configures update method for RED (rising edge delay) active register.\\0: Immediate\\Bit0 is set to 1: TEZ\\Bit1 is set to 1: TEP\\Bit2 is set to 1: Sync\\Bit3 is set to 1: Disable the update
    #[inline(always)]
    #[must_use]
    pub fn red_upmethod(&mut self) -> RED_UPMETHOD_W<DT_CFG_SPEC> {
        RED_UPMETHOD_W::new(self, 4)
    }
    ///Bit 8 - Configures S8 in table, dual-edge B mode.\\0: fed/red take effect on different path separately\\1: fed/red take effect on B path, A out is in bypass or dulpB mode
    #[inline(always)]
    #[must_use]
    pub fn deb_mode(&mut self) -> DEB_MODE_W<DT_CFG_SPEC> {
        DEB_MODE_W::new(self, 8)
    }
    ///Bit 9 - Configures S6 in table.
    #[inline(always)]
    #[must_use]
    pub fn a_outswap(&mut self) -> A_OUTSWAP_W<DT_CFG_SPEC> {
        A_OUTSWAP_W::new(self, 9)
    }
    ///Bit 10 - Configures S7 in table.
    #[inline(always)]
    #[must_use]
    pub fn b_outswap(&mut self) -> B_OUTSWAP_W<DT_CFG_SPEC> {
        B_OUTSWAP_W::new(self, 10)
    }
    ///Bit 11 - Configures S4 in table.
    #[inline(always)]
    #[must_use]
    pub fn red_insel(&mut self) -> RED_INSEL_W<DT_CFG_SPEC> {
        RED_INSEL_W::new(self, 11)
    }
    ///Bit 12 - Configures S5 in table.
    #[inline(always)]
    #[must_use]
    pub fn fed_insel(&mut self) -> FED_INSEL_W<DT_CFG_SPEC> {
        FED_INSEL_W::new(self, 12)
    }
    ///Bit 13 - Configures S2 in table.
    #[inline(always)]
    #[must_use]
    pub fn red_outinvert(&mut self) -> RED_OUTINVERT_W<DT_CFG_SPEC> {
        RED_OUTINVERT_W::new(self, 13)
    }
    ///Bit 14 - Configures S3 in table.
    #[inline(always)]
    #[must_use]
    pub fn fed_outinvert(&mut self) -> FED_OUTINVERT_W<DT_CFG_SPEC> {
        FED_OUTINVERT_W::new(self, 14)
    }
    ///Bit 15 - Configures S1 in table.
    #[inline(always)]
    #[must_use]
    pub fn a_outbypass(&mut self) -> A_OUTBYPASS_W<DT_CFG_SPEC> {
        A_OUTBYPASS_W::new(self, 15)
    }
    ///Bit 16 - Configures S0 in table.
    #[inline(always)]
    #[must_use]
    pub fn b_outbypass(&mut self) -> B_OUTBYPASS_W<DT_CFG_SPEC> {
        B_OUTBYPASS_W::new(self, 16)
    }
    ///Bit 17 - Configures dead time generator %s clock selection.\\0: PWM_clk\\1: PT_clk
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<DT_CFG_SPEC> {
        CLK_SEL_W::new(self, 17)
    }
}
/**Dead time configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dt_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DT_CFG_SPEC;
impl crate::RegisterSpec for DT_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dt_cfg::R`](R) reader structure
impl crate::Readable for DT_CFG_SPEC {}
///`write(|w| ..)` method takes [`dt_cfg::W`](W) writer structure
impl crate::Writable for DT_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DT_CFG to value 0x0001_8000
impl crate::Resettable for DT_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0001_8000;
}
