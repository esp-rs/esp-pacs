#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `CH1_AR_PROT` reader - NA"]
pub type CH1_AR_PROT_R = crate::FieldReader;
#[doc = "Field `CH1_AR_PROT` writer - NA"]
pub type CH1_AR_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_AW_PROT` reader - NA"]
pub type CH1_AW_PROT_R = crate::FieldReader;
#[doc = "Field `CH1_AW_PROT` writer - NA"]
pub type CH1_AW_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_ARLEN_EN` reader - NA"]
pub type CH1_ARLEN_EN_R = crate::BitReader;
#[doc = "Field `CH1_ARLEN_EN` writer - NA"]
pub type CH1_ARLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ARLEN` reader - NA"]
pub type CH1_ARLEN_R = crate::FieldReader;
#[doc = "Field `CH1_ARLEN` writer - NA"]
pub type CH1_ARLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH1_AWLEN_EN` reader - NA"]
pub type CH1_AWLEN_EN_R = crate::BitReader;
#[doc = "Field `CH1_AWLEN_EN` writer - NA"]
pub type CH1_AWLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_AWLEN` reader - NA"]
pub type CH1_AWLEN_R = crate::FieldReader;
#[doc = "Field `CH1_AWLEN` writer - NA"]
pub type CH1_AWLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH1_SRC_STAT_EN` reader - NA"]
pub type CH1_SRC_STAT_EN_R = crate::BitReader;
#[doc = "Field `CH1_SRC_STAT_EN` writer - NA"]
pub type CH1_SRC_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_DST_STAT_EN` reader - NA"]
pub type CH1_DST_STAT_EN_R = crate::BitReader;
#[doc = "Field `CH1_DST_STAT_EN` writer - NA"]
pub type CH1_DST_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_IOC_BLKTFR` reader - NA"]
pub type CH1_IOC_BLKTFR_R = crate::BitReader;
#[doc = "Field `CH1_IOC_BLKTFR` writer - NA"]
pub type CH1_IOC_BLKTFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_LAST` reader - NA"]
pub type CH1_SHADOWREG_OR_LLI_LAST_R = crate::BitReader;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_LAST` writer - NA"]
pub type CH1_SHADOWREG_OR_LLI_LAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_VALID` reader - NA"]
pub type CH1_SHADOWREG_OR_LLI_VALID_R = crate::BitReader;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_VALID` writer - NA"]
pub type CH1_SHADOWREG_OR_LLI_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch1_ar_prot(&self) -> CH1_AR_PROT_R {
        CH1_AR_PROT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    pub fn ch1_aw_prot(&self) -> CH1_AW_PROT_R {
        CH1_AW_PROT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_arlen_en(&self) -> CH1_ARLEN_EN_R {
        CH1_ARLEN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    pub fn ch1_arlen(&self) -> CH1_ARLEN_R {
        CH1_ARLEN_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ch1_awlen_en(&self) -> CH1_AWLEN_EN_R {
        CH1_AWLEN_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn ch1_awlen(&self) -> CH1_AWLEN_R {
        CH1_AWLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn ch1_src_stat_en(&self) -> CH1_SRC_STAT_EN_R {
        CH1_SRC_STAT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch1_dst_stat_en(&self) -> CH1_DST_STAT_EN_R {
        CH1_DST_STAT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn ch1_ioc_blktfr(&self) -> CH1_IOC_BLKTFR_R {
        CH1_IOC_BLKTFR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_last(&self) -> CH1_SHADOWREG_OR_LLI_LAST_R {
        CH1_SHADOWREG_OR_LLI_LAST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_valid(&self) -> CH1_SHADOWREG_OR_LLI_VALID_R {
        CH1_SHADOWREG_OR_LLI_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL1")
            .field("ch1_ar_prot", &self.ch1_ar_prot())
            .field("ch1_aw_prot", &self.ch1_aw_prot())
            .field("ch1_arlen_en", &self.ch1_arlen_en())
            .field("ch1_arlen", &self.ch1_arlen())
            .field("ch1_awlen_en", &self.ch1_awlen_en())
            .field("ch1_awlen", &self.ch1_awlen())
            .field("ch1_src_stat_en", &self.ch1_src_stat_en())
            .field("ch1_dst_stat_en", &self.ch1_dst_stat_en())
            .field("ch1_ioc_blktfr", &self.ch1_ioc_blktfr())
            .field(
                "ch1_shadowreg_or_lli_last",
                &self.ch1_shadowreg_or_lli_last(),
            )
            .field(
                "ch1_shadowreg_or_lli_valid",
                &self.ch1_shadowreg_or_lli_valid(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch1_ar_prot(&mut self) -> CH1_AR_PROT_W<'_, CTL1_SPEC> {
        CH1_AR_PROT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    pub fn ch1_aw_prot(&mut self) -> CH1_AW_PROT_W<'_, CTL1_SPEC> {
        CH1_AW_PROT_W::new(self, 3)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_arlen_en(&mut self) -> CH1_ARLEN_EN_W<'_, CTL1_SPEC> {
        CH1_ARLEN_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    pub fn ch1_arlen(&mut self) -> CH1_ARLEN_W<'_, CTL1_SPEC> {
        CH1_ARLEN_W::new(self, 7)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ch1_awlen_en(&mut self) -> CH1_AWLEN_EN_W<'_, CTL1_SPEC> {
        CH1_AWLEN_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn ch1_awlen(&mut self) -> CH1_AWLEN_W<'_, CTL1_SPEC> {
        CH1_AWLEN_W::new(self, 16)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn ch1_src_stat_en(&mut self) -> CH1_SRC_STAT_EN_W<'_, CTL1_SPEC> {
        CH1_SRC_STAT_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch1_dst_stat_en(&mut self) -> CH1_DST_STAT_EN_W<'_, CTL1_SPEC> {
        CH1_DST_STAT_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn ch1_ioc_blktfr(&mut self) -> CH1_IOC_BLKTFR_W<'_, CTL1_SPEC> {
        CH1_IOC_BLKTFR_W::new(self, 26)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_last(&mut self) -> CH1_SHADOWREG_OR_LLI_LAST_W<'_, CTL1_SPEC> {
        CH1_SHADOWREG_OR_LLI_LAST_W::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_valid(&mut self) -> CH1_SHADOWREG_OR_LLI_VALID_W<'_, CTL1_SPEC> {
        CH1_SHADOWREG_OR_LLI_VALID_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {}
