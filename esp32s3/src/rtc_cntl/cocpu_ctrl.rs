#[doc = "Register `COCPU_CTRL` reader"]
pub type R = crate::R<COCPU_CTRL_SPEC>;
#[doc = "Register `COCPU_CTRL` writer"]
pub type W = crate::W<COCPU_CTRL_SPEC>;
#[doc = "Field `COCPU_CLK_FO` reader - cocpu clk force on"]
pub type COCPU_CLK_FO_R = crate::BitReader;
#[doc = "Field `COCPU_CLK_FO` writer - cocpu clk force on"]
pub type COCPU_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_START_2_RESET_DIS` reader - time from start cocpu to pull down reset"]
pub type COCPU_START_2_RESET_DIS_R = crate::FieldReader;
#[doc = "Field `COCPU_START_2_RESET_DIS` writer - time from start cocpu to pull down reset"]
pub type COCPU_START_2_RESET_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COCPU_START_2_INTR_EN` reader - time from start cocpu to give start interrupt"]
pub type COCPU_START_2_INTR_EN_R = crate::FieldReader;
#[doc = "Field `COCPU_START_2_INTR_EN` writer - time from start cocpu to give start interrupt"]
pub type COCPU_START_2_INTR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COCPU_SHUT` reader - to shut cocpu"]
pub type COCPU_SHUT_R = crate::BitReader;
#[doc = "Field `COCPU_SHUT` writer - to shut cocpu"]
pub type COCPU_SHUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` reader - time from shut cocpu to disable clk"]
pub type COCPU_SHUT_2_CLK_DIS_R = crate::FieldReader;
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` writer - time from shut cocpu to disable clk"]
pub type COCPU_SHUT_2_CLK_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COCPU_SHUT_RESET_EN` reader - to reset cocpu"]
pub type COCPU_SHUT_RESET_EN_R = crate::BitReader;
#[doc = "Field `COCPU_SHUT_RESET_EN` writer - to reset cocpu"]
pub type COCPU_SHUT_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SEL` reader - 1: old ULP 0: new riscV"]
pub type COCPU_SEL_R = crate::BitReader;
#[doc = "Field `COCPU_SEL` writer - 1: old ULP 0: new riscV"]
pub type COCPU_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_DONE_FORCE` reader - 1: select riscv done 0: select ulp done"]
pub type COCPU_DONE_FORCE_R = crate::BitReader;
#[doc = "Field `COCPU_DONE_FORCE` writer - 1: select riscv done 0: select ulp done"]
pub type COCPU_DONE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_DONE` reader - done signal used by riscv to control timer."]
pub type COCPU_DONE_R = crate::BitReader;
#[doc = "Field `COCPU_DONE` writer - done signal used by riscv to control timer."]
pub type COCPU_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SW_INT_TRIGGER` writer - trigger cocpu register interrupt"]
pub type COCPU_SW_INT_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_CLKGATE_EN` reader - open ulp-riscv clk gate"]
pub type COCPU_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `COCPU_CLKGATE_EN` writer - open ulp-riscv clk gate"]
pub type COCPU_CLKGATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("cocpu_clk_fo", &self.cocpu_clk_fo())
            .field("cocpu_start_2_reset_dis", &self.cocpu_start_2_reset_dis())
            .field("cocpu_start_2_intr_en", &self.cocpu_start_2_intr_en())
            .field("cocpu_shut", &self.cocpu_shut())
            .field("cocpu_shut_2_clk_dis", &self.cocpu_shut_2_clk_dis())
            .field("cocpu_shut_reset_en", &self.cocpu_shut_reset_en())
            .field("cocpu_sel", &self.cocpu_sel())
            .field("cocpu_done_force", &self.cocpu_done_force())
            .field("cocpu_done", &self.cocpu_done())
            .field("cocpu_clkgate_en", &self.cocpu_clkgate_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - cocpu clk force on"]
    #[inline(always)]
    pub fn cocpu_clk_fo(&mut self) -> COCPU_CLK_FO_W<'_, COCPU_CTRL_SPEC> {
        COCPU_CLK_FO_W::new(self, 0)
    }
    #[doc = "Bits 1:6 - time from start cocpu to pull down reset"]
    #[inline(always)]
    pub fn cocpu_start_2_reset_dis(&mut self) -> COCPU_START_2_RESET_DIS_W<'_, COCPU_CTRL_SPEC> {
        COCPU_START_2_RESET_DIS_W::new(self, 1)
    }
    #[doc = "Bits 7:12 - time from start cocpu to give start interrupt"]
    #[inline(always)]
    pub fn cocpu_start_2_intr_en(&mut self) -> COCPU_START_2_INTR_EN_W<'_, COCPU_CTRL_SPEC> {
        COCPU_START_2_INTR_EN_W::new(self, 7)
    }
    #[doc = "Bit 13 - to shut cocpu"]
    #[inline(always)]
    pub fn cocpu_shut(&mut self) -> COCPU_SHUT_W<'_, COCPU_CTRL_SPEC> {
        COCPU_SHUT_W::new(self, 13)
    }
    #[doc = "Bits 14:21 - time from shut cocpu to disable clk"]
    #[inline(always)]
    pub fn cocpu_shut_2_clk_dis(&mut self) -> COCPU_SHUT_2_CLK_DIS_W<'_, COCPU_CTRL_SPEC> {
        COCPU_SHUT_2_CLK_DIS_W::new(self, 14)
    }
    #[doc = "Bit 22 - to reset cocpu"]
    #[inline(always)]
    pub fn cocpu_shut_reset_en(&mut self) -> COCPU_SHUT_RESET_EN_W<'_, COCPU_CTRL_SPEC> {
        COCPU_SHUT_RESET_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: old ULP 0: new riscV"]
    #[inline(always)]
    pub fn cocpu_sel(&mut self) -> COCPU_SEL_W<'_, COCPU_CTRL_SPEC> {
        COCPU_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: select riscv done 0: select ulp done"]
    #[inline(always)]
    pub fn cocpu_done_force(&mut self) -> COCPU_DONE_FORCE_W<'_, COCPU_CTRL_SPEC> {
        COCPU_DONE_FORCE_W::new(self, 24)
    }
    #[doc = "Bit 25 - done signal used by riscv to control timer."]
    #[inline(always)]
    pub fn cocpu_done(&mut self) -> COCPU_DONE_W<'_, COCPU_CTRL_SPEC> {
        COCPU_DONE_W::new(self, 25)
    }
    #[doc = "Bit 26 - trigger cocpu register interrupt"]
    #[inline(always)]
    pub fn cocpu_sw_int_trigger(&mut self) -> COCPU_SW_INT_TRIGGER_W<'_, COCPU_CTRL_SPEC> {
        COCPU_SW_INT_TRIGGER_W::new(self, 26)
    }
    #[doc = "Bit 27 - open ulp-riscv clk gate"]
    #[inline(always)]
    pub fn cocpu_clkgate_en(&mut self) -> COCPU_CLKGATE_EN_W<'_, COCPU_CTRL_SPEC> {
        COCPU_CLKGATE_EN_W::new(self, 27)
    }
}
#[doc = "configure ulp-riscv\n\nYou can [`read`](crate::Reg::read) this register and get [`cocpu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cocpu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COCPU_CTRL_SPEC;
impl crate::RegisterSpec for COCPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cocpu_ctrl::R`](R) reader structure"]
impl crate::Readable for COCPU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cocpu_ctrl::W`](W) writer structure"]
impl crate::Writable for COCPU_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COCPU_CTRL to value 0x008a_0810"]
impl crate::Resettable for COCPU_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x008a_0810;
}
