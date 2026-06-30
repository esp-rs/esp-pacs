#[doc = "Register `SDIO_PAD_BIST_CTRL` reader"]
pub type R = crate::R<SDIO_PAD_BIST_CTRL_SPEC>;
#[doc = "Register `SDIO_PAD_BIST_CTRL` writer"]
pub type W = crate::W<SDIO_PAD_BIST_CTRL_SPEC>;
#[doc = "Field `SDIO_PAD_BIST_0_SEL` reader - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_0_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_0_SEL` writer - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_0_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_1_SEL` reader - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_1_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_1_SEL` writer - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_2_SEL` reader - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_2_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_2_SEL` writer - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_2_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_3_SEL` reader - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_3_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_3_SEL` writer - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_3_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_4_SEL` reader - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_4_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_4_SEL` writer - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_4_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_5_SEL` reader - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_5_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_5_SEL` writer - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
pub type SDIO_PAD_BIST_5_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_0_sel(&self) -> SDIO_PAD_BIST_0_SEL_R {
        SDIO_PAD_BIST_0_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_1_sel(&self) -> SDIO_PAD_BIST_1_SEL_R {
        SDIO_PAD_BIST_1_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_2_sel(&self) -> SDIO_PAD_BIST_2_SEL_R {
        SDIO_PAD_BIST_2_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_3_sel(&self) -> SDIO_PAD_BIST_3_SEL_R {
        SDIO_PAD_BIST_3_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_4_sel(&self) -> SDIO_PAD_BIST_4_SEL_R {
        SDIO_PAD_BIST_4_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_5_sel(&self) -> SDIO_PAD_BIST_5_SEL_R {
        SDIO_PAD_BIST_5_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_PAD_BIST_CTRL")
            .field("sdio_pad_bist_0_sel", &self.sdio_pad_bist_0_sel())
            .field("sdio_pad_bist_1_sel", &self.sdio_pad_bist_1_sel())
            .field("sdio_pad_bist_2_sel", &self.sdio_pad_bist_2_sel())
            .field("sdio_pad_bist_3_sel", &self.sdio_pad_bist_3_sel())
            .field("sdio_pad_bist_4_sel", &self.sdio_pad_bist_4_sel())
            .field("sdio_pad_bist_5_sel", &self.sdio_pad_bist_5_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_0_sel(&mut self) -> SDIO_PAD_BIST_0_SEL_W<'_, SDIO_PAD_BIST_CTRL_SPEC> {
        SDIO_PAD_BIST_0_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_1_sel(&mut self) -> SDIO_PAD_BIST_1_SEL_W<'_, SDIO_PAD_BIST_CTRL_SPEC> {
        SDIO_PAD_BIST_1_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_2_sel(&mut self) -> SDIO_PAD_BIST_2_SEL_W<'_, SDIO_PAD_BIST_CTRL_SPEC> {
        SDIO_PAD_BIST_2_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_3_sel(&mut self) -> SDIO_PAD_BIST_3_SEL_W<'_, SDIO_PAD_BIST_CTRL_SPEC> {
        SDIO_PAD_BIST_3_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_4_sel(&mut self) -> SDIO_PAD_BIST_4_SEL_W<'_, SDIO_PAD_BIST_CTRL_SPEC> {
        SDIO_PAD_BIST_4_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to select sdio pad bist signal out, default 0 is normal pad mode"]
    #[inline(always)]
    pub fn sdio_pad_bist_5_sel(&mut self) -> SDIO_PAD_BIST_5_SEL_W<'_, SDIO_PAD_BIST_CTRL_SPEC> {
        SDIO_PAD_BIST_5_SEL_W::new(self, 5)
    }
}
#[doc = "gmac0 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_PAD_BIST_CTRL_SPEC;
impl crate::RegisterSpec for SDIO_PAD_BIST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_pad_bist_ctrl::R`](R) reader structure"]
impl crate::Readable for SDIO_PAD_BIST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_pad_bist_ctrl::W`](W) writer structure"]
impl crate::Writable for SDIO_PAD_BIST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_PAD_BIST_CTRL to value 0"]
impl crate::Resettable for SDIO_PAD_BIST_CTRL_SPEC {}
