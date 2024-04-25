#[doc = "Register `INT` reader"]
pub type R = crate::R<INT_SPEC>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INT_SPEC>;
#[doc = "Field `XFERCOMPL` reader - "]
pub type XFERCOMPL_R = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - "]
pub type XFERCOMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTD` reader - "]
pub type CHHLTD_R = crate::BitReader;
#[doc = "Field `CHHLTD` writer - "]
pub type CHHLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - "]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - "]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - "]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - "]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - "]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - "]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - "]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - "]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - "]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - "]
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR` reader - "]
pub type XACTERR_R = crate::BitReader;
#[doc = "Field `XACTERR` writer - "]
pub type XACTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERR` reader - "]
pub type BBLERR_R = crate::BitReader;
#[doc = "Field `BBLERR` writer - "]
pub type BBLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUN` reader - "]
pub type FRMOVRUN_R = crate::BitReader;
#[doc = "Field `FRMOVRUN` writer - "]
pub type FRMOVRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATGLERR` reader - "]
pub type DATATGLERR_R = crate::BitReader;
#[doc = "Field `DATATGLERR` writer - "]
pub type DATATGLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR` reader - "]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAINTR` writer - "]
pub type BNAINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCS_XACT_ERR` reader - "]
pub type XCS_XACT_ERR_R = crate::BitReader;
#[doc = "Field `XCS_XACT_ERR` writer - "]
pub type XCS_XACT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LST_ROLLINTR` reader - "]
pub type DESC_LST_ROLLINTR_R = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLINTR` writer - "]
pub type DESC_LST_ROLLINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chhltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xacterr(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bblerr(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datatglerr(&self) -> DATATGLERR_R {
        DATATGLERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xcs_xact_err(&self) -> XCS_XACT_ERR_R {
        XCS_XACT_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn desc_lst_rollintr(&self) -> DESC_LST_ROLLINTR_R {
        DESC_LST_ROLLINTR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT")
            .field("xfercompl", &format_args!("{}", self.xfercompl().bit()))
            .field("chhltd", &format_args!("{}", self.chhltd().bit()))
            .field("ahberr", &format_args!("{}", self.ahberr().bit()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("nack", &format_args!("{}", self.nack().bit()))
            .field("ack", &format_args!("{}", self.ack().bit()))
            .field("nyet", &format_args!("{}", self.nyet().bit()))
            .field("xacterr", &format_args!("{}", self.xacterr().bit()))
            .field("bblerr", &format_args!("{}", self.bblerr().bit()))
            .field("frmovrun", &format_args!("{}", self.frmovrun().bit()))
            .field("datatglerr", &format_args!("{}", self.datatglerr().bit()))
            .field("bnaintr", &format_args!("{}", self.bnaintr().bit()))
            .field(
                "xcs_xact_err",
                &format_args!("{}", self.xcs_xact_err().bit()),
            )
            .field(
                "desc_lst_rollintr",
                &format_args!("{}", self.desc_lst_rollintr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<INT_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn chhltd(&mut self) -> CHHLTD_W<INT_SPEC> {
        CHHLTD_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<INT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<INT_SPEC> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<INT_SPEC> {
        NACK_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<INT_SPEC> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<INT_SPEC> {
        NYET_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn xacterr(&mut self) -> XACTERR_W<INT_SPEC> {
        XACTERR_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bblerr(&mut self) -> BBLERR_W<INT_SPEC> {
        BBLERR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrun(&mut self) -> FRMOVRUN_W<INT_SPEC> {
        FRMOVRUN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn datatglerr(&mut self) -> DATATGLERR_W<INT_SPEC> {
        DATATGLERR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<INT_SPEC> {
        BNAINTR_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xcs_xact_err(&mut self) -> XCS_XACT_ERR_W<INT_SPEC> {
        XCS_XACT_ERR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn desc_lst_rollintr(&mut self) -> DESC_LST_ROLLINTR_W<INT_SPEC> {
        DESC_LST_ROLLINTR_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
