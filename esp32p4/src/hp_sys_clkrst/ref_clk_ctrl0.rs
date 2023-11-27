#[doc = "Register `REF_CLK_CTRL0` reader"]
pub type R = crate::R<REF_CLK_CTRL0_SPEC>;
#[doc = "Register `REF_CLK_CTRL0` writer"]
pub type W = crate::W<REF_CLK_CTRL0_SPEC>;
#[doc = "Field `REG_REF_50M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_50M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_50M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_50M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_REF_25M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_25M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_25M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_25M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_REF_240M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_240M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_240M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_240M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_REF_160M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_160M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_160M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_160M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_50m_clk_div_num(&self) -> REG_REF_50M_CLK_DIV_NUM_R {
        REG_REF_50M_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_25m_clk_div_num(&self) -> REG_REF_25M_CLK_DIV_NUM_R {
        REG_REF_25M_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_240m_clk_div_num(&self) -> REG_REF_240M_CLK_DIV_NUM_R {
        REG_REF_240M_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_160m_clk_div_num(&self) -> REG_REF_160M_CLK_DIV_NUM_R {
        REG_REF_160M_CLK_DIV_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CLK_CTRL0")
            .field(
                "reg_ref_50m_clk_div_num",
                &format_args!("{}", self.reg_ref_50m_clk_div_num().bits()),
            )
            .field(
                "reg_ref_25m_clk_div_num",
                &format_args!("{}", self.reg_ref_25m_clk_div_num().bits()),
            )
            .field(
                "reg_ref_240m_clk_div_num",
                &format_args!("{}", self.reg_ref_240m_clk_div_num().bits()),
            )
            .field(
                "reg_ref_160m_clk_div_num",
                &format_args!("{}", self.reg_ref_160m_clk_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CLK_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_50m_clk_div_num(&mut self) -> REG_REF_50M_CLK_DIV_NUM_W<REF_CLK_CTRL0_SPEC> {
        REG_REF_50M_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_25m_clk_div_num(&mut self) -> REG_REF_25M_CLK_DIV_NUM_W<REF_CLK_CTRL0_SPEC> {
        REG_REF_25M_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_240m_clk_div_num(&mut self) -> REG_REF_240M_CLK_DIV_NUM_W<REF_CLK_CTRL0_SPEC> {
        REG_REF_240M_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_160m_clk_div_num(&mut self) -> REG_REF_160M_CLK_DIV_NUM_W<REF_CLK_CTRL0_SPEC> {
        REG_REF_160M_CLK_DIV_NUM_W::new(self, 24)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ref_clk_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_clk_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for REF_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for REF_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for REF_CLK_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CLK_CTRL0 to value 0x0201_1309"]
impl crate::Resettable for REF_CLK_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0201_1309;
}
