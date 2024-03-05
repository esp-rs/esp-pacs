#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RMTWKUPSIG` reader - "]
pub type RMTWKUPSIG_R = crate::BitReader;
#[doc = "Field `RMTWKUPSIG` writer - "]
pub type RMTWKUPSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTDISCON` reader - "]
pub type SFTDISCON_R = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - "]
pub type SFTDISCON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNPINNAKSTS` reader - "]
pub type GNPINNAKSTS_R = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - "]
pub type GOUTNAKSTS_R = crate::BitReader;
#[doc = "Field `TSTCTL` reader - "]
pub type TSTCTL_R = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - "]
pub type TSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGNPINNAK` writer - "]
pub type SGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPINNAK` writer - "]
pub type CGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNAK` writer - "]
pub type SGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNAK` writer - "]
pub type CGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRONPRGDONE` reader - "]
pub type PWRONPRGDONE_R = crate::BitReader;
#[doc = "Field `PWRONPRGDONE` writer - "]
pub type PWRONPRGDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMC` reader - "]
pub type GMC_R = crate::FieldReader;
#[doc = "Field `GMC` writer - "]
pub type GMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IGNRFRMNUM` reader - "]
pub type IGNRFRMNUM_R = crate::BitReader;
#[doc = "Field `IGNRFRMNUM` writer - "]
pub type IGNRFRMNUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKONBBLE` reader - "]
pub type NAKONBBLE_R = crate::BitReader;
#[doc = "Field `NAKONBBLE` writer - "]
pub type NAKONBBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCOUNTONBNA` reader - "]
pub type ENCOUNTONBNA_R = crate::BitReader;
#[doc = "Field `ENCOUNTONBNA` writer - "]
pub type ENCOUNTONBNA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEPBESLREJECT` reader - "]
pub type DEEPSLEEPBESLREJECT_R = crate::BitReader;
#[doc = "Field `DEEPSLEEPBESLREJECT` writer - "]
pub type DEEPSLEEPBESLREJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn encountonbna(&self) -> ENCOUNTONBNA_R {
        ENCOUNTONBNA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn deepsleepbeslreject(&self) -> DEEPSLEEPBESLREJECT_R {
        DEEPSLEEPBESLREJECT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rmtwkupsig", &format_args!("{}", self.rmtwkupsig().bit()))
            .field("sftdiscon", &format_args!("{}", self.sftdiscon().bit()))
            .field("gnpinnaksts", &format_args!("{}", self.gnpinnaksts().bit()))
            .field("goutnaksts", &format_args!("{}", self.goutnaksts().bit()))
            .field("tstctl", &format_args!("{}", self.tstctl().bits()))
            .field(
                "pwronprgdone",
                &format_args!("{}", self.pwronprgdone().bit()),
            )
            .field("gmc", &format_args!("{}", self.gmc().bits()))
            .field("ignrfrmnum", &format_args!("{}", self.ignrfrmnum().bit()))
            .field("nakonbble", &format_args!("{}", self.nakonbble().bit()))
            .field(
                "encountonbna",
                &format_args!("{}", self.encountonbna().bit()),
            )
            .field(
                "deepsleepbeslreject",
                &format_args!("{}", self.deepsleepbeslreject().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W<DCTL_SPEC> {
        RMTWKUPSIG_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W<DCTL_SPEC> {
        SFTDISCON_W::new(self, 1)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TSTCTL_W<DCTL_SPEC> {
        TSTCTL_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W<DCTL_SPEC> {
        SGNPINNAK_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W<DCTL_SPEC> {
        CGNPINNAK_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<DCTL_SPEC> {
        SGOUTNAK_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<DCTL_SPEC> {
        CGOUTNAK_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W<DCTL_SPEC> {
        PWRONPRGDONE_W::new(self, 11)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    #[must_use]
    pub fn gmc(&mut self) -> GMC_W<DCTL_SPEC> {
        GMC_W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W<DCTL_SPEC> {
        IGNRFRMNUM_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W<DCTL_SPEC> {
        NAKONBBLE_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn encountonbna(&mut self) -> ENCOUNTONBNA_W<DCTL_SPEC> {
        ENCOUNTONBNA_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleepbeslreject(&mut self) -> DEEPSLEEPBESLREJECT_W<DCTL_SPEC> {
        DEEPSLEEPBESLREJECT_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0x2000"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
