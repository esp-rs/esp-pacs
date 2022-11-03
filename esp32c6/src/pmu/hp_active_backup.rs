#[doc = "Register `HP_ACTIVE_BACKUP` reader"]
pub struct R(crate::R<HP_ACTIVE_BACKUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_ACTIVE_BACKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_ACTIVE_BACKUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_ACTIVE_BACKUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_ACTIVE_BACKUP` writer"]
pub struct W(crate::W<HP_ACTIVE_BACKUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_ACTIVE_BACKUP_SPEC>;
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
impl From<crate::W<HP_ACTIVE_BACKUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_ACTIVE_BACKUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `HP_ACTIVE_RETENTION_MODE` reader - need_des"]
pub type HP_ACTIVE_RETENTION_MODE_R = crate::BitReader<bool>;
#[doc = "Field `HP_ACTIVE_RETENTION_MODE` writer - need_des"]
pub type HP_ACTIVE_RETENTION_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, bool, O>;
#[doc = "Field `HP_SLEEP2ACTIVE_RETENTION_EN` reader - need_des"]
pub type HP_SLEEP2ACTIVE_RETENTION_EN_R = crate::BitReader<bool>;
#[doc = "Field `HP_SLEEP2ACTIVE_RETENTION_EN` writer - need_des"]
pub type HP_SLEEP2ACTIVE_RETENTION_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, bool, O>;
#[doc = "Field `HP_MODEM2ACTIVE_RETENTION_EN` reader - need_des"]
pub type HP_MODEM2ACTIVE_RETENTION_EN_R = crate::BitReader<bool>;
#[doc = "Field `HP_MODEM2ACTIVE_RETENTION_EN` writer - need_des"]
pub type HP_MODEM2ACTIVE_RETENTION_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, bool, O>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_CLK_SEL` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_CLK_SEL` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODE` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_MODE` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODE` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_MODE` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_EN` reader - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HP_SLEEP2ACTIVE_BACKUP_EN` writer - need_des"]
pub type HP_SLEEP2ACTIVE_BACKUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, bool, O>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_EN` reader - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HP_MODEM2ACTIVE_BACKUP_EN` writer - need_des"]
pub type HP_MODEM2ACTIVE_BACKUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_ACTIVE_BACKUP_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_modem_clk_code(
        &mut self,
    ) -> HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_W<4> {
        HP_SLEEP2ACTIVE_BACKUP_MODEM_CLK_CODE_W::new(self)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_modem_clk_code(
        &mut self,
    ) -> HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_W<6> {
        HP_MODEM2ACTIVE_BACKUP_MODEM_CLK_CODE_W::new(self)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_retention_mode(&mut self) -> HP_ACTIVE_RETENTION_MODE_W<10> {
        HP_ACTIVE_RETENTION_MODE_W::new(self)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_retention_en(&mut self) -> HP_SLEEP2ACTIVE_RETENTION_EN_W<11> {
        HP_SLEEP2ACTIVE_RETENTION_EN_W::new(self)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_retention_en(&mut self) -> HP_MODEM2ACTIVE_RETENTION_EN_W<12> {
        HP_MODEM2ACTIVE_RETENTION_EN_W::new(self)
    }
    #[doc = "Bits 14:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_clk_sel(&mut self) -> HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_W<14> {
        HP_SLEEP2ACTIVE_BACKUP_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_clk_sel(&mut self) -> HP_MODEM2ACTIVE_BACKUP_CLK_SEL_W<16> {
        HP_MODEM2ACTIVE_BACKUP_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 20:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_mode(&mut self) -> HP_SLEEP2ACTIVE_BACKUP_MODE_W<20> {
        HP_SLEEP2ACTIVE_BACKUP_MODE_W::new(self)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_mode(&mut self) -> HP_MODEM2ACTIVE_BACKUP_MODE_W<23> {
        HP_MODEM2ACTIVE_BACKUP_MODE_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep2active_backup_en(&mut self) -> HP_SLEEP2ACTIVE_BACKUP_EN_W<29> {
        HP_SLEEP2ACTIVE_BACKUP_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2active_backup_en(&mut self) -> HP_MODEM2ACTIVE_BACKUP_EN_W<30> {
        HP_MODEM2ACTIVE_BACKUP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_active_backup](index.html) module"]
pub struct HP_ACTIVE_BACKUP_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_BACKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_active_backup::R](R) reader structure"]
impl crate::Readable for HP_ACTIVE_BACKUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_active_backup::W](W) writer structure"]
impl crate::Writable for HP_ACTIVE_BACKUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_BACKUP to value 0"]
impl crate::Resettable for HP_ACTIVE_BACKUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
