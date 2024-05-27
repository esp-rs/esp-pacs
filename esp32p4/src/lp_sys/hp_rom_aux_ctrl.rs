#[doc = "Register `HP_ROM_AUX_CTRL` reader"]
pub type R = crate::R<HP_ROM_AUX_CTRL_SPEC>;
#[doc = "Register `HP_ROM_AUX_CTRL` writer"]
pub type W = crate::W<HP_ROM_AUX_CTRL_SPEC>;
#[doc = "Field `HP_ROM_AUX_CTRL` reader - need_des"]
pub type HP_ROM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `HP_ROM_AUX_CTRL` writer - need_des"]
pub type HP_ROM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_rom_aux_ctrl(&self) -> HP_ROM_AUX_CTRL_R {
        HP_ROM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ROM_AUX_CTRL")
            .field("hp_rom_aux_ctrl", &self.hp_rom_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_rom_aux_ctrl(&mut self) -> HP_ROM_AUX_CTRL_W<HP_ROM_AUX_CTRL_SPEC> {
        HP_ROM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_rom_aux_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_rom_aux_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ROM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for HP_ROM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_rom_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_ROM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_rom_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_ROM_AUX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_ROM_AUX_CTRL to value 0x70"]
impl crate::Resettable for HP_ROM_AUX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x70;
}
