#[doc = "Register `INT_ST_CRC_FRAME_FATAL` reader"]
pub type R = crate::R<INT_ST_CRC_FRAME_FATAL_SPEC>;
#[doc = "Field `ST_ERR_FRAME_DATA_VC0` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC0_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC1` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC1_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC2` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC2_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC3` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC3_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC4` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC4_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC5` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC5_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC6` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC6_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC7` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC7_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC8` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC8_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC9` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC9_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC10` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC10_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC11` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC11_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC12` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC12_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC13` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC13_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC14` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC14_R = crate::BitReader;
#[doc = "Field `ST_ERR_FRAME_DATA_VC15` reader - NA"]
pub type ST_ERR_FRAME_DATA_VC15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc0(&self) -> ST_ERR_FRAME_DATA_VC0_R {
        ST_ERR_FRAME_DATA_VC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc1(&self) -> ST_ERR_FRAME_DATA_VC1_R {
        ST_ERR_FRAME_DATA_VC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc2(&self) -> ST_ERR_FRAME_DATA_VC2_R {
        ST_ERR_FRAME_DATA_VC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc3(&self) -> ST_ERR_FRAME_DATA_VC3_R {
        ST_ERR_FRAME_DATA_VC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc4(&self) -> ST_ERR_FRAME_DATA_VC4_R {
        ST_ERR_FRAME_DATA_VC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc5(&self) -> ST_ERR_FRAME_DATA_VC5_R {
        ST_ERR_FRAME_DATA_VC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc6(&self) -> ST_ERR_FRAME_DATA_VC6_R {
        ST_ERR_FRAME_DATA_VC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc7(&self) -> ST_ERR_FRAME_DATA_VC7_R {
        ST_ERR_FRAME_DATA_VC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc8(&self) -> ST_ERR_FRAME_DATA_VC8_R {
        ST_ERR_FRAME_DATA_VC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc9(&self) -> ST_ERR_FRAME_DATA_VC9_R {
        ST_ERR_FRAME_DATA_VC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc10(&self) -> ST_ERR_FRAME_DATA_VC10_R {
        ST_ERR_FRAME_DATA_VC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc11(&self) -> ST_ERR_FRAME_DATA_VC11_R {
        ST_ERR_FRAME_DATA_VC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc12(&self) -> ST_ERR_FRAME_DATA_VC12_R {
        ST_ERR_FRAME_DATA_VC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc13(&self) -> ST_ERR_FRAME_DATA_VC13_R {
        ST_ERR_FRAME_DATA_VC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc14(&self) -> ST_ERR_FRAME_DATA_VC14_R {
        ST_ERR_FRAME_DATA_VC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn st_err_frame_data_vc15(&self) -> ST_ERR_FRAME_DATA_VC15_R {
        ST_ERR_FRAME_DATA_VC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_CRC_FRAME_FATAL")
            .field(
                "st_err_frame_data_vc0",
                &format_args!("{}", self.st_err_frame_data_vc0().bit()),
            )
            .field(
                "st_err_frame_data_vc1",
                &format_args!("{}", self.st_err_frame_data_vc1().bit()),
            )
            .field(
                "st_err_frame_data_vc2",
                &format_args!("{}", self.st_err_frame_data_vc2().bit()),
            )
            .field(
                "st_err_frame_data_vc3",
                &format_args!("{}", self.st_err_frame_data_vc3().bit()),
            )
            .field(
                "st_err_frame_data_vc4",
                &format_args!("{}", self.st_err_frame_data_vc4().bit()),
            )
            .field(
                "st_err_frame_data_vc5",
                &format_args!("{}", self.st_err_frame_data_vc5().bit()),
            )
            .field(
                "st_err_frame_data_vc6",
                &format_args!("{}", self.st_err_frame_data_vc6().bit()),
            )
            .field(
                "st_err_frame_data_vc7",
                &format_args!("{}", self.st_err_frame_data_vc7().bit()),
            )
            .field(
                "st_err_frame_data_vc8",
                &format_args!("{}", self.st_err_frame_data_vc8().bit()),
            )
            .field(
                "st_err_frame_data_vc9",
                &format_args!("{}", self.st_err_frame_data_vc9().bit()),
            )
            .field(
                "st_err_frame_data_vc10",
                &format_args!("{}", self.st_err_frame_data_vc10().bit()),
            )
            .field(
                "st_err_frame_data_vc11",
                &format_args!("{}", self.st_err_frame_data_vc11().bit()),
            )
            .field(
                "st_err_frame_data_vc12",
                &format_args!("{}", self.st_err_frame_data_vc12().bit()),
            )
            .field(
                "st_err_frame_data_vc13",
                &format_args!("{}", self.st_err_frame_data_vc13().bit()),
            )
            .field(
                "st_err_frame_data_vc14",
                &format_args!("{}", self.st_err_frame_data_vc14().bit()),
            )
            .field(
                "st_err_frame_data_vc15",
                &format_args!("{}", self.st_err_frame_data_vc15().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_CRC_FRAME_FATAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_crc_frame_fatal::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_CRC_FRAME_FATAL_SPEC;
impl crate::RegisterSpec for INT_ST_CRC_FRAME_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_crc_frame_fatal::R`](R) reader structure"]
impl crate::Readable for INT_ST_CRC_FRAME_FATAL_SPEC {}
#[doc = "`reset()` method sets INT_ST_CRC_FRAME_FATAL to value 0"]
impl crate::Resettable for INT_ST_CRC_FRAME_FATAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
