#[doc = "Register `HP_MODEM_DIAG_EN` reader"]
pub type R = crate::R<HP_MODEM_DIAG_EN_SPEC>;
#[doc = "Register `HP_MODEM_DIAG_EN` writer"]
pub type W = crate::W<HP_MODEM_DIAG_EN_SPEC>;
#[doc = "Field `HP_REG_MODEM_DIAG_EN` reader - set reg_modem_diag_en\\[15:0\\] will let hp_probe_out\\[15:0\\] used as modem_diag\\[15:0\\], set reg_modem_diag_en\\[31:16\\] will let lcd_data_out\\[15:0\\] used as modem_diag\\[31:16\\]"]
pub type HP_REG_MODEM_DIAG_EN_R = crate::FieldReader<u32>;
#[doc = "Field `HP_REG_MODEM_DIAG_EN` writer - set reg_modem_diag_en\\[15:0\\] will let hp_probe_out\\[15:0\\] used as modem_diag\\[15:0\\], set reg_modem_diag_en\\[31:16\\] will let lcd_data_out\\[15:0\\] used as modem_diag\\[31:16\\]"]
pub type HP_REG_MODEM_DIAG_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - set reg_modem_diag_en\\[15:0\\] will let hp_probe_out\\[15:0\\] used as modem_diag\\[15:0\\], set reg_modem_diag_en\\[31:16\\] will let lcd_data_out\\[15:0\\] used as modem_diag\\[31:16\\]"]
    #[inline(always)]
    pub fn hp_reg_modem_diag_en(&self) -> HP_REG_MODEM_DIAG_EN_R {
        HP_REG_MODEM_DIAG_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_DIAG_EN")
            .field("hp_reg_modem_diag_en", &self.hp_reg_modem_diag_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - set reg_modem_diag_en\\[15:0\\] will let hp_probe_out\\[15:0\\] used as modem_diag\\[15:0\\], set reg_modem_diag_en\\[31:16\\] will let lcd_data_out\\[15:0\\] used as modem_diag\\[31:16\\]"]
    #[inline(always)]
    pub fn hp_reg_modem_diag_en(&mut self) -> HP_REG_MODEM_DIAG_EN_W<'_, HP_MODEM_DIAG_EN_SPEC> {
        HP_REG_MODEM_DIAG_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_diag_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_diag_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_DIAG_EN_SPEC;
impl crate::RegisterSpec for HP_MODEM_DIAG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_diag_en::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_DIAG_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_diag_en::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_DIAG_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_DIAG_EN to value 0"]
impl crate::Resettable for HP_MODEM_DIAG_EN_SPEC {}
