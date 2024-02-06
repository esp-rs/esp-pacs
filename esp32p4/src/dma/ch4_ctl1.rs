#[doc = "Register `CH4_CTL1` reader"]
pub type R = crate::R<CH4_CTL1_SPEC>;
#[doc = "Register `CH4_CTL1` writer"]
pub type W = crate::W<CH4_CTL1_SPEC>;
#[doc = "Field `CH4_AR_PROT` reader - NA"]
pub type CH4_AR_PROT_R = crate::FieldReader;
#[doc = "Field `CH4_AR_PROT` writer - NA"]
pub type CH4_AR_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH4_AW_PROT` reader - NA"]
pub type CH4_AW_PROT_R = crate::FieldReader;
#[doc = "Field `CH4_AW_PROT` writer - NA"]
pub type CH4_AW_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH4_ARLEN_EN` reader - NA"]
pub type CH4_ARLEN_EN_R = crate::BitReader;
#[doc = "Field `CH4_ARLEN_EN` writer - NA"]
pub type CH4_ARLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_ARLEN` reader - NA"]
pub type CH4_ARLEN_R = crate::FieldReader;
#[doc = "Field `CH4_ARLEN` writer - NA"]
pub type CH4_ARLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH4_AWLEN_EN` reader - NA"]
pub type CH4_AWLEN_EN_R = crate::BitReader;
#[doc = "Field `CH4_AWLEN_EN` writer - NA"]
pub type CH4_AWLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_AWLEN` reader - NA"]
pub type CH4_AWLEN_R = crate::FieldReader;
#[doc = "Field `CH4_AWLEN` writer - NA"]
pub type CH4_AWLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH4_SRC_STAT_EN` reader - NA"]
pub type CH4_SRC_STAT_EN_R = crate::BitReader;
#[doc = "Field `CH4_SRC_STAT_EN` writer - NA"]
pub type CH4_SRC_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_DST_STAT_EN` reader - NA"]
pub type CH4_DST_STAT_EN_R = crate::BitReader;
#[doc = "Field `CH4_DST_STAT_EN` writer - NA"]
pub type CH4_DST_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_IOC_BLKTFR` reader - NA"]
pub type CH4_IOC_BLKTFR_R = crate::BitReader;
#[doc = "Field `CH4_IOC_BLKTFR` writer - NA"]
pub type CH4_IOC_BLKTFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SHADOWREG_OR_LLI_LAST` reader - NA"]
pub type CH4_SHADOWREG_OR_LLI_LAST_R = crate::BitReader;
#[doc = "Field `CH4_SHADOWREG_OR_LLI_LAST` writer - NA"]
pub type CH4_SHADOWREG_OR_LLI_LAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SHADOWREG_OR_LLI_VALID` reader - NA"]
pub type CH4_SHADOWREG_OR_LLI_VALID_R = crate::BitReader;
#[doc = "Field `CH4_SHADOWREG_OR_LLI_VALID` writer - NA"]
pub type CH4_SHADOWREG_OR_LLI_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch4_ar_prot(&self) -> CH4_AR_PROT_R {
        CH4_AR_PROT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    pub fn ch4_aw_prot(&self) -> CH4_AW_PROT_R {
        CH4_AW_PROT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch4_arlen_en(&self) -> CH4_ARLEN_EN_R {
        CH4_ARLEN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    pub fn ch4_arlen(&self) -> CH4_ARLEN_R {
        CH4_ARLEN_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ch4_awlen_en(&self) -> CH4_AWLEN_EN_R {
        CH4_AWLEN_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn ch4_awlen(&self) -> CH4_AWLEN_R {
        CH4_AWLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn ch4_src_stat_en(&self) -> CH4_SRC_STAT_EN_R {
        CH4_SRC_STAT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch4_dst_stat_en(&self) -> CH4_DST_STAT_EN_R {
        CH4_DST_STAT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn ch4_ioc_blktfr(&self) -> CH4_IOC_BLKTFR_R {
        CH4_IOC_BLKTFR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch4_shadowreg_or_lli_last(&self) -> CH4_SHADOWREG_OR_LLI_LAST_R {
        CH4_SHADOWREG_OR_LLI_LAST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch4_shadowreg_or_lli_valid(&self) -> CH4_SHADOWREG_OR_LLI_VALID_R {
        CH4_SHADOWREG_OR_LLI_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_CTL1")
            .field(
                "ch4_ar_prot",
                &format_args!("{}", self.ch4_ar_prot().bits()),
            )
            .field(
                "ch4_aw_prot",
                &format_args!("{}", self.ch4_aw_prot().bits()),
            )
            .field(
                "ch4_arlen_en",
                &format_args!("{}", self.ch4_arlen_en().bit()),
            )
            .field("ch4_arlen", &format_args!("{}", self.ch4_arlen().bits()))
            .field(
                "ch4_awlen_en",
                &format_args!("{}", self.ch4_awlen_en().bit()),
            )
            .field("ch4_awlen", &format_args!("{}", self.ch4_awlen().bits()))
            .field(
                "ch4_src_stat_en",
                &format_args!("{}", self.ch4_src_stat_en().bit()),
            )
            .field(
                "ch4_dst_stat_en",
                &format_args!("{}", self.ch4_dst_stat_en().bit()),
            )
            .field(
                "ch4_ioc_blktfr",
                &format_args!("{}", self.ch4_ioc_blktfr().bit()),
            )
            .field(
                "ch4_shadowreg_or_lli_last",
                &format_args!("{}", self.ch4_shadowreg_or_lli_last().bit()),
            )
            .field(
                "ch4_shadowreg_or_lli_valid",
                &format_args!("{}", self.ch4_shadowreg_or_lli_valid().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_CTL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_ar_prot(&mut self) -> CH4_AR_PROT_W<CH4_CTL1_SPEC> {
        CH4_AR_PROT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_aw_prot(&mut self) -> CH4_AW_PROT_W<CH4_CTL1_SPEC> {
        CH4_AW_PROT_W::new(self, 3)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_arlen_en(&mut self) -> CH4_ARLEN_EN_W<CH4_CTL1_SPEC> {
        CH4_ARLEN_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_arlen(&mut self) -> CH4_ARLEN_W<CH4_CTL1_SPEC> {
        CH4_ARLEN_W::new(self, 7)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_awlen_en(&mut self) -> CH4_AWLEN_EN_W<CH4_CTL1_SPEC> {
        CH4_AWLEN_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_awlen(&mut self) -> CH4_AWLEN_W<CH4_CTL1_SPEC> {
        CH4_AWLEN_W::new(self, 16)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_src_stat_en(&mut self) -> CH4_SRC_STAT_EN_W<CH4_CTL1_SPEC> {
        CH4_SRC_STAT_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_dst_stat_en(&mut self) -> CH4_DST_STAT_EN_W<CH4_CTL1_SPEC> {
        CH4_DST_STAT_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_ioc_blktfr(&mut self) -> CH4_IOC_BLKTFR_W<CH4_CTL1_SPEC> {
        CH4_IOC_BLKTFR_W::new(self, 26)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_shadowreg_or_lli_last(&mut self) -> CH4_SHADOWREG_OR_LLI_LAST_W<CH4_CTL1_SPEC> {
        CH4_SHADOWREG_OR_LLI_LAST_W::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_shadowreg_or_lli_valid(&mut self) -> CH4_SHADOWREG_OR_LLI_VALID_W<CH4_CTL1_SPEC> {
        CH4_SHADOWREG_OR_LLI_VALID_W::new(self, 31)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_CTL1_SPEC;
impl crate::RegisterSpec for CH4_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_ctl1::R`](R) reader structure"]
impl crate::Readable for CH4_CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4_ctl1::W`](W) writer structure"]
impl crate::Writable for CH4_CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_CTL1 to value 0"]
impl crate::Resettable for CH4_CTL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
