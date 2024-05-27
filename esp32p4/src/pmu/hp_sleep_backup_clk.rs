///Register `HP_SLEEP_BACKUP_CLK` reader
pub type R = crate::R<HP_SLEEP_BACKUP_CLK_SPEC>;
///Register `HP_SLEEP_BACKUP_CLK` writer
pub type W = crate::W<HP_SLEEP_BACKUP_CLK_SPEC>;
///Field `HP_SLEEP_BACKUP_ICG_FUNC_EN` reader - need_des
pub type HP_SLEEP_BACKUP_ICG_FUNC_EN_R = crate::FieldReader<u32>;
///Field `HP_SLEEP_BACKUP_ICG_FUNC_EN` writer - need_des
pub type HP_SLEEP_BACKUP_ICG_FUNC_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn hp_sleep_backup_icg_func_en(&self) -> HP_SLEEP_BACKUP_ICG_FUNC_EN_R {
        HP_SLEEP_BACKUP_ICG_FUNC_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_BACKUP_CLK")
            .field(
                "hp_sleep_backup_icg_func_en",
                &self.hp_sleep_backup_icg_func_en(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_backup_icg_func_en(
        &mut self,
    ) -> HP_SLEEP_BACKUP_ICG_FUNC_EN_W<HP_SLEEP_BACKUP_CLK_SPEC> {
        HP_SLEEP_BACKUP_ICG_FUNC_EN_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_backup_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_backup_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_SLEEP_BACKUP_CLK_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BACKUP_CLK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hp_sleep_backup_clk::R`](R) reader structure
impl crate::Readable for HP_SLEEP_BACKUP_CLK_SPEC {}
///`write(|w| ..)` method takes [`hp_sleep_backup_clk::W`](W) writer structure
impl crate::Writable for HP_SLEEP_BACKUP_CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_SLEEP_BACKUP_CLK to value 0
impl crate::Resettable for HP_SLEEP_BACKUP_CLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
