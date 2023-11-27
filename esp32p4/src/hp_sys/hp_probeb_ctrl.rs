#[doc = "Register `HP_PROBEB_CTRL` reader"]
pub type R = crate::R<HP_PROBEB_CTRL_SPEC>;
#[doc = "Register `HP_PROBEB_CTRL` writer"]
pub type W = crate::W<HP_PROBEB_CTRL_SPEC>;
#[doc = "Field `HP_REG_PROBE_B_MOD_SEL` reader - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
pub type HP_REG_PROBE_B_MOD_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `HP_REG_PROBE_B_MOD_SEL` writer - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
pub type HP_REG_PROBE_B_MOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HP_REG_PROBE_B_TOP_SEL` reader - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
pub type HP_REG_PROBE_B_TOP_SEL_R = crate::FieldReader;
#[doc = "Field `HP_REG_PROBE_B_TOP_SEL` writer - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
pub type HP_REG_PROBE_B_TOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_REG_PROBE_B_EN` reader - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
pub type HP_REG_PROBE_B_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_PROBE_B_EN` writer - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
pub type HP_REG_PROBE_B_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
    #[inline(always)]
    pub fn hp_reg_probe_b_mod_sel(&self) -> HP_REG_PROBE_B_MOD_SEL_R {
        HP_REG_PROBE_B_MOD_SEL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
    #[inline(always)]
    pub fn hp_reg_probe_b_top_sel(&self) -> HP_REG_PROBE_B_TOP_SEL_R {
        HP_REG_PROBE_B_TOP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
    #[inline(always)]
    pub fn hp_reg_probe_b_en(&self) -> HP_REG_PROBE_B_EN_R {
        HP_REG_PROBE_B_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PROBEB_CTRL")
            .field(
                "hp_reg_probe_b_mod_sel",
                &format_args!("{}", self.hp_reg_probe_b_mod_sel().bits()),
            )
            .field(
                "hp_reg_probe_b_top_sel",
                &format_args!("{}", self.hp_reg_probe_b_top_sel().bits()),
            )
            .field(
                "hp_reg_probe_b_en",
                &format_args!("{}", self.hp_reg_probe_b_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_PROBEB_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_probe_b_mod_sel(&mut self) -> HP_REG_PROBE_B_MOD_SEL_W<HP_PROBEB_CTRL_SPEC> {
        HP_REG_PROBE_B_MOD_SEL_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_probe_b_top_sel(&mut self) -> HP_REG_PROBE_B_TOP_SEL_W<HP_PROBEB_CTRL_SPEC> {
        HP_REG_PROBE_B_TOP_SEL_W::new(self, 16)
    }
    #[doc = "Bit 24 - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_probe_b_en(&mut self) -> HP_REG_PROBE_B_EN_W<HP_PROBEB_CTRL_SPEC> {
        HP_REG_PROBE_B_EN_W::new(self, 24)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_probeb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_probeb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PROBEB_CTRL_SPEC;
impl crate::RegisterSpec for HP_PROBEB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_probeb_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_PROBEB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_probeb_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_PROBEB_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_PROBEB_CTRL to value 0"]
impl crate::Resettable for HP_PROBEB_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
