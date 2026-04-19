#[doc = "Register `MULT_CONF` reader"]
pub type R = crate::R<MULT_CONF_SPEC>;
#[doc = "Register `MULT_CONF` writer"]
pub type W = crate::W<MULT_CONF_SPEC>;
#[doc = "Field `MULT_START` reader - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
pub type MULT_START_R = crate::BitReader;
#[doc = "Field `MULT_START` writer - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
pub type MULT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULT_RESET` writer - Configures whether to reset ECC Accelerator. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type MULT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULT_KEY_LENGTH` reader - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
pub type MULT_KEY_LENGTH_R = crate::FieldReader;
#[doc = "Field `MULT_KEY_LENGTH` writer - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
pub type MULT_KEY_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MULT_MOD_BASE` reader - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
pub type MULT_MOD_BASE_R = crate::BitReader;
#[doc = "Field `MULT_MOD_BASE` writer - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
pub type MULT_MOD_BASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULT_WORK_MODE` reader - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
pub type MULT_WORK_MODE_R = crate::FieldReader;
#[doc = "Field `MULT_WORK_MODE` writer - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
pub type MULT_WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MULT_SECURITY_MODE` reader - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
pub type MULT_SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `MULT_SECURITY_MODE` writer - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
pub type MULT_SECURITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULT_VERIFICATION_RESULT` reader - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
pub type MULT_VERIFICATION_RESULT_R = crate::BitReader;
#[doc = "Field `MULT_CLK_EN` reader - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MULT_CLK_EN_R = crate::BitReader;
#[doc = "Field `MULT_CLK_EN` writer - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MULT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULT_MEM_CLOCK_GATE_FORCE_ON` reader - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MULT_MEM_CLOCK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MULT_MEM_CLOCK_GATE_FORCE_ON` writer - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MULT_MEM_CLOCK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
    #[inline(always)]
    pub fn mult_start(&self) -> MULT_START_R {
        MULT_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
    #[inline(always)]
    pub fn mult_key_length(&self) -> MULT_KEY_LENGTH_R {
        MULT_KEY_LENGTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
    #[inline(always)]
    pub fn mult_mod_base(&self) -> MULT_MOD_BASE_R {
        MULT_MOD_BASE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
    #[inline(always)]
    pub fn mult_work_mode(&self) -> MULT_WORK_MODE_R {
        MULT_WORK_MODE_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
    #[inline(always)]
    pub fn mult_security_mode(&self) -> MULT_SECURITY_MODE_R {
        MULT_SECURITY_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
    #[inline(always)]
    pub fn mult_verification_result(&self) -> MULT_VERIFICATION_RESULT_R {
        MULT_VERIFICATION_RESULT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mult_clk_en(&self) -> MULT_CLK_EN_R {
        MULT_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mult_mem_clock_gate_force_on(&self) -> MULT_MEM_CLOCK_GATE_FORCE_ON_R {
        MULT_MEM_CLOCK_GATE_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_CONF")
            .field("mult_start", &self.mult_start())
            .field("mult_key_length", &self.mult_key_length())
            .field("mult_mod_base", &self.mult_mod_base())
            .field("mult_work_mode", &self.mult_work_mode())
            .field("mult_security_mode", &self.mult_security_mode())
            .field("mult_verification_result", &self.mult_verification_result())
            .field("mult_clk_en", &self.mult_clk_en())
            .field(
                "mult_mem_clock_gate_force_on",
                &self.mult_mem_clock_gate_force_on(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
    #[inline(always)]
    pub fn mult_start(&mut self) -> MULT_START_W<'_, MULT_CONF_SPEC> {
        MULT_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset ECC Accelerator. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn mult_reset(&mut self) -> MULT_RESET_W<'_, MULT_CONF_SPEC> {
        MULT_RESET_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\ 2: P-384\\\\ 3: Reserved.\\\\"]
    #[inline(always)]
    pub fn mult_key_length(&mut self) -> MULT_KEY_LENGTH_W<'_, MULT_CONF_SPEC> {
        MULT_KEY_LENGTH_W::new(self, 2)
    }
    #[doc = "Bit 4 - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
    #[inline(always)]
    pub fn mult_mod_base(&mut self) -> MULT_MOD_BASE_W<'_, MULT_CONF_SPEC> {
        MULT_MOD_BASE_W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
    #[inline(always)]
    pub fn mult_work_mode(&mut self) -> MULT_WORK_MODE_W<'_, MULT_CONF_SPEC> {
        MULT_WORK_MODE_W::new(self, 5)
    }
    #[doc = "Bit 9 - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
    #[inline(always)]
    pub fn mult_security_mode(&mut self) -> MULT_SECURITY_MODE_W<'_, MULT_CONF_SPEC> {
        MULT_SECURITY_MODE_W::new(self, 9)
    }
    #[doc = "Bit 30 - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mult_clk_en(&mut self) -> MULT_CLK_EN_W<'_, MULT_CONF_SPEC> {
        MULT_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mult_mem_clock_gate_force_on(
        &mut self,
    ) -> MULT_MEM_CLOCK_GATE_FORCE_ON_W<'_, MULT_CONF_SPEC> {
        MULT_MEM_CLOCK_GATE_FORCE_ON_W::new(self, 31)
    }
}
#[doc = "ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_CONF_SPEC;
impl crate::RegisterSpec for MULT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_conf::R`](R) reader structure"]
impl crate::Readable for MULT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_conf::W`](W) writer structure"]
impl crate::Writable for MULT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_CONF to value 0"]
impl crate::Resettable for MULT_CONF_SPEC {}
