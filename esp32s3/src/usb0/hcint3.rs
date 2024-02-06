#[doc = "Register `HCINT3` reader"]
pub type R = crate::R<HCINT3_SPEC>;
#[doc = "Register `HCINT3` writer"]
pub type W = crate::W<HCINT3_SPEC>;
#[doc = "Field `H_XFERCOMPL3` reader - "]
pub type H_XFERCOMPL3_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL3` writer - "]
pub type H_XFERCOMPL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTD3` reader - "]
pub type H_CHHLTD3_R = crate::BitReader;
#[doc = "Field `H_CHHLTD3` writer - "]
pub type H_CHHLTD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERR3` reader - "]
pub type H_AHBERR3_R = crate::BitReader;
#[doc = "Field `H_AHBERR3` writer - "]
pub type H_AHBERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALL3` reader - "]
pub type H_STALL3_R = crate::BitReader;
#[doc = "Field `H_STALL3` writer - "]
pub type H_STALL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NACK3` reader - "]
pub type H_NACK3_R = crate::BitReader;
#[doc = "Field `H_NACK3` writer - "]
pub type H_NACK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACK3` reader - "]
pub type H_ACK3_R = crate::BitReader;
#[doc = "Field `H_ACK3` writer - "]
pub type H_ACK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYET3` reader - "]
pub type H_NYET3_R = crate::BitReader;
#[doc = "Field `H_NYET3` writer - "]
pub type H_NYET3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERR3` reader - "]
pub type H_XACTERR3_R = crate::BitReader;
#[doc = "Field `H_XACTERR3` writer - "]
pub type H_XACTERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERR3` reader - "]
pub type H_BBLERR3_R = crate::BitReader;
#[doc = "Field `H_BBLERR3` writer - "]
pub type H_BBLERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUN3` reader - "]
pub type H_FRMOVRUN3_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN3` writer - "]
pub type H_FRMOVRUN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERR3` reader - "]
pub type H_DATATGLERR3_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR3` writer - "]
pub type H_DATATGLERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTR3` reader - "]
pub type H_BNAINTR3_R = crate::BitReader;
#[doc = "Field `H_BNAINTR3` writer - "]
pub type H_BNAINTR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XCS_XACT_ERR3` reader - "]
pub type H_XCS_XACT_ERR3_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR3` writer - "]
pub type H_XCS_XACT_ERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTR3` reader - "]
pub type H_DESC_LST_ROLLINTR3_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR3` writer - "]
pub type H_DESC_LST_ROLLINTR3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl3(&self) -> H_XFERCOMPL3_R {
        H_XFERCOMPL3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd3(&self) -> H_CHHLTD3_R {
        H_CHHLTD3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr3(&self) -> H_AHBERR3_R {
        H_AHBERR3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall3(&self) -> H_STALL3_R {
        H_STALL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack3(&self) -> H_NACK3_R {
        H_NACK3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack3(&self) -> H_ACK3_R {
        H_ACK3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet3(&self) -> H_NYET3_R {
        H_NYET3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr3(&self) -> H_XACTERR3_R {
        H_XACTERR3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr3(&self) -> H_BBLERR3_R {
        H_BBLERR3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun3(&self) -> H_FRMOVRUN3_R {
        H_FRMOVRUN3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr3(&self) -> H_DATATGLERR3_R {
        H_DATATGLERR3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr3(&self) -> H_BNAINTR3_R {
        H_BNAINTR3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err3(&self) -> H_XCS_XACT_ERR3_R {
        H_XCS_XACT_ERR3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr3(&self) -> H_DESC_LST_ROLLINTR3_R {
        H_DESC_LST_ROLLINTR3_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT3")
            .field(
                "h_xfercompl3",
                &format_args!("{}", self.h_xfercompl3().bit()),
            )
            .field("h_chhltd3", &format_args!("{}", self.h_chhltd3().bit()))
            .field("h_ahberr3", &format_args!("{}", self.h_ahberr3().bit()))
            .field("h_stall3", &format_args!("{}", self.h_stall3().bit()))
            .field("h_nack3", &format_args!("{}", self.h_nack3().bit()))
            .field("h_ack3", &format_args!("{}", self.h_ack3().bit()))
            .field("h_nyet3", &format_args!("{}", self.h_nyet3().bit()))
            .field("h_xacterr3", &format_args!("{}", self.h_xacterr3().bit()))
            .field("h_bblerr3", &format_args!("{}", self.h_bblerr3().bit()))
            .field("h_frmovrun3", &format_args!("{}", self.h_frmovrun3().bit()))
            .field(
                "h_datatglerr3",
                &format_args!("{}", self.h_datatglerr3().bit()),
            )
            .field("h_bnaintr3", &format_args!("{}", self.h_bnaintr3().bit()))
            .field(
                "h_xcs_xact_err3",
                &format_args!("{}", self.h_xcs_xact_err3().bit()),
            )
            .field(
                "h_desc_lst_rollintr3",
                &format_args!("{}", self.h_desc_lst_rollintr3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl3(&mut self) -> H_XFERCOMPL3_W<HCINT3_SPEC> {
        H_XFERCOMPL3_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd3(&mut self) -> H_CHHLTD3_W<HCINT3_SPEC> {
        H_CHHLTD3_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr3(&mut self) -> H_AHBERR3_W<HCINT3_SPEC> {
        H_AHBERR3_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall3(&mut self) -> H_STALL3_W<HCINT3_SPEC> {
        H_STALL3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack3(&mut self) -> H_NACK3_W<HCINT3_SPEC> {
        H_NACK3_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack3(&mut self) -> H_ACK3_W<HCINT3_SPEC> {
        H_ACK3_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet3(&mut self) -> H_NYET3_W<HCINT3_SPEC> {
        H_NYET3_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr3(&mut self) -> H_XACTERR3_W<HCINT3_SPEC> {
        H_XACTERR3_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr3(&mut self) -> H_BBLERR3_W<HCINT3_SPEC> {
        H_BBLERR3_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun3(&mut self) -> H_FRMOVRUN3_W<HCINT3_SPEC> {
        H_FRMOVRUN3_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr3(&mut self) -> H_DATATGLERR3_W<HCINT3_SPEC> {
        H_DATATGLERR3_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr3(&mut self) -> H_BNAINTR3_W<HCINT3_SPEC> {
        H_BNAINTR3_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err3(&mut self) -> H_XCS_XACT_ERR3_W<HCINT3_SPEC> {
        H_XCS_XACT_ERR3_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr3(&mut self) -> H_DESC_LST_ROLLINTR3_W<HCINT3_SPEC> {
        H_DESC_LST_ROLLINTR3_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT3_SPEC;
impl crate::RegisterSpec for HCINT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint3::R`](R) reader structure"]
impl crate::Readable for HCINT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint3::W`](W) writer structure"]
impl crate::Writable for HCINT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINT3 to value 0"]
impl crate::Resettable for HCINT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
