#[doc = "Register `HP_MODEM_BACKUP` writer"]
pub type W = crate::W<HP_MODEM_BACKUP_SPEC>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM_RETENTION_MODE` writer - need_des"]
pub type HP_MODEM_RETENTION_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP2MODEM_RETENTION_EN` writer - need_des"]
pub type HP_SLEEP2MODEM_RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_MODE` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_SLEEP2MODEM_BACKUP_EN` writer - need_des"]
pub type HP_SLEEP2MODEM_BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BACKUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_modem_clk_code(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W::new(self, 4)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_modem_retention_mode(&mut self) -> HP_MODEM_RETENTION_MODE_W<HP_MODEM_BACKUP_SPEC> {
        HP_MODEM_RETENTION_MODE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_retention_en(
        &mut self,
    ) -> HP_SLEEP2MODEM_RETENTION_EN_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_RETENTION_EN_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_clk_sel(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_CLK_SEL_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_CLK_SEL_W::new(self, 14)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_mode(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_MODE_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_MODE_W::new(self, 20)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2modem_backup_en(&mut self) -> HP_SLEEP2MODEM_BACKUP_EN_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_EN_W::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_BACKUP_SPEC;
impl crate::RegisterSpec for HP_MODEM_BACKUP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_backup::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_BACKUP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_BACKUP to value 0"]
impl crate::Resettable for HP_MODEM_BACKUP_SPEC {}
