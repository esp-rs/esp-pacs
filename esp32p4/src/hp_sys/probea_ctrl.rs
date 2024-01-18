#[doc = "Register `PROBEA_CTRL` reader"]
pub type R = crate::R<PROBEA_CTRL_SPEC>;
#[doc = "Register `PROBEA_CTRL` writer"]
pub type W = crate::W<PROBEA_CTRL_SPEC>;
#[doc = "Field `REG_PROBE_A_MOD_SEL` reader - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in a mode"]
pub type REG_PROBE_A_MOD_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `REG_PROBE_A_MOD_SEL` writer - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in a mode"]
pub type REG_PROBE_A_MOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_PROBE_A_TOP_SEL` reader - Tihs field is used to selec module's probe_out\\[31:0\\] as probe out in a mode"]
pub type REG_PROBE_A_TOP_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PROBE_A_TOP_SEL` writer - Tihs field is used to selec module's probe_out\\[31:0\\] as probe out in a mode"]
pub type REG_PROBE_A_TOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_PROBE_L_SEL` reader - Tihs field is used to selec probe_out\\[31:16\\]"]
pub type REG_PROBE_L_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PROBE_L_SEL` writer - Tihs field is used to selec probe_out\\[31:16\\]"]
pub type REG_PROBE_L_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PROBE_H_SEL` reader - Tihs field is used to selec probe_out\\[31:16\\]"]
pub type REG_PROBE_H_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PROBE_H_SEL` writer - Tihs field is used to selec probe_out\\[31:16\\]"]
pub type REG_PROBE_H_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PROBE_GLOBAL_EN` reader - Set this bit to enable global debug probe in hp system."]
pub type REG_PROBE_GLOBAL_EN_R = crate::BitReader;
#[doc = "Field `REG_PROBE_GLOBAL_EN` writer - Set this bit to enable global debug probe in hp system."]
pub type REG_PROBE_GLOBAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in a mode"]
    #[inline(always)]
    pub fn reg_probe_a_mod_sel(&self) -> REG_PROBE_A_MOD_SEL_R {
        REG_PROBE_A_MOD_SEL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Tihs field is used to selec module's probe_out\\[31:0\\] as probe out in a mode"]
    #[inline(always)]
    pub fn reg_probe_a_top_sel(&self) -> REG_PROBE_A_TOP_SEL_R {
        REG_PROBE_A_TOP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Tihs field is used to selec probe_out\\[31:16\\]"]
    #[inline(always)]
    pub fn reg_probe_l_sel(&self) -> REG_PROBE_L_SEL_R {
        REG_PROBE_L_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Tihs field is used to selec probe_out\\[31:16\\]"]
    #[inline(always)]
    pub fn reg_probe_h_sel(&self) -> REG_PROBE_H_SEL_R {
        REG_PROBE_H_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Set this bit to enable global debug probe in hp system."]
    #[inline(always)]
    pub fn reg_probe_global_en(&self) -> REG_PROBE_GLOBAL_EN_R {
        REG_PROBE_GLOBAL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROBEA_CTRL")
            .field(
                "reg_probe_a_mod_sel",
                &format_args!("{}", self.reg_probe_a_mod_sel().bits()),
            )
            .field(
                "reg_probe_a_top_sel",
                &format_args!("{}", self.reg_probe_a_top_sel().bits()),
            )
            .field(
                "reg_probe_l_sel",
                &format_args!("{}", self.reg_probe_l_sel().bits()),
            )
            .field(
                "reg_probe_h_sel",
                &format_args!("{}", self.reg_probe_h_sel().bits()),
            )
            .field(
                "reg_probe_global_en",
                &format_args!("{}", self.reg_probe_global_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PROBEA_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in a mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_probe_a_mod_sel(&mut self) -> REG_PROBE_A_MOD_SEL_W<PROBEA_CTRL_SPEC> {
        REG_PROBE_A_MOD_SEL_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Tihs field is used to selec module's probe_out\\[31:0\\] as probe out in a mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_probe_a_top_sel(&mut self) -> REG_PROBE_A_TOP_SEL_W<PROBEA_CTRL_SPEC> {
        REG_PROBE_A_TOP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Tihs field is used to selec probe_out\\[31:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn reg_probe_l_sel(&mut self) -> REG_PROBE_L_SEL_W<PROBEA_CTRL_SPEC> {
        REG_PROBE_L_SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Tihs field is used to selec probe_out\\[31:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn reg_probe_h_sel(&mut self) -> REG_PROBE_H_SEL_W<PROBEA_CTRL_SPEC> {
        REG_PROBE_H_SEL_W::new(self, 26)
    }
    #[doc = "Bit 28 - Set this bit to enable global debug probe in hp system."]
    #[inline(always)]
    #[must_use]
    pub fn reg_probe_global_en(&mut self) -> REG_PROBE_GLOBAL_EN_W<PROBEA_CTRL_SPEC> {
        REG_PROBE_GLOBAL_EN_W::new(self, 28)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probea_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probea_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROBEA_CTRL_SPEC;
impl crate::RegisterSpec for PROBEA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probea_ctrl::R`](R) reader structure"]
impl crate::Readable for PROBEA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`probea_ctrl::W`](W) writer structure"]
impl crate::Writable for PROBEA_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROBEA_CTRL to value 0"]
impl crate::Resettable for PROBEA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
