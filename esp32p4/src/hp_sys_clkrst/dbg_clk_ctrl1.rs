#[doc = "Register `DBG_CLK_CTRL1` reader"]
pub type R = crate::R<DBG_CLK_CTRL1_SPEC>;
#[doc = "Register `DBG_CLK_CTRL1` writer"]
pub type W = crate::W<DBG_CLK_CTRL1_SPEC>;
#[doc = "Field `DBG_CH1_DIV_NUM` reader - Reserved"]
pub type DBG_CH1_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `DBG_CH1_DIV_NUM` writer - Reserved"]
pub type DBG_CH1_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH2_DIV_NUM` reader - Reserved"]
pub type DBG_CH2_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `DBG_CH2_DIV_NUM` writer - Reserved"]
pub type DBG_CH2_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH0_EN` reader - Reserved"]
pub type DBG_CH0_EN_R = crate::BitReader;
#[doc = "Field `DBG_CH0_EN` writer - Reserved"]
pub type DBG_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CH1_EN` reader - Reserved"]
pub type DBG_CH1_EN_R = crate::BitReader;
#[doc = "Field `DBG_CH1_EN` writer - Reserved"]
pub type DBG_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CH2_EN` reader - Reserved"]
pub type DBG_CH2_EN_R = crate::BitReader;
#[doc = "Field `DBG_CH2_EN` writer - Reserved"]
pub type DBG_CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_div_num(&self) -> DBG_CH1_DIV_NUM_R {
        DBG_CH1_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_div_num(&self) -> DBG_CH2_DIV_NUM_R {
        DBG_CH2_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_en(&self) -> DBG_CH0_EN_R {
        DBG_CH0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_en(&self) -> DBG_CH1_EN_R {
        DBG_CH1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_en(&self) -> DBG_CH2_EN_R {
        DBG_CH2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_CLK_CTRL1")
            .field(
                "dbg_ch1_div_num",
                &format_args!("{}", self.dbg_ch1_div_num().bits()),
            )
            .field(
                "dbg_ch2_div_num",
                &format_args!("{}", self.dbg_ch2_div_num().bits()),
            )
            .field("dbg_ch0_en", &format_args!("{}", self.dbg_ch0_en().bit()))
            .field("dbg_ch1_en", &format_args!("{}", self.dbg_ch1_en().bit()))
            .field("dbg_ch2_en", &format_args!("{}", self.dbg_ch2_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBG_CLK_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_ch1_div_num(&mut self) -> DBG_CH1_DIV_NUM_W<DBG_CLK_CTRL1_SPEC> {
        DBG_CH1_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_ch2_div_num(&mut self) -> DBG_CH2_DIV_NUM_W<DBG_CLK_CTRL1_SPEC> {
        DBG_CH2_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_ch0_en(&mut self) -> DBG_CH0_EN_W<DBG_CLK_CTRL1_SPEC> {
        DBG_CH0_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_ch1_en(&mut self) -> DBG_CH1_EN_W<DBG_CLK_CTRL1_SPEC> {
        DBG_CH1_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_ch2_en(&mut self) -> DBG_CH2_EN_W<DBG_CLK_CTRL1_SPEC> {
        DBG_CH2_EN_W::new(self, 18)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_clk_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_clk_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_CLK_CTRL1_SPEC;
impl crate::RegisterSpec for DBG_CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_clk_ctrl1::R`](R) reader structure"]
impl crate::Readable for DBG_CLK_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_clk_ctrl1::W`](W) writer structure"]
impl crate::Writable for DBG_CLK_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_CLK_CTRL1 to value 0x0303"]
impl crate::Resettable for DBG_CLK_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0303;
}
