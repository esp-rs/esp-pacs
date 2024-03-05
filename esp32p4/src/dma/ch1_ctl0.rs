#[doc = "Register `CH1_CTL0` reader"]
pub type R = crate::R<CH1_CTL0_SPEC>;
#[doc = "Register `CH1_CTL0` writer"]
pub type W = crate::W<CH1_CTL0_SPEC>;
#[doc = "Field `CH1_SMS` reader - NA"]
pub type CH1_SMS_R = crate::BitReader;
#[doc = "Field `CH1_SMS` writer - NA"]
pub type CH1_SMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_DMS` reader - NA"]
pub type CH1_DMS_R = crate::BitReader;
#[doc = "Field `CH1_DMS` writer - NA"]
pub type CH1_DMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SINC` reader - NA"]
pub type CH1_SINC_R = crate::BitReader;
#[doc = "Field `CH1_SINC` writer - NA"]
pub type CH1_SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_DINC` reader - NA"]
pub type CH1_DINC_R = crate::BitReader;
#[doc = "Field `CH1_DINC` writer - NA"]
pub type CH1_DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SRC_TR_WIDTH` reader - NA"]
pub type CH1_SRC_TR_WIDTH_R = crate::FieldReader;
#[doc = "Field `CH1_SRC_TR_WIDTH` writer - NA"]
pub type CH1_SRC_TR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_DST_TR_WIDTH` reader - NA"]
pub type CH1_DST_TR_WIDTH_R = crate::FieldReader;
#[doc = "Field `CH1_DST_TR_WIDTH` writer - NA"]
pub type CH1_DST_TR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_SRC_MSIZE` reader - NA"]
pub type CH1_SRC_MSIZE_R = crate::FieldReader;
#[doc = "Field `CH1_SRC_MSIZE` writer - NA"]
pub type CH1_SRC_MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1_DST_MSIZE` reader - NA"]
pub type CH1_DST_MSIZE_R = crate::FieldReader;
#[doc = "Field `CH1_DST_MSIZE` writer - NA"]
pub type CH1_DST_MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1_AR_CACHE` reader - NA"]
pub type CH1_AR_CACHE_R = crate::FieldReader;
#[doc = "Field `CH1_AR_CACHE` writer - NA"]
pub type CH1_AR_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1_AW_CACHE` reader - NA"]
pub type CH1_AW_CACHE_R = crate::FieldReader;
#[doc = "Field `CH1_AW_CACHE` writer - NA"]
pub type CH1_AW_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1_NONPOSTED_LASTWRITE_EN` reader - NA"]
pub type CH1_NONPOSTED_LASTWRITE_EN_R = crate::BitReader;
#[doc = "Field `CH1_NONPOSTED_LASTWRITE_EN` writer - NA"]
pub type CH1_NONPOSTED_LASTWRITE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_sms(&self) -> CH1_SMS_R {
        CH1_SMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_dms(&self) -> CH1_DMS_R {
        CH1_DMS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_sinc(&self) -> CH1_SINC_R {
        CH1_SINC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_dinc(&self) -> CH1_DINC_R {
        CH1_DINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - NA"]
    #[inline(always)]
    pub fn ch1_src_tr_width(&self) -> CH1_SRC_TR_WIDTH_R {
        CH1_SRC_TR_WIDTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - NA"]
    #[inline(always)]
    pub fn ch1_dst_tr_width(&self) -> CH1_DST_TR_WIDTH_R {
        CH1_DST_TR_WIDTH_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:17 - NA"]
    #[inline(always)]
    pub fn ch1_src_msize(&self) -> CH1_SRC_MSIZE_R {
        CH1_SRC_MSIZE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - NA"]
    #[inline(always)]
    pub fn ch1_dst_msize(&self) -> CH1_DST_MSIZE_R {
        CH1_DST_MSIZE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - NA"]
    #[inline(always)]
    pub fn ch1_ar_cache(&self) -> CH1_AR_CACHE_R {
        CH1_AR_CACHE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - NA"]
    #[inline(always)]
    pub fn ch1_aw_cache(&self) -> CH1_AW_CACHE_R {
        CH1_AW_CACHE_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_nonposted_lastwrite_en(&self) -> CH1_NONPOSTED_LASTWRITE_EN_R {
        CH1_NONPOSTED_LASTWRITE_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_CTL0")
            .field("ch1_sms", &format_args!("{}", self.ch1_sms().bit()))
            .field("ch1_dms", &format_args!("{}", self.ch1_dms().bit()))
            .field("ch1_sinc", &format_args!("{}", self.ch1_sinc().bit()))
            .field("ch1_dinc", &format_args!("{}", self.ch1_dinc().bit()))
            .field(
                "ch1_src_tr_width",
                &format_args!("{}", self.ch1_src_tr_width().bits()),
            )
            .field(
                "ch1_dst_tr_width",
                &format_args!("{}", self.ch1_dst_tr_width().bits()),
            )
            .field(
                "ch1_src_msize",
                &format_args!("{}", self.ch1_src_msize().bits()),
            )
            .field(
                "ch1_dst_msize",
                &format_args!("{}", self.ch1_dst_msize().bits()),
            )
            .field(
                "ch1_ar_cache",
                &format_args!("{}", self.ch1_ar_cache().bits()),
            )
            .field(
                "ch1_aw_cache",
                &format_args!("{}", self.ch1_aw_cache().bits()),
            )
            .field(
                "ch1_nonposted_lastwrite_en",
                &format_args!("{}", self.ch1_nonposted_lastwrite_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH1_CTL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_sms(&mut self) -> CH1_SMS_W<CH1_CTL0_SPEC> {
        CH1_SMS_W::new(self, 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dms(&mut self) -> CH1_DMS_W<CH1_CTL0_SPEC> {
        CH1_DMS_W::new(self, 2)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_sinc(&mut self) -> CH1_SINC_W<CH1_CTL0_SPEC> {
        CH1_SINC_W::new(self, 4)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dinc(&mut self) -> CH1_DINC_W<CH1_CTL0_SPEC> {
        CH1_DINC_W::new(self, 6)
    }
    #[doc = "Bits 8:10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_src_tr_width(&mut self) -> CH1_SRC_TR_WIDTH_W<CH1_CTL0_SPEC> {
        CH1_SRC_TR_WIDTH_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dst_tr_width(&mut self) -> CH1_DST_TR_WIDTH_W<CH1_CTL0_SPEC> {
        CH1_DST_TR_WIDTH_W::new(self, 11)
    }
    #[doc = "Bits 14:17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_src_msize(&mut self) -> CH1_SRC_MSIZE_W<CH1_CTL0_SPEC> {
        CH1_SRC_MSIZE_W::new(self, 14)
    }
    #[doc = "Bits 18:21 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dst_msize(&mut self) -> CH1_DST_MSIZE_W<CH1_CTL0_SPEC> {
        CH1_DST_MSIZE_W::new(self, 18)
    }
    #[doc = "Bits 22:25 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_ar_cache(&mut self) -> CH1_AR_CACHE_W<CH1_CTL0_SPEC> {
        CH1_AR_CACHE_W::new(self, 22)
    }
    #[doc = "Bits 26:29 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_aw_cache(&mut self) -> CH1_AW_CACHE_W<CH1_CTL0_SPEC> {
        CH1_AW_CACHE_W::new(self, 26)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_nonposted_lastwrite_en(&mut self) -> CH1_NONPOSTED_LASTWRITE_EN_W<CH1_CTL0_SPEC> {
        CH1_NONPOSTED_LASTWRITE_EN_W::new(self, 30)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_CTL0_SPEC;
impl crate::RegisterSpec for CH1_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_ctl0::R`](R) reader structure"]
impl crate::Readable for CH1_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1_ctl0::W`](W) writer structure"]
impl crate::Writable for CH1_CTL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_CTL0 to value 0x1200"]
impl crate::Resettable for CH1_CTL0_SPEC {
    const RESET_VALUE: u32 = 0x1200;
}
