#[doc = "Register `REF_CLK_CTRL1` reader"]
pub type R = crate::R<REF_CLK_CTRL1_SPEC>;
#[doc = "Register `REF_CLK_CTRL1` writer"]
pub type W = crate::W<REF_CLK_CTRL1_SPEC>;
#[doc = "Field `REF_120M_CLK_DIV_NUM` reader - Reserved"]
pub type REF_120M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REF_120M_CLK_DIV_NUM` writer - Reserved"]
pub type REF_120M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REF_80M_CLK_DIV_NUM` reader - Reserved"]
pub type REF_80M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REF_80M_CLK_DIV_NUM` writer - Reserved"]
pub type REF_80M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REF_20M_CLK_DIV_NUM` reader - Reserved"]
pub type REF_20M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REF_20M_CLK_DIV_NUM` writer - Reserved"]
pub type REF_20M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TM_400M_CLK_EN` reader - Reserved"]
pub type TM_400M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_400M_CLK_EN` writer - Reserved"]
pub type TM_400M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_200M_CLK_EN` reader - Reserved"]
pub type TM_200M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_200M_CLK_EN` writer - Reserved"]
pub type TM_200M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_100M_CLK_EN` reader - Reserved"]
pub type TM_100M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_100M_CLK_EN` writer - Reserved"]
pub type TM_100M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_50M_CLK_EN` reader - Reserved"]
pub type REF_50M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_50M_CLK_EN` writer - Reserved"]
pub type REF_50M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_25M_CLK_EN` reader - Reserved"]
pub type REF_25M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_25M_CLK_EN` writer - Reserved"]
pub type REF_25M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_480M_CLK_EN` reader - Reserved"]
pub type TM_480M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_480M_CLK_EN` writer - Reserved"]
pub type TM_480M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_240M_CLK_EN` reader - Reserved"]
pub type REF_240M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_240M_CLK_EN` writer - Reserved"]
pub type REF_240M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_240M_CLK_EN` reader - Reserved"]
pub type TM_240M_CLK_EN_R = crate::BitReader;
#[doc = "Field `TM_240M_CLK_EN` writer - Reserved"]
pub type TM_240M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn ref_120m_clk_div_num(&self) -> REF_120M_CLK_DIV_NUM_R {
        REF_120M_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn ref_80m_clk_div_num(&self) -> REF_80M_CLK_DIV_NUM_R {
        REF_80M_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn ref_20m_clk_div_num(&self) -> REF_20M_CLK_DIV_NUM_R {
        REF_20M_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn tm_400m_clk_en(&self) -> TM_400M_CLK_EN_R {
        TM_400M_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn tm_200m_clk_en(&self) -> TM_200M_CLK_EN_R {
        TM_200M_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn tm_100m_clk_en(&self) -> TM_100M_CLK_EN_R {
        TM_100M_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn ref_50m_clk_en(&self) -> REF_50M_CLK_EN_R {
        REF_50M_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn ref_25m_clk_en(&self) -> REF_25M_CLK_EN_R {
        REF_25M_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn tm_480m_clk_en(&self) -> TM_480M_CLK_EN_R {
        TM_480M_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn ref_240m_clk_en(&self) -> REF_240M_CLK_EN_R {
        REF_240M_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn tm_240m_clk_en(&self) -> TM_240M_CLK_EN_R {
        TM_240M_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CLK_CTRL1")
            .field("ref_120m_clk_div_num", &self.ref_120m_clk_div_num())
            .field("ref_80m_clk_div_num", &self.ref_80m_clk_div_num())
            .field("ref_20m_clk_div_num", &self.ref_20m_clk_div_num())
            .field("tm_400m_clk_en", &self.tm_400m_clk_en())
            .field("tm_200m_clk_en", &self.tm_200m_clk_en())
            .field("tm_100m_clk_en", &self.tm_100m_clk_en())
            .field("ref_50m_clk_en", &self.ref_50m_clk_en())
            .field("ref_25m_clk_en", &self.ref_25m_clk_en())
            .field("tm_480m_clk_en", &self.tm_480m_clk_en())
            .field("ref_240m_clk_en", &self.ref_240m_clk_en())
            .field("tm_240m_clk_en", &self.tm_240m_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn ref_120m_clk_div_num(&mut self) -> REF_120M_CLK_DIV_NUM_W<REF_CLK_CTRL1_SPEC> {
        REF_120M_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn ref_80m_clk_div_num(&mut self) -> REF_80M_CLK_DIV_NUM_W<REF_CLK_CTRL1_SPEC> {
        REF_80M_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn ref_20m_clk_div_num(&mut self) -> REF_20M_CLK_DIV_NUM_W<REF_CLK_CTRL1_SPEC> {
        REF_20M_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn tm_400m_clk_en(&mut self) -> TM_400M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        TM_400M_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn tm_200m_clk_en(&mut self) -> TM_200M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        TM_200M_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn tm_100m_clk_en(&mut self) -> TM_100M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        TM_100M_CLK_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn ref_50m_clk_en(&mut self) -> REF_50M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REF_50M_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn ref_25m_clk_en(&mut self) -> REF_25M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REF_25M_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn tm_480m_clk_en(&mut self) -> TM_480M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        TM_480M_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn ref_240m_clk_en(&mut self) -> REF_240M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        REF_240M_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn tm_240m_clk_en(&mut self) -> TM_240M_CLK_EN_W<REF_CLK_CTRL1_SPEC> {
        TM_240M_CLK_EN_W::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clk_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clk_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CLK_CTRL1_SPEC;
impl crate::RegisterSpec for REF_CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clk_ctrl1::R`](R) reader structure"]
impl crate::Readable for REF_CLK_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_clk_ctrl1::W`](W) writer structure"]
impl crate::Writable for REF_CLK_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REF_CLK_CTRL1 to value 0x5817_0503"]
impl crate::Resettable for REF_CLK_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x5817_0503;
}
