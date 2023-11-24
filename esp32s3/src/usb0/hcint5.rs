#[doc = "Register `HCINT5` reader"]
pub type R = crate::R<HCINT5_SPEC>;
#[doc = "Register `HCINT5` writer"]
pub type W = crate::W<HCINT5_SPEC>;
#[doc = "Field `H_XFERCOMPL5` reader - "]
pub type H_XFERCOMPL5_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL5` writer - "]
pub type H_XFERCOMPL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTD5` reader - "]
pub type H_CHHLTD5_R = crate::BitReader;
#[doc = "Field `H_CHHLTD5` writer - "]
pub type H_CHHLTD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERR5` reader - "]
pub type H_AHBERR5_R = crate::BitReader;
#[doc = "Field `H_AHBERR5` writer - "]
pub type H_AHBERR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALL5` reader - "]
pub type H_STALL5_R = crate::BitReader;
#[doc = "Field `H_STALL5` writer - "]
pub type H_STALL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NACK5` reader - "]
pub type H_NACK5_R = crate::BitReader;
#[doc = "Field `H_NACK5` writer - "]
pub type H_NACK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACK5` reader - "]
pub type H_ACK5_R = crate::BitReader;
#[doc = "Field `H_ACK5` writer - "]
pub type H_ACK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYET5` reader - "]
pub type H_NYET5_R = crate::BitReader;
#[doc = "Field `H_NYET5` writer - "]
pub type H_NYET5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERR5` reader - "]
pub type H_XACTERR5_R = crate::BitReader;
#[doc = "Field `H_XACTERR5` writer - "]
pub type H_XACTERR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERR5` reader - "]
pub type H_BBLERR5_R = crate::BitReader;
#[doc = "Field `H_BBLERR5` writer - "]
pub type H_BBLERR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUN5` reader - "]
pub type H_FRMOVRUN5_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN5` writer - "]
pub type H_FRMOVRUN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERR5` reader - "]
pub type H_DATATGLERR5_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR5` writer - "]
pub type H_DATATGLERR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTR5` reader - "]
pub type H_BNAINTR5_R = crate::BitReader;
#[doc = "Field `H_BNAINTR5` writer - "]
pub type H_BNAINTR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XCS_XACT_ERR5` reader - "]
pub type H_XCS_XACT_ERR5_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR5` writer - "]
pub type H_XCS_XACT_ERR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTR5` reader - "]
pub type H_DESC_LST_ROLLINTR5_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR5` writer - "]
pub type H_DESC_LST_ROLLINTR5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl5(&self) -> H_XFERCOMPL5_R {
        H_XFERCOMPL5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd5(&self) -> H_CHHLTD5_R {
        H_CHHLTD5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr5(&self) -> H_AHBERR5_R {
        H_AHBERR5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall5(&self) -> H_STALL5_R {
        H_STALL5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack5(&self) -> H_NACK5_R {
        H_NACK5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack5(&self) -> H_ACK5_R {
        H_ACK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet5(&self) -> H_NYET5_R {
        H_NYET5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr5(&self) -> H_XACTERR5_R {
        H_XACTERR5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr5(&self) -> H_BBLERR5_R {
        H_BBLERR5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun5(&self) -> H_FRMOVRUN5_R {
        H_FRMOVRUN5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr5(&self) -> H_DATATGLERR5_R {
        H_DATATGLERR5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr5(&self) -> H_BNAINTR5_R {
        H_BNAINTR5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err5(&self) -> H_XCS_XACT_ERR5_R {
        H_XCS_XACT_ERR5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr5(&self) -> H_DESC_LST_ROLLINTR5_R {
        H_DESC_LST_ROLLINTR5_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT5")
            .field(
                "h_xfercompl5",
                &format_args!("{}", self.h_xfercompl5().bit()),
            )
            .field("h_chhltd5", &format_args!("{}", self.h_chhltd5().bit()))
            .field("h_ahberr5", &format_args!("{}", self.h_ahberr5().bit()))
            .field("h_stall5", &format_args!("{}", self.h_stall5().bit()))
            .field("h_nack5", &format_args!("{}", self.h_nack5().bit()))
            .field("h_ack5", &format_args!("{}", self.h_ack5().bit()))
            .field("h_nyet5", &format_args!("{}", self.h_nyet5().bit()))
            .field("h_xacterr5", &format_args!("{}", self.h_xacterr5().bit()))
            .field("h_bblerr5", &format_args!("{}", self.h_bblerr5().bit()))
            .field("h_frmovrun5", &format_args!("{}", self.h_frmovrun5().bit()))
            .field(
                "h_datatglerr5",
                &format_args!("{}", self.h_datatglerr5().bit()),
            )
            .field("h_bnaintr5", &format_args!("{}", self.h_bnaintr5().bit()))
            .field(
                "h_xcs_xact_err5",
                &format_args!("{}", self.h_xcs_xact_err5().bit()),
            )
            .field(
                "h_desc_lst_rollintr5",
                &format_args!("{}", self.h_desc_lst_rollintr5().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl5(&mut self) -> H_XFERCOMPL5_W<HCINT5_SPEC> {
        H_XFERCOMPL5_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd5(&mut self) -> H_CHHLTD5_W<HCINT5_SPEC> {
        H_CHHLTD5_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr5(&mut self) -> H_AHBERR5_W<HCINT5_SPEC> {
        H_AHBERR5_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall5(&mut self) -> H_STALL5_W<HCINT5_SPEC> {
        H_STALL5_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack5(&mut self) -> H_NACK5_W<HCINT5_SPEC> {
        H_NACK5_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack5(&mut self) -> H_ACK5_W<HCINT5_SPEC> {
        H_ACK5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet5(&mut self) -> H_NYET5_W<HCINT5_SPEC> {
        H_NYET5_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr5(&mut self) -> H_XACTERR5_W<HCINT5_SPEC> {
        H_XACTERR5_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr5(&mut self) -> H_BBLERR5_W<HCINT5_SPEC> {
        H_BBLERR5_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun5(&mut self) -> H_FRMOVRUN5_W<HCINT5_SPEC> {
        H_FRMOVRUN5_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr5(&mut self) -> H_DATATGLERR5_W<HCINT5_SPEC> {
        H_DATATGLERR5_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr5(&mut self) -> H_BNAINTR5_W<HCINT5_SPEC> {
        H_BNAINTR5_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err5(&mut self) -> H_XCS_XACT_ERR5_W<HCINT5_SPEC> {
        H_XCS_XACT_ERR5_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr5(&mut self) -> H_DESC_LST_ROLLINTR5_W<HCINT5_SPEC> {
        H_DESC_LST_ROLLINTR5_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT5_SPEC;
impl crate::RegisterSpec for HCINT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint5::R`](R) reader structure"]
impl crate::Readable for HCINT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint5::W`](W) writer structure"]
impl crate::Writable for HCINT5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT5 to value 0"]
impl crate::Resettable for HCINT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
