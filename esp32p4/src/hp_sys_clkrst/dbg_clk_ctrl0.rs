#[doc = "Register `DBG_CLK_CTRL0` reader"]
pub type R = crate::R<DBG_CLK_CTRL0_SPEC>;
#[doc = "Register `DBG_CLK_CTRL0` writer"]
pub type W = crate::W<DBG_CLK_CTRL0_SPEC>;
#[doc = "Field `REG_DBG_CH0_SEL` reader - Reserved"]
pub type REG_DBG_CH0_SEL_R = crate::FieldReader;
#[doc = "Field `REG_DBG_CH0_SEL` writer - Reserved"]
pub type REG_DBG_CH0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DBG_CH1_SEL` reader - Reserved"]
pub type REG_DBG_CH1_SEL_R = crate::FieldReader;
#[doc = "Field `REG_DBG_CH1_SEL` writer - Reserved"]
pub type REG_DBG_CH1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DBG_CH2_SEL` reader - Reserved"]
pub type REG_DBG_CH2_SEL_R = crate::FieldReader;
#[doc = "Field `REG_DBG_CH2_SEL` writer - Reserved"]
pub type REG_DBG_CH2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DBG_CH0_DIV_NUM` reader - Reserved"]
pub type REG_DBG_CH0_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_DBG_CH0_DIV_NUM` writer - Reserved"]
pub type REG_DBG_CH0_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_dbg_ch0_sel(&self) -> REG_DBG_CH0_SEL_R {
        REG_DBG_CH0_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reg_dbg_ch1_sel(&self) -> REG_DBG_CH1_SEL_R {
        REG_DBG_CH1_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reg_dbg_ch2_sel(&self) -> REG_DBG_CH2_SEL_R {
        REG_DBG_CH2_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn reg_dbg_ch0_div_num(&self) -> REG_DBG_CH0_DIV_NUM_R {
        REG_DBG_CH0_DIV_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_CLK_CTRL0")
            .field(
                "reg_dbg_ch0_sel",
                &format_args!("{}", self.reg_dbg_ch0_sel().bits()),
            )
            .field(
                "reg_dbg_ch1_sel",
                &format_args!("{}", self.reg_dbg_ch1_sel().bits()),
            )
            .field(
                "reg_dbg_ch2_sel",
                &format_args!("{}", self.reg_dbg_ch2_sel().bits()),
            )
            .field(
                "reg_dbg_ch0_div_num",
                &format_args!("{}", self.reg_dbg_ch0_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBG_CLK_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ch0_sel(&mut self) -> REG_DBG_CH0_SEL_W<DBG_CLK_CTRL0_SPEC> {
        REG_DBG_CH0_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ch1_sel(&mut self) -> REG_DBG_CH1_SEL_W<DBG_CLK_CTRL0_SPEC> {
        REG_DBG_CH1_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ch2_sel(&mut self) -> REG_DBG_CH2_SEL_W<DBG_CLK_CTRL0_SPEC> {
        REG_DBG_CH2_SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ch0_div_num(&mut self) -> REG_DBG_CH0_DIV_NUM_W<DBG_CLK_CTRL0_SPEC> {
        REG_DBG_CH0_DIV_NUM_W::new(self, 24)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_clk_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_clk_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for DBG_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for DBG_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for DBG_CLK_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_CLK_CTRL0 to value 0x03ff_ffff"]
impl crate::Resettable for DBG_CLK_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
