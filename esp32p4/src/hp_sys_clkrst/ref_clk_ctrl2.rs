#[doc = "Register `REF_CLK_CTRL2` reader"]
pub type R = crate::R<REF_CLK_CTRL2_SPEC>;
#[doc = "Register `REF_CLK_CTRL2` writer"]
pub type W = crate::W<REF_CLK_CTRL2_SPEC>;
#[doc = "Field `REF_160M_CLK_EN` reader - Reserved"]
pub type REF_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_160M_CLK_EN` writer - Reserved"]
pub type REF_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_160M_CLK_EN` reader - Reserved"]
pub type TM_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_160M_CLK_EN` writer - Reserved"]
pub type TM_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_120M_CLK_EN` reader - Reserved"]
pub type REF_120M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_120M_CLK_EN` writer - Reserved"]
pub type REF_120M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_120M_CLK_EN` reader - Reserved"]
pub type TM_120M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_120M_CLK_EN` writer - Reserved"]
pub type TM_120M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_80M_CLK_EN` reader - Reserved"]
pub type REF_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_80M_CLK_EN` writer - Reserved"]
pub type REF_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_80M_CLK_EN` reader - Reserved"]
pub type TM_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_80M_CLK_EN` writer - Reserved"]
pub type TM_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_60M_CLK_EN` reader - Reserved"]
pub type TM_60M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_60M_CLK_EN` writer - Reserved"]
pub type TM_60M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_48M_CLK_EN` reader - Reserved"]
pub type TM_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_48M_CLK_EN` writer - Reserved"]
pub type TM_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_20M_CLK_EN` reader - Reserved"]
pub type REF_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_20M_CLK_EN` writer - Reserved"]
pub type REF_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_20M_CLK_EN` reader - Reserved"]
pub type TM_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_20M_CLK_EN` writer - Reserved"]
pub type TM_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn ref_160m_clk_en(&self) -> REF_160M_CLK_EN_R {
        REF_160M_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn tm_160m_clk_en(&self) -> TM_160M_CLK_EN_R {
        TM_160M_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ref_120m_clk_en(&self) -> REF_120M_CLK_EN_R {
        REF_120M_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn tm_120m_clk_en(&self) -> TM_120M_CLK_EN_R {
        TM_120M_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn ref_80m_clk_en(&self) -> REF_80M_CLK_EN_R {
        REF_80M_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn tm_80m_clk_en(&self) -> TM_80M_CLK_EN_R {
        TM_80M_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn tm_60m_clk_en(&self) -> TM_60M_CLK_EN_R {
        TM_60M_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn tm_48m_clk_en(&self) -> TM_48M_CLK_EN_R {
        TM_48M_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn ref_20m_clk_en(&self) -> REF_20M_CLK_EN_R {
        REF_20M_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn tm_20m_clk_en(&self) -> TM_20M_CLK_EN_R {
        TM_20M_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CLK_CTRL2")
            .field("ref_160m_clk_en", &self.ref_160m_clk_en())
            .field("tm_160m_clk_en", &self.tm_160m_clk_en())
            .field("ref_120m_clk_en", &self.ref_120m_clk_en())
            .field("tm_120m_clk_en", &self.tm_120m_clk_en())
            .field("ref_80m_clk_en", &self.ref_80m_clk_en())
            .field("tm_80m_clk_en", &self.tm_80m_clk_en())
            .field("tm_60m_clk_en", &self.tm_60m_clk_en())
            .field("tm_48m_clk_en", &self.tm_48m_clk_en())
            .field("ref_20m_clk_en", &self.ref_20m_clk_en())
            .field("tm_20m_clk_en", &self.tm_20m_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ref_160m_clk_en(&mut self) -> REF_160M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REF_160M_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tm_160m_clk_en(&mut self) -> TM_160M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        TM_160M_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ref_120m_clk_en(&mut self) -> REF_120M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REF_120M_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tm_120m_clk_en(&mut self) -> TM_120M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        TM_120M_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ref_80m_clk_en(&mut self) -> REF_80M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REF_80M_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tm_80m_clk_en(&mut self) -> TM_80M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        TM_80M_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tm_60m_clk_en(&mut self) -> TM_60M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        TM_60M_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tm_48m_clk_en(&mut self) -> TM_48M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        TM_48M_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ref_20m_clk_en(&mut self) -> REF_20M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        REF_20M_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tm_20m_clk_en(&mut self) -> TM_20M_CLK_EN_W<REF_CLK_CTRL2_SPEC> {
        TM_20M_CLK_EN_W::new(self, 9)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clk_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clk_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CLK_CTRL2_SPEC;
impl crate::RegisterSpec for REF_CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clk_ctrl2::R`](R) reader structure"]
impl crate::Readable for REF_CLK_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_clk_ctrl2::W`](W) writer structure"]
impl crate::Writable for REF_CLK_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REF_CLK_CTRL2 to value 0x0115"]
impl crate::Resettable for REF_CLK_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0115;
}
