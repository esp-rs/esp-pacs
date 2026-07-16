#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINT_SPEC>;
#[doc = "Field `INEPINT0` reader - IN Endpoint 0 Interrupt Bit"]
pub type INEPINT0_R = crate::BitReader;
#[doc = "Field `INEPINT1` reader - IN Endpoint 1 Interrupt Bit"]
pub type INEPINT1_R = crate::BitReader;
#[doc = "Field `INEPINT2` reader - IN Endpoint 2 Interrupt Bit"]
pub type INEPINT2_R = crate::BitReader;
#[doc = "Field `INEPINT3` reader - IN Endpoint 3 Interrupt Bit"]
pub type INEPINT3_R = crate::BitReader;
#[doc = "Field `INEPINT4` reader - IN Endpoint 4 Interrupt Bit"]
pub type INEPINT4_R = crate::BitReader;
#[doc = "Field `INEPINT5` reader - IN Endpoint 5 Interrupt Bit"]
pub type INEPINT5_R = crate::BitReader;
#[doc = "Field `INEPINT6` reader - IN Endpoint 6 Interrupt Bit"]
pub type INEPINT6_R = crate::BitReader;
#[doc = "Field `INEPINT7` reader - IN Endpoint 7 Interrupt Bit"]
pub type INEPINT7_R = crate::BitReader;
#[doc = "Field `INEPINT8` reader - IN Endpoint 8 Interrupt Bit"]
pub type INEPINT8_R = crate::BitReader;
#[doc = "Field `INEPINT9` reader - IN Endpoint 9 Interrupt Bit"]
pub type INEPINT9_R = crate::BitReader;
#[doc = "Field `INEPINT10` reader - IN Endpoint 10 Interrupt Bit"]
pub type INEPINT10_R = crate::BitReader;
#[doc = "Field `INEPINT11` reader - IN Endpoint 11 Interrupt Bit"]
pub type INEPINT11_R = crate::BitReader;
#[doc = "Field `INEPINT12` reader - IN Endpoint 12 Interrupt Bit"]
pub type INEPINT12_R = crate::BitReader;
#[doc = "Field `INEPINT13` reader - IN Endpoint 13 Interrupt Bit"]
pub type INEPINT13_R = crate::BitReader;
#[doc = "Field `INEPINT14` reader - IN Endpoint 14 Interrupt Bit"]
pub type INEPINT14_R = crate::BitReader;
#[doc = "Field `INEPINT15` reader - IN Endpoint 15 Interrupt Bit"]
pub type INEPINT15_R = crate::BitReader;
#[doc = "Field `OUTEPINT0` reader - OUT Endpoint 0 Interrupt Bit"]
pub type OUTEPINT0_R = crate::BitReader;
#[doc = "Field `OUTEPINT1` reader - OUT Endpoint 1 Interrupt Bit"]
pub type OUTEPINT1_R = crate::BitReader;
#[doc = "Field `OUTEPINT2` reader - OUT Endpoint 2 Interrupt Bit"]
pub type OUTEPINT2_R = crate::BitReader;
#[doc = "Field `OUTEPINT3` reader - OUT Endpoint 3 Interrupt Bit"]
pub type OUTEPINT3_R = crate::BitReader;
#[doc = "Field `OUTEPINT4` reader - OUT Endpoint 4 Interrupt Bit"]
pub type OUTEPINT4_R = crate::BitReader;
#[doc = "Field `OUTEPINT5` reader - OUT Endpoint 5 Interrupt Bit"]
pub type OUTEPINT5_R = crate::BitReader;
#[doc = "Field `OUTEPINT6` reader - OUT Endpoint 6 Interrupt Bit"]
pub type OUTEPINT6_R = crate::BitReader;
#[doc = "Field `OUTEPINT7` reader - OUT Endpoint 7 Interrupt Bit"]
pub type OUTEPINT7_R = crate::BitReader;
#[doc = "Field `OUTEPINT8` reader - OUT Endpoint 8 Interrupt Bit"]
pub type OUTEPINT8_R = crate::BitReader;
#[doc = "Field `OUTEPINT9` reader - OUT Endpoint 9 Interrupt Bit"]
pub type OUTEPINT9_R = crate::BitReader;
#[doc = "Field `OUTEPINT10` reader - OUT Endpoint 10 Interrupt Bit"]
pub type OUTEPINT10_R = crate::BitReader;
#[doc = "Field `OUTEPINT11` reader - OUT Endpoint 11 Interrupt Bit"]
pub type OUTEPINT11_R = crate::BitReader;
#[doc = "Field `OUTEPINT12` reader - OUT Endpoint 12 Interrupt Bit"]
pub type OUTEPINT12_R = crate::BitReader;
#[doc = "Field `OUTEPINT13` reader - OUT Endpoint 13 Interrupt Bit"]
pub type OUTEPINT13_R = crate::BitReader;
#[doc = "Field `OUTEPINT14` reader - OUT Endpoint 14 Interrupt Bit"]
pub type OUTEPINT14_R = crate::BitReader;
#[doc = "Field `OUTEPINT15` reader - OUT Endpoint 15 Interrupt Bit"]
pub type OUTEPINT15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT0_R {
        INEPINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT1_R {
        INEPINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT2_R {
        INEPINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT3_R {
        INEPINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Endpoint 4 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint4(&self) -> INEPINT4_R {
        INEPINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Endpoint 5 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint5(&self) -> INEPINT5_R {
        INEPINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint 6 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint6(&self) -> INEPINT6_R {
        INEPINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IN Endpoint 7 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint7(&self) -> INEPINT7_R {
        INEPINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IN Endpoint 8 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint8(&self) -> INEPINT8_R {
        INEPINT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IN Endpoint 9 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint9(&self) -> INEPINT9_R {
        INEPINT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IN Endpoint 10 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint10(&self) -> INEPINT10_R {
        INEPINT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IN Endpoint 11 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint11(&self) -> INEPINT11_R {
        INEPINT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IN Endpoint 12 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint12(&self) -> INEPINT12_R {
        INEPINT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IN Endpoint 13 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint13(&self) -> INEPINT13_R {
        INEPINT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IN Endpoint 14 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint14(&self) -> INEPINT14_R {
        INEPINT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IN Endpoint 15 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint15(&self) -> INEPINT15_R {
        INEPINT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT0_R {
        OUTEPINT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT1_R {
        OUTEPINT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT2_R {
        OUTEPINT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT3_R {
        OUTEPINT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OUT Endpoint 4 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint4(&self) -> OUTEPINT4_R {
        OUTEPINT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OUT Endpoint 5 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint5(&self) -> OUTEPINT5_R {
        OUTEPINT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OUT Endpoint 6 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint6(&self) -> OUTEPINT6_R {
        OUTEPINT6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OUT Endpoint 7 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint7(&self) -> OUTEPINT7_R {
        OUTEPINT7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OUT Endpoint 8 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint8(&self) -> OUTEPINT8_R {
        OUTEPINT8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OUT Endpoint 9 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint9(&self) -> OUTEPINT9_R {
        OUTEPINT9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OUT Endpoint 10 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint10(&self) -> OUTEPINT10_R {
        OUTEPINT10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OUT Endpoint 11 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint11(&self) -> OUTEPINT11_R {
        OUTEPINT11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - OUT Endpoint 12 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint12(&self) -> OUTEPINT12_R {
        OUTEPINT12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OUT Endpoint 13 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint13(&self) -> OUTEPINT13_R {
        OUTEPINT13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OUT Endpoint 14 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint14(&self) -> OUTEPINT14_R {
        OUTEPINT14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OUT Endpoint 15 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint15(&self) -> OUTEPINT15_R {
        OUTEPINT15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINT")
            .field("inepint0", &self.inepint0())
            .field("inepint1", &self.inepint1())
            .field("inepint2", &self.inepint2())
            .field("inepint3", &self.inepint3())
            .field("inepint4", &self.inepint4())
            .field("inepint5", &self.inepint5())
            .field("inepint6", &self.inepint6())
            .field("inepint7", &self.inepint7())
            .field("inepint8", &self.inepint8())
            .field("inepint9", &self.inepint9())
            .field("inepint10", &self.inepint10())
            .field("inepint11", &self.inepint11())
            .field("inepint12", &self.inepint12())
            .field("inepint13", &self.inepint13())
            .field("inepint14", &self.inepint14())
            .field("inepint15", &self.inepint15())
            .field("outepint0", &self.outepint0())
            .field("outepint1", &self.outepint1())
            .field("outepint2", &self.outepint2())
            .field("outepint3", &self.outepint3())
            .field("outepint4", &self.outepint4())
            .field("outepint5", &self.outepint5())
            .field("outepint6", &self.outepint6())
            .field("outepint7", &self.outepint7())
            .field("outepint8", &self.outepint8())
            .field("outepint9", &self.outepint9())
            .field("outepint10", &self.outepint10())
            .field("outepint11", &self.outepint11())
            .field("outepint12", &self.outepint12())
            .field("outepint13", &self.outepint13())
            .field("outepint14", &self.outepint14())
            .field("outepint15", &self.outepint15())
            .finish()
    }
}
#[doc = "When a significant event occurs on an endpoint, a Device All Endpoints Interrupt register interrupts the application using the Device OUT Endpoints Interrupt bit or Device IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively). This is shown in Figure 5-2. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn).\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DAINT_SPEC {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {}
