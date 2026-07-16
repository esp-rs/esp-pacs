#[doc = "Register `TM_CLK_CTRL0` reader"]
pub type R = crate::R<TM_CLK_CTRL0_SPEC>;
#[doc = "Register `TM_CLK_CTRL0` writer"]
pub type W = crate::W<TM_CLK_CTRL0_SPEC>;
#[doc = "Field `TM_480M_CLK_EN` reader - need_des"]
pub type TM_480M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_480M_CLK_EN` writer - need_des"]
pub type TM_480M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_240M_CLK_EN` reader - need_des"]
pub type TM_240M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_240M_CLK_EN` writer - need_des"]
pub type TM_240M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_160M_CLK_EN` reader - need_des"]
pub type TM_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_160M_CLK_EN` writer - need_des"]
pub type TM_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_120M_CLK_EN` reader - need_des"]
pub type TM_120M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_120M_CLK_EN` writer - need_des"]
pub type TM_120M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_80M_CLK_EN` reader - need_des"]
pub type TM_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_80M_CLK_EN` writer - need_des"]
pub type TM_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_60M_CLK_EN` reader - need_des"]
pub type TM_60M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_60M_CLK_EN` writer - need_des"]
pub type TM_60M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_48M_CLK_EN` reader - need_des"]
pub type TM_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_48M_CLK_EN` writer - need_des"]
pub type TM_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_20M_CLK_EN` reader - need_des"]
pub type TM_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_20M_CLK_EN` writer - need_des"]
pub type TM_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_250M_CLK_EN` reader - need_des"]
pub type TM_250M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_250M_CLK_EN` writer - need_des"]
pub type TM_250M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn tm_480m_clk_en(&self) -> TM_480M_CLK_EN_R {
        TM_480M_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn tm_240m_clk_en(&self) -> TM_240M_CLK_EN_R {
        TM_240M_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn tm_160m_clk_en(&self) -> TM_160M_CLK_EN_R {
        TM_160M_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn tm_120m_clk_en(&self) -> TM_120M_CLK_EN_R {
        TM_120M_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn tm_80m_clk_en(&self) -> TM_80M_CLK_EN_R {
        TM_80M_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn tm_60m_clk_en(&self) -> TM_60M_CLK_EN_R {
        TM_60M_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn tm_48m_clk_en(&self) -> TM_48M_CLK_EN_R {
        TM_48M_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn tm_20m_clk_en(&self) -> TM_20M_CLK_EN_R {
        TM_20M_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn tm_250m_clk_en(&self) -> TM_250M_CLK_EN_R {
        TM_250M_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TM_CLK_CTRL0")
            .field("tm_480m_clk_en", &self.tm_480m_clk_en())
            .field("tm_240m_clk_en", &self.tm_240m_clk_en())
            .field("tm_160m_clk_en", &self.tm_160m_clk_en())
            .field("tm_120m_clk_en", &self.tm_120m_clk_en())
            .field("tm_80m_clk_en", &self.tm_80m_clk_en())
            .field("tm_60m_clk_en", &self.tm_60m_clk_en())
            .field("tm_48m_clk_en", &self.tm_48m_clk_en())
            .field("tm_20m_clk_en", &self.tm_20m_clk_en())
            .field("tm_250m_clk_en", &self.tm_250m_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn tm_480m_clk_en(&mut self) -> TM_480M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_480M_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn tm_240m_clk_en(&mut self) -> TM_240M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_240M_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn tm_160m_clk_en(&mut self) -> TM_160M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_160M_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn tm_120m_clk_en(&mut self) -> TM_120M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_120M_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn tm_80m_clk_en(&mut self) -> TM_80M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_80M_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn tm_60m_clk_en(&mut self) -> TM_60M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_60M_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn tm_48m_clk_en(&mut self) -> TM_48M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_48M_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn tm_20m_clk_en(&mut self) -> TM_20M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_20M_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn tm_250m_clk_en(&mut self) -> TM_250M_CLK_EN_W<'_, TM_CLK_CTRL0_SPEC> {
        TM_250M_CLK_EN_W::new(self, 8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_clk_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_clk_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TM_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for TM_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for TM_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tm_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for TM_CLK_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_CLK_CTRL0 to value 0"]
impl crate::Resettable for TM_CLK_CTRL0_SPEC {}
