#[doc = "Register `HP_MODEM_BACKUP_CLK` reader"]
pub type R = crate::R<HP_MODEM_BACKUP_CLK_SPEC>;
#[doc = "Register `HP_MODEM_BACKUP_CLK` writer"]
pub type W = crate::W<HP_MODEM_BACKUP_CLK_SPEC>;
#[doc = "Field `HP_MODEM_BACKUP_ICG_FUNC_EN` reader - need_des"]
pub type HP_MODEM_BACKUP_ICG_FUNC_EN_R = crate::FieldReader<u32>;
#[doc = "Field `HP_MODEM_BACKUP_ICG_FUNC_EN` writer - need_des"]
pub type HP_MODEM_BACKUP_ICG_FUNC_EN_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_backup_icg_func_en(&self) -> HP_MODEM_BACKUP_ICG_FUNC_EN_R {
        HP_MODEM_BACKUP_ICG_FUNC_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_BACKUP_CLK")
            .field(
                "hp_modem_backup_icg_func_en",
                &format_args!("{}", self.hp_modem_backup_icg_func_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BACKUP_CLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_backup_icg_func_en(
        &mut self,
    ) -> HP_MODEM_BACKUP_ICG_FUNC_EN_W<HP_MODEM_BACKUP_CLK_SPEC, 0> {
        HP_MODEM_BACKUP_ICG_FUNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_modem_backup_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_backup_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_BACKUP_CLK_SPEC;
impl crate::RegisterSpec for HP_MODEM_BACKUP_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_backup_clk::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_BACKUP_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_backup_clk::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_BACKUP_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_BACKUP_CLK to value 0"]
impl crate::Resettable for HP_MODEM_BACKUP_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
