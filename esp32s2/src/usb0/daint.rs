///Register `DAINT` reader
pub type R = crate::R<DAINT_SPEC>;
///Field `INEPINT(0-6)` reader -
pub type INEPINT_R = crate::BitReader;
///Field `OUTEPINT(0-6)` reader -
pub type OUTEPINT_R = crate::BitReader;
impl R {
    ///
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `INEPINT0` field
    #[inline(always)]
    pub fn inepint(&self, n: u8) -> INEPINT_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        INEPINT_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///
    #[inline(always)]
    pub fn inepint_iter(&self) -> impl Iterator<Item = INEPINT_R> + '_ {
        (0..7).map(move |n| INEPINT_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - INEPINT0
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT_R {
        INEPINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - INEPINT1
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT_R {
        INEPINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - INEPINT2
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT_R {
        INEPINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - INEPINT3
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT_R {
        INEPINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - INEPINT4
    #[inline(always)]
    pub fn inepint4(&self) -> INEPINT_R {
        INEPINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - INEPINT5
    #[inline(always)]
    pub fn inepint5(&self) -> INEPINT_R {
        INEPINT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - INEPINT6
    #[inline(always)]
    pub fn inepint6(&self) -> INEPINT_R {
        INEPINT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `OUTEPINT0` field
    #[inline(always)]
    pub fn outepint(&self, n: u8) -> OUTEPINT_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        OUTEPINT_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///
    #[inline(always)]
    pub fn outepint_iter(&self) -> impl Iterator<Item = OUTEPINT_R> + '_ {
        (0..7).map(move |n| OUTEPINT_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - OUTEPINT0
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - OUTEPINT1
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - OUTEPINT2
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OUTEPINT3
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - OUTEPINT4
    #[inline(always)]
    pub fn outepint4(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OUTEPINT5
    #[inline(always)]
    pub fn outepint5(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OUTEPINT6
    #[inline(always)]
    pub fn outepint6(&self) -> OUTEPINT_R {
        OUTEPINT_R::new(((self.bits >> 22) & 1) != 0)
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
            .field("outepint0", &self.outepint0())
            .field("outepint1", &self.outepint1())
            .field("outepint2", &self.outepint2())
            .field("outepint3", &self.outepint3())
            .field("outepint4", &self.outepint4())
            .field("outepint5", &self.outepint5())
            .field("outepint6", &self.outepint6())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`daint::R`](R) reader structure
impl crate::Readable for DAINT_SPEC {}
///`reset()` method sets DAINT to value 0
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
