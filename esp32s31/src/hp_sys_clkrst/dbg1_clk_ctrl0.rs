#[doc = "Register `DBG1_CLK_CTRL0` reader"]
pub type R = crate::R<DBG1_CLK_CTRL0_SPEC>;
#[doc = "Register `DBG1_CLK_CTRL0` writer"]
pub type W = crate::W<DBG1_CLK_CTRL0_SPEC>;
#[doc = "Field `REG_DBG_CH1_SEL` reader - need_des"]
pub type REG_DBG_CH1_SEL_R = crate::FieldReader;
#[doc = "Field `REG_DBG_CH1_SEL` writer - need_des"]
pub type REG_DBG_CH1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DBG_CH1_DIV_NUM` reader - need_des"]
pub type REG_DBG_CH1_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_DBG_CH1_DIV_NUM` writer - need_des"]
pub type REG_DBG_CH1_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DBG_CH1_EN` reader - need_des"]
pub type REG_DBG_CH1_EN_R = crate::BitReader;
#[doc = "Field `REG_DBG_CH1_EN` writer - need_des"]
pub type REG_DBG_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn reg_dbg_ch1_sel(&self) -> REG_DBG_CH1_SEL_R {
        REG_DBG_CH1_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn reg_dbg_ch1_div_num(&self) -> REG_DBG_CH1_DIV_NUM_R {
        REG_DBG_CH1_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn reg_dbg_ch1_en(&self) -> REG_DBG_CH1_EN_R {
        REG_DBG_CH1_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG1_CLK_CTRL0")
            .field("reg_dbg_ch1_sel", &self.reg_dbg_ch1_sel())
            .field("reg_dbg_ch1_div_num", &self.reg_dbg_ch1_div_num())
            .field("reg_dbg_ch1_en", &self.reg_dbg_ch1_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn reg_dbg_ch1_sel(&mut self) -> REG_DBG_CH1_SEL_W<'_, DBG1_CLK_CTRL0_SPEC> {
        REG_DBG_CH1_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn reg_dbg_ch1_div_num(&mut self) -> REG_DBG_CH1_DIV_NUM_W<'_, DBG1_CLK_CTRL0_SPEC> {
        REG_DBG_CH1_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn reg_dbg_ch1_en(&mut self) -> REG_DBG_CH1_EN_W<'_, DBG1_CLK_CTRL0_SPEC> {
        REG_DBG_CH1_EN_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg1_clk_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg1_clk_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG1_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for DBG1_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg1_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for DBG1_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg1_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for DBG1_CLK_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG1_CLK_CTRL0 to value 0x03ff"]
impl crate::Resettable for DBG1_CLK_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
