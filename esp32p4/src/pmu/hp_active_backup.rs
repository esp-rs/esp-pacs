#[doc = "Register `HP_ACTIVE_BACKUP` reader"]
pub type R = crate::R<HP_ACTIVE_BACKUP_SPEC>;
#[doc = "Register `HP_ACTIVE_BACKUP` writer"]
pub type W = crate::W<HP_ACTIVE_BACKUP_SPEC>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_ACTIVE_RETENTION_MODE` reader - need_des"]
pub type HP_ACTIVE_RETENTION_MODE_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_RETENTION_MODE` writer - need_des"]
pub type HP_ACTIVE_RETENTION_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP2ACTIVE_RETENTION_EN` reader - need_des"]
pub type HP_SLEEP2ACTIVE_RETENTION_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP2ACTIVE_RETENTION_EN` writer - need_des"]
pub type HP_SLEEP2ACTIVE_RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM2ACTIVE_RETENTION_EN` reader - need_des"]
pub type HP_MODEM2ACTIVE_RETENTION_EN_R = crate::BitReader;
#[doc = "Field `HP_MODEM2ACTIVE_RETENTION_EN` writer - need_des"]
pub type HP_MODEM2ACTIVE_RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODE` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODE` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODE` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODE` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_EN` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_EN` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_EN` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_EN_R = crate::BitReader;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_EN` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2active_backup_modem_clk_code(&self) -> HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_R {
        HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    pub fn hp_modem2active_backup_modem_clk_code(&self) -> HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_R {
        HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_active_retention_mode(&self) -> HP_ACTIVE_RETENTION_MODE_R {
        HP_ACTIVE_RETENTION_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2active_retention_en(&self) -> HP_SLEEP2ACTIVE_RETENTION_EN_R {
        HP_SLEEP2ACTIVE_RETENTION_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn hp_modem2active_retention_en(&self) -> HP_MODEM2ACTIVE_RETENTION_EN_R {
        HP_MODEM2ACTIVE_RETENTION_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2active_backup_clk_sel(&self) -> HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_R {
        HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn hp_modem2active_backup_clk_sel(&self) -> HP_MODEM2ACTIVE_BACKUP_CLK_SEL_R {
        HP_MODEM2ACTIVE_BACKUP_CLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2active_backup_mode(&self) -> HP_SLEEP2ACTIVE_BACKUP_MODE_R {
        HP_SLEEP2ACTIVE_BACKUP_MODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn hp_modem2active_backup_mode(&self) -> HP_MODEM2ACTIVE_BACKUP_MODE_R {
        HP_MODEM2ACTIVE_BACKUP_MODE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep2active_backup_en(&self) -> HP_SLEEP2ACTIVE_BACKUP_EN_R {
        HP_SLEEP2ACTIVE_BACKUP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem2active_backup_en(&self) -> HP_MODEM2ACTIVE_BACKUP_EN_R {
        HP_MODEM2ACTIVE_BACKUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_BACKUP")
            .field(
                "hp_sleep2active_backup_modem_clk_code",
                &format_args!("{}", self.hp_sleep2active_backup_modem_clk_code().bits()),
            )
            .field(
                "hp_modem2active_backup_modem_clk_code",
                &format_args!("{}", self.hp_modem2active_backup_modem_clk_code().bits()),
            )
            .field(
                "hp_active_retention_mode",
                &format_args!("{}", self.hp_active_retention_mode().bit()),
            )
            .field(
                "hp_sleep2active_retention_en",
                &format_args!("{}", self.hp_sleep2active_retention_en().bit()),
            )
            .field(
                "hp_modem2active_retention_en",
                &format_args!("{}", self.hp_modem2active_retention_en().bit()),
            )
            .field(
                "hp_sleep2active_backup_clk_sel",
                &format_args!("{}", self.hp_sleep2active_backup_clk_sel().bits()),
            )
            .field(
                "hp_modem2active_backup_clk_sel",
                &format_args!("{}", self.hp_modem2active_backup_clk_sel().bits()),
            )
            .field(
                "hp_sleep2active_backup_mode",
                &format_args!("{}", self.hp_sleep2active_backup_mode().bits()),
            )
            .field(
                "hp_modem2active_backup_mode",
                &format_args!("{}", self.hp_modem2active_backup_mode().bits()),
            )
            .field(
                "hp_sleep2active_backup_en",
                &format_args!("{}", self.hp_sleep2active_backup_en().bit()),
            )
            .field(
                "hp_modem2active_backup_en",
                &format_args!("{}", self.hp_modem2active_backup_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_ACTIVE_BACKUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_modem_clk_code(
        &mut self,
    ) -> HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_modem_clk_code(
        &mut self,
    ) -> HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_W::new(self, 6)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_retention_mode(
        &mut self,
    ) -> HP_ACTIVE_RETENTION_MODE_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_ACTIVE_RETENTION_MODE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_retention_en(
        &mut self,
    ) -> HP_SLEEP2ACTIVE_RETENTION_EN_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_SLEEP2ACTIVE_RETENTION_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_retention_en(
        &mut self,
    ) -> HP_MODEM2ACTIVE_RETENTION_EN_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_MODEM2ACTIVE_RETENTION_EN_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_clk_sel(
        &mut self,
    ) -> HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_clk_sel(
        &mut self,
    ) -> HP_MODEM2ACTIVE_BACKUP_CLK_SEL_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_MODEM2ACTIVE_BACKUP_CLK_SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_mode(
        &mut self,
    ) -> HP_SLEEP2ACTIVE_BACKUP_MODE_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_SLEEP2ACTIVE_BACKUP_MODE_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_mode(
        &mut self,
    ) -> HP_MODEM2ACTIVE_BACKUP_MODE_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_MODEM2ACTIVE_BACKUP_MODE_W::new(self, 23)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_en(
        &mut self,
    ) -> HP_SLEEP2ACTIVE_BACKUP_EN_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_SLEEP2ACTIVE_BACKUP_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_en(
        &mut self,
    ) -> HP_MODEM2ACTIVE_BACKUP_EN_W<HP_ACTIVE_BACKUP_SPEC> {
        HP_MODEM2ACTIVE_BACKUP_EN_W::new(self, 30)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_active_backup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_backup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_BACKUP_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_BACKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_backup::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_BACKUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_backup::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_BACKUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_BACKUP to value 0"]
impl crate::Resettable for HP_ACTIVE_BACKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
