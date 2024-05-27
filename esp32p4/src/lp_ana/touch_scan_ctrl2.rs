///Register `TOUCH_SCAN_CTRL2` reader
pub type R = crate::R<TOUCH_SCAN_CTRL2_SPEC>;
///Register `TOUCH_SCAN_CTRL2` writer
pub type W = crate::W<TOUCH_SCAN_CTRL2_SPEC>;
///Field `TOUCH_TIMEOUT_NUM` reader - need_des
pub type TOUCH_TIMEOUT_NUM_R = crate::FieldReader<u16>;
///Field `TOUCH_TIMEOUT_NUM` writer - need_des
pub type TOUCH_TIMEOUT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TOUCH_TIMEOUT_EN` reader - need_des
pub type TOUCH_TIMEOUT_EN_R = crate::BitReader;
///Field `TOUCH_TIMEOUT_EN` writer - need_des
pub type TOUCH_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_OUT_RING` reader - need_des
pub type TOUCH_OUT_RING_R = crate::FieldReader;
///Field `TOUCH_OUT_RING` writer - need_des
pub type TOUCH_OUT_RING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FREQ_SCAN_EN` reader - need_des
pub type FREQ_SCAN_EN_R = crate::BitReader;
///Field `FREQ_SCAN_EN` writer - need_des
pub type FREQ_SCAN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FREQ_SCAN_CNT_LIMIT` reader - need_des
pub type FREQ_SCAN_CNT_LIMIT_R = crate::FieldReader;
///Field `FREQ_SCAN_CNT_LIMIT` writer - need_des
pub type FREQ_SCAN_CNT_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 6:21 - need_des
    #[inline(always)]
    pub fn touch_timeout_num(&self) -> TOUCH_TIMEOUT_NUM_R {
        TOUCH_TIMEOUT_NUM_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    ///Bit 22 - need_des
    #[inline(always)]
    pub fn touch_timeout_en(&self) -> TOUCH_TIMEOUT_EN_R {
        TOUCH_TIMEOUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:26 - need_des
    #[inline(always)]
    pub fn touch_out_ring(&self) -> TOUCH_OUT_RING_R {
        TOUCH_OUT_RING_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn freq_scan_en(&self) -> FREQ_SCAN_EN_R {
        FREQ_SCAN_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - need_des
    #[inline(always)]
    pub fn freq_scan_cnt_limit(&self) -> FREQ_SCAN_CNT_LIMIT_R {
        FREQ_SCAN_CNT_LIMIT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_SCAN_CTRL2")
            .field("touch_timeout_num", &self.touch_timeout_num())
            .field("touch_timeout_en", &self.touch_timeout_en())
            .field("touch_out_ring", &self.touch_out_ring())
            .field("freq_scan_en", &self.freq_scan_en())
            .field("freq_scan_cnt_limit", &self.freq_scan_cnt_limit())
            .finish()
    }
}
impl W {
    ///Bits 6:21 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_num(&mut self) -> TOUCH_TIMEOUT_NUM_W<TOUCH_SCAN_CTRL2_SPEC> {
        TOUCH_TIMEOUT_NUM_W::new(self, 6)
    }
    ///Bit 22 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_en(&mut self) -> TOUCH_TIMEOUT_EN_W<TOUCH_SCAN_CTRL2_SPEC> {
        TOUCH_TIMEOUT_EN_W::new(self, 22)
    }
    ///Bits 23:26 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_out_ring(&mut self) -> TOUCH_OUT_RING_W<TOUCH_SCAN_CTRL2_SPEC> {
        TOUCH_OUT_RING_W::new(self, 23)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn freq_scan_en(&mut self) -> FREQ_SCAN_EN_W<TOUCH_SCAN_CTRL2_SPEC> {
        FREQ_SCAN_EN_W::new(self, 27)
    }
    ///Bits 28:29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn freq_scan_cnt_limit(&mut self) -> FREQ_SCAN_CNT_LIMIT_W<TOUCH_SCAN_CTRL2_SPEC> {
        FREQ_SCAN_CNT_LIMIT_W::new(self, 28)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_SCAN_CTRL2_SPEC;
impl crate::RegisterSpec for TOUCH_SCAN_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_scan_ctrl2::R`](R) reader structure
impl crate::Readable for TOUCH_SCAN_CTRL2_SPEC {}
///`write(|w| ..)` method takes [`touch_scan_ctrl2::W`](W) writer structure
impl crate::Writable for TOUCH_SCAN_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_SCAN_CTRL2 to value 0x37bf_ffc0
impl crate::Resettable for TOUCH_SCAN_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x37bf_ffc0;
}
