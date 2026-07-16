#[doc = "Register `PWR_GLITCH_CONF` reader"]
pub type R = crate::R<PWR_GLITCH_CONF_SPEC>;
#[doc = "Register `PWR_GLITCH_CONF` writer"]
pub type W = crate::W<PWR_GLITCH_CONF_SPEC>;
#[doc = "Field `GLITCH_EFUSE_CTRL` reader - needs field desc"]
pub type GLITCH_EFUSE_CTRL_R = crate::BitReader;
#[doc = "Field `GLITCH_EFUSE_CTRL` writer - needs field desc"]
pub type GLITCH_EFUSE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_EN` reader - needs field desc"]
pub type GLITCH_EN_R = crate::BitReader;
#[doc = "Field `GLITCH_EN` writer - needs field desc"]
pub type GLITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_CELL_SEL` reader - needs field desc"]
pub type GLITCH_CELL_SEL_R = crate::FieldReader;
#[doc = "Field `GLITCH_CELL_SEL` writer - needs field desc"]
pub type GLITCH_CELL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GLITCH_LIMIT` reader - needs field desc"]
pub type GLITCH_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `GLITCH_LIMIT` writer - needs field desc"]
pub type GLITCH_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn glitch_efuse_ctrl(&self) -> GLITCH_EFUSE_CTRL_R {
        GLITCH_EFUSE_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn glitch_en(&self) -> GLITCH_EN_R {
        GLITCH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - needs field desc"]
    #[inline(always)]
    pub fn glitch_cell_sel(&self) -> GLITCH_CELL_SEL_R {
        GLITCH_CELL_SEL_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:24 - needs field desc"]
    #[inline(always)]
    pub fn glitch_limit(&self) -> GLITCH_LIMIT_R {
        GLITCH_LIMIT_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_GLITCH_CONF")
            .field("glitch_efuse_ctrl", &self.glitch_efuse_ctrl())
            .field("glitch_en", &self.glitch_en())
            .field("glitch_cell_sel", &self.glitch_cell_sel())
            .field("glitch_limit", &self.glitch_limit())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn glitch_efuse_ctrl(&mut self) -> GLITCH_EFUSE_CTRL_W<'_, PWR_GLITCH_CONF_SPEC> {
        GLITCH_EFUSE_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn glitch_en(&mut self) -> GLITCH_EN_W<'_, PWR_GLITCH_CONF_SPEC> {
        GLITCH_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:8 - needs field desc"]
    #[inline(always)]
    pub fn glitch_cell_sel(&mut self) -> GLITCH_CELL_SEL_W<'_, PWR_GLITCH_CONF_SPEC> {
        GLITCH_CELL_SEL_W::new(self, 2)
    }
    #[doc = "Bits 9:24 - needs field desc"]
    #[inline(always)]
    pub fn glitch_limit(&mut self) -> GLITCH_LIMIT_W<'_, PWR_GLITCH_CONF_SPEC> {
        GLITCH_LIMIT_W::new(self, 9)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_glitch_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_glitch_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_GLITCH_CONF_SPEC;
impl crate::RegisterSpec for PWR_GLITCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_glitch_conf::R`](R) reader structure"]
impl crate::Readable for PWR_GLITCH_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_glitch_conf::W`](W) writer structure"]
impl crate::Writable for PWR_GLITCH_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_GLITCH_CONF to value 0x0001_fe01"]
impl crate::Resettable for PWR_GLITCH_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_fe01;
}
