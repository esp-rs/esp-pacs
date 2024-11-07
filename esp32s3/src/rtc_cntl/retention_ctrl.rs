#[doc = "Register `RETENTION_CTRL` reader"]
pub type R = crate::R<RETENTION_CTRL_SPEC>;
#[doc = "Register `RETENTION_CTRL` writer"]
pub type W = crate::W<RETENTION_CTRL_SPEC>;
#[doc = "Field `RETENTION_TAG_MODE` reader - No public"]
pub type RETENTION_TAG_MODE_R = crate::FieldReader;
#[doc = "Field `RETENTION_TAG_MODE` writer - No public"]
pub type RETENTION_TAG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RETENTION_TARGET` reader - congfigure retention target cpu and/or tag"]
pub type RETENTION_TARGET_R = crate::FieldReader;
#[doc = "Field `RETENTION_TARGET` writer - congfigure retention target cpu and/or tag"]
pub type RETENTION_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RETENTION_CLK_SEL` reader - No public"]
pub type RETENTION_CLK_SEL_R = crate::BitReader;
#[doc = "Field `RETENTION_CLK_SEL` writer - No public"]
pub type RETENTION_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETENTION_DONE_WAIT` reader - wait retention done cycle"]
pub type RETENTION_DONE_WAIT_R = crate::FieldReader;
#[doc = "Field `RETENTION_DONE_WAIT` writer - wait retention done cycle"]
pub type RETENTION_DONE_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RETENTION_CLKOFF_WAIT` reader - wait clk off cycle"]
pub type RETENTION_CLKOFF_WAIT_R = crate::FieldReader;
#[doc = "Field `RETENTION_CLKOFF_WAIT` writer - wait clk off cycle"]
pub type RETENTION_CLKOFF_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RETENTION_EN` reader - enable retention"]
pub type RETENTION_EN_R = crate::BitReader;
#[doc = "Field `RETENTION_EN` writer - enable retention"]
pub type RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETENTION_WAIT` reader - wait cycles for rention operation"]
pub type RETENTION_WAIT_R = crate::FieldReader;
#[doc = "Field `RETENTION_WAIT` writer - wait cycles for rention operation"]
pub type RETENTION_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 10:13 - No public"]
    #[inline(always)]
    pub fn retention_tag_mode(&self) -> RETENTION_TAG_MODE_R {
        RETENTION_TAG_MODE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - congfigure retention target cpu and/or tag"]
    #[inline(always)]
    pub fn retention_target(&self) -> RETENTION_TARGET_R {
        RETENTION_TARGET_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - No public"]
    #[inline(always)]
    pub fn retention_clk_sel(&self) -> RETENTION_CLK_SEL_R {
        RETENTION_CLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - wait retention done cycle"]
    #[inline(always)]
    pub fn retention_done_wait(&self) -> RETENTION_DONE_WAIT_R {
        RETENTION_DONE_WAIT_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - wait clk off cycle"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&self) -> RETENTION_CLKOFF_WAIT_R {
        RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - enable retention"]
    #[inline(always)]
    pub fn retention_en(&self) -> RETENTION_EN_R {
        RETENTION_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&self) -> RETENTION_WAIT_R {
        RETENTION_WAIT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL")
            .field("retention_tag_mode", &self.retention_tag_mode())
            .field("retention_target", &self.retention_target())
            .field("retention_clk_sel", &self.retention_clk_sel())
            .field("retention_done_wait", &self.retention_done_wait())
            .field("retention_clkoff_wait", &self.retention_clkoff_wait())
            .field("retention_en", &self.retention_en())
            .field("retention_wait", &self.retention_wait())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:13 - No public"]
    #[inline(always)]
    pub fn retention_tag_mode(&mut self) -> RETENTION_TAG_MODE_W<RETENTION_CTRL_SPEC> {
        RETENTION_TAG_MODE_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - congfigure retention target cpu and/or tag"]
    #[inline(always)]
    pub fn retention_target(&mut self) -> RETENTION_TARGET_W<RETENTION_CTRL_SPEC> {
        RETENTION_TARGET_W::new(self, 14)
    }
    #[doc = "Bit 16 - No public"]
    #[inline(always)]
    pub fn retention_clk_sel(&mut self) -> RETENTION_CLK_SEL_W<RETENTION_CTRL_SPEC> {
        RETENTION_CLK_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - wait retention done cycle"]
    #[inline(always)]
    pub fn retention_done_wait(&mut self) -> RETENTION_DONE_WAIT_W<RETENTION_CTRL_SPEC> {
        RETENTION_DONE_WAIT_W::new(self, 17)
    }
    #[doc = "Bits 20:23 - wait clk off cycle"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&mut self) -> RETENTION_CLKOFF_WAIT_W<RETENTION_CTRL_SPEC> {
        RETENTION_CLKOFF_WAIT_W::new(self, 20)
    }
    #[doc = "Bit 24 - enable retention"]
    #[inline(always)]
    pub fn retention_en(&mut self) -> RETENTION_EN_W<RETENTION_CTRL_SPEC> {
        RETENTION_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:31 - wait cycles for rention operation"]
    #[inline(always)]
    pub fn retention_wait(&mut self) -> RETENTION_WAIT_W<RETENTION_CTRL_SPEC> {
        RETENTION_WAIT_W::new(self, 25)
    }
}
#[doc = "configure retention\n\nYou can [`read`](crate::Reg::read) this register and get [`retention_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`retention_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_ctrl::R`](R) reader structure"]
impl crate::Readable for RETENTION_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_ctrl::W`](W) writer structure"]
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL to value 0x2834_0000"]
impl crate::Resettable for RETENTION_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2834_0000;
}
