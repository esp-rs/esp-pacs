#[doc = "Register `LP_ROM_AUX_CTRL` reader"]
pub type R = crate::R<LP_ROM_AUX_CTRL_SPEC>;
#[doc = "Register `LP_ROM_AUX_CTRL` writer"]
pub type W = crate::W<LP_ROM_AUX_CTRL_SPEC>;
#[doc = "Field `LP_ROM_AUX_CTRL` reader - need_des"]
pub type LP_ROM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `LP_ROM_AUX_CTRL` writer - need_des"]
pub type LP_ROM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_rom_aux_ctrl(&self) -> LP_ROM_AUX_CTRL_R {
        LP_ROM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ROM_AUX_CTRL")
            .field(
                "lp_rom_aux_ctrl",
                &format_args!("{}", self.lp_rom_aux_ctrl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ROM_AUX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_rom_aux_ctrl(&mut self) -> LP_ROM_AUX_CTRL_W<LP_ROM_AUX_CTRL_SPEC> {
        LP_ROM_AUX_CTRL_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rom_aux_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rom_aux_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ROM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for LP_ROM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rom_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_ROM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rom_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_ROM_AUX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_ROM_AUX_CTRL to value 0x70"]
impl crate::Resettable for LP_ROM_AUX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x70;
}