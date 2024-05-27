///Register `CTL0` reader
pub type R = crate::R<CTL0_SPEC>;
///Register `CTL0` writer
pub type W = crate::W<CTL0_SPEC>;
///Field `CH1_SMS` reader - NA
pub type CH1_SMS_R = crate::BitReader;
///Field `CH1_SMS` writer - NA
pub type CH1_SMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_DMS` reader - NA
pub type CH1_DMS_R = crate::BitReader;
///Field `CH1_DMS` writer - NA
pub type CH1_DMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_SINC` reader - NA
pub type CH1_SINC_R = crate::BitReader;
///Field `CH1_SINC` writer - NA
pub type CH1_SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_DINC` reader - NA
pub type CH1_DINC_R = crate::BitReader;
///Field `CH1_DINC` writer - NA
pub type CH1_DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_SRC_TR_WIDTH` reader - NA
pub type CH1_SRC_TR_WIDTH_R = crate::FieldReader;
///Field `CH1_SRC_TR_WIDTH` writer - NA
pub type CH1_SRC_TR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CH1_DST_TR_WIDTH` reader - NA
pub type CH1_DST_TR_WIDTH_R = crate::FieldReader;
///Field `CH1_DST_TR_WIDTH` writer - NA
pub type CH1_DST_TR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CH1_SRC_MSIZE` reader - NA
pub type CH1_SRC_MSIZE_R = crate::FieldReader;
///Field `CH1_SRC_MSIZE` writer - NA
pub type CH1_SRC_MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CH1_DST_MSIZE` reader - NA
pub type CH1_DST_MSIZE_R = crate::FieldReader;
///Field `CH1_DST_MSIZE` writer - NA
pub type CH1_DST_MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CH1_AR_CACHE` reader - NA
pub type CH1_AR_CACHE_R = crate::FieldReader;
///Field `CH1_AR_CACHE` writer - NA
pub type CH1_AR_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CH1_AW_CACHE` reader - NA
pub type CH1_AW_CACHE_R = crate::FieldReader;
///Field `CH1_AW_CACHE` writer - NA
pub type CH1_AW_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CH1_NONPOSTED_LASTWRITE_EN` reader - NA
pub type CH1_NONPOSTED_LASTWRITE_EN_R = crate::BitReader;
///Field `CH1_NONPOSTED_LASTWRITE_EN` writer - NA
pub type CH1_NONPOSTED_LASTWRITE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn ch1_sms(&self) -> CH1_SMS_R {
        CH1_SMS_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn ch1_dms(&self) -> CH1_DMS_R {
        CH1_DMS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - NA
    #[inline(always)]
    pub fn ch1_sinc(&self) -> CH1_SINC_R {
        CH1_SINC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - NA
    #[inline(always)]
    pub fn ch1_dinc(&self) -> CH1_DINC_R {
        CH1_DINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:10 - NA
    #[inline(always)]
    pub fn ch1_src_tr_width(&self) -> CH1_SRC_TR_WIDTH_R {
        CH1_SRC_TR_WIDTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - NA
    #[inline(always)]
    pub fn ch1_dst_tr_width(&self) -> CH1_DST_TR_WIDTH_R {
        CH1_DST_TR_WIDTH_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:17 - NA
    #[inline(always)]
    pub fn ch1_src_msize(&self) -> CH1_SRC_MSIZE_R {
        CH1_SRC_MSIZE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bits 18:21 - NA
    #[inline(always)]
    pub fn ch1_dst_msize(&self) -> CH1_DST_MSIZE_R {
        CH1_DST_MSIZE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:25 - NA
    #[inline(always)]
    pub fn ch1_ar_cache(&self) -> CH1_AR_CACHE_R {
        CH1_AR_CACHE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bits 26:29 - NA
    #[inline(always)]
    pub fn ch1_aw_cache(&self) -> CH1_AW_CACHE_R {
        CH1_AW_CACHE_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bit 30 - NA
    #[inline(always)]
    pub fn ch1_nonposted_lastwrite_en(&self) -> CH1_NONPOSTED_LASTWRITE_EN_R {
        CH1_NONPOSTED_LASTWRITE_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL0")
            .field("ch1_sms", &self.ch1_sms())
            .field("ch1_dms", &self.ch1_dms())
            .field("ch1_sinc", &self.ch1_sinc())
            .field("ch1_dinc", &self.ch1_dinc())
            .field("ch1_src_tr_width", &self.ch1_src_tr_width())
            .field("ch1_dst_tr_width", &self.ch1_dst_tr_width())
            .field("ch1_src_msize", &self.ch1_src_msize())
            .field("ch1_dst_msize", &self.ch1_dst_msize())
            .field("ch1_ar_cache", &self.ch1_ar_cache())
            .field("ch1_aw_cache", &self.ch1_aw_cache())
            .field(
                "ch1_nonposted_lastwrite_en",
                &self.ch1_nonposted_lastwrite_en(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_sms(&mut self) -> CH1_SMS_W<CTL0_SPEC> {
        CH1_SMS_W::new(self, 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_dms(&mut self) -> CH1_DMS_W<CTL0_SPEC> {
        CH1_DMS_W::new(self, 2)
    }
    ///Bit 4 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_sinc(&mut self) -> CH1_SINC_W<CTL0_SPEC> {
        CH1_SINC_W::new(self, 4)
    }
    ///Bit 6 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_dinc(&mut self) -> CH1_DINC_W<CTL0_SPEC> {
        CH1_DINC_W::new(self, 6)
    }
    ///Bits 8:10 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_src_tr_width(&mut self) -> CH1_SRC_TR_WIDTH_W<CTL0_SPEC> {
        CH1_SRC_TR_WIDTH_W::new(self, 8)
    }
    ///Bits 11:13 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_dst_tr_width(&mut self) -> CH1_DST_TR_WIDTH_W<CTL0_SPEC> {
        CH1_DST_TR_WIDTH_W::new(self, 11)
    }
    ///Bits 14:17 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_src_msize(&mut self) -> CH1_SRC_MSIZE_W<CTL0_SPEC> {
        CH1_SRC_MSIZE_W::new(self, 14)
    }
    ///Bits 18:21 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_dst_msize(&mut self) -> CH1_DST_MSIZE_W<CTL0_SPEC> {
        CH1_DST_MSIZE_W::new(self, 18)
    }
    ///Bits 22:25 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_ar_cache(&mut self) -> CH1_AR_CACHE_W<CTL0_SPEC> {
        CH1_AR_CACHE_W::new(self, 22)
    }
    ///Bits 26:29 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_aw_cache(&mut self) -> CH1_AW_CACHE_W<CTL0_SPEC> {
        CH1_AW_CACHE_W::new(self, 26)
    }
    ///Bit 30 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_nonposted_lastwrite_en(&mut self) -> CH1_NONPOSTED_LASTWRITE_EN_W<CTL0_SPEC> {
        CH1_NONPOSTED_LASTWRITE_EN_W::new(self, 30)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ctl0::R`](R) reader structure
impl crate::Readable for CTL0_SPEC {}
///`write(|w| ..)` method takes [`ctl0::W`](W) writer structure
impl crate::Writable for CTL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTL0 to value 0x1200
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: u32 = 0x1200;
}
