#[doc = "Register `HCINT6` reader"]
pub type R = crate::R<HCINT6_SPEC>;
#[doc = "Register `HCINT6` writer"]
pub type W = crate::W<HCINT6_SPEC>;
#[doc = "Field `H_XFERCOMPL6` reader - "]
pub type H_XFERCOMPL6_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL6` writer - "]
pub type H_XFERCOMPL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTD6` reader - "]
pub type H_CHHLTD6_R = crate::BitReader;
#[doc = "Field `H_CHHLTD6` writer - "]
pub type H_CHHLTD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERR6` reader - "]
pub type H_AHBERR6_R = crate::BitReader;
#[doc = "Field `H_AHBERR6` writer - "]
pub type H_AHBERR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALL6` reader - "]
pub type H_STALL6_R = crate::BitReader;
#[doc = "Field `H_STALL6` writer - "]
pub type H_STALL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NACK6` reader - "]
pub type H_NACK6_R = crate::BitReader;
#[doc = "Field `H_NACK6` writer - "]
pub type H_NACK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACK6` reader - "]
pub type H_ACK6_R = crate::BitReader;
#[doc = "Field `H_ACK6` writer - "]
pub type H_ACK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYET6` reader - "]
pub type H_NYET6_R = crate::BitReader;
#[doc = "Field `H_NYET6` writer - "]
pub type H_NYET6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERR6` reader - "]
pub type H_XACTERR6_R = crate::BitReader;
#[doc = "Field `H_XACTERR6` writer - "]
pub type H_XACTERR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERR6` reader - "]
pub type H_BBLERR6_R = crate::BitReader;
#[doc = "Field `H_BBLERR6` writer - "]
pub type H_BBLERR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUN6` reader - "]
pub type H_FRMOVRUN6_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN6` writer - "]
pub type H_FRMOVRUN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERR6` reader - "]
pub type H_DATATGLERR6_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR6` writer - "]
pub type H_DATATGLERR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTR6` reader - "]
pub type H_BNAINTR6_R = crate::BitReader;
#[doc = "Field `H_BNAINTR6` writer - "]
pub type H_BNAINTR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XCS_XACT_ERR6` reader - "]
pub type H_XCS_XACT_ERR6_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR6` writer - "]
pub type H_XCS_XACT_ERR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTR6` reader - "]
pub type H_DESC_LST_ROLLINTR6_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR6` writer - "]
pub type H_DESC_LST_ROLLINTR6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl6(&self) -> H_XFERCOMPL6_R {
        H_XFERCOMPL6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd6(&self) -> H_CHHLTD6_R {
        H_CHHLTD6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr6(&self) -> H_AHBERR6_R {
        H_AHBERR6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall6(&self) -> H_STALL6_R {
        H_STALL6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack6(&self) -> H_NACK6_R {
        H_NACK6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack6(&self) -> H_ACK6_R {
        H_ACK6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet6(&self) -> H_NYET6_R {
        H_NYET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr6(&self) -> H_XACTERR6_R {
        H_XACTERR6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr6(&self) -> H_BBLERR6_R {
        H_BBLERR6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun6(&self) -> H_FRMOVRUN6_R {
        H_FRMOVRUN6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr6(&self) -> H_DATATGLERR6_R {
        H_DATATGLERR6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr6(&self) -> H_BNAINTR6_R {
        H_BNAINTR6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err6(&self) -> H_XCS_XACT_ERR6_R {
        H_XCS_XACT_ERR6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr6(&self) -> H_DESC_LST_ROLLINTR6_R {
        H_DESC_LST_ROLLINTR6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT6")
            .field(
                "h_xfercompl6",
                &format_args!("{}", self.h_xfercompl6().bit()),
            )
            .field("h_chhltd6", &format_args!("{}", self.h_chhltd6().bit()))
            .field("h_ahberr6", &format_args!("{}", self.h_ahberr6().bit()))
            .field("h_stall6", &format_args!("{}", self.h_stall6().bit()))
            .field("h_nack6", &format_args!("{}", self.h_nack6().bit()))
            .field("h_ack6", &format_args!("{}", self.h_ack6().bit()))
            .field("h_nyet6", &format_args!("{}", self.h_nyet6().bit()))
            .field("h_xacterr6", &format_args!("{}", self.h_xacterr6().bit()))
            .field("h_bblerr6", &format_args!("{}", self.h_bblerr6().bit()))
            .field("h_frmovrun6", &format_args!("{}", self.h_frmovrun6().bit()))
            .field(
                "h_datatglerr6",
                &format_args!("{}", self.h_datatglerr6().bit()),
            )
            .field("h_bnaintr6", &format_args!("{}", self.h_bnaintr6().bit()))
            .field(
                "h_xcs_xact_err6",
                &format_args!("{}", self.h_xcs_xact_err6().bit()),
            )
            .field(
                "h_desc_lst_rollintr6",
                &format_args!("{}", self.h_desc_lst_rollintr6().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl6(&mut self) -> H_XFERCOMPL6_W<HCINT6_SPEC> {
        H_XFERCOMPL6_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd6(&mut self) -> H_CHHLTD6_W<HCINT6_SPEC> {
        H_CHHLTD6_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr6(&mut self) -> H_AHBERR6_W<HCINT6_SPEC> {
        H_AHBERR6_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall6(&mut self) -> H_STALL6_W<HCINT6_SPEC> {
        H_STALL6_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack6(&mut self) -> H_NACK6_W<HCINT6_SPEC> {
        H_NACK6_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack6(&mut self) -> H_ACK6_W<HCINT6_SPEC> {
        H_ACK6_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet6(&mut self) -> H_NYET6_W<HCINT6_SPEC> {
        H_NYET6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr6(&mut self) -> H_XACTERR6_W<HCINT6_SPEC> {
        H_XACTERR6_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr6(&mut self) -> H_BBLERR6_W<HCINT6_SPEC> {
        H_BBLERR6_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun6(&mut self) -> H_FRMOVRUN6_W<HCINT6_SPEC> {
        H_FRMOVRUN6_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr6(&mut self) -> H_DATATGLERR6_W<HCINT6_SPEC> {
        H_DATATGLERR6_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr6(&mut self) -> H_BNAINTR6_W<HCINT6_SPEC> {
        H_BNAINTR6_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err6(&mut self) -> H_XCS_XACT_ERR6_W<HCINT6_SPEC> {
        H_XCS_XACT_ERR6_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr6(&mut self) -> H_DESC_LST_ROLLINTR6_W<HCINT6_SPEC> {
        H_DESC_LST_ROLLINTR6_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT6_SPEC;
impl crate::RegisterSpec for HCINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint6::R`](R) reader structure"]
impl crate::Readable for HCINT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint6::W`](W) writer structure"]
impl crate::Writable for HCINT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT6 to value 0"]
impl crate::Resettable for HCINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
