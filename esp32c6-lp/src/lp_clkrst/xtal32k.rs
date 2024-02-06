#[doc = "Register `XTAL32K` reader"]
pub type R = crate::R<XTAL32K_SPEC>;
#[doc = "Register `XTAL32K` writer"]
pub type W = crate::W<XTAL32K_SPEC>;
#[doc = "Field `DRES_XTAL32K` reader - need_des"]
pub type DRES_XTAL32K_R = crate::FieldReader;
#[doc = "Field `DRES_XTAL32K` writer - need_des"]
pub type DRES_XTAL32K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DGM_XTAL32K` reader - need_des"]
pub type DGM_XTAL32K_R = crate::FieldReader;
#[doc = "Field `DGM_XTAL32K` writer - need_des"]
pub type DGM_XTAL32K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBUF_XTAL32K` reader - need_des"]
pub type DBUF_XTAL32K_R = crate::BitReader;
#[doc = "Field `DBUF_XTAL32K` writer - need_des"]
pub type DBUF_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_XTAL32K` reader - need_des"]
pub type DAC_XTAL32K_R = crate::FieldReader;
#[doc = "Field `DAC_XTAL32K` writer - need_des"]
pub type DAC_XTAL32K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn dres_xtal32k(&self) -> DRES_XTAL32K_R {
        DRES_XTAL32K_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    pub fn dgm_xtal32k(&self) -> DGM_XTAL32K_R {
        DGM_XTAL32K_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn dbuf_xtal32k(&self) -> DBUF_XTAL32K_R {
        DBUF_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn dac_xtal32k(&self) -> DAC_XTAL32K_R {
        DAC_XTAL32K_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL32K")
            .field(
                "dres_xtal32k",
                &format_args!("{}", self.dres_xtal32k().bits()),
            )
            .field(
                "dgm_xtal32k",
                &format_args!("{}", self.dgm_xtal32k().bits()),
            )
            .field(
                "dbuf_xtal32k",
                &format_args!("{}", self.dbuf_xtal32k().bit()),
            )
            .field(
                "dac_xtal32k",
                &format_args!("{}", self.dac_xtal32k().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL32K_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dres_xtal32k(&mut self) -> DRES_XTAL32K_W<XTAL32K_SPEC> {
        DRES_XTAL32K_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dgm_xtal32k(&mut self) -> DGM_XTAL32K_W<XTAL32K_SPEC> {
        DGM_XTAL32K_W::new(self, 25)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dbuf_xtal32k(&mut self) -> DBUF_XTAL32K_W<XTAL32K_SPEC> {
        DBUF_XTAL32K_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dac_xtal32k(&mut self) -> DAC_XTAL32K_W<XTAL32K_SPEC> {
        DAC_XTAL32K_W::new(self, 29)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL32K_SPEC;
impl crate::RegisterSpec for XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal32k::R`](R) reader structure"]
impl crate::Readable for XTAL32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal32k::W`](W) writer structure"]
impl crate::Writable for XTAL32K_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTAL32K to value 0x66c0_0000"]
impl crate::Resettable for XTAL32K_SPEC {
    const RESET_VALUE: u32 = 0x66c0_0000;
}
