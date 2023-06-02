#[doc = "Register `DB0_CFG` reader"]
pub struct R(crate::R<DB0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DB0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DB0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DB0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DB0_CFG` writer"]
pub struct W(crate::W<DB0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DB0_CFG_SPEC>;
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
impl From<crate::W<DB0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DB0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB0_FED_UPMETHOD` reader - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB0_FED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DB0_FED_UPMETHOD` writer - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB0_FED_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, DB0_CFG_SPEC, 4, O>;
#[doc = "Field `DB0_RED_UPMETHOD` reader - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB0_RED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DB0_RED_UPMETHOD` writer - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB0_RED_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, DB0_CFG_SPEC, 4, O>;
#[doc = "Field `DB0_DEB_MODE` reader - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB0_DEB_MODE_R = crate::BitReader;
#[doc = "Field `DB0_DEB_MODE` writer - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB0_DEB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_A_OUTSWAP` reader - S6 in documentation"]
pub type DB0_A_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DB0_A_OUTSWAP` writer - S6 in documentation"]
pub type DB0_A_OUTSWAP_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_B_OUTSWAP` reader - S7 in documentation"]
pub type DB0_B_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DB0_B_OUTSWAP` writer - S7 in documentation"]
pub type DB0_B_OUTSWAP_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_RED_INSEL` reader - S4 in documentation"]
pub type DB0_RED_INSEL_R = crate::BitReader;
#[doc = "Field `DB0_RED_INSEL` writer - S4 in documentation"]
pub type DB0_RED_INSEL_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_FED_INSEL` reader - S5 in documentation"]
pub type DB0_FED_INSEL_R = crate::BitReader;
#[doc = "Field `DB0_FED_INSEL` writer - S5 in documentation"]
pub type DB0_FED_INSEL_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_RED_OUTINVERT` reader - S2 in documentation"]
pub type DB0_RED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DB0_RED_OUTINVERT` writer - S2 in documentation"]
pub type DB0_RED_OUTINVERT_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_FED_OUTINVERT` reader - S3 in documentation"]
pub type DB0_FED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DB0_FED_OUTINVERT` writer - S3 in documentation"]
pub type DB0_FED_OUTINVERT_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_A_OUTBYPASS` reader - S1 in documentation"]
pub type DB0_A_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DB0_A_OUTBYPASS` writer - S1 in documentation"]
pub type DB0_A_OUTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_B_OUTBYPASS` reader - S0 in documentation"]
pub type DB0_B_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DB0_B_OUTBYPASS` writer - S0 in documentation"]
pub type DB0_B_OUTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
#[doc = "Field `DB0_CLK_SEL` reader - Dead time generator 0 clock selection. 0: PWM_clk, 1: PT_clk"]
pub type DB0_CLK_SEL_R = crate::BitReader;
#[doc = "Field `DB0_CLK_SEL` writer - Dead time generator 0 clock selection. 0: PWM_clk, 1: PT_clk"]
pub type DB0_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DB0_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db0_fed_upmethod(&self) -> DB0_FED_UPMETHOD_R {
        DB0_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db0_red_upmethod(&self) -> DB0_RED_UPMETHOD_R {
        DB0_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db0_deb_mode(&self) -> DB0_DEB_MODE_R {
        DB0_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn db0_a_outswap(&self) -> DB0_A_OUTSWAP_R {
        DB0_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn db0_b_outswap(&self) -> DB0_B_OUTSWAP_R {
        DB0_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn db0_red_insel(&self) -> DB0_RED_INSEL_R {
        DB0_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn db0_fed_insel(&self) -> DB0_FED_INSEL_R {
        DB0_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn db0_red_outinvert(&self) -> DB0_RED_OUTINVERT_R {
        DB0_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn db0_fed_outinvert(&self) -> DB0_FED_OUTINVERT_R {
        DB0_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn db0_a_outbypass(&self) -> DB0_A_OUTBYPASS_R {
        DB0_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn db0_b_outbypass(&self) -> DB0_B_OUTBYPASS_R {
        DB0_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead time generator 0 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    pub fn db0_clk_sel(&self) -> DB0_CLK_SEL_R {
        DB0_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB0_CFG")
            .field(
                "db0_fed_upmethod",
                &format_args!("{}", self.db0_fed_upmethod().bits()),
            )
            .field(
                "db0_red_upmethod",
                &format_args!("{}", self.db0_red_upmethod().bits()),
            )
            .field(
                "db0_deb_mode",
                &format_args!("{}", self.db0_deb_mode().bit()),
            )
            .field(
                "db0_a_outswap",
                &format_args!("{}", self.db0_a_outswap().bit()),
            )
            .field(
                "db0_b_outswap",
                &format_args!("{}", self.db0_b_outswap().bit()),
            )
            .field(
                "db0_red_insel",
                &format_args!("{}", self.db0_red_insel().bit()),
            )
            .field(
                "db0_fed_insel",
                &format_args!("{}", self.db0_fed_insel().bit()),
            )
            .field(
                "db0_red_outinvert",
                &format_args!("{}", self.db0_red_outinvert().bit()),
            )
            .field(
                "db0_fed_outinvert",
                &format_args!("{}", self.db0_fed_outinvert().bit()),
            )
            .field(
                "db0_a_outbypass",
                &format_args!("{}", self.db0_a_outbypass().bit()),
            )
            .field(
                "db0_b_outbypass",
                &format_args!("{}", self.db0_b_outbypass().bit()),
            )
            .field("db0_clk_sel", &format_args!("{}", self.db0_clk_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DB0_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    #[must_use]
    pub fn db0_fed_upmethod(&mut self) -> DB0_FED_UPMETHOD_W<0> {
        DB0_FED_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    #[must_use]
    pub fn db0_red_upmethod(&mut self) -> DB0_RED_UPMETHOD_W<4> {
        DB0_RED_UPMETHOD_W::new(self)
    }
    #[doc = "Bit 8 - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    #[must_use]
    pub fn db0_deb_mode(&mut self) -> DB0_DEB_MODE_W<8> {
        DB0_DEB_MODE_W::new(self)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_a_outswap(&mut self) -> DB0_A_OUTSWAP_W<9> {
        DB0_A_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_b_outswap(&mut self) -> DB0_B_OUTSWAP_W<10> {
        DB0_B_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_red_insel(&mut self) -> DB0_RED_INSEL_W<11> {
        DB0_RED_INSEL_W::new(self)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_fed_insel(&mut self) -> DB0_FED_INSEL_W<12> {
        DB0_FED_INSEL_W::new(self)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_red_outinvert(&mut self) -> DB0_RED_OUTINVERT_W<13> {
        DB0_RED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_fed_outinvert(&mut self) -> DB0_FED_OUTINVERT_W<14> {
        DB0_FED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_a_outbypass(&mut self) -> DB0_A_OUTBYPASS_W<15> {
        DB0_A_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db0_b_outbypass(&mut self) -> DB0_B_OUTBYPASS_W<16> {
        DB0_B_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 17 - Dead time generator 0 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    #[must_use]
    pub fn db0_clk_sel(&mut self) -> DB0_CLK_SEL_W<17> {
        DB0_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dead time type selection and configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [db0_cfg](index.html) module"]
pub struct DB0_CFG_SPEC;
impl crate::RegisterSpec for DB0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [db0_cfg::R](R) reader structure"]
impl crate::Readable for DB0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [db0_cfg::W](W) writer structure"]
impl crate::Writable for DB0_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DB0_CFG to value 0x0001_8000"]
impl crate::Resettable for DB0_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8000;
}
