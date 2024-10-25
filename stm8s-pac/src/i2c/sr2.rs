#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2_SPEC>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<SR2_SPEC>;
#[doc = "Field `BERR` reader - "]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - "]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLO` reader - "]
pub type ARLO_R = crate::BitReader;
#[doc = "Field `ARLO` writer - "]
pub type ARLO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF` reader - "]
pub type AF_R = crate::BitReader;
#[doc = "Field `AF` writer - "]
pub type AF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - "]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - "]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUFH` reader - "]
pub type WUFH_R = crate::BitReader;
#[doc = "Field `WUFH` writer - "]
pub type WUFH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wufh(&self) -> WUFH_R {
        WUFH_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<SR2_SPEC> {
        BERR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn arlo(&mut self) -> ARLO_W<SR2_SPEC> {
        ARLO_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn af(&mut self) -> AF_W<SR2_SPEC> {
        AF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<SR2_SPEC> {
        OVR_W::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn wufh(&mut self) -> WUFH_W<SR2_SPEC> {
        WUFH_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for SR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
