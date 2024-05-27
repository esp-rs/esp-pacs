///Register `BLC_CTRL1` reader
pub type R = crate::R<BLC_CTRL1_SPEC>;
///Register `BLC_CTRL1` writer
pub type W = crate::W<BLC_CTRL1_SPEC>;
///Field `BLC_WINDOW_TOP` reader - this field configures blc average calculation window top
pub type BLC_WINDOW_TOP_R = crate::FieldReader<u16>;
///Field `BLC_WINDOW_TOP` writer - this field configures blc average calculation window top
pub type BLC_WINDOW_TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `BLC_WINDOW_LEFT` reader - this field configures blc average calculation window left
pub type BLC_WINDOW_LEFT_R = crate::FieldReader<u16>;
///Field `BLC_WINDOW_LEFT` writer - this field configures blc average calculation window left
pub type BLC_WINDOW_LEFT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `BLC_WINDOW_VNUM` reader - this field configures blc average calculation window vnum
pub type BLC_WINDOW_VNUM_R = crate::FieldReader;
///Field `BLC_WINDOW_VNUM` writer - this field configures blc average calculation window vnum
pub type BLC_WINDOW_VNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BLC_WINDOW_HNUM` reader - this field configures blc average calculation window hnum
pub type BLC_WINDOW_HNUM_R = crate::FieldReader;
///Field `BLC_WINDOW_HNUM` writer - this field configures blc average calculation window hnum
pub type BLC_WINDOW_HNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BLC_FILTER_EN` reader - this bit configures enable blc average input filter. 0: disable, 1: enable
pub type BLC_FILTER_EN_R = crate::BitReader;
///Field `BLC_FILTER_EN` writer - this bit configures enable blc average input filter. 0: disable, 1: enable
pub type BLC_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - this field configures blc average calculation window top
    #[inline(always)]
    pub fn blc_window_top(&self) -> BLC_WINDOW_TOP_R {
        BLC_WINDOW_TOP_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:21 - this field configures blc average calculation window left
    #[inline(always)]
    pub fn blc_window_left(&self) -> BLC_WINDOW_LEFT_R {
        BLC_WINDOW_LEFT_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    ///Bits 22:25 - this field configures blc average calculation window vnum
    #[inline(always)]
    pub fn blc_window_vnum(&self) -> BLC_WINDOW_VNUM_R {
        BLC_WINDOW_VNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bits 26:29 - this field configures blc average calculation window hnum
    #[inline(always)]
    pub fn blc_window_hnum(&self) -> BLC_WINDOW_HNUM_R {
        BLC_WINDOW_HNUM_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bit 30 - this bit configures enable blc average input filter. 0: disable, 1: enable
    #[inline(always)]
    pub fn blc_filter_en(&self) -> BLC_FILTER_EN_R {
        BLC_FILTER_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLC_CTRL1")
            .field("blc_window_top", &self.blc_window_top())
            .field("blc_window_left", &self.blc_window_left())
            .field("blc_window_vnum", &self.blc_window_vnum())
            .field("blc_window_hnum", &self.blc_window_hnum())
            .field("blc_filter_en", &self.blc_filter_en())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - this field configures blc average calculation window top
    #[inline(always)]
    #[must_use]
    pub fn blc_window_top(&mut self) -> BLC_WINDOW_TOP_W<BLC_CTRL1_SPEC> {
        BLC_WINDOW_TOP_W::new(self, 0)
    }
    ///Bits 11:21 - this field configures blc average calculation window left
    #[inline(always)]
    #[must_use]
    pub fn blc_window_left(&mut self) -> BLC_WINDOW_LEFT_W<BLC_CTRL1_SPEC> {
        BLC_WINDOW_LEFT_W::new(self, 11)
    }
    ///Bits 22:25 - this field configures blc average calculation window vnum
    #[inline(always)]
    #[must_use]
    pub fn blc_window_vnum(&mut self) -> BLC_WINDOW_VNUM_W<BLC_CTRL1_SPEC> {
        BLC_WINDOW_VNUM_W::new(self, 22)
    }
    ///Bits 26:29 - this field configures blc average calculation window hnum
    #[inline(always)]
    #[must_use]
    pub fn blc_window_hnum(&mut self) -> BLC_WINDOW_HNUM_W<BLC_CTRL1_SPEC> {
        BLC_WINDOW_HNUM_W::new(self, 26)
    }
    ///Bit 30 - this bit configures enable blc average input filter. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn blc_filter_en(&mut self) -> BLC_FILTER_EN_W<BLC_CTRL1_SPEC> {
        BLC_FILTER_EN_W::new(self, 30)
    }
}
/**blc window control register

You can [`read`](crate::generic::Reg::read) this register and get [`blc_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blc_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLC_CTRL1_SPEC;
impl crate::RegisterSpec for BLC_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blc_ctrl1::R`](R) reader structure
impl crate::Readable for BLC_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`blc_ctrl1::W`](W) writer structure
impl crate::Writable for BLC_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLC_CTRL1 to value 0
impl crate::Resettable for BLC_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
