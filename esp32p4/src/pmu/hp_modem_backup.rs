///Register `HP_MODEM_BACKUP` writer
pub type W = crate::W<HP_MODEM_BACKUP_SPEC>;
///Field `HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE` writer - need_des
pub type HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HP_MODEM_RETENTION_MODE` writer - need_des
pub type HP_MODEM_RETENTION_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_SLEEP2MODEM_RETENTION_EN` writer - need_des
pub type HP_SLEEP2MODEM_RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_SLEEP2MODEM_BACKUP_CLK_SEL` writer - need_des
pub type HP_SLEEP2MODEM_BACKUP_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HP_SLEEP2MODEM_BACKUP_MODE` writer - need_des
pub type HP_SLEEP2MODEM_BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HP_SLEEP2MODEM_BACKUP_EN` writer - need_des
pub type HP_SLEEP2MODEM_BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BACKUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 4:5 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_modem_clk_code(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_MODEM_CLK_CODE_W::new(self, 4)
    }
    ///Bit 10 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_retention_mode(&mut self) -> HP_MODEM_RETENTION_MODE_W<HP_MODEM_BACKUP_SPEC> {
        HP_MODEM_RETENTION_MODE_W::new(self, 10)
    }
    ///Bit 11 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_retention_en(
        &mut self,
    ) -> HP_SLEEP2MODEM_RETENTION_EN_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_RETENTION_EN_W::new(self, 11)
    }
    ///Bits 14:15 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_clk_sel(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_CLK_SEL_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_CLK_SEL_W::new(self, 14)
    }
    ///Bits 20:22 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_mode(
        &mut self,
    ) -> HP_SLEEP2MODEM_BACKUP_MODE_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_MODE_W::new(self, 20)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2modem_backup_en(&mut self) -> HP_SLEEP2MODEM_BACKUP_EN_W<HP_MODEM_BACKUP_SPEC> {
        HP_SLEEP2MODEM_BACKUP_EN_W::new(self, 29)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_backup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_MODEM_BACKUP_SPEC;
impl crate::RegisterSpec for HP_MODEM_BACKUP_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hp_modem_backup::W`](W) writer structure
impl crate::Writable for HP_MODEM_BACKUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_MODEM_BACKUP to value 0
impl crate::Resettable for HP_MODEM_BACKUP_SPEC {
    const RESET_VALUE: u32 = 0;
}
