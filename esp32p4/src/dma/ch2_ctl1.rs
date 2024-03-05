#[doc = "Register `CH2_CTL1` reader"]
pub type R = crate::R<CH2_CTL1_SPEC>;
#[doc = "Register `CH2_CTL1` writer"]
pub type W = crate::W<CH2_CTL1_SPEC>;
#[doc = "Field `CH2_AR_PROT` reader - NA"]
pub type CH2_AR_PROT_R = crate::FieldReader;
#[doc = "Field `CH2_AR_PROT` writer - NA"]
pub type CH2_AR_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2_AW_PROT` reader - NA"]
pub type CH2_AW_PROT_R = crate::FieldReader;
#[doc = "Field `CH2_AW_PROT` writer - NA"]
pub type CH2_AW_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2_ARLEN_EN` reader - NA"]
pub type CH2_ARLEN_EN_R = crate::BitReader;
#[doc = "Field `CH2_ARLEN_EN` writer - NA"]
pub type CH2_ARLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_ARLEN` reader - NA"]
pub type CH2_ARLEN_R = crate::FieldReader;
#[doc = "Field `CH2_ARLEN` writer - NA"]
pub type CH2_ARLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH2_AWLEN_EN` reader - NA"]
pub type CH2_AWLEN_EN_R = crate::BitReader;
#[doc = "Field `CH2_AWLEN_EN` writer - NA"]
pub type CH2_AWLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_AWLEN` reader - NA"]
pub type CH2_AWLEN_R = crate::FieldReader;
#[doc = "Field `CH2_AWLEN` writer - NA"]
pub type CH2_AWLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH2_SRC_STAT_EN` reader - NA"]
pub type CH2_SRC_STAT_EN_R = crate::BitReader;
#[doc = "Field `CH2_SRC_STAT_EN` writer - NA"]
pub type CH2_SRC_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_DST_STAT_EN` reader - NA"]
pub type CH2_DST_STAT_EN_R = crate::BitReader;
#[doc = "Field `CH2_DST_STAT_EN` writer - NA"]
pub type CH2_DST_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_IOC_BLKTFR` reader - NA"]
pub type CH2_IOC_BLKTFR_R = crate::BitReader;
#[doc = "Field `CH2_IOC_BLKTFR` writer - NA"]
pub type CH2_IOC_BLKTFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_SHADOWREG_OR_LLI_LAST` reader - NA"]
pub type CH2_SHADOWREG_OR_LLI_LAST_R = crate::BitReader;
#[doc = "Field `CH2_SHADOWREG_OR_LLI_LAST` writer - NA"]
pub type CH2_SHADOWREG_OR_LLI_LAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_SHADOWREG_OR_LLI_VALID` reader - NA"]
pub type CH2_SHADOWREG_OR_LLI_VALID_R = crate::BitReader;
#[doc = "Field `CH2_SHADOWREG_OR_LLI_VALID` writer - NA"]
pub type CH2_SHADOWREG_OR_LLI_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch2_ar_prot(&self) -> CH2_AR_PROT_R {
        CH2_AR_PROT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    pub fn ch2_aw_prot(&self) -> CH2_AW_PROT_R {
        CH2_AW_PROT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch2_arlen_en(&self) -> CH2_ARLEN_EN_R {
        CH2_ARLEN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    pub fn ch2_arlen(&self) -> CH2_ARLEN_R {
        CH2_ARLEN_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ch2_awlen_en(&self) -> CH2_AWLEN_EN_R {
        CH2_AWLEN_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn ch2_awlen(&self) -> CH2_AWLEN_R {
        CH2_AWLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn ch2_src_stat_en(&self) -> CH2_SRC_STAT_EN_R {
        CH2_SRC_STAT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch2_dst_stat_en(&self) -> CH2_DST_STAT_EN_R {
        CH2_DST_STAT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn ch2_ioc_blktfr(&self) -> CH2_IOC_BLKTFR_R {
        CH2_IOC_BLKTFR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch2_shadowreg_or_lli_last(&self) -> CH2_SHADOWREG_OR_LLI_LAST_R {
        CH2_SHADOWREG_OR_LLI_LAST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch2_shadowreg_or_lli_valid(&self) -> CH2_SHADOWREG_OR_LLI_VALID_R {
        CH2_SHADOWREG_OR_LLI_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_CTL1")
            .field(
                "ch2_ar_prot",
                &format_args!("{}", self.ch2_ar_prot().bits()),
            )
            .field(
                "ch2_aw_prot",
                &format_args!("{}", self.ch2_aw_prot().bits()),
            )
            .field(
                "ch2_arlen_en",
                &format_args!("{}", self.ch2_arlen_en().bit()),
            )
            .field("ch2_arlen", &format_args!("{}", self.ch2_arlen().bits()))
            .field(
                "ch2_awlen_en",
                &format_args!("{}", self.ch2_awlen_en().bit()),
            )
            .field("ch2_awlen", &format_args!("{}", self.ch2_awlen().bits()))
            .field(
                "ch2_src_stat_en",
                &format_args!("{}", self.ch2_src_stat_en().bit()),
            )
            .field(
                "ch2_dst_stat_en",
                &format_args!("{}", self.ch2_dst_stat_en().bit()),
            )
            .field(
                "ch2_ioc_blktfr",
                &format_args!("{}", self.ch2_ioc_blktfr().bit()),
            )
            .field(
                "ch2_shadowreg_or_lli_last",
                &format_args!("{}", self.ch2_shadowreg_or_lli_last().bit()),
            )
            .field(
                "ch2_shadowreg_or_lli_valid",
                &format_args!("{}", self.ch2_shadowreg_or_lli_valid().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_CTL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_ar_prot(&mut self) -> CH2_AR_PROT_W<CH2_CTL1_SPEC> {
        CH2_AR_PROT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_aw_prot(&mut self) -> CH2_AW_PROT_W<CH2_CTL1_SPEC> {
        CH2_AW_PROT_W::new(self, 3)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_arlen_en(&mut self) -> CH2_ARLEN_EN_W<CH2_CTL1_SPEC> {
        CH2_ARLEN_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_arlen(&mut self) -> CH2_ARLEN_W<CH2_CTL1_SPEC> {
        CH2_ARLEN_W::new(self, 7)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_awlen_en(&mut self) -> CH2_AWLEN_EN_W<CH2_CTL1_SPEC> {
        CH2_AWLEN_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_awlen(&mut self) -> CH2_AWLEN_W<CH2_CTL1_SPEC> {
        CH2_AWLEN_W::new(self, 16)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_src_stat_en(&mut self) -> CH2_SRC_STAT_EN_W<CH2_CTL1_SPEC> {
        CH2_SRC_STAT_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dst_stat_en(&mut self) -> CH2_DST_STAT_EN_W<CH2_CTL1_SPEC> {
        CH2_DST_STAT_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_ioc_blktfr(&mut self) -> CH2_IOC_BLKTFR_W<CH2_CTL1_SPEC> {
        CH2_IOC_BLKTFR_W::new(self, 26)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_shadowreg_or_lli_last(&mut self) -> CH2_SHADOWREG_OR_LLI_LAST_W<CH2_CTL1_SPEC> {
        CH2_SHADOWREG_OR_LLI_LAST_W::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_shadowreg_or_lli_valid(&mut self) -> CH2_SHADOWREG_OR_LLI_VALID_W<CH2_CTL1_SPEC> {
        CH2_SHADOWREG_OR_LLI_VALID_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_CTL1_SPEC;
impl crate::RegisterSpec for CH2_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_ctl1::R`](R) reader structure"]
impl crate::Readable for CH2_CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_ctl1::W`](W) writer structure"]
impl crate::Writable for CH2_CTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_CTL1 to value 0"]
impl crate::Resettable for CH2_CTL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
