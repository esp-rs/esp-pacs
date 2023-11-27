#[doc = "Register `REF_CLK_CTRL1` reader"]
pub type R = crate::R<REF_CLK_CTRL1_SPEC>;
#[doc = "Register `REF_CLK_CTRL1` writer"]
pub type W = crate::W<REF_CLK_CTRL1_SPEC>;
#[doc = "Field `REG_REF_120M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_120M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_120M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_120M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_REF_80M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_80M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_80M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_80M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_REF_20M_CLK_DIV_NUM` reader - Reserved"]
pub type REG_REF_20M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_REF_20M_CLK_DIV_NUM` writer - Reserved"]
pub type REG_REF_20M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TM_400M_CLK_EN` reader - Reserved"]
pub type REG_TM_400M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_400M_CLK_EN` writer - Reserved"]
pub type REG_TM_400M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_200M_CLK_EN` reader - Reserved"]
pub type REG_TM_200M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_200M_CLK_EN` writer - Reserved"]
pub type REG_TM_200M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_100M_CLK_EN` reader - Reserved"]
pub type REG_TM_100M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_100M_CLK_EN` writer - Reserved"]
pub type REG_TM_100M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_50M_CLK_EN` reader - Reserved"]
pub type REG_REF_50M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_50M_CLK_EN` writer - Reserved"]
pub type REG_REF_50M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_25M_CLK_EN` reader - Reserved"]
pub type REG_REF_25M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_25M_CLK_EN` writer - Reserved"]
pub type REG_REF_25M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_480M_CLK_EN` reader - Reserved"]
pub type REG_TM_480M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_480M_CLK_EN` writer - Reserved"]
pub type REG_TM_480M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_240M_CLK_EN` reader - Reserved"]
pub type REG_REF_240M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REF_240M_CLK_EN` writer - Reserved"]
pub type REG_REF_240M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TM_240M_CLK_EN` reader - Reserved"]
pub type REG_TM_240M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TM_240M_CLK_EN` writer - Reserved"]
pub type REG_TM_240M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_120m_clk_div_num(&self) -> REG_REF_120M_CLK_DIV_NUM_R {
        REG_REF_120M_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_80m_clk_div_num(&self) -> REG_REF_80M_CLK_DIV_NUM_R {
        REG_REF_80M_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_20m_clk_div_num(&self) -> REG_REF_20M_CLK_DIV_NUM_R {
        REG_REF_20M_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_400m_clk_en(&self) -> REG_TM_400M_CLK_EN_R {
        REG_TM_400M_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_200m_clk_en(&self) -> REG_TM_200M_CLK_EN_R {
        REG_TM_200M_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_100m_clk_en(&self) -> REG_TM_100M_CLK_EN_R {
        REG_TM_100M_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_50m_clk_en(&self) -> REG_REF_50M_CLK_EN_R {
        REG_REF_50M_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_25m_clk_en(&self) -> REG_REF_25M_CLK_EN_R {
        REG_REF_25M_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_480m_clk_en(&self) -> REG_TM_480M_CLK_EN_R {
        REG_TM_480M_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reg_ref_240m_clk_en(&self) -> REG_REF_240M_CLK_EN_R {
        REG_REF_240M_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn reg_tm_240m_clk_en(&self) -> REG_TM_240M_CLK_EN_R {
        REG_TM_240M_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CLK_CTRL1")
            .field(
                "reg_ref_120m_clk_div_num",
                &format_args!("{}", self.reg_ref_120m_clk_div_num().bits()),
            )
            .field(
                "reg_ref_80m_clk_div_num",
                &format_args!("{}", self.reg_ref_80m_clk_div_num().bits()),
            )
            .field(
                "reg_ref_20m_clk_div_num",
                &format_args!("{}", self.reg_ref_20m_clk_div_num().bits()),
            )
            .field(
                "reg_tm_400m_clk_en",
                &format_args!("{}", self.reg_tm_400m_clk_en().bit()),
            )
            .field(
                "reg_tm_200m_clk_en",
                &format_args!("{}", self.reg_tm_200m_clk_en().bit()),
            )
            .field(
                "reg_tm_100m_clk_en",
                &format_args!("{}", self.reg_tm_100m_clk_en().bit()),
            )
            .field(
                "reg_ref_50m_clk_en",
                &format_args!("{}", self.reg_ref_50m_clk_en().bit()),
            )
            .field(
                "reg_ref_25m_clk_en",
                &format_args!("{}", self.reg_ref_25m_clk_en().bit()),
            )
            .field(
                "reg_tm_480m_clk_en",
                &format_args!("{}", self.reg_tm_480m_clk_en().bit()),
            )
            .field(
                "reg_ref_240m_clk_en",
                &format_args!("{}", self.reg_ref_240m_clk_en().bit()),
            )
            .field(
                "reg_tm_240m_clk_en",
                &format_args!("{}", self.reg_tm_240m_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CLK_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_120m_clk_div_num(&mut self) -> REG_REF_120M_CLK_DIV_NUM_W<REF_CLK_CTRL1_SPEC> {
        REG_REF_120M_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_80m_clk_div_num(&mut self) -> REG_REF_80M_CLK_DIV_NUM_W<REF_CLK_CTRL1_SPEC> {
        REG_REF_80M_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_20m_clk_div_num(&mut self) -> REG_REF_20M_CLK_DIV_NUM_W<REF_CLK_CTRL1_SPEC> {
        REG_REF_20M_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_400m_clk_en(&mut self) -> REG_TM_400M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_TM_400M_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_200m_clk_en(&mut self) -> REG_TM_200M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_TM_200M_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_100m_clk_en(&mut self) -> REG_TM_100M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_TM_100M_CLK_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_50m_clk_en(&mut self) -> REG_REF_50M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_REF_50M_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_25m_clk_en(&mut self) -> REG_REF_25M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_REF_25M_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_480m_clk_en(&mut self) -> REG_TM_480M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_TM_480M_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_240m_clk_en(&mut self) -> REG_REF_240M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_REF_240M_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tm_240m_clk_en(&mut self) -> REG_TM_240M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REG_TM_240M_CLK_EN_W::new(self, 31)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ref_clk_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_clk_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CLK_CTRL1_SPEC;
impl crate::RegisterSpec for REF_CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clk_ctrl1::R`](R) reader structure"]
impl crate::Readable for REF_CLK_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_clk_ctrl1::W`](W) writer structure"]
impl crate::Writable for REF_CLK_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CLK_CTRL1 to value 0x5817_0503"]
impl crate::Resettable for REF_CLK_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x5817_0503;
}
