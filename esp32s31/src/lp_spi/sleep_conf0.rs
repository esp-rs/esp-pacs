#[doc = "Register `SLEEP_CONF0` reader"]
pub type R = crate::R<SLEEP_CONF0_SPEC>;
#[doc = "Register `SLEEP_CONF0` writer"]
pub type W = crate::W<SLEEP_CONF0_SPEC>;
#[doc = "Field `SLV_WK_CHAR0` reader - "]
pub type SLV_WK_CHAR0_R = crate::FieldReader;
#[doc = "Field `SLV_WK_CHAR0` writer - "]
pub type SLV_WK_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLV_WK_CHAR_NUM` reader - "]
pub type SLV_WK_CHAR_NUM_R = crate::FieldReader;
#[doc = "Field `SLV_WK_CHAR_NUM` writer - "]
pub type SLV_WK_CHAR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLV_WK_CHAR_MASK` reader - "]
pub type SLV_WK_CHAR_MASK_R = crate::FieldReader;
#[doc = "Field `SLV_WK_CHAR_MASK` writer - "]
pub type SLV_WK_CHAR_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SLV_WK_MODE_SEL` reader - "]
pub type SLV_WK_MODE_SEL_R = crate::BitReader;
#[doc = "Field `SLV_WK_MODE_SEL` writer - "]
pub type SLV_WK_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_EN` reader - "]
pub type SLEEP_EN_R = crate::BitReader;
#[doc = "Field `SLEEP_EN` writer - "]
pub type SLEEP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_DIS_RXFIFO_WR_EN` reader - "]
pub type SLEEP_DIS_RXFIFO_WR_EN_R = crate::BitReader;
#[doc = "Field `SLEEP_DIS_RXFIFO_WR_EN` writer - "]
pub type SLEEP_DIS_RXFIFO_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_WK_DATA_SEL` reader - "]
pub type SLEEP_WK_DATA_SEL_R = crate::BitReader;
#[doc = "Field `SLEEP_WK_DATA_SEL` writer - "]
pub type SLEEP_WK_DATA_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slv_wk_char0(&self) -> SLV_WK_CHAR0_R {
        SLV_WK_CHAR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn slv_wk_char_num(&self) -> SLV_WK_CHAR_NUM_R {
        SLV_WK_CHAR_NUM_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn slv_wk_char_mask(&self) -> SLV_WK_CHAR_MASK_R {
        SLV_WK_CHAR_MASK_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_wk_mode_sel(&self) -> SLV_WK_MODE_SEL_R {
        SLV_WK_MODE_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sleep_dis_rxfifo_wr_en(&self) -> SLEEP_DIS_RXFIFO_WR_EN_R {
        SLEEP_DIS_RXFIFO_WR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sleep_wk_data_sel(&self) -> SLEEP_WK_DATA_SEL_R {
        SLEEP_WK_DATA_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF0")
            .field("slv_wk_char0", &self.slv_wk_char0())
            .field("slv_wk_char_num", &self.slv_wk_char_num())
            .field("slv_wk_char_mask", &self.slv_wk_char_mask())
            .field("slv_wk_mode_sel", &self.slv_wk_mode_sel())
            .field("sleep_en", &self.sleep_en())
            .field("sleep_dis_rxfifo_wr_en", &self.sleep_dis_rxfifo_wr_en())
            .field("sleep_wk_data_sel", &self.sleep_wk_data_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slv_wk_char0(&mut self) -> SLV_WK_CHAR0_W<'_, SLEEP_CONF0_SPEC> {
        SLV_WK_CHAR0_W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn slv_wk_char_num(&mut self) -> SLV_WK_CHAR_NUM_W<'_, SLEEP_CONF0_SPEC> {
        SLV_WK_CHAR_NUM_W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn slv_wk_char_mask(&mut self) -> SLV_WK_CHAR_MASK_W<'_, SLEEP_CONF0_SPEC> {
        SLV_WK_CHAR_MASK_W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_wk_mode_sel(&mut self) -> SLV_WK_MODE_SEL_W<'_, SLEEP_CONF0_SPEC> {
        SLV_WK_MODE_SEL_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<'_, SLEEP_CONF0_SPEC> {
        SLEEP_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sleep_dis_rxfifo_wr_en(&mut self) -> SLEEP_DIS_RXFIFO_WR_EN_W<'_, SLEEP_CONF0_SPEC> {
        SLEEP_DIS_RXFIFO_WR_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sleep_wk_data_sel(&mut self) -> SLEEP_WK_DATA_SEL_W<'_, SLEEP_CONF0_SPEC> {
        SLEEP_WK_DATA_SEL_W::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF0_SPEC;
impl crate::RegisterSpec for SLEEP_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf0::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf0::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF0 to value 0x0a"]
impl crate::Resettable for SLEEP_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
