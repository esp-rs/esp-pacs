#[doc = "Register `HP_SLEEP_BACKUP` reader"]
pub struct R(crate::R<HP_SLEEP_BACKUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_SLEEP_BACKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_SLEEP_BACKUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_SLEEP_BACKUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_SLEEP_BACKUP` writer"]
pub struct W(crate::W<HP_SLEEP_BACKUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_SLEEP_BACKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HP_SLEEP_BACKUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_SLEEP_BACKUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_BACKUP_SPEC, 2, O>;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_BACKUP_SPEC, 2, O>;
#[doc = "Field `HP_SLEEP_RETENTION_MODE` reader - need_des"]
pub type HP_SLEEP_RETENTION_MODE_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_RETENTION_MODE` writer - need_des"]
pub type HP_SLEEP_RETENTION_MODE_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_BACKUP_SPEC, O>;
#[doc = "Field `HP_MODEM2SLEEP_RETENTION_EN` reader - need_des"]
pub type HP_MODEM2SLEEP_RETENTION_EN_R = crate::BitReader;
#[doc = "Field `HP_MODEM2SLEEP_RETENTION_EN` writer - need_des"]
pub type HP_MODEM2SLEEP_RETENTION_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_BACKUP_SPEC, O>;
#[doc = "Field `HP_ACTIVE2SLEEP_RETENTION_EN` reader - need_des"]
pub type HP_ACTIVE2SLEEP_RETENTION_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE2SLEEP_RETENTION_EN` writer - need_des"]
pub type HP_ACTIVE2SLEEP_RETENTION_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_BACKUP_SPEC, O>;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_BACKUP_SPEC, 2, O>;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_BACKUP_SPEC, 2, O>;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_MODE` reader - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_MODE` writer - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_BACKUP_SPEC, 3, O>;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_MODE` reader - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_MODE` writer - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_BACKUP_SPEC, 3, O>;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_EN` reader - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_EN_R = crate::BitReader;
#[doc = "Field `HP_MODEM2SLEEP_BACKUP_EN` writer - need_des"]
pub type HP_MODEM2SLEEP_BACKUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_BACKUP_SPEC, O>;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_EN` reader - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE2SLEEP_BACKUP_EN` writer - need_des"]
pub type HP_ACTIVE2SLEEP_BACKUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_BACKUP_SPEC, O>;
impl R {
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    pub fn hp_modem2sleep_backup_modem_clk_code(&self) -> HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_R {
        HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - need_des"]
    #[inline(always)]
    pub fn hp_active2sleep_backup_modem_clk_code(&self) -> HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_R {
        HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_retention_mode(&self) -> HP_SLEEP_RETENTION_MODE_R {
        HP_SLEEP_RETENTION_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn hp_modem2sleep_retention_en(&self) -> HP_MODEM2SLEEP_RETENTION_EN_R {
        HP_MODEM2SLEEP_RETENTION_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn hp_active2sleep_retention_en(&self) -> HP_ACTIVE2SLEEP_RETENTION_EN_R {
        HP_ACTIVE2SLEEP_RETENTION_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn hp_modem2sleep_backup_clk_sel(&self) -> HP_MODEM2SLEEP_BACKUP_CLK_SEL_R {
        HP_MODEM2SLEEP_BACKUP_CLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - need_des"]
    #[inline(always)]
    pub fn hp_active2sleep_backup_clk_sel(&self) -> HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_R {
        HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn hp_modem2sleep_backup_mode(&self) -> HP_MODEM2SLEEP_BACKUP_MODE_R {
        HP_MODEM2SLEEP_BACKUP_MODE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - need_des"]
    #[inline(always)]
    pub fn hp_active2sleep_backup_mode(&self) -> HP_ACTIVE2SLEEP_BACKUP_MODE_R {
        HP_ACTIVE2SLEEP_BACKUP_MODE_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem2sleep_backup_en(&self) -> HP_MODEM2SLEEP_BACKUP_EN_R {
        HP_MODEM2SLEEP_BACKUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_active2sleep_backup_en(&self) -> HP_ACTIVE2SLEEP_BACKUP_EN_R {
        HP_ACTIVE2SLEEP_BACKUP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_BACKUP")
            .field(
                "hp_modem2sleep_backup_modem_clk_code",
                &format_args!("{}", self.hp_modem2sleep_backup_modem_clk_code().bits()),
            )
            .field(
                "hp_active2sleep_backup_modem_clk_code",
                &format_args!("{}", self.hp_active2sleep_backup_modem_clk_code().bits()),
            )
            .field(
                "hp_sleep_retention_mode",
                &format_args!("{}", self.hp_sleep_retention_mode().bit()),
            )
            .field(
                "hp_modem2sleep_retention_en",
                &format_args!("{}", self.hp_modem2sleep_retention_en().bit()),
            )
            .field(
                "hp_active2sleep_retention_en",
                &format_args!("{}", self.hp_active2sleep_retention_en().bit()),
            )
            .field(
                "hp_modem2sleep_backup_clk_sel",
                &format_args!("{}", self.hp_modem2sleep_backup_clk_sel().bits()),
            )
            .field(
                "hp_active2sleep_backup_clk_sel",
                &format_args!("{}", self.hp_active2sleep_backup_clk_sel().bits()),
            )
            .field(
                "hp_modem2sleep_backup_mode",
                &format_args!("{}", self.hp_modem2sleep_backup_mode().bits()),
            )
            .field(
                "hp_active2sleep_backup_mode",
                &format_args!("{}", self.hp_active2sleep_backup_mode().bits()),
            )
            .field(
                "hp_modem2sleep_backup_en",
                &format_args!("{}", self.hp_modem2sleep_backup_en().bit()),
            )
            .field(
                "hp_active2sleep_backup_en",
                &format_args!("{}", self.hp_active2sleep_backup_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_BACKUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_modem_clk_code(
        &mut self,
    ) -> HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_W<6> {
        HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_W::new(self)
    }
    #[doc = "Bits 8:9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_modem_clk_code(
        &mut self,
    ) -> HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_W<8> {
        HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_W::new(self)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_retention_mode(&mut self) -> HP_SLEEP_RETENTION_MODE_W<10> {
        HP_SLEEP_RETENTION_MODE_W::new(self)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_retention_en(&mut self) -> HP_MODEM2SLEEP_RETENTION_EN_W<12> {
        HP_MODEM2SLEEP_RETENTION_EN_W::new(self)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_retention_en(&mut self) -> HP_ACTIVE2SLEEP_RETENTION_EN_W<13> {
        HP_ACTIVE2SLEEP_RETENTION_EN_W::new(self)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_clk_sel(&mut self) -> HP_MODEM2SLEEP_BACKUP_CLK_SEL_W<16> {
        HP_MODEM2SLEEP_BACKUP_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_clk_sel(&mut self) -> HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_W<18> {
        HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_mode(&mut self) -> HP_MODEM2SLEEP_BACKUP_MODE_W<23> {
        HP_MODEM2SLEEP_BACKUP_MODE_W::new(self)
    }
    #[doc = "Bits 26:28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_mode(&mut self) -> HP_ACTIVE2SLEEP_BACKUP_MODE_W<26> {
        HP_ACTIVE2SLEEP_BACKUP_MODE_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_en(&mut self) -> HP_MODEM2SLEEP_BACKUP_EN_W<30> {
        HP_MODEM2SLEEP_BACKUP_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_en(&mut self) -> HP_ACTIVE2SLEEP_BACKUP_EN_W<31> {
        HP_ACTIVE2SLEEP_BACKUP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_sleep_backup](index.html) module"]
pub struct HP_SLEEP_BACKUP_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BACKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_sleep_backup::R](R) reader structure"]
impl crate::Readable for HP_SLEEP_BACKUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_sleep_backup::W](W) writer structure"]
impl crate::Writable for HP_SLEEP_BACKUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_BACKUP to value 0"]
impl crate::Resettable for HP_SLEEP_BACKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
