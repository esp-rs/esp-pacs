#[doc = "Register `PROBEB_CTRL` reader"]
pub type R = crate::R<PROBEB_CTRL_SPEC>;
#[doc = "Register `PROBEB_CTRL` writer"]
pub type W = crate::W<PROBEB_CTRL_SPEC>;
#[doc = "Field `REG_PROBE_B_MOD_SEL` reader - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
pub type REG_PROBE_B_MOD_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `REG_PROBE_B_MOD_SEL` writer - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
pub type REG_PROBE_B_MOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_PROBE_B_TOP_SEL` reader - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
pub type REG_PROBE_B_TOP_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PROBE_B_TOP_SEL` writer - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
pub type REG_PROBE_B_TOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_PROBE_B_EN` reader - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
pub type REG_PROBE_B_EN_R = crate::BitReader;
#[doc = "Field `REG_PROBE_B_EN` writer - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
pub type REG_PROBE_B_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
    #[inline(always)]
    pub fn reg_probe_b_mod_sel(&self) -> REG_PROBE_B_MOD_SEL_R {
        REG_PROBE_B_MOD_SEL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
    #[inline(always)]
    pub fn reg_probe_b_top_sel(&self) -> REG_PROBE_B_TOP_SEL_R {
        REG_PROBE_B_TOP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
    #[inline(always)]
    pub fn reg_probe_b_en(&self) -> REG_PROBE_B_EN_R {
        REG_PROBE_B_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROBEB_CTRL")
            .field("reg_probe_b_mod_sel", &self.reg_probe_b_mod_sel())
            .field("reg_probe_b_top_sel", &self.reg_probe_b_top_sel())
            .field("reg_probe_b_en", &self.reg_probe_b_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out\\[31:0\\] in b mode."]
    #[inline(always)]
    pub fn reg_probe_b_mod_sel(&mut self) -> REG_PROBE_B_MOD_SEL_W<PROBEB_CTRL_SPEC> {
        REG_PROBE_B_MOD_SEL_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Tihs field is used to select module's probe_out\\[31:0\\] as probe_out in b mode"]
    #[inline(always)]
    pub fn reg_probe_b_top_sel(&mut self) -> REG_PROBE_B_TOP_SEL_W<PROBEB_CTRL_SPEC> {
        REG_PROBE_B_TOP_SEL_W::new(self, 16)
    }
    #[doc = "Bit 24 - Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode."]
    #[inline(always)]
    pub fn reg_probe_b_en(&mut self) -> REG_PROBE_B_EN_W<PROBEB_CTRL_SPEC> {
        REG_PROBE_B_EN_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`probeb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`probeb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROBEB_CTRL_SPEC;
impl crate::RegisterSpec for PROBEB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probeb_ctrl::R`](R) reader structure"]
impl crate::Readable for PROBEB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`probeb_ctrl::W`](W) writer structure"]
impl crate::Writable for PROBEB_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROBEB_CTRL to value 0"]
impl crate::Resettable for PROBEB_CTRL_SPEC {}
