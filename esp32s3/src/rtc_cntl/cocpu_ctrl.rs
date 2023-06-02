#[doc = "Register `COCPU_CTRL` reader"]
pub struct R(crate::R<COCPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COCPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COCPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COCPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COCPU_CTRL` writer"]
pub struct W(crate::W<COCPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COCPU_CTRL_SPEC>;
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
impl From<crate::W<COCPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COCPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COCPU_CLK_FO` reader - cocpu clk force on"]
pub type COCPU_CLK_FO_R = crate::BitReader;
#[doc = "Field `COCPU_CLK_FO` writer - cocpu clk force on"]
pub type COCPU_CLK_FO_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_START_2_RESET_DIS` reader - time from start cocpu to pull down reset"]
pub type COCPU_START_2_RESET_DIS_R = crate::FieldReader;
#[doc = "Field `COCPU_START_2_RESET_DIS` writer - time from start cocpu to pull down reset"]
pub type COCPU_START_2_RESET_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, COCPU_CTRL_SPEC, 6, O>;
#[doc = "Field `COCPU_START_2_INTR_EN` reader - time from start cocpu to give start interrupt"]
pub type COCPU_START_2_INTR_EN_R = crate::FieldReader;
#[doc = "Field `COCPU_START_2_INTR_EN` writer - time from start cocpu to give start interrupt"]
pub type COCPU_START_2_INTR_EN_W<'a, const O: u8> = crate::FieldWriter<'a, COCPU_CTRL_SPEC, 6, O>;
#[doc = "Field `COCPU_SHUT` reader - to shut cocpu"]
pub type COCPU_SHUT_R = crate::BitReader;
#[doc = "Field `COCPU_SHUT` writer - to shut cocpu"]
pub type COCPU_SHUT_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` reader - time from shut cocpu to disable clk"]
pub type COCPU_SHUT_2_CLK_DIS_R = crate::FieldReader;
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` writer - time from shut cocpu to disable clk"]
pub type COCPU_SHUT_2_CLK_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, COCPU_CTRL_SPEC, 8, O>;
#[doc = "Field `COCPU_SHUT_RESET_EN` reader - to reset cocpu"]
pub type COCPU_SHUT_RESET_EN_R = crate::BitReader;
#[doc = "Field `COCPU_SHUT_RESET_EN` writer - to reset cocpu"]
pub type COCPU_SHUT_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_SEL` reader - 1: old ULP 0: new riscV"]
pub type COCPU_SEL_R = crate::BitReader;
#[doc = "Field `COCPU_SEL` writer - 1: old ULP 0: new riscV"]
pub type COCPU_SEL_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_DONE_FORCE` reader - 1: select riscv done 0: select ulp done"]
pub type COCPU_DONE_FORCE_R = crate::BitReader;
#[doc = "Field `COCPU_DONE_FORCE` writer - 1: select riscv done 0: select ulp done"]
pub type COCPU_DONE_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_DONE` reader - done signal used by riscv to control timer."]
pub type COCPU_DONE_R = crate::BitReader;
#[doc = "Field `COCPU_DONE` writer - done signal used by riscv to control timer."]
pub type COCPU_DONE_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_SW_INT_TRIGGER` writer - trigger cocpu register interrupt"]
pub type COCPU_SW_INT_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
#[doc = "Field `COCPU_CLKGATE_EN` reader - open ulp-riscv clk gate"]
pub type COCPU_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `COCPU_CLKGATE_EN` writer - open ulp-riscv clk gate"]
pub type COCPU_CLKGATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - cocpu clk force on"]
    #[inline(always)]
    pub fn cocpu_clk_fo(&self) -> COCPU_CLK_FO_R {
        COCPU_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - time from start cocpu to pull down reset"]
    #[inline(always)]
    pub fn cocpu_start_2_reset_dis(&self) -> COCPU_START_2_RESET_DIS_R {
        COCPU_START_2_RESET_DIS_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:12 - time from start cocpu to give start interrupt"]
    #[inline(always)]
    pub fn cocpu_start_2_intr_en(&self) -> COCPU_START_2_INTR_EN_R {
        COCPU_START_2_INTR_EN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - to shut cocpu"]
    #[inline(always)]
    pub fn cocpu_shut(&self) -> COCPU_SHUT_R {
        COCPU_SHUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - time from shut cocpu to disable clk"]
    #[inline(always)]
    pub fn cocpu_shut_2_clk_dis(&self) -> COCPU_SHUT_2_CLK_DIS_R {
        COCPU_SHUT_2_CLK_DIS_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - to reset cocpu"]
    #[inline(always)]
    pub fn cocpu_shut_reset_en(&self) -> COCPU_SHUT_RESET_EN_R {
        COCPU_SHUT_RESET_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: old ULP 0: new riscV"]
    #[inline(always)]
    pub fn cocpu_sel(&self) -> COCPU_SEL_R {
        COCPU_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: select riscv done 0: select ulp done"]
    #[inline(always)]
    pub fn cocpu_done_force(&self) -> COCPU_DONE_FORCE_R {
        COCPU_DONE_FORCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - done signal used by riscv to control timer."]
    #[inline(always)]
    pub fn cocpu_done(&self) -> COCPU_DONE_R {
        COCPU_DONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - open ulp-riscv clk gate"]
    #[inline(always)]
    pub fn cocpu_clkgate_en(&self) -> COCPU_CLKGATE_EN_R {
        COCPU_CLKGATE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COCPU_CTRL")
            .field(
                "cocpu_clk_fo",
                &format_args!("{}", self.cocpu_clk_fo().bit()),
            )
            .field(
                "cocpu_start_2_reset_dis",
                &format_args!("{}", self.cocpu_start_2_reset_dis().bits()),
            )
            .field(
                "cocpu_start_2_intr_en",
                &format_args!("{}", self.cocpu_start_2_intr_en().bits()),
            )
            .field("cocpu_shut", &format_args!("{}", self.cocpu_shut().bit()))
            .field(
                "cocpu_shut_2_clk_dis",
                &format_args!("{}", self.cocpu_shut_2_clk_dis().bits()),
            )
            .field(
                "cocpu_shut_reset_en",
                &format_args!("{}", self.cocpu_shut_reset_en().bit()),
            )
            .field("cocpu_sel", &format_args!("{}", self.cocpu_sel().bit()))
            .field(
                "cocpu_done_force",
                &format_args!("{}", self.cocpu_done_force().bit()),
            )
            .field("cocpu_done", &format_args!("{}", self.cocpu_done().bit()))
            .field(
                "cocpu_clkgate_en",
                &format_args!("{}", self.cocpu_clkgate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COCPU_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - cocpu clk force on"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_clk_fo(&mut self) -> COCPU_CLK_FO_W<0> {
        COCPU_CLK_FO_W::new(self)
    }
    #[doc = "Bits 1:6 - time from start cocpu to pull down reset"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_2_reset_dis(&mut self) -> COCPU_START_2_RESET_DIS_W<1> {
        COCPU_START_2_RESET_DIS_W::new(self)
    }
    #[doc = "Bits 7:12 - time from start cocpu to give start interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_2_intr_en(&mut self) -> COCPU_START_2_INTR_EN_W<7> {
        COCPU_START_2_INTR_EN_W::new(self)
    }
    #[doc = "Bit 13 - to shut cocpu"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_shut(&mut self) -> COCPU_SHUT_W<13> {
        COCPU_SHUT_W::new(self)
    }
    #[doc = "Bits 14:21 - time from shut cocpu to disable clk"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_shut_2_clk_dis(&mut self) -> COCPU_SHUT_2_CLK_DIS_W<14> {
        COCPU_SHUT_2_CLK_DIS_W::new(self)
    }
    #[doc = "Bit 22 - to reset cocpu"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_shut_reset_en(&mut self) -> COCPU_SHUT_RESET_EN_W<22> {
        COCPU_SHUT_RESET_EN_W::new(self)
    }
    #[doc = "Bit 23 - 1: old ULP 0: new riscV"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sel(&mut self) -> COCPU_SEL_W<23> {
        COCPU_SEL_W::new(self)
    }
    #[doc = "Bit 24 - 1: select riscv done 0: select ulp done"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_done_force(&mut self) -> COCPU_DONE_FORCE_W<24> {
        COCPU_DONE_FORCE_W::new(self)
    }
    #[doc = "Bit 25 - done signal used by riscv to control timer."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_done(&mut self) -> COCPU_DONE_W<25> {
        COCPU_DONE_W::new(self)
    }
    #[doc = "Bit 26 - trigger cocpu register interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sw_int_trigger(&mut self) -> COCPU_SW_INT_TRIGGER_W<26> {
        COCPU_SW_INT_TRIGGER_W::new(self)
    }
    #[doc = "Bit 27 - open ulp-riscv clk gate"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_clkgate_en(&mut self) -> COCPU_CLKGATE_EN_W<27> {
        COCPU_CLKGATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp-riscv\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cocpu_ctrl](index.html) module"]
pub struct COCPU_CTRL_SPEC;
impl crate::RegisterSpec for COCPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cocpu_ctrl::R](R) reader structure"]
impl crate::Readable for COCPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cocpu_ctrl::W](W) writer structure"]
impl crate::Writable for COCPU_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COCPU_CTRL to value 0x008a_0810"]
impl crate::Resettable for COCPU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x008a_0810;
}
