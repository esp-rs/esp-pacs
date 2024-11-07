#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFG_SPEC>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFG_SPEC>;
#[doc = "Field `GLBLLNTRMSK` reader - "]
pub type GLBLLNTRMSK_R = crate::BitReader;
#[doc = "Field `GLBLLNTRMSK` writer - "]
pub type GLBLLNTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - "]
pub type HBSTLEN_R = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - "]
pub type HBSTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - "]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - "]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPLVL` reader - "]
pub type NPTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - "]
pub type NPTXFEMPLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPLVL` reader - "]
pub type PTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `PTXFEMPLVL` writer - "]
pub type PTXFEMPLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMMEMSUPP` reader - "]
pub type REMMEMSUPP_R = crate::BitReader;
#[doc = "Field `REMMEMSUPP` writer - "]
pub type REMMEMSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTIALLDMAWRIT` reader - "]
pub type NOTIALLDMAWRIT_R = crate::BitReader;
#[doc = "Field `NOTIALLDMAWRIT` writer - "]
pub type NOTIALLDMAWRIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSINGLE` reader - "]
pub type AHBSINGLE_R = crate::BitReader;
#[doc = "Field `AHBSINGLE` writer - "]
pub type AHBSINGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVDESCENDIANESS` reader - "]
pub type INVDESCENDIANESS_R = crate::BitReader;
#[doc = "Field `INVDESCENDIANESS` writer - "]
pub type INVDESCENDIANESS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn glbllntrmsk(&self) -> GLBLLNTRMSK_R {
        GLBLLNTRMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> REMMEMSUPP_R {
        REMMEMSUPP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRIT_R {
        NOTIALLDMAWRIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn invdescendianess(&self) -> INVDESCENDIANESS_R {
        INVDESCENDIANESS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("glbllntrmsk", &self.glbllntrmsk())
            .field("hbstlen", &self.hbstlen())
            .field("dmaen", &self.dmaen())
            .field("nptxfemplvl", &self.nptxfemplvl())
            .field("ptxfemplvl", &self.ptxfemplvl())
            .field("remmemsupp", &self.remmemsupp())
            .field("notialldmawrit", &self.notialldmawrit())
            .field("ahbsingle", &self.ahbsingle())
            .field("invdescendianess", &self.invdescendianess())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn glbllntrmsk(&mut self) -> GLBLLNTRMSK_W<GAHBCFG_SPEC> {
        GLBLLNTRMSK_W::new(self, 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<GAHBCFG_SPEC> {
        HBSTLEN_W::new(self, 1)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFG_SPEC> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W<GAHBCFG_SPEC> {
        NPTXFEMPLVL_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptxfemplvl(&mut self) -> PTXFEMPLVL_W<GAHBCFG_SPEC> {
        PTXFEMPLVL_W::new(self, 8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn remmemsupp(&mut self) -> REMMEMSUPP_W<GAHBCFG_SPEC> {
        REMMEMSUPP_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn notialldmawrit(&mut self) -> NOTIALLDMAWRIT_W<GAHBCFG_SPEC> {
        NOTIALLDMAWRIT_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W<GAHBCFG_SPEC> {
        AHBSINGLE_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn invdescendianess(&mut self) -> INVDESCENDIANESS_W<GAHBCFG_SPEC> {
        INVDESCENDIANESS_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
