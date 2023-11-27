#[doc = "Register `REF_CLK_CTRL2` reader"]
pub type R = crate::R<REF_CLK_CTRL2_SPEC>;
#[doc = "Register `REF_CLK_CTRL2` writer"]
pub type W = crate::W<REF_CLK_CTRL2_SPEC>;
#[doc = "Field `REG_REF_160M_CLK_EN` reader - Reserved"]
pub type REG_REF_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_160M_CLK_EN` writer - Reserved"]
pub type REG_REF_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_160M_CLK_EN` reader - Reserved"]
pub type REG_TM_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_160M_CLK_EN` writer - Reserved"]
pub type REG_TM_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_120M_CLK_EN` reader - Reserved"]
pub type REG_REF_120M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_120M_CLK_EN` writer - Reserved"]
pub type REG_REF_120M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_120M_CLK_EN` reader - Reserved"]
pub type REG_TM_120M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_120M_CLK_EN` writer - Reserved"]
pub type REG_TM_120M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_80M_CLK_EN` reader - Reserved"]
pub type REG_REF_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_80M_CLK_EN` writer - Reserved"]
pub type REG_REF_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_80M_CLK_EN` reader - Reserved"]
pub type REG_TM_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_80M_CLK_EN` writer - Reserved"]
pub type REG_TM_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_60M_CLK_EN` reader - Reserved"]
pub type REG_TM_60M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_60M_CLK_EN` writer - Reserved"]
pub type REG_TM_60M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_48M_CLK_EN` reader - Reserved"]
pub type REG_TM_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_48M_CLK_EN` writer - Reserved"]
pub type REG_TM_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_20M_CLK_EN` reader - Reserved"]
pub type REG_REF_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_20M_CLK_EN` writer - Reserved"]
pub type REG_REF_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_20M_CLK_EN` reader - Reserved"]
pub type REG_TM_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_20M_CLK_EN` writer - Reserved"]
pub type REG_TM_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_160m_clk_en(&self) -> REG_REF_160M_CLK_EN_R {
        REG_REF_160M_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_160m_clk_en(&self) -> REG_TM_160M_CLK_EN_R {
        REG_TM_160M_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_120m_clk_en(&self) -> REG_REF_120M_CLK_EN_R {
        REG_REF_120M_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_120m_clk_en(&self) -> REG_TM_120M_CLK_EN_R {
        REG_TM_120M_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_80m_clk_en(&self) -> REG_REF_80M_CLK_EN_R {
        REG_REF_80M_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_80m_clk_en(&self) -> REG_TM_80M_CLK_EN_R {
        REG_TM_80M_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_60m_clk_en(&self) -> REG_TM_60M_CLK_EN_R {
        REG_TM_60M_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_48m_clk_en(&self) -> REG_TM_48M_CLK_EN_R {
        REG_TM_48M_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_20m_clk_en(&self) -> REG_REF_20M_CLK_EN_R {
        REG_REF_20M_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_20m_clk_en(&self) -> REG_TM_20M_CLK_EN_R {
        REG_TM_20M_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CLK_CTRL2")
            .field(
                "reg_ref_160m_clk_en",
                &format_args!("{}", self.reg_ref_160m_clk_en().bit()),
            )
            .field(
                "reg_tm_160m_clk_en",
                &format_args!("{}", self.reg_tm_160m_clk_en().bit()),
            )
            .field(
                "reg_ref_120m_clk_en",
                &format_args!("{}", self.reg_ref_120m_clk_en().bit()),
            )
            .field(
                "reg_tm_120m_clk_en",
                &format_args!("{}", self.reg_tm_120m_clk_en().bit()),
            )
            .field(
                "reg_ref_80m_clk_en",
                &format_args!("{}", self.reg_ref_80m_clk_en().bit()),
            )
            .field(
                "reg_tm_80m_clk_en",
                &format_args!("{}", self.reg_tm_80m_clk_en().bit()),
            )
            .field(
                "reg_tm_60m_clk_en",
                &format_args!("{}", self.reg_tm_60m_clk_en().bit()),
            )
            .field(
                "reg_tm_48m_clk_en",
                &format_args!("{}", self.reg_tm_48m_clk_en().bit()),
            )
            .field(
                "reg_ref_20m_clk_en",
                &format_args!("{}", self.reg_ref_20m_clk_en().bit()),
            )
            .field(
                "reg_tm_20m_clk_en",
                &format_args!("{}", self.reg_tm_20m_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CLK_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_160m_clk_en(&mut self) -> REG_REF_160M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_REF_160M_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_160m_clk_en(&mut self) -> REG_TM_160M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_TM_160M_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_120m_clk_en(&mut self) -> REG_REF_120M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_REF_120M_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_120m_clk_en(&mut self) -> REG_TM_120M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_TM_120M_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_80m_clk_en(&mut self) -> REG_REF_80M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_REF_80M_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_80m_clk_en(&mut self) -> REG_TM_80M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_TM_80M_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_60m_clk_en(&mut self) -> REG_TM_60M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_TM_60M_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_48m_clk_en(&mut self) -> REG_TM_48M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_TM_48M_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_20m_clk_en(&mut self) -> REG_REF_20M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_REF_20M_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_20m_clk_en(&mut self) -> REG_TM_20M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REG_TM_20M_CLK_EN_W::new(self, 9)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ref_clk_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_clk_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CLK_CTRL2_SPEC;
impl crate::RegisterSpec for REF_CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clk_ctrl2::R`](R) reader structure"]
impl crate::Readable for REF_CLK_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_clk_ctrl2::W`](W) writer structure"]
impl crate::Writable for REF_CLK_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CLK_CTRL2 to value 0x0115"]
impl crate::Resettable for REF_CLK_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0115;
}
