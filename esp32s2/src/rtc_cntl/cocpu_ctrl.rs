#[doc = "Register `COCPU_CTRL` reader"]
pub type R = crate::R<COCPU_CTRL_SPEC>;
#[doc = "Register `COCPU_CTRL` writer"]
pub type W = crate::W<COCPU_CTRL_SPEC>;
#[doc = "Field `COCPU_CLK_FO` reader - ULP-RISCV clock force on"]
pub type COCPU_CLK_FO_R = crate::BitReader;
#[doc = "Field `COCPU_CLK_FO` writer - ULP-RISCV clock force on"]
pub type COCPU_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_START_2_RESET_DIS` reader - Time from ULP-RISCV startup to pull down reset"]
pub type COCPU_START_2_RESET_DIS_R = crate::FieldReader;
#[doc = "Field `COCPU_START_2_RESET_DIS` writer - Time from ULP-RISCV startup to pull down reset"]
pub type COCPU_START_2_RESET_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COCPU_START_2_INTR_EN` reader - Time from ULP-RISCV startup to send out RISCV_START_INT interrupt"]
pub type COCPU_START_2_INTR_EN_R = crate::FieldReader;
#[doc = "Field `COCPU_START_2_INTR_EN` writer - Time from ULP-RISCV startup to send out RISCV_START_INT interrupt"]
pub type COCPU_START_2_INTR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COCPU_SHUT` reader - Shut down ULP-RISCV"]
pub type COCPU_SHUT_R = crate::BitReader;
#[doc = "Field `COCPU_SHUT` writer - Shut down ULP-RISCV"]
pub type COCPU_SHUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` reader - Time from shut down ULP-RISCV to disable clock"]
pub type COCPU_SHUT_2_CLK_DIS_R = crate::FieldReader;
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` writer - Time from shut down ULP-RISCV to disable clock"]
pub type COCPU_SHUT_2_CLK_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COCPU_SHUT_RESET_EN` reader - This bit is used to reset ULP-RISCV"]
pub type COCPU_SHUT_RESET_EN_R = crate::BitReader;
#[doc = "Field `COCPU_SHUT_RESET_EN` writer - This bit is used to reset ULP-RISCV"]
pub type COCPU_SHUT_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SEL` reader - 0: select ULP-RISCV. 1: select ULP-FSM"]
pub type COCPU_SEL_R = crate::BitReader;
#[doc = "Field `COCPU_SEL` writer - 0: select ULP-RISCV. 1: select ULP-FSM"]
pub type COCPU_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_DONE_FORCE` reader - 0: select ULP-FSM DONE signal. 1: select ULP-RISCV DONE signal"]
pub type COCPU_DONE_FORCE_R = crate::BitReader;
#[doc = "Field `COCPU_DONE_FORCE` writer - 0: select ULP-FSM DONE signal. 1: select ULP-RISCV DONE signal"]
pub type COCPU_DONE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_DONE` reader - DONE signal. Write 1 to this bit, ULP-RISCV will go to HALT and the timer starts counting"]
pub type COCPU_DONE_R = crate::BitReader;
#[doc = "Field `COCPU_DONE` writer - DONE signal. Write 1 to this bit, ULP-RISCV will go to HALT and the timer starts counting"]
pub type COCPU_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SW_INT_TRIGGER` writer - Trigger ULP-RISCV register interrupt"]
pub type COCPU_SW_INT_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ULP-RISCV clock force on"]
    #[inline(always)]
    pub fn cocpu_clk_fo(&self) -> COCPU_CLK_FO_R {
        COCPU_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Time from ULP-RISCV startup to pull down reset"]
    #[inline(always)]
    pub fn cocpu_start_2_reset_dis(&self) -> COCPU_START_2_RESET_DIS_R {
        COCPU_START_2_RESET_DIS_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:12 - Time from ULP-RISCV startup to send out RISCV_START_INT interrupt"]
    #[inline(always)]
    pub fn cocpu_start_2_intr_en(&self) -> COCPU_START_2_INTR_EN_R {
        COCPU_START_2_INTR_EN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Shut down ULP-RISCV"]
    #[inline(always)]
    pub fn cocpu_shut(&self) -> COCPU_SHUT_R {
        COCPU_SHUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Time from shut down ULP-RISCV to disable clock"]
    #[inline(always)]
    pub fn cocpu_shut_2_clk_dis(&self) -> COCPU_SHUT_2_CLK_DIS_R {
        COCPU_SHUT_2_CLK_DIS_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - This bit is used to reset ULP-RISCV"]
    #[inline(always)]
    pub fn cocpu_shut_reset_en(&self) -> COCPU_SHUT_RESET_EN_R {
        COCPU_SHUT_RESET_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 0: select ULP-RISCV. 1: select ULP-FSM"]
    #[inline(always)]
    pub fn cocpu_sel(&self) -> COCPU_SEL_R {
        COCPU_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 0: select ULP-FSM DONE signal. 1: select ULP-RISCV DONE signal"]
    #[inline(always)]
    pub fn cocpu_done_force(&self) -> COCPU_DONE_FORCE_R {
        COCPU_DONE_FORCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DONE signal. Write 1 to this bit, ULP-RISCV will go to HALT and the timer starts counting"]
    #[inline(always)]
    pub fn cocpu_done(&self) -> COCPU_DONE_R {
        COCPU_DONE_R::new(((self.bits >> 25) & 1) != 0)
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
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COCPU_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - ULP-RISCV clock force on"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_clk_fo(&mut self) -> COCPU_CLK_FO_W<COCPU_CTRL_SPEC> {
        COCPU_CLK_FO_W::new(self, 0)
    }
    #[doc = "Bits 1:6 - Time from ULP-RISCV startup to pull down reset"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_2_reset_dis(&mut self) -> COCPU_START_2_RESET_DIS_W<COCPU_CTRL_SPEC> {
        COCPU_START_2_RESET_DIS_W::new(self, 1)
    }
    #[doc = "Bits 7:12 - Time from ULP-RISCV startup to send out RISCV_START_INT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_2_intr_en(&mut self) -> COCPU_START_2_INTR_EN_W<COCPU_CTRL_SPEC> {
        COCPU_START_2_INTR_EN_W::new(self, 7)
    }
    #[doc = "Bit 13 - Shut down ULP-RISCV"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_shut(&mut self) -> COCPU_SHUT_W<COCPU_CTRL_SPEC> {
        COCPU_SHUT_W::new(self, 13)
    }
    #[doc = "Bits 14:21 - Time from shut down ULP-RISCV to disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_shut_2_clk_dis(&mut self) -> COCPU_SHUT_2_CLK_DIS_W<COCPU_CTRL_SPEC> {
        COCPU_SHUT_2_CLK_DIS_W::new(self, 14)
    }
    #[doc = "Bit 22 - This bit is used to reset ULP-RISCV"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_shut_reset_en(&mut self) -> COCPU_SHUT_RESET_EN_W<COCPU_CTRL_SPEC> {
        COCPU_SHUT_RESET_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - 0: select ULP-RISCV. 1: select ULP-FSM"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sel(&mut self) -> COCPU_SEL_W<COCPU_CTRL_SPEC> {
        COCPU_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - 0: select ULP-FSM DONE signal. 1: select ULP-RISCV DONE signal"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_done_force(&mut self) -> COCPU_DONE_FORCE_W<COCPU_CTRL_SPEC> {
        COCPU_DONE_FORCE_W::new(self, 24)
    }
    #[doc = "Bit 25 - DONE signal. Write 1 to this bit, ULP-RISCV will go to HALT and the timer starts counting"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_done(&mut self) -> COCPU_DONE_W<COCPU_CTRL_SPEC> {
        COCPU_DONE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Trigger ULP-RISCV register interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sw_int_trigger(&mut self) -> COCPU_SW_INT_TRIGGER_W<COCPU_CTRL_SPEC> {
        COCPU_SW_INT_TRIGGER_W::new(self, 26)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ULP-RISCV configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cocpu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COCPU_CTRL_SPEC;
impl crate::RegisterSpec for COCPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cocpu_ctrl::R`](R) reader structure"]
impl crate::Readable for COCPU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cocpu_ctrl::W`](W) writer structure"]
impl crate::Writable for COCPU_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COCPU_CTRL to value 0x008a_0810"]
impl crate::Resettable for COCPU_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x008a_0810;
}
